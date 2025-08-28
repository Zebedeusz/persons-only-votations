use super::crypto_helpers::{
    TestKeyPair, create_poll_signature_message, create_vote_signature_message,
};
use serde_json::Value;
use std::collections::HashMap;

pub struct TestPoll {
    pub title: String,
    pub description: String,
    pub options: Vec<String>,
    pub required_identity_proof: bool,
}

impl Default for TestPoll {
    fn default() -> Self {
        Self {
            title: "Test Poll".to_string(),
            description: "A test poll for e2e testing".to_string(),
            options: vec!["Option A".to_string(), "Option B".to_string()],
            required_identity_proof: true,
        }
    }
}

impl TestPoll {
    pub fn to_json_with_keypair(&self, creator_keypair: &TestKeyPair) -> Value {
        // Create a poll that ends in 1 hour (3600 seconds from now)
        let ends_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64
            + 3600;

        // Create the signature message (JSON of poll_data)
        let message = create_poll_signature_message(&self.title, &self.description, ends_at);

        // Sign the message
        let signature = creator_keypair.sign_message(&message).expect("Failed to sign message");

        serde_json::json!({
            "creator_member_hex": creator_keypair.member_hex,
            "creator_signature": signature,
            "poll_data": {
                "name": self.title,
                "description": self.description,
                "ends_at": ends_at
            }
        })
    }

    // Backward compatibility method - generates a new keypair each time
    pub fn to_json(&self) -> Value {
        let creator_keypair = TestKeyPair::generate();
        self.to_json_with_keypair(&creator_keypair)
    }
}

pub struct TestHelpers;

impl TestHelpers {
    pub async fn create_poll_via_api(
        http_client: &reqwest::Client,
        base_url: &str,
        poll: &TestPoll,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let response = http_client
            .post(format!("{base_url}/polls"))
            .header("Content-Type", "application/json")
            .json(&poll.to_json())
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(format!("Failed to create poll: {}", response.status()).into());
        }

        let response_body: Value = response.json().await?;
        let poll_id = response_body["poll_id"]
            .as_u64()
            .ok_or("Poll ID not found in response")?
            .to_string();

        Ok(poll_id)
    }

    pub async fn get_poll_via_api(
        http_client: &reqwest::Client,
        base_url: &str,
        poll_id: &str,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {
        let response = http_client
            .get(format!("{base_url}/polls/{poll_id}"))
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(format!("Failed to get poll: {}", response.status()).into());
        }

        Ok(response.json().await?)
    }

    pub async fn submit_vote_via_api(
        http_client: &reqwest::Client,
        base_url: &str,
        poll_id: &str,
        vote_data: &HashMap<&str, Value>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {
        let response = http_client
            .post(format!("{base_url}/polls/{poll_id}/vote"))
            .header("Content-Type", "application/json")
            .json(vote_data)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(format!("Failed to submit vote: {error_text}").into());
        }

        Ok(response.json().await?)
    }

    pub fn create_signed_vote_data(
        poll_id: u32,
        vote_type: &str,
        voter_keypair: &TestKeyPair,
    ) -> HashMap<&'static str, Value> {
        // Create signature message
        let message =
            create_vote_signature_message(poll_id, &voter_keypair.member_hex, vote_type);

        // Sign the message
        let signature = voter_keypair.sign_message(&message).expect("Failed to sign message");

        let mut vote_data = HashMap::new();
        vote_data.insert("poll_id", Value::Number(poll_id.into()));
        vote_data.insert(
            "voter_member_hex",
            Value::String(voter_keypair.member_hex.clone()),
        );
        vote_data.insert("vote_type", Value::String(vote_type.to_string()));
        vote_data.insert("voter_signature", Value::String(signature));
        vote_data.insert("ring_proof", Value::String("0123456789abcdef".to_string()));

        vote_data
    }
}
