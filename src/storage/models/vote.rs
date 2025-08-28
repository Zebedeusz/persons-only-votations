use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum VoteType {
    Yay,
    Nay,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vote {
    pub poll_id: u32,
    pub voter_member_hex: String, // Hex-encoded EncodedPublicKey for personhood verification
    pub voter_alias: String,      // Derived from ring proof for deduplication
    pub vote_type: VoteType,
    pub voter_signature: String, // Hex-encoded verifiable crypto signature
    pub ring_proof: String,      // Hex-encoded ring proof
    pub created_at: i64,
}

impl Vote {
    pub fn new(
        poll_id: u32,
        voter_member_hex: String,
        voter_alias: String,
        vote_type: VoteType,
        voter_signature: String,
        ring_proof: String,
        created_at: i64,
    ) -> Self {
        Self {
            poll_id,
            voter_member_hex,
            voter_alias,
            vote_type,
            voter_signature,
            ring_proof,
            created_at,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CastVoteRequest {
    pub poll_id: u32,
    pub voter_member_hex: String, // Hex-encoded EncodedPublicKey
    pub vote_type: VoteType,
    pub voter_signature: String, // Hex-encoded verifiable crypto signature
    pub ring_proof: String,      // Hex-encoded ring proof from client
}

#[derive(Debug, Clone, Serialize)]
pub struct CastVoteResponse {
    pub poll_id: u32,
    pub message: String,
}
