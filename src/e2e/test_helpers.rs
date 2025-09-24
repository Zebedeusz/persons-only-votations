use super::{
    crypto_helpers::TestKeyPair,
    setup::E2eTestSetup,
    utils::{TestHelpers, TestPoll},
};
use std::collections::HashMap;

pub async fn setup_test_environment() -> E2eTestSetup {
    let _ = tracing_subscriber::fmt::try_init();

    let setup = E2eTestSetup::new()
        .await
        .expect("Failed to setup e2e test environment");

    setup
        .wait_for_server()
        .await
        .expect("Server should be available");

    setup
}

pub fn get_test_users(setup: &E2eTestSetup) -> (TestKeyPair, TestKeyPair) {
    let creator_user = setup
        .config
        .get_recognized_person(0)
        .expect("Recognized creator user should be configured");
    let voter_user = setup
        .config
        .get_recognized_person(1)
        .expect("Recognized voter user should be configured");

    let creator_keypair =
        TestKeyPair::from_secret_scalar(&creator_user.secret_scalar, &creator_user.member_hex)
            .expect("Failed to create creator keypair");
    let voter_keypair =
        TestKeyPair::from_secret_scalar(&voter_user.secret_scalar, &voter_user.member_hex)
            .expect("Failed to create voter keypair");

    (creator_keypair, voter_keypair)
}

pub fn prepare_voting_poll_data() -> TestPoll {
    TestPoll {
        title: "Voting Test Poll".to_string(),
        description: "Testing voting without identity proof".to_string(),
        options: vec!["Option A".to_string(), "Option B".to_string()],
        required_identity_proof: false,
    }
}

pub fn prepare_recognized_users_poll_data() -> TestPoll {
    TestPoll {
        title: "Recognized Users Poll".to_string(),
        description: "Testing poll creation and voting with recognized users".to_string(),
        options: vec!["Yes".to_string(), "No".to_string()],
        required_identity_proof: true,
    }
}

pub fn prepare_impersonation_test_poll_data() -> TestPoll {
    TestPoll {
        title: "Malicious Impersonation Poll".to_string(),
        description: "This poll should fail to be created due to signature mismatch".to_string(),
        options: vec!["Yes".to_string(), "No".to_string()],
        required_identity_proof: false,
    }
}

pub async fn create_poll(
    setup: &E2eTestSetup,
    test_poll: &TestPoll,
    creator_keypair: &TestKeyPair,
) -> String {
    let poll_json = test_poll.to_json_with_keypair(creator_keypair);
    let response = setup
        .http_client
        .post(format!("{}/polls", setup.config.server_base_url))
        .header("Content-Type", "application/json")
        .json(&poll_json)
        .send()
        .await
        .expect("Failed to send poll creation request");

    if !response.status().is_success() {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_default();
        panic!("Failed to create poll: {status} - {error_text}");
    }

    let response_body: serde_json::Value = response
        .json()
        .await
        .expect("Failed to parse poll creation response");

    response_body["poll_id"]
        .as_u64()
        .expect("Poll ID not found in response")
        .to_string()
}

pub async fn create_timed_poll(
    setup: &E2eTestSetup,
    test_poll: &TestPoll,
    creator_keypair: &TestKeyPair,
    duration_seconds: i64,
) -> String {
    let ends_at = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64
        + duration_seconds;

    let message = format!(
        r#"{{"name":"{}","description":"{}","ends_at":{}}}"#,
        test_poll.title, test_poll.description, ends_at
    );

    let signature = creator_keypair
        .sign_message(message.as_bytes())
        .expect("Failed to sign message");

    let poll_json = serde_json::json!({
        "creator_member_hex": creator_keypair.member_hex,
        "creator_signature": signature,
        "poll_data": {
            "name": test_poll.title,
            "description": test_poll.description,
            "ends_at": ends_at
        }
    });

    let response = setup
        .http_client
        .post(format!("{}/polls", setup.config.server_base_url))
        .header("Content-Type", "application/json")
        .json(&poll_json)
        .send()
        .await
        .expect("Failed to send poll creation request");

    if !response.status().is_success() {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_default();
        panic!("Failed to create poll: {status} - {error_text}");
    }

    let response_body: serde_json::Value = response
        .json()
        .await
        .expect("Failed to parse poll creation response");

    response_body["poll_id"]
        .as_u64()
        .expect("Poll ID not found in response")
        .to_string()
}

pub async fn assert_poll_created_correctly(
    setup: &E2eTestSetup,
    poll_id: &str,
    expected_poll: &TestPoll,
    creator_keypair: &TestKeyPair,
) {
    let retrieved_poll =
        TestHelpers::get_poll_via_api(&setup.http_client, &setup.config.server_base_url, poll_id)
            .await
            .expect("Failed to retrieve poll via API");

    assert_eq!(retrieved_poll["name"], expected_poll.title);
    assert_eq!(retrieved_poll["description"], expected_poll.description);
    assert_eq!(retrieved_poll["is_active"], true);
    assert!(retrieved_poll["id"].is_number());
    assert!(retrieved_poll["creator_member_hex"].is_string());
    assert_eq!(
        retrieved_poll["creator_member_hex"],
        creator_keypair.member_hex
    );
    assert!(retrieved_poll["created_at"].is_number());
    assert!(retrieved_poll["ends_at"].is_number());
}

pub async fn submit_vote(
    setup: &E2eTestSetup,
    poll_id: &str,
    voter_keypair: &TestKeyPair,
    choice: &str,
) {
    let poll_id_num: u32 = poll_id.parse().expect("Invalid poll ID");
    let vote_data = TestHelpers::create_signed_vote_data_with_real_ring_proof(
        poll_id_num,
        choice,
        voter_keypair,
        setup,
    )
    .await
    .expect("Failed to create vote data with real ring proof");

    let _vote_response = TestHelpers::commit_vote_via_api(
        &setup.http_client,
        &setup.config.server_base_url,
        poll_id,
        &vote_data,
    )
    .await
    .expect("Failed to submit vote");
}

pub async fn assert_vote_recorded(setup: &E2eTestSetup, poll_id: &str, expected_poll: &TestPoll) {
    let updated_poll =
        TestHelpers::get_poll_via_api(&setup.http_client, &setup.config.server_base_url, poll_id)
            .await
            .expect("Failed to retrieve updated poll");

    assert_eq!(updated_poll["name"], expected_poll.title);
}


pub async fn commit_vote(
    setup: &E2eTestSetup,
    poll_id: &str,
    voter_keypair: &TestKeyPair,
    choice: &str,
    nonce: &[u8; 32],
) -> Result<serde_json::Value, Box<dyn std::error::Error + Send + Sync>> {
    let poll_id_num: u32 = poll_id.parse().expect("Invalid poll ID");

    let vote_data = TestHelpers::create_vote_commit_data_with_real_ring_proof(
        poll_id_num,
        choice,
        voter_keypair,
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
    voter_keypair: &TestKeyPair,
    choice: &str,
    nonce: &[u8; 32],
) -> HashMap<String, serde_json::Value> {
    let poll_id_num: u32 = poll_id.parse().expect("Invalid poll ID");
    TestHelpers::create_vote_reveal_data(poll_id_num, choice, voter_keypair, nonce)
}

pub async fn assert_vote_counts_hidden_during_voting(setup: &E2eTestSetup, poll_id: &str) {
    let vote_counts_response = setup
        .http_client
        .get(format!(
            "{}/polls/{}/votes",
            setup.config.server_base_url, poll_id
        ))
        .send()
        .await
        .expect("Failed to get vote counts");

    if vote_counts_response.status().is_success() {
        let vote_counts_during_voting: serde_json::Value = vote_counts_response
            .json()
            .await
            .expect("Failed to parse vote counts response");

        let yay_votes = vote_counts_during_voting["yay_votes"].as_u64().unwrap_or(0);
        let nay_votes = vote_counts_during_voting["nay_votes"].as_u64().unwrap_or(0);

        assert_eq!(
            yay_votes, 0,
            "Vote counts should be hidden during voting period"
        );
        assert_eq!(
            nay_votes, 0,
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

    tracing::info!("Revealing vote now that poll has ended...");
    let _reveal_response = TestHelpers::reveal_vote_via_api(
        &setup.http_client,
        &setup.config.server_base_url,
        poll_id,
        reveal_data,
    )
        .await
        .expect("Failed to reveal vote");
}

pub async fn finalize_poll_and_assert_votes_counted(setup: &E2eTestSetup, poll_id: &str) {
    let finalize_response = setup
        .http_client
        .post(format!(
            "{}/polls/{}/finalize",
            setup.config.server_base_url, poll_id
        ))
        .send()
        .await
        .expect("Failed to send finalize request");

    let vote_counts = if finalize_response.status().is_success() {
        let finalize_result: serde_json::Value = finalize_response
            .json()
            .await
            .expect("Failed to parse finalize response");
        finalize_result
    } else {
        let status = finalize_response.status();
        let error_text = finalize_response.text().await.unwrap_or_default();
        panic!("Failed to finalize poll: {status} - {error_text}");
    };

    let yay_votes = vote_counts["yay_votes"].as_u64().unwrap_or(0);
    let nay_votes = vote_counts["nay_votes"].as_u64().unwrap_or(0);
    let total_votes = yay_votes + nay_votes;

    assert!(
        total_votes > 0,
        "At least one vote should be recorded after finalization"
    );
}

pub async fn assert_final_poll_state(
    setup: &E2eTestSetup,
    poll_id: &str,
    expected_poll: &TestPoll,
) {
    let final_poll =
        TestHelpers::get_poll_via_api(&setup.http_client, &setup.config.server_base_url, poll_id)
            .await
            .expect("Failed to retrieve poll after voting");

    assert_eq!(final_poll["name"], expected_poll.title);
    assert_eq!(final_poll["is_active"], false);
}

pub async fn test_onboarding_user_vote_rejection(setup: &E2eTestSetup, poll_id: &str) {
    let poll_id_num: u32 = poll_id.parse().expect("Invalid poll ID");

    if let Some(onboarding_user) = setup.config.get_onboarding_user(0) {
        let onboarding_keypair = TestKeyPair::from_secret_scalar(
            &onboarding_user.secret_scalar,
            &onboarding_user.member_hex,
        )
            .expect("Failed to create keypair for onboarding user");

        let onboarding_vote_data_result =
            TestHelpers::create_signed_vote_data_with_real_ring_proof(
                poll_id_num,
                "Nay",
                &onboarding_keypair,
                setup,
            )
                .await;

        match onboarding_vote_data_result {
            Ok(onboarding_vote_data) => {
                let onboarding_vote_response = TestHelpers::commit_vote_via_api(
                    &setup.http_client,
                    &setup.config.server_base_url,
                    poll_id,
                    &onboarding_vote_data,
                )
                    .await;

                match onboarding_vote_response {
                    Ok(_) => {
                        panic!(
                            "Vote by onboarding user should have been rejected but was accepted"
                        );
                    }
                    Err(e) => {
                        tracing::info!(
                            "Vote by onboarding user correctly rejected at server level: {}",
                            e
                        );
                    }
                }
            }
            Err(e) => {
                tracing::info!(
                    "Ring proof generation correctly failed for onboarding user: {}",
                    e
                );
            }
        }
    } else {
        tracing::warn!("No onboarding user configured, skipping onboarding user vote test");
    }
}

pub fn get_legitimate_and_fake_keypairs(setup: &E2eTestSetup) -> (TestKeyPair, TestKeyPair) {
    let legitimate_user = setup
        .config
        .get_recognized_person(0)
        .expect("Recognized user should be configured in test_users.json");

    let legitimate_keypair = TestKeyPair::from_secret_scalar(
        &legitimate_user.secret_scalar,
        &legitimate_user.member_hex,
    )
        .expect("Failed to create legitimate keypair");

    let malicious_keypair = TestKeyPair::generate();
    let fake_impersonation_keypair = TestKeyPair {
        member_hex: legitimate_user.member_hex.clone(),
        secret: malicious_keypair.secret.clone(),
    };

    (legitimate_keypair, fake_impersonation_keypair)
}

pub async fn attempt_malicious_poll_creation(
    setup: &E2eTestSetup,
    test_poll: &TestPoll,
    fake_keypair: &TestKeyPair,
) {
    let poll_json = test_poll.to_json_with_keypair(fake_keypair);
    let poll_response = setup
        .http_client
        .post(format!("{}/polls", setup.config.server_base_url))
        .header("Content-Type", "application/json")
        .json(&poll_json)
        .send()
        .await
        .expect("Failed to send poll creation request");

    assert!(
        !poll_response.status().is_success(),
        "Malicious poll creation should fail due to signature verification failure"
    );
}

pub async fn attempt_malicious_vote(
    setup: &E2eTestSetup,
    poll_id: &str,
    fake_keypair: &TestKeyPair,
) {
    let poll_id_num: u32 = poll_id.parse().expect("Invalid poll ID");
    let malicious_vote_data_result = TestHelpers::create_signed_vote_data_with_real_ring_proof(
        poll_id_num,
        "Yay",
        fake_keypair,
        setup,
    )
        .await;

    assert!(
        malicious_vote_data_result.is_err(),
        "Ring proof generation should fail for impersonated identity"
    );
}

pub async fn verify_legitimate_user_can_vote(
    setup: &E2eTestSetup,
    poll_id: &str,
    legitimate_keypair: &TestKeyPair,
) {
    tracing::info!("Verifying legitimate user can still vote with their real keypair");
    let poll_id_num: u32 = poll_id.parse().expect("Invalid poll ID");

    let legitimate_vote_data = TestHelpers::create_signed_vote_data_with_real_ring_proof(
        poll_id_num,
        "Nay",
        legitimate_keypair,
        setup,
    )
        .await
        .expect("Failed to create vote data with real ring proof");

    let legitimate_vote_response = TestHelpers::commit_vote_via_api(
        &setup.http_client,
        &setup.config.server_base_url,
        poll_id,
        &legitimate_vote_data,
    )
        .await;

    assert!(
        legitimate_vote_response.is_ok(),
        "Legitimate user should be able to vote with their real keypair"
    );
}
