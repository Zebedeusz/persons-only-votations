use sp_core::{sr25519, Pair};

pub struct TestKeyPair {
    pub public_key_hex: String,
    pub pair: sr25519::Pair,
}

impl TestKeyPair {
    pub fn generate() -> Self {
        let pair = sr25519::Pair::generate().0;
        let public_key_hex = hex::encode(pair.public().0);
        
        Self {
            public_key_hex,
            pair,
        }
    }

    pub fn sign_message(&self, message: &[u8]) -> String {
        let signature = self.pair.sign(message);
        hex::encode(signature.0)
    }
}

pub fn create_poll_signature_message(
    poll_name: &str,
    poll_description: &str,
    ends_at: i64,
) -> Vec<u8> {
    // This matches the poll_service.rs create_signature_message method
    // which JSON serializes the poll_data
    let poll_data = serde_json::json!({
        "name": poll_name,
        "description": poll_description,
        "ends_at": ends_at
    });
    
    serde_json::to_string(&poll_data)
        .unwrap()
        .as_bytes()
        .to_vec()
}

pub fn create_vote_signature_message(
    poll_id: u32,
    voter_public_key: &str,
    vote_type: &str,
) -> Vec<u8> {
    // For votes, we'll create a simple message format
    // The vote service only checks personhood, not direct signature validation
    format!("vote:{}:{}:{}", poll_id, voter_public_key, vote_type)
        .as_bytes()
        .to_vec()
}