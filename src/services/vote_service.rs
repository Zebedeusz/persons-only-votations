use crate::chain::people_multi::{ChainClient, ChainError};
use crate::crypto::commitments::{hex_to_commitment, hex_to_nonce, verify_commitment};
use crate::crypto::contexts::string_to_context;
use crate::crypto::ring_proof::{RingProofError, validate_ring_proof};
use crate::storage::models::poll::Poll;
use crate::storage::models::vote::{
    CastVoteResponse, CommitVoteRequest, RevealVoteRequest, Vote, VotePhase, VoteType,
};
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
    #[error("Vote not found: no committed vote found for voter")]
    VoteNotFound,
    #[error("Invalid commitment: commitment verification failed")]
    InvalidCommitment,
    #[error("Vote already revealed: cannot reveal vote twice")]
    VoteAlreadyRevealed,
    #[error("Poll not in reveal phase: cannot reveal vote during voting period")]
    PollNotInRevealPhase,
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

    fn get_current_timestamp() -> Result<i64> {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| VoteServiceError::SystemTimeError)
            .map(|duration| duration.as_secs() as i64)
    }

    fn validate_poll_active(poll: &Poll, current_time: i64) -> Result<()> {
        if !poll.is_active || current_time > poll.ends_at {
            return Err(VoteServiceError::PollNotActive);
        }
        Ok(())
    }

    pub async fn commit_vote(&self, request: CommitVoteRequest) -> Result<CastVoteResponse> {
        let poll = self
            .poll_repository
            .get_poll_by_id(request.poll_id)?
            .ok_or(VoteServiceError::PollNotFound(request.poll_id))?;

        let current_time = Self::get_current_timestamp()?;
        Self::validate_poll_active(&poll, current_time)?;

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

        let commit_message = self.create_commit_message(&request, &poll.context)?;
        let signature_valid = self
            .chain_client
            .validate_signature(
                &request.voter_member_hex,
                &request.voter_signature,
                &commit_message,
            )
            .await?;

        if !signature_valid {
            return Err(VoteServiceError::AuthorizationFailed(
                "Invalid commit signature".to_string(),
            ));
        }

        let context_bytes = string_to_context(&poll.context)
            .map_err(|e| VoteServiceError::InvalidData(format!("Invalid poll context: {e}")))?;

        let voter_ring_index = self.get_voter_ring_index(&request.voter_member_hex).await?;

        let ring_root = self
            .chain_client
            .get_ring_root(voter_ring_index)
            .await
            .map_err(VoteServiceError::ChainError)?
            .ok_or_else(|| {
                VoteServiceError::AuthorizationFailed(format!(
                    "Ring root not found for ring index {voter_ring_index}"
                ))
            })?;

        let voter_alias = validate_ring_proof(
            &request.ring_proof,
            &ring_root,
            &context_bytes,
            &commit_message,
        )?;

        let vote = Vote::new_commit(
            request.poll_id,
            request.voter_member_hex,
            voter_alias,
            request.vote_commitment,
            request.voter_signature,
            request.ring_proof,
            current_time,
        );

        self.vote_repository.create_vote(vote)?;

        Ok(CastVoteResponse {
            poll_id: request.poll_id,
            message: "Vote commitment submitted successfully".to_string(),
        })
    }

    pub async fn reveal_vote(&self, request: RevealVoteRequest) -> Result<CastVoteResponse> {
        let poll = self
            .poll_repository
            .get_poll_by_id(request.poll_id)?
            .ok_or(VoteServiceError::PollNotFound(request.poll_id))?;

        let current_time = Self::get_current_timestamp()?;

        if poll.is_active && current_time <= poll.ends_at {
            return Err(VoteServiceError::PollNotInRevealPhase);
        }

        let mut vote = self
            .vote_repository
            .get_vote_by_poll_and_voter(request.poll_id, &request.voter_member_hex)?
            .ok_or(VoteServiceError::VoteNotFound)?;

        if vote.phase == VotePhase::Revealed {
            return Err(VoteServiceError::VoteAlreadyRevealed);
        }

        let commitment = hex_to_commitment(&vote.vote_commitment)
            .map_err(|e| VoteServiceError::InvalidData(format!("Invalid commitment hex: {e}")))?;
        let nonce = hex_to_nonce(&request.vote_nonce)
            .map_err(|e| VoteServiceError::InvalidData(format!("Invalid nonce hex: {e}")))?;

        if !verify_commitment(&commitment, &request.vote_type, &nonce) {
            return Err(VoteServiceError::InvalidCommitment);
        }

        vote.reveal(request.vote_type.clone(), request.vote_nonce, current_time);
        self.vote_repository.update_vote(vote)?;

        let (yay_count, nay_count) = self.count_revealed_votes(request.poll_id)?;
        let mut updated_poll = poll;
        updated_poll.yay_votes = yay_count;
        updated_poll.nay_votes = nay_count;
        self.poll_repository.update_poll(updated_poll)?;

        let vote_type_message = match request.vote_type {
            VoteType::Yay => "Yay vote revealed successfully",
            VoteType::Nay => "Nay vote revealed successfully",
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
        let poll = self
            .poll_repository
            .get_poll_by_id(poll_id)?
            .ok_or(VoteServiceError::PollNotFound(poll_id))?;

        let current_time = Self::get_current_timestamp()?;

        if poll.is_active && current_time <= poll.ends_at {
            return Ok((0, 0));
        }

        self.count_revealed_votes(poll_id)
    }

    pub async fn finalize_poll_results(&self, poll_id: u32) -> Result<(u32, u32)> {
        let mut poll = self
            .poll_repository
            .get_poll_by_id(poll_id)?
            .ok_or(VoteServiceError::PollNotFound(poll_id))?;

        let current_time = Self::get_current_timestamp()?;
        if poll.is_active && current_time <= poll.ends_at {
            return Err(VoteServiceError::PollNotActive);
        }

        let (yay_count, nay_count) = self.count_revealed_votes(poll_id)?;

        poll.yay_votes = yay_count;
        poll.nay_votes = nay_count;
        poll.is_active = false;

        self.poll_repository.update_poll(poll)?;

        Ok((yay_count, nay_count))
    }

    async fn get_voter_ring_index(&self, member_hex: &str) -> Result<u16> {
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

        let ring_index = self
            .chain_client
            .get_person_ring_index(member_hex)
            .await
            .map_err(VoteServiceError::ChainError)?
            .ok_or_else(|| {
                VoteServiceError::AuthorizationFailed(
                    "Voter is not included in any ring (may be onboarding)".to_string(),
                )
            })?;

        Ok(ring_index)
    }

    fn create_commit_message(
        &self,
        request: &CommitVoteRequest,
        poll_context: &str,
    ) -> Result<Vec<u8>> {
        let commit_data = serde_json::json!({
            "poll_id": request.poll_id,
            "vote_commitment": request.vote_commitment,
            "context": poll_context
        });

        let message = serde_json::to_string(&commit_data).map_err(|e| {
            VoteServiceError::InvalidData(format!("JSON serialization failed: {e}"))
        })?;

        Ok(message.as_bytes().to_vec())
    }

    fn count_revealed_votes(&self, poll_id: u32) -> Result<(u32, u32)> {
        let votes = self.vote_repository.get_votes_by_poll(poll_id)?;
        let mut yay_count = 0;
        let mut nay_count = 0;

        for vote in votes {
            if vote.phase == VotePhase::Revealed {
                if let Some(vote_type) = vote.vote_type {
                    match vote_type {
                        VoteType::Yay => yay_count += 1,
                        VoteType::Nay => nay_count += 1,
                    }
                }
            }
        }

        Ok((yay_count, nay_count))
    }
}
