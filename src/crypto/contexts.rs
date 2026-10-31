use blake2::{Blake2b512, Digest};

/// Ring VRF context of a poll: Blake2b("PERSONS_POLL_{id}") truncated to 32 bytes.
/// A distinct context per poll gives each voter a distinct alias per poll.
pub fn generate_poll_context(poll_id: u32) -> [u8; 32] {
    let mut hasher = Blake2b512::new();
    hasher.update(format!("PERSONS_POLL_{poll_id}").as_bytes());
    hasher.finalize()[..32].try_into().expect("32 bytes")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn context_is_unique_per_poll() {
        assert_eq!(generate_poll_context(123), generate_poll_context(123));
        assert_ne!(generate_poll_context(123), generate_poll_context(124));
    }
}
