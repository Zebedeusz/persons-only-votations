use crate::storage::models::vote::VoteType;
use blake2::{Blake2b512, Digest};
use rand::RngCore;

pub fn generate_nonce() -> [u8; 32] {
    let mut nonce = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut nonce);
    nonce
}

/// Blake2b(vote byte || nonce), truncated to 32 bytes.
pub fn create_vote_commitment(vote: &VoteType, nonce: &[u8; 32]) -> [u8; 32] {
    let vote_byte = match vote {
        VoteType::Yay => [1u8],
        VoteType::Nay => [0u8],
    };
    let mut hasher = Blake2b512::new();
    hasher.update(vote_byte);
    hasher.update(nonce);
    hasher.finalize()[..32].try_into().expect("32 bytes")
}

pub fn verify_commitment(commitment: &[u8; 32], vote: &VoteType, nonce: &[u8; 32]) -> bool {
    commitment == &create_vote_commitment(vote, nonce)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn commitment_is_deterministic_and_binding() {
        let nonce = generate_nonce();
        let commitment = create_vote_commitment(&VoteType::Yay, &nonce);

        assert_eq!(commitment, create_vote_commitment(&VoteType::Yay, &nonce));
        assert_ne!(commitment, create_vote_commitment(&VoteType::Nay, &nonce));
        assert_ne!(
            commitment,
            create_vote_commitment(&VoteType::Yay, &generate_nonce())
        );
        assert!(verify_commitment(&commitment, &VoteType::Yay, &nonce));
        assert!(!verify_commitment(&commitment, &VoteType::Nay, &nonce));
    }
}
