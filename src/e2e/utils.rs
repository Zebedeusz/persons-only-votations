use super::crypto_helpers::{
    TestKeyPair, create_poll_signature_message, create_vote_signature_message,
};
use super::setup::E2eTestSetup;
use crate::crypto::contexts::{context_to_string, generate_poll_context};
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
        let signature = creator_keypair
            .sign_message(&message)
            .expect("Failed to sign message");

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
        // Generate the correct poll context
        let context_bytes = generate_poll_context(poll_id);
        let poll_context = context_to_string(&context_bytes);

        // Create signature message
        let message = create_vote_signature_message(
            poll_id,
            &voter_keypair.member_hex,
            vote_type,
            &poll_context,
        );

        // Sign the message
        let signature = voter_keypair
            .sign_message(&message)
            .expect("Failed to sign message");

        let mut vote_data = HashMap::new();
        vote_data.insert("poll_id", Value::Number(poll_id.into()));
        vote_data.insert(
            "voter_member_hex",
            Value::String(voter_keypair.member_hex.clone()),
        );
        vote_data.insert("vote_type", Value::String(vote_type.to_string()));
        vote_data.insert("voter_signature", Value::String(signature));
        // For e2e testing, we need to generate real ring proofs - no more test bypasses
        // This will fail with an appropriate error if we can't generate a real proof
        let ring_proof = "RING_PROOF_PLACEHOLDER".to_string(); // Will be replaced by async version

        vote_data.insert("ring_proof", Value::String(ring_proof));

        vote_data
    }

    pub async fn create_signed_vote_data_with_real_ring_proof(
        poll_id: u32,
        vote_type: &str,
        voter_keypair: &TestKeyPair,
        setup: &E2eTestSetup,
    ) -> Result<HashMap<&'static str, Value>, Box<dyn std::error::Error + Send + Sync>> {
        use crate::crypto::contexts::{context_to_string, generate_poll_context};
        use crate::e2e::crypto_helpers::create_vote_signature_message;

        // Generate the correct poll context
        let context_bytes = generate_poll_context(poll_id);
        let poll_context = context_to_string(&context_bytes);

        // Create signature message
        let message = create_vote_signature_message(
            poll_id,
            &voter_keypair.member_hex,
            vote_type,
            &poll_context,
        );

        // Sign the message
        let signature = voter_keypair
            .sign_message(&message)
            .expect("Failed to sign message");

        // Get the voter's ring index by querying the chain
        let voter_ring_index =
            Self::get_voter_ring_index_from_chain(&voter_keypair.member_hex, setup).await?;

        // Get the individual ring keys from the chain
        let ring_keys = Self::get_ring_keys_from_chain(voter_ring_index, setup).await?;

        // Generate the real ring proof
        let ring_proof =
            voter_keypair.generate_ring_proof(ring_keys.into_iter(), &context_bytes, &message)?;

        let mut vote_data = HashMap::new();
        vote_data.insert("poll_id", Value::Number(poll_id.into()));
        vote_data.insert(
            "voter_member_hex",
            Value::String(voter_keypair.member_hex.clone()),
        );
        vote_data.insert("vote_type", Value::String(vote_type.to_string()));
        vote_data.insert("voter_signature", Value::String(signature));
        vote_data.insert("ring_proof", Value::String(ring_proof));

        Ok(vote_data)
    }

    async fn get_voter_ring_index_from_chain(
        member_hex: &str,
        setup: &E2eTestSetup,
    ) -> Result<u16, Box<dyn std::error::Error + Send + Sync>> {
        use crate::chain::subxt_generated_people_apis::api as people_api;

        let people_client = setup
            .people_client()
            .ok_or("People chain client not available")?;

        // Parse the hex member key
        let member_bytes =
            hex::decode(member_hex).map_err(|e| format!("Invalid hex format: {e}"))?;

        if member_bytes.len() != 32 {
            return Err("Member key must be 32 bytes".into());
        }

        // Create EncodedPublicKey
        let mut key_array = [0u8; 32];
        key_array.copy_from_slice(&member_bytes);
        let encoded_key =
            people_api::runtime_types::verifiable::ring_vrf_impl::EncodedPublicKey(key_array);

        // Query Keys storage to get PersonalId
        let storage_query = people_api::storage().people().keys(&encoded_key);
        let personal_id = people_client
            .storage()
            .at_latest()
            .await?
            .fetch(&storage_query)
            .await?
            .ok_or("Person not found in Keys storage")?;

        // Query People storage to get PersonRecord
        let people_query = people_api::storage().people().people(personal_id);
        let person_record = people_client
            .storage()
            .at_latest()
            .await?
            .fetch(&people_query)
            .await?
            .ok_or("Person record not found")?;

        // Extract ring index from position
        match person_record.position {
            people_api::runtime_types::pallet_people_multi::types::RingPosition::Included {
                ring_index,
                ..
            } => Ok(ring_index as u16),
            _ => Err("Person is not included in any ring (may be onboarding)".into()),
        }
    }

    async fn get_ring_keys_from_chain(
        ring_index: u16,
        setup: &E2eTestSetup,
    ) -> Result<
        Vec<verifiable::ring_vrf_impl::EncodedPublicKey>,
        Box<dyn std::error::Error + Send + Sync>,
    > {
        use crate::chain::subxt_generated_people_apis::api as people_api;

        let people_client = setup
            .people_client()
            .ok_or("People chain client not available")?;

        // Query RingKeys storage to get the individual keys
        let keys_query = people_api::storage().people().ring_keys(ring_index as u32);
        let ring_keys = people_client
            .storage()
            .at_latest()
            .await?
            .fetch(&keys_query)
            .await?
            .ok_or_else(|| format!("Ring keys not found for ring index {}", ring_index))?;

        // Query RingKeysStatus to know how many keys are included
        let status_query = people_api::storage()
            .people()
            .ring_keys_status(ring_index as u32);
        let ring_status = people_client
            .storage()
            .at_latest()
            .await?
            .fetch(&status_query)
            .await?
            .ok_or_else(|| format!("Ring keys status not found for ring index {}", ring_index))?;

        // Convert BoundedVec to Vec and take only the included keys
        let included_keys: Vec<verifiable::ring_vrf_impl::EncodedPublicKey> = ring_keys
            .0 // Access the inner Vec from BoundedVec
            .into_iter()
            .take(ring_status.included as usize)
            .map(|key| verifiable::ring_vrf_impl::EncodedPublicKey(key.0)) // Convert between types
            .collect();

        Ok(included_keys)
    }
}
