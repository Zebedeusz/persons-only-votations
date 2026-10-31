use super::crypto_helpers::TestKeyPair;
use super::setup::E2eTestSetup;
use super::test_helpers::ring_proof_for;
use crate::chain::subxt_generated_people_apis::api as people_api;
use crate::crypto::commitments::create_vote_commitment;
use crate::crypto::contexts::generate_poll_context;
use crate::services::poll_service::poll_message;
use crate::services::vote_service::commit_message;
use crate::storage::models::poll::PollData;
use crate::storage::models::vote::VoteType;
use serde_json::{Value, json};
use std::collections::HashMap;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub struct TestPoll {
    pub title: String,
    pub description: String,
}

impl TestPoll {
    /// Signed create-poll request for a poll ending `duration_seconds` from now.
    pub fn to_signed_request(&self, creator: &TestKeyPair, duration_seconds: i64) -> Value {
        let poll_data = PollData {
            name: self.title.clone(),
            description: self.description.clone(),
            ends_at: crate::utils::get_current_timestamp() + duration_seconds,
        };
        let signature = creator
            .sign_message(&poll_message(&poll_data))
            .expect("Failed to sign message");
        json!({
            "creator_member_hex": creator.member_hex,
            "creator_signature": signature,
            "poll_data": poll_data
        })
    }
}

pub fn parse_vote_type(vote_type: &str) -> VoteType {
    match vote_type.to_lowercase().as_str() {
        "nay" | "no" => VoteType::Nay,
        _ => VoteType::Yay,
    }
}

pub struct TestHelpers;

impl TestHelpers {
    pub async fn get_poll_via_api(
        http_client: &reqwest::Client,
        base_url: &str,
        poll_id: &str,
    ) -> Result<Value> {
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
    ) -> Result<Value> {
        Self::post_json(
            http_client,
            &format!("{base_url}/polls/{poll_id}/commit"),
            commit_data,
            "commit vote",
        )
        .await
    }

    pub async fn reveal_vote_via_api(
        http_client: &reqwest::Client,
        base_url: &str,
        poll_id: &str,
        reveal_data: &HashMap<String, Value>,
    ) -> Result<Value> {
        Self::post_json(
            http_client,
            &format!("{base_url}/polls/{poll_id}/reveal"),
            reveal_data,
            "reveal vote",
        )
        .await
    }

    async fn post_json(
        http_client: &reqwest::Client,
        url: &str,
        body: &HashMap<String, Value>,
        action: &str,
    ) -> Result<Value> {
        let response = http_client.post(url).json(body).send().await?;
        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(format!("Failed to {action}: {error_text}").into());
        }
        Ok(response.json().await?)
    }

    /// Commit request signed by the voter, with a real ring proof from the voter's ring on chain.
    pub async fn create_vote_commit_data_with_real_ring_proof(
        poll_id: u32,
        vote_type: &str,
        voter_keypair: &TestKeyPair,
        nonce: &[u8; 32],
        setup: &E2eTestSetup,
    ) -> Result<HashMap<String, Value>> {
        let commitment_hex =
            hex::encode(create_vote_commitment(&parse_vote_type(vote_type), nonce));
        let context = generate_poll_context(poll_id);
        let message = commit_message(poll_id, &commitment_hex, &hex::encode(context));
        let signature = voter_keypair.sign_message(&message)?;
        let ring_proof = ring_proof_for(setup, voter_keypair, &context, &message).await?;

        Ok(HashMap::from([
            ("poll_id".to_string(), json!(poll_id)),
            (
                "voter_member_hex".to_string(),
                json!(voter_keypair.member_hex),
            ),
            ("vote_commitment".to_string(), json!(commitment_hex)),
            ("voter_signature".to_string(), json!(signature)),
            ("ring_proof".to_string(), json!(ring_proof)),
        ]))
    }

    pub fn create_vote_reveal_data(
        poll_id: u32,
        vote_type: &str,
        voter_keypair: &TestKeyPair,
        nonce: &[u8; 32],
    ) -> HashMap<String, Value> {
        HashMap::from([
            ("poll_id".to_string(), json!(poll_id)),
            (
                "voter_member_hex".to_string(),
                json!(voter_keypair.member_hex),
            ),
            ("vote_type".to_string(), json!(vote_type)),
            ("vote_nonce".to_string(), json!(hex::encode(nonce))),
        ])
    }

    /// Keys currently included in the ring, in ring order.
    pub async fn get_ring_keys_from_chain(
        ring_index: u16,
        setup: &E2eTestSetup,
    ) -> Result<Vec<verifiable::ring_vrf_impl::EncodedPublicKey>> {
        let storage = setup.chain.client().storage().at_latest().await?;
        let ring_keys = storage
            .fetch(&people_api::storage().people().ring_keys(ring_index as u32))
            .await?
            .ok_or_else(|| format!("Ring keys not found for ring index {ring_index}"))?;
        let ring_status = storage
            .fetch(
                &people_api::storage()
                    .people()
                    .ring_keys_status(ring_index as u32),
            )
            .await?
            .ok_or_else(|| format!("Ring keys status not found for ring index {ring_index}"))?;

        Ok(ring_keys
            .0
            .into_iter()
            .take(ring_status.included as usize)
            .map(|key| verifiable::ring_vrf_impl::EncodedPublicKey(key.0))
            .collect())
    }
}
