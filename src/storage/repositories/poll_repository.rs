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
    fn update_poll(&self, poll: Poll) -> Result<()>;
}

#[derive(Debug, Clone)]
pub struct PollRepository {
    storage: Arc<Mutex<HashMap<u32, Poll>>>,
    next_id: Arc<Mutex<u32>>,
}

impl PollRepository {
    pub fn new() -> Self {
        Self {
            storage: Arc::new(Mutex::new(HashMap::new())),
            next_id: Arc::new(Mutex::new(1)),
        }
    }

    pub fn create_poll(&self, mut poll: Poll) -> Result<u32> {
        let mut next_id = self.next_id.lock().map_err(|_| PollRepositoryError::LockError)?;
        let mut storage = self.storage.lock().map_err(|_| PollRepositoryError::LockError)?;
        
        let id = *next_id;
        poll.id = id;
        storage.insert(id, poll);
        *next_id += 1;
        
        Ok(id)
    }

    pub fn get_poll_by_id(&self, id: u32) -> Result<Option<Poll>> {
        let storage = self.storage.lock().map_err(|_| PollRepositoryError::LockError)?;
        Ok(storage.get(&id).cloned())
    }

    pub fn get_polls_by_creator(&self, creator_key: &str) -> Result<Vec<Poll>> {
        let storage = self.storage.lock().map_err(|_| PollRepositoryError::LockError)?;
        let polls = storage
            .values()
            .filter(|poll| poll.creator_public_key == creator_key)
            .cloned()
            .collect();
        Ok(polls)
    }

    pub fn update_poll(&self, poll: Poll) -> Result<()> {
        let mut storage = self.storage.lock().map_err(|_| PollRepositoryError::LockError)?;
        if storage.contains_key(&poll.id) {
            storage.insert(poll.id, poll);
            Ok(())
        } else {
            Err(PollRepositoryError::PollNotFound(poll.id))
        }
    }
}

impl Repository for PollRepository {
    fn create_poll(&self, poll: Poll) -> Result<u32> {
        self.create_poll(poll)
    }

    fn get_poll_by_id(&self, id: u32) -> Result<Option<Poll>> {
        self.get_poll_by_id(id)
    }

    fn get_polls_by_creator(&self, creator_key: &str) -> Result<Vec<Poll>> {
        self.get_polls_by_creator(creator_key)
    }

    fn update_poll(&self, poll: Poll) -> Result<()> {
        self.update_poll(poll)
    }
}

impl Default for PollRepository {
    fn default() -> Self {
        Self::new()
    }
}