use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Poll {
    pub id: u32,
    pub creator_member_hex: String, // Hex-encoded EncodedPublicKey from verifiable crypto
    pub name: String,
    pub description: String,
    pub yay_votes: u32,
    pub nay_votes: u32,
    pub created_at: i64,
    pub ends_at: i64,
    pub context: String,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PollData {
    pub name: String,
    pub description: String,
    pub ends_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePollRequest {
    pub creator_member_hex: String, // Hex-encoded EncodedPublicKey from verifiable crypto
    pub creator_signature: String,  // Hex-encoded verifiable crypto signature
    pub poll_data: PollData,
}

impl Poll {
    pub fn new(
        id: u32,
        creator_member_hex: String,
        poll_data: PollData,
        context: String,
        created_at: i64,
    ) -> Self {
        Self {
            id,
            creator_member_hex,
            name: poll_data.name,
            description: poll_data.description,
            yay_votes: 0,
            nay_votes: 0,
            created_at,
            ends_at: poll_data.ends_at,
            context,
            is_active: true,
        }
    }
}
