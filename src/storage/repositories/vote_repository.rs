use crate::storage::models::vote::Vote;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
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

pub trait VoteRepository: Send + Sync {
    fn create_vote(&self, vote: Vote) -> Result<()>;
    fn get_votes_by_poll(&self, poll_id: u32) -> Result<Vec<Vote>>;
    fn get_vote_by_poll_and_voter(
        &self,
        poll_id: u32,
        voter_public_key: &str,
    ) -> Result<Option<Vote>>;
    fn get_vote_by_poll_and_alias(&self, poll_id: u32, voter_alias: &str) -> Result<Option<Vote>>;
    fn count_votes_by_poll(&self, poll_id: u32) -> Result<(u32, u32)>; // (yay_count, nay_count)
}

pub struct InMemoryVoteRepository {
    votes: Arc<Mutex<Vec<Vote>>>,
    voter_poll_index: Arc<Mutex<HashMap<(u32, String), bool>>>, // (poll_id, voter_key) -> exists
    alias_poll_index: Arc<Mutex<HashMap<(u32, String), bool>>>, // (poll_id, voter_alias) -> exists
}

impl InMemoryVoteRepository {
    pub fn new() -> Self {
        Self {
            votes: Arc::new(Mutex::new(Vec::new())),
            voter_poll_index: Arc::new(Mutex::new(HashMap::new())),
            alias_poll_index: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl VoteRepository for InMemoryVoteRepository {
    fn create_vote(&self, vote: Vote) -> Result<()> {
        let mut votes = self
            .votes
            .lock()
            .map_err(|_| VoteRepositoryError::LockError)?;
        let mut voter_poll_index = self
            .voter_poll_index
            .lock()
            .map_err(|_| VoteRepositoryError::LockError)?;
        let mut alias_poll_index = self
            .alias_poll_index
            .lock()
            .map_err(|_| VoteRepositoryError::LockError)?;

        // Check for duplicate vote by public key
        let voter_index_key = (vote.poll_id, vote.voter_public_key.clone());
        if voter_poll_index.contains_key(&voter_index_key) {
            return Err(VoteRepositoryError::DuplicateVote {
                poll_id: vote.poll_id,
                voter_key: vote.voter_public_key.clone(),
            });
        }

        // Check for duplicate vote by alias (primary privacy check)
        let alias_index_key = (vote.poll_id, vote.voter_alias.clone());
        if alias_poll_index.contains_key(&alias_index_key) {
            return Err(VoteRepositoryError::DuplicateVote {
                poll_id: vote.poll_id,
                voter_key: vote.voter_alias.clone(),
            });
        }

        votes.push(vote);
        voter_poll_index.insert(voter_index_key, true);
        alias_poll_index.insert(alias_index_key, true);

        Ok(())
    }

    fn get_votes_by_poll(&self, poll_id: u32) -> Result<Vec<Vote>> {
        let votes = self
            .votes
            .lock()
            .map_err(|_| VoteRepositoryError::LockError)?;
        Ok(votes
            .iter()
            .filter(|vote| vote.poll_id == poll_id)
            .cloned()
            .collect())
    }

    fn get_vote_by_poll_and_voter(
        &self,
        poll_id: u32,
        voter_public_key: &str,
    ) -> Result<Option<Vote>> {
        let votes = self
            .votes
            .lock()
            .map_err(|_| VoteRepositoryError::LockError)?;

        Ok(votes
            .iter()
            .find(|vote| vote.poll_id == poll_id && vote.voter_public_key == voter_public_key)
            .cloned())
    }

    fn get_vote_by_poll_and_alias(&self, poll_id: u32, voter_alias: &str) -> Result<Option<Vote>> {
        let votes = self
            .votes
            .lock()
            .map_err(|_| VoteRepositoryError::LockError)?;

        Ok(votes
            .iter()
            .find(|vote| vote.poll_id == poll_id && vote.voter_alias == voter_alias)
            .cloned())
    }

    fn count_votes_by_poll(&self, poll_id: u32) -> Result<(u32, u32)> {
        use crate::storage::models::vote::VoteType;

        let votes = self
            .votes
            .lock()
            .map_err(|_| VoteRepositoryError::LockError)?;
        let poll_votes: Vec<&Vote> = votes
            .iter()
            .filter(|vote| vote.poll_id == poll_id)
            .collect();

        let yay_count = poll_votes
            .iter()
            .filter(|vote| vote.vote_type == VoteType::Yay)
            .count() as u32;

        let nay_count = poll_votes
            .iter()
            .filter(|vote| vote.vote_type == VoteType::Nay)
            .count() as u32;

        Ok((yay_count, nay_count))
    }
}
