use crate::chain::people_multi::{ChainClient, ChainError};
use crate::crypto::commitments::verify_commitment;
use crate::crypto::hex_to_bytes32;
use crate::crypto::homomorphic::{HomomorphicError, TfheVotingSystem};
use crate::crypto::ring_proof::{RingProofError, validate_ring_proof};
use crate::storage::models::poll::Poll;
use crate::storage::models::vote::{
    CastEncryptedVoteRequest, CastVoteResponse, CommitVoteRequest, EncryptedVote,
    RevealVoteRequest, Vote, VotePhase, VoteType,
};
use crate::storage::repositories::poll_repository::{PollRepository, PollRepositoryError};
use crate::storage::repositories::vote_repository::{VoteRepository, VoteRepositoryError};
use crate::utils::get_current_timestamp;
use std::sync::Arc;
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
    PollRepositoryError(#[from] PollRepositoryError),
    #[error("Ring proof error: {0}")]
    RingProofError(#[from] RingProofError),
    #[error("Homomorphic encryption error: {0}")]
    HomomorphicError(#[from] HomomorphicError),
}

pub type Result<T> = std::result::Result<T, VoteServiceError>;

/// Message a voter signs when committing a vote; also the ring-proof message.
pub fn commit_message(poll_id: u32, vote_commitment: &str, poll_context: &str) -> Vec<u8> {
    serde_json::to_vec(&serde_json::json!({
        "poll_id": poll_id,
        "vote_commitment": vote_commitment,
        "context": poll_context
    }))
    .expect("JSON value always serializes")
}

/// Message a voter signs when casting an encrypted ballot; also the ring-proof message.
pub fn encrypted_vote_message(poll_id: u32, encrypted_vote: &str, poll_context: &str) -> Vec<u8> {
    serde_json::to_vec(&serde_json::json!({
        "poll_id": poll_id,
        "encrypted_vote": encrypted_vote,
        "context": poll_context
    }))
    .expect("JSON value always serializes")
}

pub struct VoteService {
    vote_repository: Arc<VoteRepository>,
    poll_repository: Arc<PollRepository>,
    chain_client: Arc<dyn ChainClient>,
    tfhe: TfheVotingSystem,
}

impl VoteService {
    pub fn new(
        vote_repository: Arc<VoteRepository>,
        poll_repository: Arc<PollRepository>,
        chain_client: Arc<dyn ChainClient>,
    ) -> Self {
        Self {
            vote_repository,
            poll_repository,
            chain_client,
            tfhe: TfheVotingSystem::new(),
        }
    }

    fn load_poll(&self, poll_id: u32) -> Result<Poll> {
        self.poll_repository
            .get_poll_by_id(poll_id)?
            .ok_or(VoteServiceError::PollNotFound(poll_id))
    }

    /// Verifies personhood, the plain signature and the ring proof over `message`;
    /// returns the voter's ring alias used for one-vote-per-person.
    async fn authorize_voter(
        &self,
        poll: &Poll,
        member_hex: &str,
        signature: &str,
        ring_proof: &str,
        message: &[u8],
    ) -> Result<String> {
        let ring_index = self
            .chain_client
            .ring_index(member_hex)
            .await?
            .ok_or_else(|| {
                VoteServiceError::AuthorizationFailed(
                    "Voter is not included in any ring (may be onboarding)".to_string(),
                )
            })?;
        let ring_root = self
            .chain_client
            .ring_root(ring_index)
            .await?
            .ok_or_else(|| {
                VoteServiceError::AuthorizationFailed(format!(
                    "Ring root not found for ring index {ring_index}"
                ))
            })?;
        if !self
            .chain_client
            .validate_signature(member_hex, signature, message)
            .await?
        {
            return Err(VoteServiceError::AuthorizationFailed(
                "Invalid signature".to_string(),
            ));
        }
        let context = hex_to_bytes32(&poll.context)
            .map_err(|e| VoteServiceError::InvalidData(format!("Invalid poll context: {e}")))?;
        Ok(validate_ring_proof(
            ring_proof, &ring_root, &context, message,
        )?)
    }

    pub async fn commit_vote(&self, request: CommitVoteRequest) -> Result<CastVoteResponse> {
        let poll = self.load_poll(request.poll_id)?;
        let now = get_current_timestamp();
        if !poll.is_open(now) {
            return Err(VoteServiceError::PollNotActive);
        }
        if self
            .vote_repository
            .get_vote_by_poll_and_voter(request.poll_id, &request.voter_member_hex)?
            .is_some()
        {
            return Err(VoteServiceError::DuplicateVote);
        }

        let message = commit_message(request.poll_id, &request.vote_commitment, &poll.context);
        let voter_alias = self
            .authorize_voter(
                &poll,
                &request.voter_member_hex,
                &request.voter_signature,
                &request.ring_proof,
                &message,
            )
            .await?;

        self.vote_repository.create_vote(Vote::new_commit(
            request.poll_id,
            request.voter_member_hex,
            voter_alias,
            request.vote_commitment,
            request.voter_signature,
            request.ring_proof,
            now,
        ))?;

        Ok(CastVoteResponse {
            poll_id: request.poll_id,
            message: "Vote commitment submitted successfully".to_string(),
        })
    }

    pub async fn reveal_vote(&self, request: RevealVoteRequest) -> Result<CastVoteResponse> {
        let mut poll = self.load_poll(request.poll_id)?;
        let now = get_current_timestamp();
        if poll.is_open(now) {
            return Err(VoteServiceError::PollNotInRevealPhase);
        }

        let mut vote = self
            .vote_repository
            .get_vote_by_poll_and_voter(request.poll_id, &request.voter_member_hex)?
            .ok_or(VoteServiceError::VoteNotFound)?;
        if vote.phase == VotePhase::Revealed {
            return Err(VoteServiceError::VoteAlreadyRevealed);
        }

        let commitment = hex_to_bytes32(&vote.vote_commitment)
            .map_err(|e| VoteServiceError::InvalidData(format!("Invalid commitment hex: {e}")))?;
        let nonce = hex_to_bytes32(&request.vote_nonce)
            .map_err(|e| VoteServiceError::InvalidData(format!("Invalid nonce hex: {e}")))?;
        if !verify_commitment(&commitment, &request.vote_type, &nonce) {
            return Err(VoteServiceError::InvalidCommitment);
        }

        vote.reveal(request.vote_type.clone(), request.vote_nonce, now);
        self.vote_repository.update_vote(vote)?;

        (poll.yay_votes, poll.nay_votes) = self.count_revealed_votes(request.poll_id)?;
        self.poll_repository.update_poll(poll)?;

        let message = match request.vote_type {
            VoteType::Yay => "Yay vote revealed successfully",
            VoteType::Nay => "Nay vote revealed successfully",
        };
        Ok(CastVoteResponse {
            poll_id: request.poll_id,
            message: message.to_string(),
        })
    }

    /// Revealed (yay, nay) counts; hidden as (0, 0) while the poll is open.
    pub async fn get_vote_counts(&self, poll_id: u32) -> Result<(u32, u32)> {
        let poll = self.load_poll(poll_id)?;
        if poll.is_open(get_current_timestamp()) {
            return Ok((0, 0));
        }
        self.count_revealed_votes(poll_id)
    }

    pub async fn finalize_poll_results(&self, poll_id: u32) -> Result<(u32, u32)> {
        let mut poll = self.load_poll(poll_id)?;
        if poll.is_open(get_current_timestamp()) {
            return Err(VoteServiceError::PollNotActive);
        }
        let counts = self.count_revealed_votes(poll_id)?;
        (poll.yay_votes, poll.nay_votes) = counts;
        poll.is_active = false;
        self.poll_repository.update_poll(poll)?;
        Ok(counts)
    }

    fn count_revealed_votes(&self, poll_id: u32) -> Result<(u32, u32)> {
        let votes = self.vote_repository.get_votes_by_poll(poll_id)?;
        let count = |wanted: VoteType| {
            votes
                .iter()
                .filter(|v| v.vote_type == Some(wanted.clone()))
                .count() as u32
        };
        Ok((count(VoteType::Yay), count(VoteType::Nay)))
    }

    pub async fn cast_encrypted_vote(
        &self,
        request: CastEncryptedVoteRequest,
    ) -> Result<CastVoteResponse> {
        let poll = self.load_poll(request.poll_id)?;
        let now = get_current_timestamp();
        if !poll.is_open(now) {
            return Err(VoteServiceError::PollNotActive);
        }
        if self
            .vote_repository
            .get_encrypted_vote_by_poll_and_voter(request.poll_id, &request.voter_member_hex)?
            .is_some()
        {
            return Err(VoteServiceError::DuplicateVote);
        }
        // Rejecting ballots that are not ciphertexts under the server key before any chain work
        let _ = self
            .tfhe
            .decode_ciphertext(&request.encrypted_vote)
            .map_err(|e| VoteServiceError::InvalidData(format!("Invalid encrypted vote: {e}")))?;

        let message =
            encrypted_vote_message(request.poll_id, &request.encrypted_vote, &poll.context);
        let voter_alias = self
            .authorize_voter(
                &poll,
                &request.voter_member_hex,
                &request.voter_signature,
                &request.ring_proof,
                &message,
            )
            .await?;

        self.vote_repository.create_encrypted_vote(EncryptedVote {
            poll_id: request.poll_id,
            voter_member_hex: request.voter_member_hex,
            voter_alias,
            encrypted_vote: request.encrypted_vote,
            ring_proof: request.ring_proof,
            voter_signature: request.voter_signature,
            created_at: now,
        })?;

        Ok(CastVoteResponse {
            poll_id: request.poll_id,
            message: "Encrypted vote submitted successfully".to_string(),
        })
    }

    pub async fn get_poll_encrypted_votes(&self, poll_id: u32) -> Result<Vec<EncryptedVote>> {
        self.load_poll(poll_id)?;
        Ok(self.vote_repository.get_encrypted_votes_by_poll(poll_id)?)
    }

    /// Homomorphic (yay, nay) tally; hidden as (0, 0) while the poll is open.
    pub async fn compute_homomorphic_tally(&self, poll_id: u32) -> Result<(u64, u64)> {
        let poll = self.load_poll(poll_id)?;
        if poll.is_open(get_current_timestamp()) {
            return Ok((0, 0));
        }
        let encrypted_votes = self.vote_repository.get_encrypted_votes_by_poll(poll_id)?;
        let ciphertexts = encrypted_votes
            .iter()
            .map(|vote| self.tfhe.decode_ciphertext(&vote.encrypted_vote))
            .collect::<std::result::Result<Vec<_>, _>>()?;
        let yay_count = self.tfhe.tally(&ciphertexts);
        let nay_count = (encrypted_votes.len() as u64).saturating_sub(yay_count);
        Ok((yay_count, nay_count))
    }

    pub fn get_public_encryption_key(&self) -> Result<String> {
        Ok(self.tfhe.public_key_hex()?)
    }
}
