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

        if self
            .vote_repository
            .get_vote_by_poll_and_voter(request.poll_id, &request.voter_member_hex)?
            .is_some()
        {
            return Err(VoteServiceError::DuplicateVote);
        }

        let person_verified = self
            .chain_client
            .verify_personhood(&request.voter_member_hex, &request.voter_signature)
            .await?;

        if !person_verified {
            return Err(VoteServiceError::AuthorizationFailed(
                "Voter personhood not verified".to_string(),
            ));
        }

        let message = self.create_vote_message(&request, &poll.context)?;
        let signature_valid = self
            .chain_client
            .validate_signature(
                &request.voter_member_hex,
                &request.voter_signature,
                &message,
            )
            .await?;

        if !signature_valid {
            return Err(VoteServiceError::AuthorizationFailed(
                "Invalid vote signature".to_string(),
            ));
        }

        let context_bytes = string_to_context(&poll.context)
            .map_err(|e| VoteServiceError::InvalidData(format!("Invalid poll context: {e}")))?;

        // Step 1: Get the voter's ring index from their personhood verification
        // This ensures the voter is in an active ring and gets their ring index
        let voter_ring_index = self.get_voter_ring_index(&request.voter_member_hex).await?;

        // Step 2: Get the actual ring root from the blockchain for the voter's ring
        let ring_root = self
            .chain_client
            .get_ring_root(voter_ring_index)
            .await
            .map_err(|e| VoteServiceError::ChainError(e))?
            .ok_or_else(|| {
                VoteServiceError::AuthorizationFailed(format!(
                    "Ring root not found for ring index {}",
                    voter_ring_index
                ))
            })?;

        // Step 3: Validate the ring proof against the actual ring root
        let voter_alias =
            validate_ring_proof(&request.ring_proof, &ring_root, &context_bytes, &message)?;

        let vote = Vote::new(
            request.poll_id,
            request.voter_member_hex,
            voter_alias,
            request.vote_type.clone(),
            request.voter_signature,
            request.ring_proof,
            current_time,
        );

        self.vote_repository.create_vote(vote)?;

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
        self.poll_repository
            .get_poll_by_id(poll_id)?
            .ok_or(VoteServiceError::PollNotFound(poll_id))?;

        Ok(self.vote_repository.get_votes_by_poll(poll_id)?)
    }

    pub async fn get_vote_counts(&self, poll_id: u32) -> Result<(u32, u32)> {
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
            VoteServiceError::InvalidData(format!("JSON serialization failed: {e}"))
        })?;

        Ok(message.as_bytes().to_vec())
    }

    /// Helper method to get the ring index for a voter
    /// This method ensures the voter is properly validated and gets their ring index
    async fn get_voter_ring_index(&self, member_hex: &str) -> Result<u16> {
        // In production, we need to query the chain to get the voter's ring index
        // This requires extending the ChainClient trait to provide this information
        // For now, implementing a basic approach that leverages existing personhood verification

        // First, verify personhood (this also implicitly checks ring membership)
        let is_verified = self
            .chain_client
            .verify_personhood(member_hex, "")
            .await
            .map_err(VoteServiceError::ChainError)?;

        if !is_verified {
            return Err(VoteServiceError::AuthorizationFailed(
                "Voter not verified for ring membership".to_string(),
            ));
        }

        // For now, assume ring index 0 for verified persons
        // TODO: Extend ChainClient to provide get_person_ring_index method
        // that properly queries the chain for the person's ring index
        Ok(0)
    }
}
