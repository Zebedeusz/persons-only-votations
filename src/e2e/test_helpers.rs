use super::{
    crypto_helpers::TestKeyPair,
    setup::E2eTestSetup,
    utils::{TestHelpers, TestPoll},
};
use crate::chain::ChainClient;
use crate::crypto::commitments::generate_nonce;
use std::collections::HashMap;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub async fn setup_test_environment() -> E2eTestSetup {
    let _ = tracing_subscriber::fmt::try_init();

    let setup = E2eTestSetup::new()
        .await
        .expect("Failed to setup e2e test environment");
    setup
        .wait_for_server()
        .await
        .expect("Server should be available");
    verify_users_registered_on_chain(&setup).await;
    setup
}

/// Panics unless the first six recognized test users are included in rings on chain.
pub async fn verify_users_registered_on_chain(setup: &E2eTestSetup) {
    let required_user_count = 6;

    let block_number = setup
        .chain
        .client()
        .blocks()
        .at_latest()
        .await
        .expect(
            "Failed to connect to People chain. Ensure the People chain is running and accessible.",
        )
        .header()
        .number;
    tracing::info!("Connected to People chain at block: {block_number}");

    for i in 0..required_user_count {
        let user = setup.config.get_recognized_person(i).unwrap_or_else(|| {
            panic!("Test user {i} not found. Ensure test_users.json has at least {required_user_count} users.")
        });
        if user.secret_scalar.is_empty() || user.member_hex.is_empty() {
            panic!(
                "Test user {i} has empty credentials. Run the chain setup script to populate user data."
            );
        }
        let ring_index = setup
            .chain
            .ring_index(&user.member_hex)
            .await
            .unwrap_or_else(|e| panic!("Failed to verify ring membership for test user {i}: {e}"))
            .unwrap_or_else(|| {
                panic!(
                    "Test user {i} ({}) is not included in any ring. Ensure users are registered and included in active rings.",
                    &user.member_hex[0..8]
                )
            });
        let ring_size = TestHelpers::get_ring_keys_from_chain(ring_index, setup)
            .await
            .map(|keys| keys.len())
            .unwrap_or_else(|e| panic!("Failed to get ring keys for ring {ring_index}: {e}"));
        tracing::info!("User {i} is in ring {ring_index} with {ring_size} members");
    }
}

/// Keypair of the recognized test user at `index` in test_users.json.
pub fn recognized_keypair(setup: &E2eTestSetup, index: usize) -> TestKeyPair {
    let user = setup
        .config
        .get_recognized_person(index)
        .unwrap_or_else(|| panic!("Need recognized user at index {index}"));
    TestKeyPair::from_secret_scalar(&user.secret_scalar, &user.member_hex)
        .expect("Failed to create keypair")
}

/// Real ring proof for `voter` over `message`, built from the voter's ring on chain.
pub async fn ring_proof_for(
    setup: &E2eTestSetup,
    voter: &TestKeyPair,
    context: &[u8; 32],
    message: &[u8],
) -> Result<String> {
    let ring_index = setup
        .chain
        .ring_index(&voter.member_hex)
        .await?
        .ok_or("Person is not included in any ring (may be onboarding)")?;
    let ring_keys = TestHelpers::get_ring_keys_from_chain(ring_index, setup).await?;
    voter.generate_ring_proof(ring_keys.into_iter(), context, message)
}

/// (creator, voter) = recognized users 0 and 1.
pub fn get_test_users(setup: &E2eTestSetup) -> (TestKeyPair, TestKeyPair) {
    (recognized_keypair(setup, 0), recognized_keypair(setup, 1))
}

pub fn prepare_voting_poll_data() -> TestPoll {
    TestPoll {
        title: "Voting Test Poll".to_string(),
        description: "Testing voting without identity proof".to_string(),
    }
}

pub fn prepare_recognized_users_poll_data() -> TestPoll {
    TestPoll {
        title: "Recognized Users Poll".to_string(),
        description: "Testing poll creation and voting with recognized users".to_string(),
    }
}

pub fn prepare_impersonation_test_poll_data() -> TestPoll {
    TestPoll {
        title: "Malicious Impersonation Poll".to_string(),
        description: "This poll should fail to be created due to signature mismatch".to_string(),
    }
}

async fn post_poll(
    setup: &E2eTestSetup,
    test_poll: &TestPoll,
    creator: &TestKeyPair,
    duration_seconds: i64,
) -> reqwest::Response {
    setup
        .http_client
        .post(format!("{}/polls", setup.config.server_base_url))
        .json(&test_poll.to_signed_request(creator, duration_seconds))
        .send()
        .await
        .expect("Failed to send poll creation request")
}

/// Creates a poll that ends in one hour and returns its id.
pub async fn create_poll(
    setup: &E2eTestSetup,
    test_poll: &TestPoll,
    creator: &TestKeyPair,
) -> String {
    create_timed_poll(setup, test_poll, creator, 3600).await
}

pub async fn create_timed_poll(
    setup: &E2eTestSetup,
    test_poll: &TestPoll,
    creator: &TestKeyPair,
    duration_seconds: i64,
) -> String {
    let response = post_poll(setup, test_poll, creator, duration_seconds).await;
    if !response.status().is_success() {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_default();
        panic!("Failed to create poll: {status} - {error_text}");
    }
    let body: serde_json::Value = response
        .json()
        .await
        .expect("Failed to parse poll creation response");
    body["poll_id"]
        .as_u64()
        .expect("Poll ID not found in response")
        .to_string()
}

pub async fn assert_poll_created_correctly(
    setup: &E2eTestSetup,
    poll_id: &str,
    expected_poll: &TestPoll,
    creator: &TestKeyPair,
) {
    let poll =
        TestHelpers::get_poll_via_api(&setup.http_client, &setup.config.server_base_url, poll_id)
            .await
            .expect("Failed to retrieve poll via API");

    assert_eq!(poll["name"], expected_poll.title);
    assert_eq!(poll["description"], expected_poll.description);
    assert_eq!(poll["is_active"], true);
    assert!(poll["id"].is_number());
    assert_eq!(poll["creator_member_hex"], creator.member_hex);
    assert!(poll["created_at"].is_number());
    assert!(poll["ends_at"].is_number());
}

/// Commits a vote with a fresh nonce; the vote is never revealed.
pub async fn submit_vote(setup: &E2eTestSetup, poll_id: &str, voter: &TestKeyPair, choice: &str) {
    commit_vote(setup, poll_id, voter, choice, &generate_nonce())
        .await
        .expect("Failed to submit vote");
}

pub async fn assert_vote_recorded(setup: &E2eTestSetup, poll_id: &str, expected_poll: &TestPoll) {
    let poll =
        TestHelpers::get_poll_via_api(&setup.http_client, &setup.config.server_base_url, poll_id)
            .await
            .expect("Failed to retrieve updated poll");
    assert_eq!(poll["name"], expected_poll.title);
}

pub async fn commit_vote(
    setup: &E2eTestSetup,
    poll_id: &str,
    voter: &TestKeyPair,
    choice: &str,
    nonce: &[u8; 32],
) -> Result<serde_json::Value> {
    let poll_id_num: u32 = poll_id.parse().expect("Invalid poll ID");
    let vote_data = TestHelpers::create_vote_commit_data_with_real_ring_proof(
        poll_id_num,
        choice,
        voter,
        nonce,
        setup,
    )
    .await?;
    TestHelpers::commit_vote_via_api(
        &setup.http_client,
        &setup.config.server_base_url,
        poll_id,
        &vote_data,
    )
    .await
}

pub fn create_reveal_data(
    poll_id: &str,
    voter: &TestKeyPair,
    choice: &str,
    nonce: &[u8; 32],
) -> HashMap<String, serde_json::Value> {
    let poll_id_num: u32 = poll_id.parse().expect("Invalid poll ID");
    TestHelpers::create_vote_reveal_data(poll_id_num, choice, voter, nonce)
}

pub async fn assert_vote_counts_hidden_during_voting(setup: &E2eTestSetup, poll_id: &str) {
    let response = setup
        .http_client
        .get(format!(
            "{}/polls/{}/votes",
            setup.config.server_base_url, poll_id
        ))
        .send()
        .await
        .expect("Failed to get vote counts");
    if response.status().is_success() {
        let counts: serde_json::Value = response.json().await.expect("Failed to parse vote counts");
        assert_eq!(
            counts["yay_votes"].as_u64().unwrap_or(0),
            0,
            "Vote counts should be hidden during voting period"
        );
        assert_eq!(
            counts["nay_votes"].as_u64().unwrap_or(0),
            0,
            "Vote counts should be hidden during voting period"
        );
    }
}

pub async fn wait_for_poll_to_end_and_reveal(
    setup: &E2eTestSetup,
    poll_id: &str,
    reveal_data: &HashMap<String, serde_json::Value>,
    wait_seconds: u64,
) {
    tracing::info!("Waiting for poll to end...");
    tokio::time::sleep(tokio::time::Duration::from_secs(wait_seconds)).await;
    TestHelpers::reveal_vote_via_api(
        &setup.http_client,
        &setup.config.server_base_url,
        poll_id,
        reveal_data,
    )
    .await
    .expect("Failed to reveal vote");
}

pub async fn finalize_poll_and_assert_votes_counted(setup: &E2eTestSetup, poll_id: &str) {
    let response = setup
        .http_client
        .post(format!(
            "{}/polls/{}/finalize",
            setup.config.server_base_url, poll_id
        ))
        .send()
        .await
        .expect("Failed to send finalize request");
    if !response.status().is_success() {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_default();
        panic!("Failed to finalize poll: {status} - {error_text}");
    }
    let counts: serde_json::Value = response
        .json()
        .await
        .expect("Failed to parse finalize response");
    let total =
        counts["yay_votes"].as_u64().unwrap_or(0) + counts["nay_votes"].as_u64().unwrap_or(0);
    assert!(
        total > 0,
        "At least one vote should be recorded after finalization"
    );
}

pub async fn assert_final_poll_state(
    setup: &E2eTestSetup,
    poll_id: &str,
    expected_poll: &TestPoll,
) {
    let poll =
        TestHelpers::get_poll_via_api(&setup.http_client, &setup.config.server_base_url, poll_id)
            .await
            .expect("Failed to retrieve poll after voting");
    assert_eq!(poll["name"], expected_poll.title);
    assert_eq!(poll["is_active"], false);
}

/// An onboarding user must fail either at ring-proof generation (not in a ring) or at the server.
pub async fn test_onboarding_user_vote_rejection(setup: &E2eTestSetup, poll_id: &str) {
    let Some(onboarding_user) = setup.config.get_onboarding_user(0) else {
        tracing::warn!("No onboarding user configured, skipping onboarding user vote test");
        return;
    };
    let onboarding_keypair = TestKeyPair::from_secret_scalar(
        &onboarding_user.secret_scalar,
        &onboarding_user.member_hex,
    )
    .expect("Failed to create keypair for onboarding user");

    match commit_vote(
        setup,
        poll_id,
        &onboarding_keypair,
        "Nay",
        &generate_nonce(),
    )
    .await
    {
        Ok(_) => panic!("Vote by onboarding user should have been rejected but was accepted"),
        Err(e) => tracing::info!("Vote by onboarding user correctly rejected: {e}"),
    }
}

/// (legitimate keypair, keypair claiming the legitimate member key with a random secret).
pub fn get_legitimate_and_fake_keypairs(setup: &E2eTestSetup) -> (TestKeyPair, TestKeyPair) {
    let legitimate = recognized_keypair(setup, 0);
    let fake = TestKeyPair {
        member_hex: legitimate.member_hex.clone(),
        secret: TestKeyPair::generate().secret,
    };
    (legitimate, fake)
}

pub async fn attempt_malicious_poll_creation(
    setup: &E2eTestSetup,
    test_poll: &TestPoll,
    fake_keypair: &TestKeyPair,
) {
    let response = post_poll(setup, test_poll, fake_keypair, 3600).await;
    assert!(
        !response.status().is_success(),
        "Malicious poll creation should fail due to signature verification failure"
    );
}

pub async fn attempt_malicious_vote(
    setup: &E2eTestSetup,
    poll_id: &str,
    fake_keypair: &TestKeyPair,
) {
    let result = commit_vote(setup, poll_id, fake_keypair, "Yay", &generate_nonce()).await;
    assert!(
        result.is_err(),
        "Vote with an impersonated identity should be rejected"
    );
}

pub async fn verify_legitimate_user_can_vote(
    setup: &E2eTestSetup,
    poll_id: &str,
    legitimate_keypair: &TestKeyPair,
) {
    let result = commit_vote(setup, poll_id, legitimate_keypair, "Nay", &generate_nonce()).await;
    assert!(
        result.is_ok(),
        "Legitimate user should be able to vote with their real keypair"
    );
}
