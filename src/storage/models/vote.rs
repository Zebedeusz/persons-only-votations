use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum VoteType {
    Yay,
    Nay,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum VotePhase {
    Committed,
    Revealed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vote {
    pub poll_id: u32,
    pub voter_member_hex: String, // Hex-encoded EncodedPublicKey for personhood verification
    pub voter_alias: String,      // Derived from ring proof for deduplication
    pub vote_commitment: String,  // Blake2b commitment of vote + nonce
    pub vote_nonce: String,       // Random nonce for commitment (empty during commit phase)
    pub vote_type: Option<VoteType>, // None during commit phase, Some during reveal phase
    pub voter_signature: String,  // Hex-encoded verifiable crypto signature
    pub ring_proof: String,       // Hex-encoded ring proof
    pub created_at: i64,
    pub revealed_at: Option<i64>, // Timestamp of reveal phase
    pub phase: VotePhase,         // Current phase of the vote
}

impl Vote {
    pub fn new_commit(
        poll_id: u32,
        voter_member_hex: String,
        voter_alias: String,
        vote_commitment: String,
        voter_signature: String,
        ring_proof: String,
        created_at: i64,
    ) -> Self {
        Self {
            poll_id,
            voter_member_hex,
            voter_alias,
            vote_commitment,
            vote_nonce: String::new(), // Empty during commit phase
            vote_type: None,           // None during commit phase
            voter_signature,
            ring_proof,
            created_at,
            revealed_at: None,
            phase: VotePhase::Committed,
        }
    }

    pub fn reveal(&mut self, vote_type: VoteType, vote_nonce: String, revealed_at: i64) {
        self.vote_type = Some(vote_type);
        self.vote_nonce = vote_nonce;
        self.revealed_at = Some(revealed_at);
        self.phase = VotePhase::Revealed;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitVoteRequest {
    pub poll_id: u32,
    pub voter_member_hex: String, // Hex-encoded EncodedPublicKey
    pub vote_commitment: String,  // Hex-encoded commitment
    pub voter_signature: String,  // Hex-encoded verifiable crypto signature
    pub ring_proof: String,       // Hex-encoded ring proof
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevealVoteRequest {
    pub poll_id: u32,
    pub voter_member_hex: String, // Hex-encoded EncodedPublicKey
    pub vote_type: VoteType,
    pub vote_nonce: String, // Hex-encoded nonce for commitment verification
}

#[derive(Debug, Clone, Serialize)]
pub struct CastVoteResponse {
    pub poll_id: u32,
    pub message: String,
}

// Encrypted voting models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedVote {
    pub poll_id: u32,
    pub voter_member_hex: String,
    pub voter_alias: String,
    pub encrypted_vote: String,        // Hex-encoded Paillier-encrypted vote
    pub vote_commitment: String,       // Pedersen commitment
    pub validity_proof: String,        // Zero-knowledge proof that vote ∈ {0,1}
    pub ring_proof: String,           // Existing anonymity proof
    pub voter_signature: String,      // Hex-encoded verifiable crypto signature
    pub created_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CastEncryptedVoteRequest {
    pub poll_id: u32,
    pub voter_member_hex: String,
    pub encrypted_vote: String,
    pub vote_commitment: String,
    pub validity_proof: String,
    pub ring_proof: String,
    pub voter_signature: String,
}
