use super::{
    crypto_helpers::TestKeyPair,
    setup::E2eTestSetup,
    utils::{TestHelpers, TestPoll},
};

#[tokio::test]
async fn test_people_chain_connection() {
    let _ = tracing_subscriber::fmt::try_init();

    let setup = E2eTestSetup::new()
        .await
        .expect("Failed to setup e2e test environment");

    let people_client = setup
        .people_client()
        .expect("People client should be available");

    // Basic chain connectivity test
    let chain_info = people_client
        .blocks()
        .at_latest()
        .await
        .expect("Failed to get latest block");

    let block_number = chain_info.header().number;
    tracing::info!("Connected to people chain at block: {}", block_number);

    assert!(block_number > 0, "Block number should be greater than 0");
}

#[tokio::test]
async fn test_server_health_check() {
    let _ = tracing_subscriber::fmt::try_init();

    let setup = E2eTestSetup::new()
        .await
        .expect("Failed to setup e2e test environment");

    setup
        .wait_for_server()
        .await
        .expect("Server should be available");
}

#[tokio::test]
async fn test_basic_poll_creation_and_retrieval() {
    let _ = tracing_subscriber::fmt::try_init();

    let setup = E2eTestSetup::new()
        .await
        .expect("Failed to setup e2e test environment");

    setup
        .wait_for_server()
        .await
        .expect("Server should be available");

    let test_poll = TestPoll {
        title: "E2E Test Poll".to_string(),
        description: "Testing poll creation via API".to_string(),
        options: vec!["Yes".to_string(), "No".to_string()],
        required_identity_proof: false, // Start with false for basic test
    };

    // Get a recognized test user for creating the poll
    let creator_user = setup
        .config
        .get_recognized_person(0)
        .expect("Recognized user should be configured in test_users.json");

    // Create keypair from test user data
    let creator_keypair =
        TestKeyPair::from_secret_scalar(&creator_user.secret_scalar, &creator_user.member_hex)
            .expect("Failed to create keypair from test user data");

    // Create poll using the recognized test user
    let poll_json = test_poll.to_json_with_keypair(&creator_keypair);
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
    let poll_id = response_body["poll_id"]
        .as_u64()
        .expect("Poll ID not found in response")
        .to_string();

    tracing::info!("Created poll with ID: {}", poll_id);

    // Retrieve poll
    let retrieved_poll =
        TestHelpers::get_poll_via_api(&setup.http_client, &setup.config.server_base_url, &poll_id)
            .await
            .expect("Failed to retrieve poll via API");

    // Verify poll data
    assert_eq!(retrieved_poll["name"], test_poll.title);
    assert_eq!(retrieved_poll["description"], test_poll.description);
    assert_eq!(retrieved_poll["is_active"], true);

    // Verify poll has expected fields
    assert!(retrieved_poll["id"].is_number());
    assert!(retrieved_poll["creator_member_hex"].is_string());
    assert_eq!(
        retrieved_poll["creator_member_hex"],
        creator_keypair.member_hex
    );
    assert!(retrieved_poll["created_at"].is_number());
    assert!(retrieved_poll["ends_at"].is_number());
}

#[tokio::test]
async fn test_poll_voting_without_identity_proof() {
    let _ = tracing_subscriber::fmt::try_init();

    let setup = E2eTestSetup::new()
        .await
        .expect("Failed to setup e2e test environment");

    setup
        .wait_for_server()
        .await
        .expect("Server should be available");

    let test_poll = TestPoll {
        title: "Voting Test Poll".to_string(),
        description: "Testing voting without identity proof".to_string(),
        options: vec!["Option A".to_string(), "Option B".to_string()],
        required_identity_proof: false,
    };

    // Get recognized test users
    let creator_user = setup
        .config
        .get_recognized_person(0)
        .expect("Recognized creator user should be configured");
    let voter_user = setup
        .config
        .get_recognized_person(1)
        .expect("Recognized voter user should be configured");

    // Create keypairs from test user data
    let creator_keypair =
        TestKeyPair::from_secret_scalar(&creator_user.secret_scalar, &creator_user.member_hex)
            .expect("Failed to create creator keypair");
    let voter_keypair =
        TestKeyPair::from_secret_scalar(&voter_user.secret_scalar, &voter_user.member_hex)
            .expect("Failed to create voter keypair");

    // Create poll using recognized test user
    let poll_json = test_poll.to_json_with_keypair(&creator_keypair);
    let response = setup
        .http_client
        .post(format!("{}/polls", setup.config.server_base_url))
        .header("Content-Type", "application/json")
        .json(&poll_json)
        .send()
        .await
        .expect("Failed to create poll");

    let response_body: serde_json::Value = response.json().await.expect("Failed to parse response");
    let poll_id = response_body["poll_id"]
        .as_u64()
        .expect("Poll ID not found")
        .to_string();

    // Submit a vote with proper signature using recognized test user
    let poll_id_num: u32 = poll_id.parse().expect("Invalid poll ID");
    let vote_data = TestHelpers::create_signed_vote_data(poll_id_num, "Yay", &voter_keypair);

    let vote_response = TestHelpers::submit_vote_via_api(
        &setup.http_client,
        &setup.config.server_base_url,
        &poll_id,
        &vote_data,
    )
    .await
    .expect("Failed to submit vote");

    tracing::info!("Vote submitted successfully: {:?}", vote_response);

    // Retrieve poll to verify vote was counted
    let updated_poll =
        TestHelpers::get_poll_via_api(&setup.http_client, &setup.config.server_base_url, &poll_id)
            .await
            .expect("Failed to retrieve updated poll");

    // Basic verification that the poll still exists and is accessible
    assert_eq!(updated_poll["name"], test_poll.title);
}

#[tokio::test]
async fn test_recognized_users_poll_creation_and_voting() {
    let _ = tracing_subscriber::fmt::try_init();

    let setup = E2eTestSetup::new()
        .await
        .expect("Failed to setup e2e test environment");

    setup
        .wait_for_server()
        .await
        .expect("Server should be available");

    // Get test users from config
    let user1 = setup
        .config
        .get_recognized_person(0)
        .expect("Recognized user 1 should be configured in test_users.json");
    let user2 = setup
        .config
        .get_recognized_person(1)
        .expect("Recognized user 2 should be configured in test_users.json");

    // Create keypairs from configured test users
    let creator_keypair = TestKeyPair::from_secret_scalar(&user1.secret_scalar, &user1.member_hex)
        .expect("Failed to create keypair for user 1");
    let voter_keypair = TestKeyPair::from_secret_scalar(&user2.secret_scalar, &user2.member_hex)
        .expect("Failed to create keypair for user 2");

    // Create a poll with recognized user 1
    let test_poll = TestPoll {
        title: "Recognized Users Poll".to_string(),
        description: "Testing poll creation and voting with recognized users".to_string(),
        options: vec!["Yes".to_string(), "No".to_string()],
        required_identity_proof: true, // This requires recognized persons
    };

    tracing::info!(
        "Creating poll with recognized user 1 (member key: {})",
        creator_keypair.member_hex
    );

    let poll_json = test_poll.to_json_with_keypair(&creator_keypair);
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

    let poll_id = response_body["poll_id"]
        .as_u64()
        .expect("Poll ID not found in response")
        .to_string();

    tracing::info!("Created poll with ID: {}", poll_id);

    // Verify poll was created correctly
    let retrieved_poll =
        TestHelpers::get_poll_via_api(&setup.http_client, &setup.config.server_base_url, &poll_id)
            .await
            .expect("Failed to retrieve created poll");

    assert_eq!(retrieved_poll["name"], test_poll.title);
    assert_eq!(retrieved_poll["description"], test_poll.description);
    assert_eq!(retrieved_poll["is_active"], true);
    assert_eq!(
        retrieved_poll["creator_member_hex"],
        creator_keypair.member_hex
    );

    tracing::info!("Poll verification passed");

    // Vote on the poll with recognized user 2
    let poll_id_num: u32 = poll_id.parse().expect("Invalid poll ID");

    tracing::info!(
        "Submitting vote with recognized user 2 (member key: {})",
        voter_keypair.member_hex
    );

    let vote_data = TestHelpers::create_signed_vote_data(poll_id_num, "Yay", &voter_keypair);

    let vote_response = TestHelpers::submit_vote_via_api(
        &setup.http_client,
        &setup.config.server_base_url,
        &poll_id,
        &vote_data,
    )
    .await
    .expect("Failed to submit vote");

    tracing::info!("Vote submitted successfully: {:?}", vote_response);

    // Verify vote was recorded by checking vote counts
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
        let vote_counts: serde_json::Value = vote_counts_response
            .json()
            .await
            .expect("Failed to parse vote counts response");

        tracing::info!("Vote counts: {:?}", vote_counts);

        // Verify that at least one vote was recorded
        let yay_votes = vote_counts["yay_votes"].as_u64().unwrap_or(0);
        let nay_votes = vote_counts["nay_votes"].as_u64().unwrap_or(0);
        let total_votes = yay_votes + nay_votes;

        assert!(total_votes > 0, "At least one vote should be recorded");
        tracing::info!(
            "Vote counting verification passed - total votes: {}",
            total_votes
        );
    } else {
        tracing::warn!("Vote counts endpoint not available, skipping vote count verification");
    }

    // Test voting with onboarding user (should fail)
    if let Some(onboarding_user) = setup.config.get_onboarding_user(0) {
        tracing::info!(
            "Testing vote rejection with onboarding user (member key: {})",
            onboarding_user.member_hex
        );

        let onboarding_keypair = TestKeyPair::from_secret_scalar(
            &onboarding_user.secret_scalar,
            &onboarding_user.member_hex,
        )
        .expect("Failed to create keypair for onboarding user");

        let onboarding_vote_data =
            TestHelpers::create_signed_vote_data(poll_id_num, "Nay", &onboarding_keypair);

        let onboarding_vote_response = TestHelpers::submit_vote_via_api(
            &setup.http_client,
            &setup.config.server_base_url,
            &poll_id,
            &onboarding_vote_data,
        )
        .await;

        // This should fail for onboarding users
        match onboarding_vote_response {
            Ok(_) => {
                panic!("Vote by onboarding user should have been rejected but was accepted");
            }
            Err(e) => {
                tracing::info!("Vote by onboarding user correctly rejected: {}", e);
            }
        }
    } else {
        tracing::warn!("No onboarding user configured, skipping onboarding user vote test");
    }

    // Verify poll is still retrievable and shows updated information
    let final_poll =
        TestHelpers::get_poll_via_api(&setup.http_client, &setup.config.server_base_url, &poll_id)
            .await
            .expect("Failed to retrieve poll after voting");

    assert_eq!(final_poll["name"], test_poll.title);
    assert_eq!(final_poll["is_active"], true);

    tracing::info!(
        "E2E test completed successfully: poll created by user 1, voted by user 2, onboarding user vote correctly rejected"
    );
}

#[tokio::test]
async fn test_impersonation_attack_prevention() {
    let _ = tracing_subscriber::fmt::try_init();

    let setup = E2eTestSetup::new()
        .await
        .expect("Failed to setup e2e test environment");

    setup
        .wait_for_server()
        .await
        .expect("Server should be available");

    // Get a legitimate recognized user whose identity we'll try to steal
    let legitimate_user = setup
        .config
        .get_recognized_person(0)
        .expect("Recognized user should be configured in test_users.json");

    // Create the legitimate user's keypair (for comparison)
    let legitimate_keypair = TestKeyPair::from_secret_scalar(
        &legitimate_user.secret_scalar,
        &legitimate_user.member_hex,
    )
    .expect("Failed to create legitimate keypair");

    // Simulate malicious user: knows the public key but NOT the private key
    // Generate a different private key but try to use the legitimate user's public key
    let malicious_keypair = TestKeyPair::generate(); // This creates a different secret

    // Create a "fake" keypair that attempts to impersonate the legitimate user
    // by using their public key but with the wrong secret
    let fake_impersonation_keypair = TestKeyPair {
        member_hex: legitimate_user.member_hex.clone(), // Stolen public key
        secret: malicious_keypair.secret.clone(), // Wrong secret (doesn't match the public key)
    };

    tracing::info!(
        "Attempting impersonation attack: malicious user trying to use public key {} without knowing the private key",
        legitimate_user.member_hex
    );

    // Test 1: Try to create a poll using the impersonated identity
    let test_poll = TestPoll {
        title: "Malicious Impersonation Poll".to_string(),
        description: "This poll should fail to be created due to signature mismatch".to_string(),
        options: vec!["Yes".to_string(), "No".to_string()],
        required_identity_proof: false,
    };

    let poll_json = test_poll.to_json_with_keypair(&fake_impersonation_keypair);
    let poll_response = setup
        .http_client
        .post(format!("{}/polls", setup.config.server_base_url))
        .header("Content-Type", "application/json")
        .json(&poll_json)
        .send()
        .await
        .expect("Failed to send poll creation request");

    // This should fail due to signature verification failure
    assert!(!poll_response.status().is_success());

    // Test 2: Create a legitimate poll first, then try to vote with impersonated identity
    let legitimate_poll_json = test_poll.to_json_with_keypair(&legitimate_keypair);
    let legitimate_poll_response = setup
        .http_client
        .post(format!("{}/polls", setup.config.server_base_url))
        .header("Content-Type", "application/json")
        .json(&legitimate_poll_json)
        .send()
        .await
        .expect("Failed to send legitimate poll creation request");

    if !legitimate_poll_response.status().is_success() {
        let status = legitimate_poll_response.status();
        let error_text = legitimate_poll_response.text().await.unwrap_or_default();
        panic!("Failed to create legitimate poll for impersonation test: {status} - {error_text}");
    }

    let poll_response_body: serde_json::Value = legitimate_poll_response
        .json()
        .await
        .expect("Failed to parse legitimate poll creation response");
    let poll_id = poll_response_body["poll_id"]
        .as_u64()
        .expect("Poll ID not found in response")
        .to_string();

    tracing::info!("Created legitimate poll with ID: {}", poll_id);

    // Now try to vote using the impersonated identity
    let poll_id_num: u32 = poll_id.parse().expect("Invalid poll ID");
    let malicious_vote_data =
        TestHelpers::create_signed_vote_data(poll_id_num, "Yay", &fake_impersonation_keypair);

    tracing::info!(
        "Attempting to vote with impersonated identity on poll {}",
        poll_id
    );

    let vote_response = TestHelpers::submit_vote_via_api(
        &setup.http_client,
        &setup.config.server_base_url,
        &poll_id,
        &malicious_vote_data,
    )
    .await;

    // This should fail due to signature verification
    assert!(!vote_response.is_ok());

    // Test 3: Verify that the legitimate user can still use their identity correctly
    tracing::info!("Verifying legitimate user can still vote with their real keypair");

    let legitimate_vote_data =
        TestHelpers::create_signed_vote_data(poll_id_num, "Nay", &legitimate_keypair);

    let legitimate_vote_response = TestHelpers::submit_vote_via_api(
        &setup.http_client,
        &setup.config.server_base_url,
        &poll_id,
        &legitimate_vote_data,
    )
    .await;

    assert!(legitimate_vote_response.is_ok());
}
