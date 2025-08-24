use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Poll {
    pub id: u32,
    pub creator_public_key: String,
    pub name: String,
    pub description: String,
    pub is_public: bool,
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
    pub is_public: bool,
    pub ends_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePollRequest {
    pub creator_public_key: String,
    pub creator_signature: String,
    pub poll_data: PollData,
}

impl Poll {
    pub fn new(
        id: u32,
        creator_public_key: String,
        poll_data: PollData,
        context: String,
        created_at: i64,
    ) -> Self {
        Self {
            id,
            creator_public_key,
            name: poll_data.name,
            description: poll_data.description,
            is_public: poll_data.is_public,
            yay_votes: 0,
            nay_votes: 0,
            created_at,
            ends_at: poll_data.ends_at,
            context,
            is_active: true,
        }
    }
}
