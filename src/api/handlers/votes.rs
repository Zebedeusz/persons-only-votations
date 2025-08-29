use super::polls::{ApiError, to_json_response};
use crate::services::vote_service::{VoteService, VoteServiceError};
use crate::storage::models::vote::CastVoteRequest;
use serde_json;
use std::sync::Arc;

impl From<VoteServiceError> for ApiError {
    fn from(err: VoteServiceError) -> Self {
        match err {
            VoteServiceError::InvalidData(msg) => ApiError::BadRequest(msg),
            VoteServiceError::AuthorizationFailed(msg) => ApiError::Unauthorized(msg),
            VoteServiceError::PollNotFound(_) => ApiError::NotFound,
            VoteServiceError::PollNotActive => {
                ApiError::BadRequest("Poll is not active or has expired".to_string())
            }
            VoteServiceError::DuplicateVote => ApiError::BadRequest(
                "Duplicate vote: voter has already voted on this poll".to_string(),
            ),
            VoteServiceError::ChainError(e) => ApiError::InternalError(format!("Chain error: {e}")),
            VoteServiceError::VoteRepositoryError(e) => {
                ApiError::InternalError(format!("Vote storage error: {e}"))
            }
            VoteServiceError::PollRepositoryError(e) => {
                ApiError::InternalError(format!("Poll storage error: {e}"))
            }
            VoteServiceError::SystemTimeError => {
                ApiError::InternalError("System time error".to_string())
            }
            VoteServiceError::RingProofError(e) => {
                ApiError::BadRequest(format!("Ring proof validation failed: {e}"))
            }
        }
    }
}

pub struct VoteHandler {
    vote_service: Arc<VoteService>,
}

impl VoteHandler {
    pub fn new(vote_service: Arc<VoteService>) -> Self {
        Self { vote_service }
    }

    pub async fn cast_vote(&self, poll_id: &str, json_body: &str) -> Result<String, ApiError> {
        // Parse poll_id
        let poll_id: u32 = poll_id
            .parse()
            .map_err(|_| ApiError::BadRequest("Invalid poll ID format".to_string()))?;

        // Parse request
        let mut request: CastVoteRequest = serde_json::from_str(json_body)
            .map_err(|e| ApiError::BadRequest(format!("Invalid JSON: {e}")))?;

        // Set the poll_id from the URL parameter
        request.poll_id = poll_id;

        // Call service
        let response = self.vote_service.cast_vote(request).await?;

        // Return response as JSON
        to_json_response(&response)
    }

    pub async fn get_poll_votes(&self, poll_id: &str) -> Result<String, ApiError> {
        // Parse poll_id
        let poll_id: u32 = poll_id
            .parse()
            .map_err(|_| ApiError::BadRequest("Invalid poll ID format".to_string()))?;

        // Call service to get vote counts
        let (yay_count, nay_count) = self.vote_service.get_vote_counts(poll_id).await?;

        // Return counts as JSON
        let response = serde_json::json!({
            "poll_id": poll_id,
            "yay_votes": yay_count,
            "nay_votes": nay_count
        });

        to_json_response(&response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::vote_service::VoteServiceError;

    #[test]
    fn test_vote_service_error_conversion() {
        let auth_error = VoteServiceError::AuthorizationFailed("test".to_string());
        let api_error: ApiError = auth_error.into();
        match api_error {
            ApiError::Unauthorized(_) => (),
            _ => panic!("Expected Unauthorized error"),
        }

        let not_found_error = VoteServiceError::PollNotFound(123);
        let api_error: ApiError = not_found_error.into();
        match api_error {
            ApiError::NotFound => (),
            _ => panic!("Expected NotFound error"),
        }
    }
}
