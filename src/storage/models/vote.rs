use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum VoteType {
    Yay,
    Nay,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vote {
    pub poll_id: u32,
    pub voter_public_key: String,
    pub vote_type: VoteType,
    pub voter_signature: String,
    pub created_at: i64,
}

impl Vote {
    pub fn new(
        poll_id: u32,
        voter_public_key: String,
        vote_type: VoteType,
        voter_signature: String,
        created_at: i64,
    ) -> Self {
        Self {
            poll_id,
            voter_public_key,
            vote_type,
            voter_signature,
            created_at,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CastVoteRequest {
    pub poll_id: u32,
    pub voter_public_key: String,
    pub vote_type: VoteType,
    pub voter_signature: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct CastVoteResponse {
    pub poll_id: u32,
    pub message: String,
}