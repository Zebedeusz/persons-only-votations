use std::time::{SystemTime, UNIX_EPOCH};

/// Returns the current Unix time in seconds.
pub fn get_current_timestamp() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock is set before 1970")
        .as_secs() as i64
}
