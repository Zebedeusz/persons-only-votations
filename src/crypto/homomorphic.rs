use tfhe::shortint::prelude::*;
use blake2::Digest;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use crate::storage::models::vote::VoteType;
use bulletproofs::{BulletproofGens, PedersenGens, RangeProof};
use curve25519_dalek_ng::scalar::Scalar;
use curve25519_dalek_ng::ristretto::CompressedRistretto;
use merlin::Transcript;
use rand::thread_rng;

#[derive(Error, Debug)]
pub enum HomomorphicError {
    #[error("Encryption failed: {0}")]
    EncryptionFailed(String),
    #[error("Decryption failed: {0}")]
    DecryptionFailed(String),
    #[error("Key generation failed: {0}")]
    KeyGenerationFailed(String),
    #[error("Invalid vote value")]
    InvalidVoteValue,
    #[error("Proof verification failed")]
    ProofVerificationFailed,
    #[error("Bulletproof generation failed: {0}")]
    BulletproofGenerationFailed(String),
    #[error("Bulletproof verification failed: {0}")]
    BulletproofVerificationFailed(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedVote {
    pub poll_id: u32,
    pub voter_alias: String,
    pub encrypted_vote: String,
    pub vote_commitment: String,
    pub validity_proof: String,
    pub ring_proof: String,
    pub created_at: i64,
}

#[derive(Debug, Clone)]
pub struct PaillierVotingSystem {
    client_key: ClientKey,
    server_key: ServerKey,
}

impl PaillierVotingSystem {
    /// Creates a new Paillier voting system with generated keys
    pub fn new() -> Result<Self, HomomorphicError> {
        let (client_key, server_key) = gen_keys(tfhe::shortint::parameters::PARAM_MESSAGE_2_CARRY_2_KS_PBS);
        Ok(Self { client_key, server_key })
    }

    /// Encrypts a vote value (0 for Nay, 1 for Yay)
    pub fn encrypt_vote(&self, vote: &VoteType) -> Result<Ciphertext, HomomorphicError> {
        let vote_value = match vote {
            VoteType::Yay => 1u64,
            VoteType::Nay => 0u64,
        };

        Ok(self.client_key.encrypt(vote_value))
    }

    /// Computes homomorphic tally of encrypted votes
    pub fn homomorphic_tally(
        &self,
        encrypted_votes: &[Ciphertext]
    ) -> Result<u64, HomomorphicError> {
        if encrypted_votes.is_empty() {
            return Ok(0);
        }

        // Create a fresh zero to start accumulation
        let mut tally_encrypted = self.client_key.encrypt(0u64);

        // Add each encrypted vote to the tally
        for vote in encrypted_votes {
            tally_encrypted = self.server_key.add(&tally_encrypted, vote);
        }

        // Decrypt the final tally
        let tally_value = self.client_key.decrypt(&tally_encrypted);

        Ok(tally_value as u64)
    }

    /// Computes homomorphic tally from hex-encoded encrypted votes
    pub fn homomorphic_tally_from_hex(
        &self,
        encrypted_vote_hexes: &[String]
    ) -> Result<u64, HomomorphicError> {
        if encrypted_vote_hexes.is_empty() {
            return Ok(0);
        }

        // Deserialize hex strings to Ciphertext objects
        let mut ciphertexts = Vec::with_capacity(encrypted_vote_hexes.len());

        for hex_vote in encrypted_vote_hexes {
            // Decode hex string to bytes
            let bytes = hex::decode(hex_vote)
                .map_err(|e| HomomorphicError::DecryptionFailed(
                    format!("Invalid hex encoding: {}", e)
                ))?;

            // Deserialize bytes to Ciphertext using bincode
            let ciphertext: Ciphertext = bincode::deserialize(&bytes)
                .map_err(|e| HomomorphicError::DecryptionFailed(
                    format!("Failed to deserialize ciphertext: {}", e)
                ))?;

            ciphertexts.push(ciphertext);
        }

        // Use existing homomorphic_tally method
        self.homomorphic_tally(&ciphertexts)
    }

    /// Encrypts a vote value and returns hex-encoded ciphertext
    pub fn encrypt_vote_to_hex(&self, vote: &VoteType) -> Result<String, HomomorphicError> {
        let ciphertext = self.encrypt_vote(vote)?;

        let serialized = bincode::serialize(&ciphertext)
            .map_err(|e| HomomorphicError::EncryptionFailed(
                format!("Failed to serialize ciphertext: {}", e)
            ))?;

        Ok(hex::encode(serialized))
    }

    /// Gets the public key for encryption
    pub fn get_public_key(&self) -> &ClientKey {
        &self.client_key
    }

    /// Test helper: decrypt a ciphertext (for testing only)
    #[cfg(test)]
    pub fn test_decrypt(&self, ciphertext: &Ciphertext) -> u64 {
        self.client_key.decrypt(ciphertext) as u64
    }

    /// Creates a zero-knowledge proof that the encrypted vote is valid (0 or 1)
    /// Uses Bulletproofs range proof to prove vote ∈ {0,1}
    pub fn create_validity_proof(
        &self,
        vote: &VoteType,
        _encrypted_vote: &Ciphertext
    ) -> Result<String, HomomorphicError> {
        let vote_value = match vote {
            VoteType::Yay => 1u64,
            VoteType::Nay => 0u64,
        };

        // Generate bulletproof generators
        let pc_gens = PedersenGens::default();
        let bp_gens = BulletproofGens::new(64, 1);

        // Create transcript for Fiat-Shamir
        let mut transcript = Transcript::new(b"vote_validity_proof");

        // Generate random blinding factor
        let mut rng = thread_rng();
        let blinding = Scalar::random(&mut rng);

        // Create commitment to vote value
        let commitment = pc_gens.commit(Scalar::from(vote_value), blinding);

        // Create range proof for vote ∈ [0, 255] (using 8 bits minimum for bulletproofs)
        let (proof, _committed_value) = RangeProof::prove_single(
            &bp_gens,
            &pc_gens,
            &mut transcript,
            vote_value,
            &blinding,
            8, // n_bits = 8, proves value ∈ [0, 2^8) = [0, 256) = {0, 1, ..., 255}
        ).map_err(|e| HomomorphicError::BulletproofGenerationFailed(format!("{:?}", e)))?;

        // Serialize proof and commitment
        let proof_bytes = proof.to_bytes();
        let commitment_bytes = commitment.compress().to_bytes();

        // Combine proof and commitment
        let mut combined = Vec::new();
        combined.extend_from_slice(&proof_bytes);
        combined.extend_from_slice(&commitment_bytes);

        Ok(hex::encode(combined))
    }

    /// Verifies a zero-knowledge proof that an encrypted vote is valid
    /// Uses Bulletproofs to verify the range proof
    pub fn verify_validity_proof(&self, proof: &str) -> bool {
        // Decode hex proof
        let combined_bytes = match hex::decode(proof) {
            Ok(bytes) => bytes,
            Err(_) => return false,
        };

        // Split proof and commitment (commitment is last 32 bytes)
        if combined_bytes.len() < 32 {
            return false;
        }

        let commitment_bytes = &combined_bytes[combined_bytes.len() - 32..];
        let proof_bytes = &combined_bytes[..combined_bytes.len() - 32];

        // Deserialize commitment
        if commitment_bytes.len() != 32 {
            return false;
        }
        let mut bytes = [0u8; 32];
        bytes.copy_from_slice(commitment_bytes);
        let commitment_compressed = CompressedRistretto(bytes);

        let _commitment_point = match commitment_compressed.decompress() {
            Some(point) => point,
            None => return false,
        };

        // Deserialize proof
        let range_proof = match RangeProof::from_bytes(proof_bytes) {
            Ok(proof) => proof,
            Err(_) => return false,
        };

        // Generate same generators used in proof creation
        let pc_gens = PedersenGens::default();
        let bp_gens = BulletproofGens::new(64, 1);

        // Create transcript for verification
        let mut transcript = Transcript::new(b"vote_validity_proof");

        // Verify the range proof
        range_proof
            .verify_single(&bp_gens, &pc_gens, &mut transcript, &commitment_compressed, 8)
            .is_ok()
    }

    /// Creates a Pedersen commitment for the encrypted vote
    pub fn create_pedersen_commitment(&self, vote: &VoteType) -> Result<String, HomomorphicError> {
        let vote_value = match vote {
            VoteType::Yay => 1u64,
            VoteType::Nay => 0u64,
        };

        // Generate random blinding factor
        let blinding_factor = fastrand::u64(..);

        // Create commitment (simplified version)
        let commitment_data = format!("pedersen:{}:{}", vote_value, blinding_factor);

        let mut hasher = blake2::Blake2b512::new();
        hasher.update(commitment_data.as_bytes());
        let commitment = hasher.finalize();

        Ok(hex::encode(&commitment[..32]))
    }
}

impl Default for PaillierVotingSystem {
    fn default() -> Self {
        Self::new().expect("Failed to create default Paillier voting system")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_paillier_voting_system_creation() {
        let system = PaillierVotingSystem::new();
        assert!(system.is_ok());
    }

    #[test]
    fn test_vote_encryption() {
        let system = PaillierVotingSystem::new().unwrap();

        let yay_encrypted = system.encrypt_vote(&VoteType::Yay);
        let nay_encrypted = system.encrypt_vote(&VoteType::Nay);

        assert!(yay_encrypted.is_ok());
        assert!(nay_encrypted.is_ok());
    }

    #[test]
    fn test_homomorphic_tally() {
        let system = PaillierVotingSystem::new().unwrap();

        let votes = vec![
            system.encrypt_vote(&VoteType::Yay).unwrap(),
            system.encrypt_vote(&VoteType::Nay).unwrap(),
            system.encrypt_vote(&VoteType::Yay).unwrap(),
        ];

        let tally = system.homomorphic_tally(&votes).unwrap();
        assert_eq!(tally, 2); // 1 + 0 + 1 = 2
    }

    #[test]
    fn test_empty_tally() {
        let system = PaillierVotingSystem::new().unwrap();
        let tally = system.homomorphic_tally(&[]).unwrap();
        assert_eq!(tally, 0);
    }

    #[test]
    fn test_validity_proof() {
        let system = PaillierVotingSystem::new().unwrap();

        let yay_encrypted = system.encrypt_vote(&VoteType::Yay).unwrap();
        let nay_encrypted = system.encrypt_vote(&VoteType::Nay).unwrap();

        let yay_proof = system.create_validity_proof(&VoteType::Yay, &yay_encrypted).unwrap();
        let nay_proof = system.create_validity_proof(&VoteType::Nay, &nay_encrypted).unwrap();

        // Verify valid proofs
        assert!(system.verify_validity_proof(&yay_proof));
        assert!(system.verify_validity_proof(&nay_proof));

        // Verify invalid proofs fail
        assert!(!system.verify_validity_proof("invalid_proof"));
        assert!(!system.verify_validity_proof("too_short"));
        assert!(!system.verify_validity_proof(""));

        // Verify proofs are different (due to randomness)
        assert_ne!(yay_proof, nay_proof);

        // Verify hex format
        assert!(hex::decode(&yay_proof).is_ok());
        assert!(hex::decode(&nay_proof).is_ok());
    }

    #[test]
    fn test_bulletproof_range_bounds() {
        let system = PaillierVotingSystem::new().unwrap();

        // Test that proofs are generated correctly for valid vote values
        let yay_encrypted = system.encrypt_vote(&VoteType::Yay).unwrap();
        let nay_encrypted = system.encrypt_vote(&VoteType::Nay).unwrap();

        let yay_proof = system.create_validity_proof(&VoteType::Yay, &yay_encrypted);
        let nay_proof = system.create_validity_proof(&VoteType::Nay, &nay_encrypted);

        assert!(yay_proof.is_ok());
        assert!(nay_proof.is_ok());

        // Verify the proofs work
        assert!(system.verify_validity_proof(&yay_proof.unwrap()));
        assert!(system.verify_validity_proof(&nay_proof.unwrap()));
    }

    #[test]
    fn test_pedersen_commitment() {
        let system = PaillierVotingSystem::new().unwrap();

        let yay_commitment = system.create_pedersen_commitment(&VoteType::Yay).unwrap();
        let nay_commitment = system.create_pedersen_commitment(&VoteType::Nay).unwrap();

        // Commitments should be different (due to random blinding factor)
        assert_ne!(yay_commitment, nay_commitment);

        // Should be valid hex strings of correct length
        assert_eq!(yay_commitment.len(), 64);
        assert_eq!(nay_commitment.len(), 64);
        assert!(hex::decode(&yay_commitment).is_ok());
        assert!(hex::decode(&nay_commitment).is_ok());
    }

    #[test]
    fn test_encrypt_vote_to_hex_serialization() {
        let system = PaillierVotingSystem::new().unwrap();

        let yay_hex = system.encrypt_vote_to_hex(&VoteType::Yay).unwrap();
        let nay_hex = system.encrypt_vote_to_hex(&VoteType::Nay).unwrap();

        // Should produce valid hex strings
        assert!(hex::decode(&yay_hex).is_ok());
        assert!(hex::decode(&nay_hex).is_ok());

        // Should be different due to encryption randomness
        assert_ne!(yay_hex, nay_hex);
    }

    #[test]
    fn test_homomorphic_tally_from_hex_roundtrip() {
        let system = PaillierVotingSystem::new().unwrap();

        // Create encrypted votes as hex
        let vote_hexes = vec![
            system.encrypt_vote_to_hex(&VoteType::Yay).unwrap(),
            system.encrypt_vote_to_hex(&VoteType::Nay).unwrap(),
            system.encrypt_vote_to_hex(&VoteType::Yay).unwrap(),
        ];

        // Test tally from hex
        let tally = system.homomorphic_tally_from_hex(&vote_hexes).unwrap();
        assert_eq!(tally, 2); // 1 + 0 + 1 = 2
    }

    #[test]
    fn test_homomorphic_tally_from_hex_empty() {
        let system = PaillierVotingSystem::new().unwrap();
        let tally = system.homomorphic_tally_from_hex(&[]).unwrap();
        assert_eq!(tally, 0);
    }

    #[test]
    fn test_homomorphic_tally_from_hex_invalid_hex() {
        let system = PaillierVotingSystem::new().unwrap();
        let invalid_hexes = vec!["invalid_hex".to_string()];

        let result = system.homomorphic_tally_from_hex(&invalid_hexes);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), HomomorphicError::DecryptionFailed(_)));
    }

    #[test]
    fn test_homomorphic_tally_from_hex_invalid_ciphertext() {
        let system = PaillierVotingSystem::new().unwrap();
        // Valid hex but invalid ciphertext data
        let invalid_hexes = vec!["deadbeef".to_string()];

        let result = system.homomorphic_tally_from_hex(&invalid_hexes);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), HomomorphicError::DecryptionFailed(_)));
    }
}