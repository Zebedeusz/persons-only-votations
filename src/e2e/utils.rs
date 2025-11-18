use super::crypto_helpers::{
    TestKeyPair, create_commit_signature_message, create_poll_signature_message,
};
use super::setup::E2eTestSetup;
use crate::crypto::commitments::{
    commitment_to_hex, create_vote_commitment, generate_nonce, nonce_to_hex,
};
use crate::crypto::contexts::{context_to_string, generate_poll_context};
use crate::storage::models::vote::VoteType;
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

    pub async fn commit_vote_via_api(
        http_client: &reqwest::Client,
        base_url: &str,
        poll_id: &str,
        commit_data: &HashMap<String, Value>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {
        let response = http_client
            .post(format!("{base_url}/polls/{poll_id}/commit"))
            .header("Content-Type", "application/json")
            .json(commit_data)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(format!("Failed to commit vote: {error_text}").into());
        }

        Ok(response.json().await?)
    }

    pub async fn reveal_vote_via_api(
        http_client: &reqwest::Client,
        base_url: &str,
        poll_id: &str,
        reveal_data: &HashMap<String, Value>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {
        let response = http_client
            .post(format!("{base_url}/polls/{poll_id}/reveal"))
            .header("Content-Type", "application/json")
            .json(reveal_data)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(format!("Failed to reveal vote: {error_text}").into());
        }

        Ok(response.json().await?)
    }

    pub fn create_signed_vote_data(
        poll_id: u32,
        vote_type: &str,
        voter_keypair: &TestKeyPair,
    ) -> HashMap<String, Value> {
        // For backward compatibility, this now creates commit data
        let nonce = generate_nonce();
        Self::create_vote_commit_data(poll_id, vote_type, voter_keypair, &nonce)
    }

    pub async fn create_signed_vote_data_with_real_ring_proof(
        poll_id: u32,
        vote_type: &str,
        voter_keypair: &TestKeyPair,
        setup: &E2eTestSetup,
    ) -> Result<HashMap<String, Value>, Box<dyn std::error::Error + Send + Sync>> {
        // For backward compatibility, this now creates commit data with real ring proof
        let nonce = generate_nonce();
        Self::create_vote_commit_data_with_real_ring_proof(
            poll_id,
            vote_type,
            voter_keypair,
            &nonce,
            setup,
        )
        .await
    }

    pub async fn create_vote_commit_data_with_real_ring_proof(
        poll_id: u32,
        vote_type: &str,
        voter_keypair: &TestKeyPair,
        nonce: &[u8; 32],
        setup: &E2eTestSetup,
    ) -> Result<HashMap<String, Value>, Box<dyn std::error::Error + Send + Sync>> {
        // Parse vote type
        let vote_type_enum = match vote_type.to_lowercase().as_str() {
            "yay" | "yes" => VoteType::Yay,
            "nay" | "no" => VoteType::Nay,
            _ => VoteType::Yay, // default
        };

        // Create commitment
        let commitment = create_vote_commitment(&vote_type_enum, nonce);
        let commitment_hex = commitment_to_hex(&commitment);

        // Generate the correct poll context
        let context_bytes = generate_poll_context(poll_id);
        let poll_context = context_to_string(&context_bytes);

        // Create signature message for commit phase (must match service format)
        let message = create_commit_signature_message(poll_id, &commitment_hex, &poll_context);

        // Sign the message
        let signature = voter_keypair
            .sign_message(&message)
            .expect("Failed to sign message");

        // Get the voter's ring index from the chain
        let voter_ring_index =
            Self::get_voter_ring_index_from_chain(&voter_keypair.member_hex, setup).await?;

        // Get the individual ring keys from the chain
        let ring_keys = Self::get_ring_keys_from_chain(voter_ring_index, setup).await?;

        // Generate the real ring proof
        let ring_proof =
            voter_keypair.generate_ring_proof(ring_keys.into_iter(), &context_bytes, &message)?;

        let mut commit_data = HashMap::new();
        commit_data.insert("poll_id".to_string(), Value::Number(poll_id.into()));
        commit_data.insert(
            "voter_member_hex".to_string(),
            Value::String(voter_keypair.member_hex.clone()),
        );
        commit_data.insert(
            "vote_commitment".to_string(),
            Value::String(commitment_hex.clone()),
        );
        commit_data.insert(
            "voter_signature".to_string(),
            Value::String(signature.clone()),
        );
        commit_data.insert("ring_proof".to_string(), Value::String(ring_proof));

        Ok(commit_data)
    }

    pub fn create_vote_commit_data(
        poll_id: u32,
        vote_type: &str,
        voter_keypair: &TestKeyPair,
        nonce: &[u8; 32],
    ) -> HashMap<String, Value> {
        // Parse vote type
        let vote_type_enum = match vote_type.to_lowercase().as_str() {
            "yay" | "yes" => VoteType::Yay,
            "nay" | "no" => VoteType::Nay,
            _ => VoteType::Yay, // default
        };

        // Create commitment
        let commitment = create_vote_commitment(&vote_type_enum, nonce);
        let commitment_hex = commitment_to_hex(&commitment);

        // Generate the correct poll context
        let context_bytes = generate_poll_context(poll_id);
        let poll_context = context_to_string(&context_bytes);

        // Create signature message for commit phase (must match service format)
        let message = create_commit_signature_message(poll_id, &commitment_hex, &poll_context);

        // Sign the message
        let signature = voter_keypair
            .sign_message(&message)
            .expect("Failed to sign message");

        let mut commit_data = HashMap::new();
        commit_data.insert("poll_id".to_string(), Value::Number(poll_id.into()));
        commit_data.insert(
            "voter_member_hex".to_string(),
            Value::String(voter_keypair.member_hex.clone()),
        );
        commit_data.insert("vote_commitment".to_string(), Value::String(commitment_hex));
        commit_data.insert("voter_signature".to_string(), Value::String(signature));
        // For e2e testing, use placeholder ring proof
        commit_data.insert(
            "ring_proof".to_string(),
            Value::String("RING_PROOF_PLACEHOLDER".to_string()),
        );

        commit_data
    }

    pub fn create_vote_reveal_data(
        poll_id: u32,
        vote_type: &str,
        voter_keypair: &TestKeyPair,
        nonce: &[u8; 32],
    ) -> HashMap<String, Value> {
        // Parse vote type
        let _vote_type_enum = match vote_type.to_lowercase().as_str() {
            "yay" | "yes" => VoteType::Yay,
            "nay" | "no" => VoteType::Nay,
            _ => VoteType::Yay, // default
        };

        let nonce_hex = nonce_to_hex(nonce);

        let mut reveal_data = HashMap::new();
        reveal_data.insert("poll_id".to_string(), Value::Number(poll_id.into()));
        reveal_data.insert(
            "voter_member_hex".to_string(),
            Value::String(voter_keypair.member_hex.clone()),
        );
        reveal_data.insert(
            "vote_type".to_string(),
            Value::String(vote_type.to_string()),
        );
        reveal_data.insert("vote_nonce".to_string(), Value::String(nonce_hex));

        reveal_data
    }

    pub async fn submit_full_vote_via_commit_reveal(
        http_client: &reqwest::Client,
        base_url: &str,
        poll_id: &str,
        vote_type: &str,
        voter_keypair: &TestKeyPair,
    ) -> Result<(Value, Value), Box<dyn std::error::Error + Send + Sync>> {
        let poll_id_num: u32 = poll_id.parse()?;

        // Generate nonce for commitment
        let nonce = generate_nonce();

        // Create commit data
        let commit_data =
            Self::create_vote_commit_data(poll_id_num, vote_type, voter_keypair, &nonce);

        // Submit commit
        let commit_response =
            Self::commit_vote_via_api(http_client, base_url, poll_id, &commit_data).await?;

        // Create reveal data
        let reveal_data =
            Self::create_vote_reveal_data(poll_id_num, vote_type, voter_keypair, &nonce);

        // For e2e testing, we would normally wait for poll to end, but for testing we'll reveal immediately
        // In production, this would wait for the poll voting period to end
        let reveal_response =
            Self::reveal_vote_via_api(http_client, base_url, poll_id, &reveal_data).await?;

        Ok((commit_response, reveal_response))
    }

    pub async fn get_voter_ring_index_from_chain(
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

    pub async fn get_ring_keys_from_chain(
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
            .ok_or_else(|| format!("Ring keys not found for ring index {ring_index}"))?;

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
            .ok_or_else(|| format!("Ring keys status not found for ring index {ring_index}"))?;

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
