use super::{bytes32_to_hex, hex_to_bytes32};
use crate::storage::models::vote::VoteType;
use blake2::{Blake2b512, Digest};
use rand::{RngCore, SeedableRng};
use rand_chacha::ChaCha20Rng;

pub fn generate_nonce() -> [u8; 32] {
    let mut rng = ChaCha20Rng::from_entropy();
    let mut nonce = [0u8; 32];
    rng.fill_bytes(&mut nonce);
    nonce
}

pub fn create_vote_commitment(vote: &VoteType, nonce: &[u8; 32]) -> [u8; 32] {
    let vote_bytes = match vote {
        VoteType::Yay => [1u8],
        VoteType::Nay => [0u8],
    };

    let mut hasher = Blake2b512::new();
    hasher.update(vote_bytes);
    hasher.update(nonce);
    let result = hasher.finalize();

    let mut commitment = [0u8; 32];
    commitment.copy_from_slice(&result[..32]);
    commitment
}

pub fn verify_commitment(commitment: &[u8; 32], vote: &VoteType, nonce: &[u8; 32]) -> bool {
    let computed = create_vote_commitment(vote, nonce);
    commitment == &computed
}

pub fn commitment_to_hex(commitment: &[u8; 32]) -> String {
    bytes32_to_hex(commitment)
}

pub fn hex_to_commitment(hex_str: &str) -> Result<[u8; 32], hex::FromHexError> {
    hex_to_bytes32(hex_str)
}

pub fn nonce_to_hex(nonce: &[u8; 32]) -> String {
    bytes32_to_hex(nonce)
}

pub fn hex_to_nonce(hex_str: &str) -> Result<[u8; 32], hex::FromHexError> {
    hex_to_bytes32(hex_str)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_vote_commitment() {
        let nonce = generate_nonce();
        let vote = VoteType::Yay;

        let commitment1 = create_vote_commitment(&vote, &nonce);
        let commitment2 = create_vote_commitment(&vote, &nonce);

        assert_eq!(commitment1, commitment2);
    }

    #[test]
    fn test_different_votes_different_commitments() {
        let nonce = generate_nonce();

        let yay_commitment = create_vote_commitment(&VoteType::Yay, &nonce);
        let nay_commitment = create_vote_commitment(&VoteType::Nay, &nonce);

        assert_ne!(yay_commitment, nay_commitment);
    }

    #[test]
    fn test_different_nonces_different_commitments() {
        let nonce1 = generate_nonce();
        let nonce2 = generate_nonce();
        let vote = VoteType::Yay;

        let commitment1 = create_vote_commitment(&vote, &nonce1);
        let commitment2 = create_vote_commitment(&vote, &nonce2);

        assert_ne!(commitment1, commitment2);
    }

    #[test]
    fn test_verify_commitment() {
        let nonce = generate_nonce();
        let vote = VoteType::Yay;
        let commitment = create_vote_commitment(&vote, &nonce);

        assert!(verify_commitment(&commitment, &vote, &nonce));
        assert!(!verify_commitment(&commitment, &VoteType::Nay, &nonce));
    }

    #[test]
    fn test_hex_conversions() {
        let nonce = generate_nonce();
        let vote = VoteType::Yay;
        let commitment = create_vote_commitment(&vote, &nonce);

        let commitment_hex = commitment_to_hex(&commitment);
        let recovered_commitment = hex_to_commitment(&commitment_hex).unwrap();
        assert_eq!(commitment, recovered_commitment);

        let nonce_hex = nonce_to_hex(&nonce);
        let recovered_nonce = hex_to_nonce(&nonce_hex).unwrap();
        assert_eq!(nonce, recovered_nonce);
    }
}
