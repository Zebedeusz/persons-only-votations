use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
};
use serde_json::Value;
use std::sync::Arc;

use super::handlers::{polls::ApiError, polls::PollHandler, votes::VoteHandler};
use crate::services::{poll_service::PollService, vote_service::VoteService};

pub struct AppState {
    pub poll_service: Arc<PollService>,
    pub vote_service: Arc<VoteService>,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            ApiError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            ApiError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg),
            ApiError::InternalError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            ApiError::NotFound => (StatusCode::NOT_FOUND, "Not found".to_string()),
        };

        let body = serde_json::json!({ "error": message });
        (status, Json(body)).into_response()
    }
}

fn payload_to_string(payload: &Value) -> Result<String, ApiError> {
    serde_json::to_string(payload).map_err(|e| ApiError::BadRequest(format!("Invalid JSON: {e}")))
}

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/polls", get(get_all_polls))
        .route("/polls", post(create_poll))
        .route("/polls/:poll_id", get(get_poll))
        .route("/polls/:poll_id/commit", post(commit_vote))
        .route("/polls/:poll_id/reveal", post(reveal_vote))
        .route("/polls/:poll_id/votes", get(get_poll_votes))
        .route("/polls/:poll_id/finalize", post(finalize_poll))
        // Encrypted voting endpoints
        .route("/polls/:poll_id/cast-encrypted", post(cast_encrypted_vote))
        .route("/polls/:poll_id/encrypted-votes", get(get_encrypted_votes))
        .route(
            "/polls/:poll_id/homomorphic-tally",
            get(get_homomorphic_tally),
        )
        .with_state(Arc::new(state))
}

async fn health_check() -> impl IntoResponse {
    Json(serde_json::json!({
        "status": "healthy",
        "service": "persons-only-votations"
    }))
}

async fn create_poll(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Value>,
) -> Result<impl IntoResponse, ApiError> {
    let handler = PollHandler::new(state.poll_service.clone());
    let json_body = payload_to_string(&payload)?;
    let response = handler.create_poll(&json_body).await?;
    Ok((StatusCode::CREATED, response))
}

async fn get_all_polls(State(state): State<Arc<AppState>>) -> Result<impl IntoResponse, ApiError> {
    let handler = PollHandler::new(state.poll_service.clone());
    let response = handler.get_all_polls().await?;
    Ok(response)
}

async fn get_poll(
    Path(poll_id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, ApiError> {
    let poll_id: u32 = poll_id
        .parse()
        .map_err(|_| ApiError::BadRequest("Invalid poll ID format".to_string()))?;

    let handler = PollHandler::new(state.poll_service.clone());
    let response = handler.get_poll(poll_id).await?;
    Ok(response)
}

async fn commit_vote(
    Path(poll_id): Path<String>,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Value>,
) -> Result<impl IntoResponse, ApiError> {
    let handler = VoteHandler::new(state.vote_service.clone());
    let json_body = payload_to_string(&payload)?;
    let response = handler.commit_vote(&poll_id, &json_body).await?;
    Ok((StatusCode::CREATED, response))
}

async fn reveal_vote(
    Path(poll_id): Path<String>,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Value>,
) -> Result<impl IntoResponse, ApiError> {
    let handler = VoteHandler::new(state.vote_service.clone());
    let json_body = payload_to_string(&payload)?;
    let response = handler.reveal_vote(&poll_id, &json_body).await?;
    Ok((StatusCode::OK, response))
}

async fn finalize_poll(
    Path(poll_id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, ApiError> {
    let handler = VoteHandler::new(state.vote_service.clone());
    let response = handler.finalize_poll(&poll_id).await?;
    Ok((StatusCode::OK, response))
}

async fn get_poll_votes(
    Path(poll_id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, ApiError> {
    let handler = VoteHandler::new(state.vote_service.clone());
    let response = handler.get_poll_votes(&poll_id).await?;
    Ok(response)
}

// Phase 2: Encrypted voting endpoints
async fn cast_encrypted_vote(
    Path(poll_id): Path<String>,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Value>,
) -> Result<impl IntoResponse, ApiError> {
    let handler = VoteHandler::new(state.vote_service.clone());
    let json_body = payload_to_string(&payload)?;
    let response = handler.cast_encrypted_vote(&poll_id, &json_body).await?;
    Ok((StatusCode::CREATED, response))
}

async fn get_encrypted_votes(
    Path(poll_id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, ApiError> {
    let handler = VoteHandler::new(state.vote_service.clone());
    let response = handler.get_encrypted_votes(&poll_id).await?;
    Ok(response)
}

async fn get_homomorphic_tally(
    Path(poll_id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, ApiError> {
    let handler = VoteHandler::new(state.vote_service.clone());
    let response = handler.get_homomorphic_tally(&poll_id).await?;
    Ok(response)
}
