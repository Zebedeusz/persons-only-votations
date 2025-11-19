use super::polls::{ApiError, to_json_response};
use crate::services::vote_service::{VoteService, VoteServiceError};
use crate::storage::models::vote::{
    CastEncryptedVoteRequest, CommitVoteRequest, RevealVoteRequest,
};
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
            VoteServiceError::VoteNotFound => ApiError::NotFound,
            VoteServiceError::InvalidCommitment => ApiError::BadRequest(
                "Invalid commitment: commitment verification failed".to_string(),
            ),
            VoteServiceError::VoteAlreadyRevealed => {
                ApiError::BadRequest("Vote already revealed: cannot reveal vote twice".to_string())
            }
            VoteServiceError::PollNotInRevealPhase => ApiError::BadRequest(
                "Poll not in reveal phase: cannot reveal vote during voting period".to_string(),
            ),
            VoteServiceError::ChainError(e) => ApiError::InternalError(format!("Chain error: {e}")),
            VoteServiceError::VoteRepositoryError(e) => {
                ApiError::InternalError(format!("Vote storage error: {e}"))
            }
            VoteServiceError::PollRepositoryError(e) => {
                ApiError::InternalError(format!("Poll storage error: {e}"))
            }
            VoteServiceError::TimeError(e) => ApiError::InternalError(format!("Time error: {e}")),
            VoteServiceError::RingProofError(e) => {
                ApiError::BadRequest(format!("Ring proof validation failed: {e}"))
            }
            VoteServiceError::HomomorphicError(e) => {
                ApiError::InternalError(format!("Homomorphic encryption error: {e}"))
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

    fn parse_poll_id(poll_id: &str) -> Result<u32, ApiError> {
        poll_id
            .parse()
            .map_err(|_| ApiError::BadRequest("Invalid poll ID format".to_string()))
    }

    fn parse_request_with_poll_id<T>(
        json_body: &str,
        poll_id: u32,
        set_poll_id: impl FnOnce(&mut T, u32),
    ) -> Result<T, ApiError>
    where
        T: serde::de::DeserializeOwned,
    {
        let mut request: T = serde_json::from_str(json_body)
            .map_err(|e| ApiError::BadRequest(format!("Invalid JSON: {e}")))?;
        set_poll_id(&mut request, poll_id);
        Ok(request)
    }

    pub async fn commit_vote(&self, poll_id: &str, json_body: &str) -> Result<String, ApiError> {
        let poll_id = Self::parse_poll_id(poll_id)?;
        let request = Self::parse_request_with_poll_id(
            json_body,
            poll_id,
            |req: &mut CommitVoteRequest, id| {
                req.poll_id = id;
            },
        )?;
        let response = self.vote_service.commit_vote(request).await?;
        to_json_response(&response)
    }

    pub async fn reveal_vote(&self, poll_id: &str, json_body: &str) -> Result<String, ApiError> {
        let poll_id = Self::parse_poll_id(poll_id)?;
        let request = Self::parse_request_with_poll_id(
            json_body,
            poll_id,
            |req: &mut RevealVoteRequest, id| {
                req.poll_id = id;
            },
        )?;
        let response = self.vote_service.reveal_vote(request).await?;
        to_json_response(&response)
    }

    pub async fn finalize_poll(&self, poll_id: &str) -> Result<String, ApiError> {
        let poll_id = Self::parse_poll_id(poll_id)?;
        let (yay_count, nay_count) = self.vote_service.finalize_poll_results(poll_id).await?;
        let response = serde_json::json!({
            "poll_id": poll_id,
            "yay_votes": yay_count,
            "nay_votes": nay_count,
            "status": "finalized"
        });
        to_json_response(&response)
    }

    pub async fn get_poll_votes(&self, poll_id: &str) -> Result<String, ApiError> {
        let poll_id = Self::parse_poll_id(poll_id)?;
        let (yay_count, nay_count) = self.vote_service.get_vote_counts(poll_id).await?;
        let response = serde_json::json!({
            "poll_id": poll_id,
            "yay_votes": yay_count,
            "nay_votes": nay_count
        });
        to_json_response(&response)
    }

    pub async fn cast_encrypted_vote(
        &self,
        poll_id: &str,
        json_body: &str,
    ) -> Result<String, ApiError> {
        let poll_id = Self::parse_poll_id(poll_id)?;
        let request = Self::parse_request_with_poll_id(
            json_body,
            poll_id,
            |req: &mut CastEncryptedVoteRequest, id| {
                req.poll_id = id;
            },
        )?;
        let response = self.vote_service.cast_encrypted_vote(request).await?;
        to_json_response(&response)
    }

    pub async fn get_encrypted_votes(&self, poll_id: &str) -> Result<String, ApiError> {
        let poll_id = Self::parse_poll_id(poll_id)?;
        let encrypted_votes = self.vote_service.get_poll_encrypted_votes(poll_id).await?;
        let response = serde_json::json!({
            "poll_id": poll_id,
            "encrypted_votes": encrypted_votes
        });
        to_json_response(&response)
    }

    pub async fn get_homomorphic_tally(&self, poll_id: &str) -> Result<String, ApiError> {
        let poll_id = Self::parse_poll_id(poll_id)?;
        let (yay_count, nay_count) = self.vote_service.compute_homomorphic_tally(poll_id).await?;
        let response = serde_json::json!({
            "poll_id": poll_id,
            "total_yay_votes": yay_count,
            "total_nay_votes": nay_count,
            "tally_type": "homomorphic"
        });
        to_json_response(&response)
    }

    pub async fn get_encryption_key(&self) -> Result<String, ApiError> {
        let public_key = self.vote_service.get_public_encryption_key()?;
        let response = serde_json::json!({
            "public_key": public_key,
            "key_type": "tfhe_client_key"
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
