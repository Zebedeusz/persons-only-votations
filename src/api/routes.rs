use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
};
use serde_json::{Value, json};
use std::sync::Arc;
use thiserror::Error;

use crate::services::poll_service::{CreatePollResponse, PollService, PollServiceError};
use crate::services::vote_service::{VoteService, VoteServiceError};
use crate::storage::models::poll::{CreatePollRequest, Poll};
use crate::storage::models::vote::{
    CastEncryptedVoteRequest, CastVoteResponse, CommitVoteRequest, RevealVoteRequest,
};

pub struct AppState {
    pub poll_service: PollService,
    pub vote_service: VoteService,
}

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Bad request: {0}")]
    BadRequest(String),
    #[error("Unauthorized: {0}")]
    Unauthorized(String),
    #[error("Internal server error: {0}")]
    InternalError(String),
    #[error("Not found")]
    NotFound,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            ApiError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            ApiError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg),
            ApiError::InternalError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            ApiError::NotFound => (StatusCode::NOT_FOUND, "Not found".to_string()),
        };
        (status, Json(json!({ "error": message }))).into_response()
    }
}

impl From<PollServiceError> for ApiError {
    fn from(err: PollServiceError) -> Self {
        match err {
            PollServiceError::InvalidData(msg) => ApiError::BadRequest(msg),
            PollServiceError::AuthorizationFailed(msg) => ApiError::Unauthorized(msg),
            PollServiceError::ChainError(e) => ApiError::InternalError(format!("Chain error: {e}")),
            PollServiceError::RepositoryError(e) => {
                ApiError::InternalError(format!("Storage error: {e}"))
            }
        }
    }
}

impl From<VoteServiceError> for ApiError {
    fn from(err: VoteServiceError) -> Self {
        use VoteServiceError::*;
        match err {
            InvalidData(msg) => ApiError::BadRequest(msg),
            AuthorizationFailed(msg) => ApiError::Unauthorized(msg),
            PollNotFound(_) | VoteNotFound => ApiError::NotFound,
            PollNotActive | DuplicateVote | InvalidCommitment | VoteAlreadyRevealed
            | PollNotInRevealPhase => ApiError::BadRequest(err.to_string()),
            RingProofError(e) => ApiError::BadRequest(format!("Ring proof validation failed: {e}")),
            ChainError(e) => ApiError::InternalError(format!("Chain error: {e}")),
            VoteRepositoryError(e) => ApiError::InternalError(format!("Vote storage error: {e}")),
            PollRepositoryError(e) => ApiError::InternalError(format!("Poll storage error: {e}")),
            HomomorphicError(e) => {
                ApiError::InternalError(format!("Homomorphic encryption error: {e}"))
            }
        }
    }
}

type ApiResult<T> = Result<T, ApiError>;
type App = State<Arc<AppState>>;

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/polls", get(get_all_polls).post(create_poll))
        .route("/polls/:poll_id", get(get_poll))
        .route("/polls/:poll_id/commit", post(commit_vote))
        .route("/polls/:poll_id/reveal", post(reveal_vote))
        .route("/polls/:poll_id/votes", get(get_poll_votes))
        .route("/polls/:poll_id/finalize", post(finalize_poll))
        .route("/encryption-key", get(get_encryption_key))
        .route("/polls/:poll_id/cast-encrypted", post(cast_encrypted_vote))
        .route("/polls/:poll_id/encrypted-votes", get(get_encrypted_votes))
        .route(
            "/polls/:poll_id/homomorphic-tally",
            get(get_homomorphic_tally),
        )
        .with_state(Arc::new(state))
}

async fn health_check() -> Json<Value> {
    Json(json!({ "status": "healthy", "service": "persons-only-votations" }))
}

async fn create_poll(
    State(state): App,
    Json(request): Json<CreatePollRequest>,
) -> ApiResult<(StatusCode, Json<CreatePollResponse>)> {
    let response = state.poll_service.create_poll(request).await?;
    Ok((StatusCode::CREATED, Json(response)))
}

async fn get_all_polls(State(state): App) -> ApiResult<Json<Vec<Poll>>> {
    Ok(Json(state.poll_service.get_all_polls().await?))
}

async fn get_poll(Path(poll_id): Path<u32>, State(state): App) -> ApiResult<Json<Poll>> {
    let poll = state.poll_service.get_poll(poll_id).await?;
    poll.map(Json).ok_or(ApiError::NotFound)
}

async fn commit_vote(
    Path(poll_id): Path<u32>,
    State(state): App,
    Json(mut request): Json<CommitVoteRequest>,
) -> ApiResult<(StatusCode, Json<CastVoteResponse>)> {
    request.poll_id = poll_id;
    let response = state.vote_service.commit_vote(request).await?;
    Ok((StatusCode::CREATED, Json(response)))
}

async fn reveal_vote(
    Path(poll_id): Path<u32>,
    State(state): App,
    Json(mut request): Json<RevealVoteRequest>,
) -> ApiResult<Json<CastVoteResponse>> {
    request.poll_id = poll_id;
    Ok(Json(state.vote_service.reveal_vote(request).await?))
}

async fn finalize_poll(Path(poll_id): Path<u32>, State(state): App) -> ApiResult<Json<Value>> {
    let (yay_votes, nay_votes) = state.vote_service.finalize_poll_results(poll_id).await?;
    Ok(Json(json!({
        "poll_id": poll_id,
        "yay_votes": yay_votes,
        "nay_votes": nay_votes,
        "status": "finalized"
    })))
}

async fn get_poll_votes(Path(poll_id): Path<u32>, State(state): App) -> ApiResult<Json<Value>> {
    let (yay_votes, nay_votes) = state.vote_service.get_vote_counts(poll_id).await?;
    Ok(Json(json!({
        "poll_id": poll_id,
        "yay_votes": yay_votes,
        "nay_votes": nay_votes
    })))
}

async fn cast_encrypted_vote(
    Path(poll_id): Path<u32>,
    State(state): App,
    Json(mut request): Json<CastEncryptedVoteRequest>,
) -> ApiResult<(StatusCode, Json<CastVoteResponse>)> {
    request.poll_id = poll_id;
    let response = state.vote_service.cast_encrypted_vote(request).await?;
    Ok((StatusCode::CREATED, Json(response)))
}

async fn get_encrypted_votes(
    Path(poll_id): Path<u32>,
    State(state): App,
) -> ApiResult<Json<Value>> {
    let encrypted_votes = state.vote_service.get_poll_encrypted_votes(poll_id).await?;
    Ok(Json(json!({
        "poll_id": poll_id,
        "encrypted_votes": encrypted_votes
    })))
}

async fn get_homomorphic_tally(
    Path(poll_id): Path<u32>,
    State(state): App,
) -> ApiResult<Json<Value>> {
    let (yay, nay) = state
        .vote_service
        .compute_homomorphic_tally(poll_id)
        .await?;
    Ok(Json(json!({
        "poll_id": poll_id,
        "total_yay_votes": yay,
        "total_nay_votes": nay,
        "tally_type": "homomorphic"
    })))
}

async fn get_encryption_key(State(state): App) -> ApiResult<Json<Value>> {
    let public_key = state.vote_service.get_public_encryption_key()?;
    Ok(Json(json!({
        "public_key": public_key,
        "key_type": "tfhe_compact_public_key"
    })))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vote_service_errors_map_to_http_classes() {
        let auth: ApiError = VoteServiceError::AuthorizationFailed("test".to_string()).into();
        assert!(matches!(auth, ApiError::Unauthorized(_)));

        let missing: ApiError = VoteServiceError::PollNotFound(123).into();
        assert!(matches!(missing, ApiError::NotFound));

        let closed: ApiError = VoteServiceError::PollNotActive.into();
        assert!(matches!(closed, ApiError::BadRequest(_)));
    }
}
