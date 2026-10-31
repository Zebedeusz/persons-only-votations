use crate::storage::models::poll::Poll;
use std::collections::HashMap;
use std::sync::{Mutex, MutexGuard};
use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum PollRepositoryError {
    #[error("Poll with id {0} not found")]
    PollNotFound(u32),
    #[error("Failed to acquire lock on storage")]
    LockError,
}

pub type Result<T> = std::result::Result<T, PollRepositoryError>;

/// In-memory poll store.
#[derive(Default)]
pub struct PollRepository {
    storage: Mutex<HashMap<u32, Poll>>,
}

impl PollRepository {
    fn storage(&self) -> Result<MutexGuard<'_, HashMap<u32, Poll>>> {
        self.storage
            .lock()
            .map_err(|_| PollRepositoryError::LockError)
    }

    /// Stores the poll under a fresh random id and returns that id.
    pub fn create_poll(&self, mut poll: Poll) -> Result<u32> {
        let mut storage = self.storage()?;
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

    pub fn get_poll_by_id(&self, id: u32) -> Result<Option<Poll>> {
        Ok(self.storage()?.get(&id).cloned())
    }

    pub fn get_all_polls(&self) -> Result<Vec<Poll>> {
        Ok(self.storage()?.values().cloned().collect())
    }

    pub fn update_poll(&self, poll: Poll) -> Result<()> {
        let mut storage = self.storage()?;
        if !storage.contains_key(&poll.id) {
            return Err(PollRepositoryError::PollNotFound(poll.id));
        }
        storage.insert(poll.id, poll);
        Ok(())
    }
}
