use crate::storage::models::vote::{EncryptedVote, Vote};
use std::sync::{Mutex, MutexGuard};
use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum VoteRepositoryError {
    #[error("Lock error")]
    LockError,
    #[error("Vote not found: {0}")]
    VoteNotFound(u32),
    #[error("Duplicate vote for poll {poll_id} by voter {voter_key}")]
    DuplicateVote { poll_id: u32, voter_key: String },
}

pub type Result<T> = std::result::Result<T, VoteRepositoryError>;

/// In-memory store for commit-reveal votes and encrypted ballots.
// ponytail: every lookup is a linear scan; add per-poll indexes when votes exceed ~10k
#[derive(Default)]
pub struct VoteRepository {
    votes: Mutex<Vec<Vote>>,
    encrypted_votes: Mutex<Vec<EncryptedVote>>,
}

fn lock<T>(mutex: &Mutex<T>) -> Result<MutexGuard<'_, T>> {
    mutex.lock().map_err(|_| VoteRepositoryError::LockError)
}

impl VoteRepository {
    /// Rejects a second vote on the same poll from the same member key or ring alias.
    pub fn create_vote(&self, vote: Vote) -> Result<()> {
        let mut votes = lock(&self.votes)?;
        let duplicate = votes.iter().find(|v| {
            v.poll_id == vote.poll_id
                && (v.voter_member_hex == vote.voter_member_hex
                    || v.voter_alias == vote.voter_alias)
        });
        if let Some(existing) = duplicate {
            return Err(VoteRepositoryError::DuplicateVote {
                poll_id: vote.poll_id,
                voter_key: if existing.voter_member_hex == vote.voter_member_hex {
                    vote.voter_member_hex
                } else {
                    vote.voter_alias
                },
            });
        }
        votes.push(vote);
        Ok(())
    }

    pub fn update_vote(&self, updated: Vote) -> Result<()> {
        let mut votes = lock(&self.votes)?;
        let slot = votes
            .iter_mut()
            .find(|v| {
                v.poll_id == updated.poll_id && v.voter_member_hex == updated.voter_member_hex
            })
            .ok_or(VoteRepositoryError::VoteNotFound(updated.poll_id))?;
        *slot = updated;
        Ok(())
    }

    pub fn get_votes_by_poll(&self, poll_id: u32) -> Result<Vec<Vote>> {
        Ok(lock(&self.votes)?
            .iter()
            .filter(|v| v.poll_id == poll_id)
            .cloned()
            .collect())
    }

    pub fn get_vote_by_poll_and_voter(
        &self,
        poll_id: u32,
        voter_member_hex: &str,
    ) -> Result<Option<Vote>> {
        Ok(lock(&self.votes)?
            .iter()
            .find(|v| v.poll_id == poll_id && v.voter_member_hex == voter_member_hex)
            .cloned())
    }

    pub fn create_encrypted_vote(&self, vote: EncryptedVote) -> Result<()> {
        let mut votes = lock(&self.encrypted_votes)?;
        if votes
            .iter()
            .any(|v| v.poll_id == vote.poll_id && v.voter_member_hex == vote.voter_member_hex)
        {
            return Err(VoteRepositoryError::DuplicateVote {
                poll_id: vote.poll_id,
                voter_key: vote.voter_member_hex,
            });
        }
        votes.push(vote);
        Ok(())
    }

    pub fn get_encrypted_votes_by_poll(&self, poll_id: u32) -> Result<Vec<EncryptedVote>> {
        Ok(lock(&self.encrypted_votes)?
            .iter()
            .filter(|v| v.poll_id == poll_id)
            .cloned()
            .collect())
    }

    pub fn get_encrypted_vote_by_poll_and_voter(
        &self,
        poll_id: u32,
        voter_member_hex: &str,
    ) -> Result<Option<EncryptedVote>> {
        Ok(lock(&self.encrypted_votes)?
            .iter()
            .find(|v| v.poll_id == poll_id && v.voter_member_hex == voter_member_hex)
            .cloned())
    }
}
