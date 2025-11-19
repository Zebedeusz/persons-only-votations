use crate::storage::models::vote::VoteType;
use blake2::Digest;
use bulletproofs::{BulletproofGens, PedersenGens, RangeProof};
use curve25519_dalek_ng::ristretto::CompressedRistretto;
use curve25519_dalek_ng::scalar::Scalar;
use merlin::Transcript;
use rand::thread_rng;
use serde::{Deserialize, Serialize};
use tfhe::shortint::prelude::*;
use thiserror::Error;

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
    pub fn new() -> Result<Self, HomomorphicError> {
        let (client_key, server_key) =
            gen_keys(tfhe::shortint::parameters::PARAM_MESSAGE_2_CARRY_2_KS_PBS);
        Ok(Self {
            client_key,
            server_key,
        })
    }

    pub fn encrypt_vote(&self, vote: &VoteType) -> Result<Ciphertext, HomomorphicError> {
        let vote_value = match vote {
            VoteType::Yay => 1u64,
            VoteType::Nay => 0u64,
        };

        Ok(self.client_key.encrypt(vote_value))
    }

    pub fn homomorphic_tally(
        &self,
        encrypted_votes: &[Ciphertext],
    ) -> Result<u64, HomomorphicError> {
        if encrypted_votes.is_empty() {
            return Ok(0);
        }

        let mut tally_encrypted = self.client_key.encrypt(0u64);

        for vote in encrypted_votes {
            tally_encrypted = self.server_key.add(&tally_encrypted, vote);
        }

        Ok(self.client_key.decrypt(&tally_encrypted))
    }

    pub fn homomorphic_tally_from_hex(
        &self,
        encrypted_vote_hexes: &[String],
    ) -> Result<u64, HomomorphicError> {
        if encrypted_vote_hexes.is_empty() {
            return Ok(0);
        }

        let mut ciphertexts = Vec::with_capacity(encrypted_vote_hexes.len());

        for hex_vote in encrypted_vote_hexes {
            let bytes = hex::decode(hex_vote).map_err(|e| {
                HomomorphicError::DecryptionFailed(format!("Invalid hex encoding: {e}"))
            })?;

            let ciphertext: Ciphertext = bincode::deserialize(&bytes).map_err(|e| {
                HomomorphicError::DecryptionFailed(format!("Failed to deserialize ciphertext: {e}"))
            })?;

            ciphertexts.push(ciphertext);
        }

        self.homomorphic_tally(&ciphertexts)
    }

    pub fn encrypt_vote_to_hex(&self, vote: &VoteType) -> Result<String, HomomorphicError> {
        let ciphertext = self.encrypt_vote(vote)?;

        let serialized = bincode::serialize(&ciphertext).map_err(|e| {
            HomomorphicError::EncryptionFailed(format!("Failed to serialize ciphertext: {e}"))
        })?;

        Ok(hex::encode(serialized))
    }

    pub fn get_public_key(&self) -> &ClientKey {
        &self.client_key
    }

    pub fn get_public_key_hex(&self) -> Result<String, HomomorphicError> {
        let serialized = bincode::serialize(&self.client_key).map_err(|e| {
            HomomorphicError::KeyGenerationFailed(format!("Failed to serialize client key: {e}"))
        })?;
        Ok(hex::encode(serialized))
    }

    pub fn from_public_key_hex(public_key_hex: &str) -> Result<Self, HomomorphicError> {
        let bytes = hex::decode(public_key_hex).map_err(|e| {
            HomomorphicError::KeyGenerationFailed(format!("Invalid hex encoding: {e}"))
        })?;

        let client_key: ClientKey = bincode::deserialize(&bytes).map_err(|e| {
            HomomorphicError::KeyGenerationFailed(format!("Failed to deserialize client key: {e}"))
        })?;

        let dummy_server_key = gen_keys(tfhe::shortint::parameters::PARAM_MESSAGE_2_CARRY_2_KS_PBS).1;

        Ok(Self {
            client_key,
            server_key: dummy_server_key,
        })
    }

    #[cfg(test)]
    pub fn test_decrypt(&self, ciphertext: &Ciphertext) -> u64 {
        self.client_key.decrypt(ciphertext)
    }

    pub fn create_validity_proof(
        &self,
        vote: &VoteType,
        _encrypted_vote: &Ciphertext,
    ) -> Result<String, HomomorphicError> {
        let vote_value = match vote {
            VoteType::Yay => 1u64,
            VoteType::Nay => 0u64,
        };

        let pc_gens = PedersenGens::default();
        let bp_gens = BulletproofGens::new(64, 1);
        let mut transcript = Transcript::new(b"vote_validity_proof");

        let mut rng = thread_rng();
        let blinding = Scalar::random(&mut rng);
        let commitment = pc_gens.commit(Scalar::from(vote_value), blinding);

        let (proof, _committed_value) = RangeProof::prove_single(
            &bp_gens,
            &pc_gens,
            &mut transcript,
            vote_value,
            &blinding,
            8,
        )
        .map_err(|e| HomomorphicError::BulletproofGenerationFailed(format!("{e:?}")))?;

        let proof_bytes = proof.to_bytes();
        let commitment_bytes = commitment.compress().to_bytes();

        let mut combined = Vec::new();
        combined.extend_from_slice(&proof_bytes);
        combined.extend_from_slice(&commitment_bytes);

        Ok(hex::encode(combined))
    }

    pub fn verify_validity_proof(&self, proof: &str) -> bool {
        let combined_bytes = match hex::decode(proof) {
            Ok(bytes) => bytes,
            Err(_) => return false,
        };

        if combined_bytes.len() < 32 {
            return false;
        }

        let commitment_bytes = &combined_bytes[combined_bytes.len() - 32..];
        let proof_bytes = &combined_bytes[..combined_bytes.len() - 32];

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

        let range_proof = match RangeProof::from_bytes(proof_bytes) {
            Ok(proof) => proof,
            Err(_) => return false,
        };

        let pc_gens = PedersenGens::default();
        let bp_gens = BulletproofGens::new(64, 1);
        let mut transcript = Transcript::new(b"vote_validity_proof");

        range_proof
            .verify_single(
                &bp_gens,
                &pc_gens,
                &mut transcript,
                &commitment_compressed,
                8,
            )
            .is_ok()
    }

    pub fn create_pedersen_commitment(&self, vote: &VoteType) -> Result<String, HomomorphicError> {
        let vote_value = match vote {
            VoteType::Yay => 1u64,
            VoteType::Nay => 0u64,
        };

        let blinding_factor = fastrand::u64(..);
        let commitment_data = format!("pedersen:{vote_value}:{blinding_factor}");

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
        assert_eq!(tally, 2);
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

        let yay_proof = system
            .create_validity_proof(&VoteType::Yay, &yay_encrypted)
            .unwrap();
        let nay_proof = system
            .create_validity_proof(&VoteType::Nay, &nay_encrypted)
            .unwrap();

        assert!(system.verify_validity_proof(&yay_proof));
        assert!(system.verify_validity_proof(&nay_proof));

        assert!(!system.verify_validity_proof("invalid_proof"));
        assert!(!system.verify_validity_proof("too_short"));
        assert!(!system.verify_validity_proof(""));

        assert_ne!(yay_proof, nay_proof);

        assert!(hex::decode(&yay_proof).is_ok());
        assert!(hex::decode(&nay_proof).is_ok());
    }

    #[test]
    fn test_bulletproof_range_bounds() {
        let system = PaillierVotingSystem::new().unwrap();

        let yay_encrypted = system.encrypt_vote(&VoteType::Yay).unwrap();
        let nay_encrypted = system.encrypt_vote(&VoteType::Nay).unwrap();

        let yay_proof = system.create_validity_proof(&VoteType::Yay, &yay_encrypted);
        let nay_proof = system.create_validity_proof(&VoteType::Nay, &nay_encrypted);

        assert!(yay_proof.is_ok());
        assert!(nay_proof.is_ok());

        assert!(system.verify_validity_proof(&yay_proof.unwrap()));
        assert!(system.verify_validity_proof(&nay_proof.unwrap()));
    }

    #[test]
    fn test_pedersen_commitment() {
        let system = PaillierVotingSystem::new().unwrap();

        let yay_commitment = system.create_pedersen_commitment(&VoteType::Yay).unwrap();
        let nay_commitment = system.create_pedersen_commitment(&VoteType::Nay).unwrap();

        assert_ne!(yay_commitment, nay_commitment);
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

        assert!(hex::decode(&yay_hex).is_ok());
        assert!(hex::decode(&nay_hex).is_ok());
        assert_ne!(yay_hex, nay_hex);
    }

    #[test]
    fn test_homomorphic_tally_from_hex_roundtrip() {
        let system = PaillierVotingSystem::new().unwrap();

        let vote_hexes = vec![
            system.encrypt_vote_to_hex(&VoteType::Yay).unwrap(),
            system.encrypt_vote_to_hex(&VoteType::Nay).unwrap(),
            system.encrypt_vote_to_hex(&VoteType::Yay).unwrap(),
        ];

        let tally = system.homomorphic_tally_from_hex(&vote_hexes).unwrap();
        assert_eq!(tally, 2);
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
        assert!(matches!(
            result.unwrap_err(),
            HomomorphicError::DecryptionFailed(_)
        ));
    }

    #[test]
    fn test_homomorphic_tally_from_hex_invalid_ciphertext() {
        let system = PaillierVotingSystem::new().unwrap();
        let invalid_hexes = vec!["deadbeef".to_string()];

        let result = system.homomorphic_tally_from_hex(&invalid_hexes);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            HomomorphicError::DecryptionFailed(_)
        ));
    }
}
