use std::time::{SystemTime, UNIX_EPOCH};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TimeError {
    #[error("System time error")]
    SystemTimeError,
}

pub fn get_current_timestamp() -> Result<i64, TimeError> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| TimeError::SystemTimeError)
        .map(|duration| duration.as_secs() as i64)
}
