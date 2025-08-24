use crate::chain::people_multi::{ChainClient, ChainError};
use crate::crypto::contexts::string_to_context;
use crate::crypto::ring_proof::{RingProofError, validate_ring_proof};
use crate::storage::models::vote::{CastVoteRequest, CastVoteResponse, Vote, VoteType};
use crate::storage::repositories::poll_repository::Repository;
use crate::storage::repositories::vote_repository::{VoteRepository, VoteRepositoryError};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum VoteServiceError {
    #[error("Invalid vote data: {0}")]
    InvalidData(String),
    #[error("Authorization failed: {0}")]
    AuthorizationFailed(String),
    #[error("Poll not found: {0}")]
    PollNotFound(u32),
    #[error("Poll not active or expired")]
    PollNotActive,
    #[error("Duplicate vote: voter has already voted on this poll")]
    DuplicateVote,
    #[error("Chain error: {0}")]
    ChainError(#[from] ChainError),
    #[error("Vote repository error: {0}")]
    VoteRepositoryError(#[from] VoteRepositoryError),
    #[error("Poll repository error: {0}")]
    PollRepositoryError(#[from] crate::storage::repositories::poll_repository::PollRepositoryError),
    #[error("Ring proof error: {0}")]
    RingProofError(#[from] RingProofError),
    #[error("System time error")]
    SystemTimeError,
}

pub type Result<T> = std::result::Result<T, VoteServiceError>;

pub struct VoteService {
    vote_repository: Arc<dyn VoteRepository>,
    poll_repository: Arc<dyn Repository>,
    chain_client: Arc<dyn ChainClient>,
}

impl VoteService {
    pub fn new(
        vote_repository: Arc<dyn VoteRepository>,
        poll_repository: Arc<dyn Repository>,
        chain_client: Arc<dyn ChainClient>,
    ) -> Self {
        Self {
            vote_repository,
            poll_repository,
            chain_client,
        }
    }

    pub async fn cast_vote(&self, request: CastVoteRequest) -> Result<CastVoteResponse> {
        // 1. Validate poll exists and is active
        let poll = self
            .poll_repository
            .get_poll_by_id(request.poll_id)?
            .ok_or(VoteServiceError::PollNotFound(request.poll_id))?;

        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| VoteServiceError::SystemTimeError)?
            .as_secs() as i64;

        if !poll.is_active || current_time > poll.ends_at {
            return Err(VoteServiceError::PollNotActive);
        }

        // 2. Check for duplicate vote (we'll validate with alias after ring proof validation)
        // For now, still check by public key - we'll update this after getting the alias
        if self
            .vote_repository
            .get_vote_by_poll_and_voter(request.poll_id, &request.voter_public_key)?
            .is_some()
        {
            return Err(VoteServiceError::DuplicateVote);
        }

        // 3. Verify voter personhood
        let person_verified = self
            .chain_client
            .verify_personhood(&request.voter_public_key, &request.voter_signature)
            .await?;

        if !person_verified {
            return Err(VoteServiceError::AuthorizationFailed(
                "Voter personhood not verified".to_string(),
            ));
        }

        // 4. Verify signature against vote message
        let message = self.create_vote_message(&request, &poll.context)?;
        let signature_valid = self
            .chain_client
            .validate_signature(
                &request.voter_public_key,
                &request.voter_signature,
                &message,
            )
            .await?;

        if !signature_valid {
            return Err(VoteServiceError::AuthorizationFailed(
                "Invalid vote signature".to_string(),
            ));
        }

        // 5. Validate ring inclusion proof
        let context_bytes = string_to_context(&poll.context)
            .map_err(|e| VoteServiceError::InvalidData(format!("Invalid poll context: {}", e)))?;

        // Get ring root from chain - we need this for ring proof validation
        // For now, we'll use a placeholder approach - in production this should come from the chain
        let ring_root = sp_core::sr25519::Public::from_raw([1u8; 32]); // TODO: Get from chain

        let voter_alias =
            validate_ring_proof(&request.ring_proof, &ring_root, &context_bytes, &message)?;

        // 6. Create and store vote
        let vote = Vote::new(
            request.poll_id,
            request.voter_public_key,
            voter_alias,
            request.vote_type.clone(),
            request.voter_signature,
            request.ring_proof,
            current_time,
        );

        self.vote_repository.create_vote(vote)?;

        // 7. Update poll vote counts
        let (yay_count, nay_count) = self.vote_repository.count_votes_by_poll(request.poll_id)?;
        let mut updated_poll = poll;
        updated_poll.yay_votes = yay_count;
        updated_poll.nay_votes = nay_count;

        self.poll_repository.update_poll(updated_poll)?;

        let vote_type_message = match request.vote_type {
            VoteType::Yay => "Yay vote cast successfully",
            VoteType::Nay => "Nay vote cast successfully",
        };

        Ok(CastVoteResponse {
            poll_id: request.poll_id,
            message: vote_type_message.to_string(),
        })
    }

    pub async fn get_poll_votes(&self, poll_id: u32) -> Result<Vec<Vote>> {
        // Verify poll exists
        self.poll_repository
            .get_poll_by_id(poll_id)?
            .ok_or(VoteServiceError::PollNotFound(poll_id))?;

        Ok(self.vote_repository.get_votes_by_poll(poll_id)?)
    }

    pub async fn get_vote_counts(&self, poll_id: u32) -> Result<(u32, u32)> {
        // Verify poll exists
        self.poll_repository
            .get_poll_by_id(poll_id)?
            .ok_or(VoteServiceError::PollNotFound(poll_id))?;

        Ok(self.vote_repository.count_votes_by_poll(poll_id)?)
    }

    fn create_vote_message(
        &self,
        request: &CastVoteRequest,
        poll_context: &str,
    ) -> Result<Vec<u8>> {
        let vote_data = serde_json::json!({
            "poll_id": request.poll_id,
            "vote_type": request.vote_type,
            "context": poll_context
        });

        let message = serde_json::to_string(&vote_data).map_err(|e| {
            VoteServiceError::InvalidData(format!("JSON serialization failed: {}", e))
        })?;

        Ok(message.as_bytes().to_vec())
    }
}
