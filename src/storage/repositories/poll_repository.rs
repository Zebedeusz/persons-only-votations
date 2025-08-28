use crate::storage::models::poll::Poll;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum PollRepositoryError {
    #[error("Poll with id {0} not found")]
    PollNotFound(u32),
    #[error("Failed to acquire lock on storage")]
    LockError,
}

pub type Result<T> = std::result::Result<T, PollRepositoryError>;

pub trait Repository: Send + Sync {
    fn create_poll(&self, poll: Poll) -> Result<u32>;
    fn get_poll_by_id(&self, id: u32) -> Result<Option<Poll>>;
    fn get_polls_by_creator(&self, creator_key: &str) -> Result<Vec<Poll>>;
    fn get_all_polls(&self) -> Result<Vec<Poll>>;
    fn update_poll(&self, poll: Poll) -> Result<()>;
}

#[derive(Debug, Clone)]
pub struct PollRepository {
    storage: Arc<Mutex<HashMap<u32, Poll>>>,
}

impl PollRepository {
    pub fn new() -> Self {
        Self {
            storage: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    fn create_poll_impl(&self, mut poll: Poll) -> Result<u32> {
        let mut storage = self
            .storage
            .lock()
            .map_err(|_| PollRepositoryError::LockError)?;

        // Generate random ID and ensure it doesn't exist
        let id = loop {
            let candidate = fastrand::u32(1..u32::MAX);
            if !storage.contains_key(&candidate) {
                break candidate;
            }
        };

        poll.id = id;
        storage.insert(id, poll);

        Ok(id)
    }

    fn get_poll_by_id_impl(&self, id: u32) -> Result<Option<Poll>> {
        let storage = self
            .storage
            .lock()
            .map_err(|_| PollRepositoryError::LockError)?;
        Ok(storage.get(&id).cloned())
    }

    fn get_polls_by_creator_impl(&self, creator_key: &str) -> Result<Vec<Poll>> {
        let storage = self
            .storage
            .lock()
            .map_err(|_| PollRepositoryError::LockError)?;
        let polls = storage
            .values()
            .filter(|poll| poll.creator_member_hex == creator_key)
            .cloned()
            .collect();
        Ok(polls)
    }

    fn get_all_polls_impl(&self) -> Result<Vec<Poll>> {
        let storage = self
            .storage
            .lock()
            .map_err(|_| PollRepositoryError::LockError)?;
        let polls = storage.values().cloned().collect();
        Ok(polls)
    }

    fn update_poll_impl(&self, poll: Poll) -> Result<()> {
        let mut storage = self
            .storage
            .lock()
            .map_err(|_| PollRepositoryError::LockError)?;
        if let std::collections::hash_map::Entry::Occupied(mut e) = storage.entry(poll.id) {
            e.insert(poll);
            Ok(())
        } else {
            Err(PollRepositoryError::PollNotFound(poll.id))
        }
    }
}

impl Repository for PollRepository {
    fn create_poll(&self, poll: Poll) -> Result<u32> {
        self.create_poll_impl(poll)
    }

    fn get_poll_by_id(&self, id: u32) -> Result<Option<Poll>> {
        self.get_poll_by_id_impl(id)
    }

    fn get_polls_by_creator(&self, creator_key: &str) -> Result<Vec<Poll>> {
        self.get_polls_by_creator_impl(creator_key)
    }

    fn get_all_polls(&self) -> Result<Vec<Poll>> {
        self.get_all_polls_impl()
    }

    fn update_poll(&self, poll: Poll) -> Result<()> {
        self.update_poll_impl(poll)
    }
}

impl Default for PollRepository {
    fn default() -> Self {
        Self::new()
    }
}
