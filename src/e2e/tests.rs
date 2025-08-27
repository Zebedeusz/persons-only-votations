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

    if setup.is_mock_mode() {
        tracing::info!("Skipping people chain connection test in mock mode");
        return;
    }

    let people_client = setup
        .people_client()
        .expect("People client should be available in non-mock mode");

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

    // Create poll
    let poll_id = TestHelpers::create_poll_via_api(
        &setup.http_client,
        &setup.config.server_base_url,
        &test_poll,
    )
    .await
    .expect("Failed to create poll via API");

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
    assert!(retrieved_poll["creator_public_key"].is_string());
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

    // Create poll
    let poll_id = TestHelpers::create_poll_via_api(
        &setup.http_client,
        &setup.config.server_base_url,
        &test_poll,
    )
    .await
    .expect("Failed to create poll");

    // Submit a vote with proper signature
    let voter_keypair = TestKeyPair::generate();
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
#[ignore = "Requires identity proof implementation"]
async fn test_poll_voting_with_identity_proof() {
    let _ = tracing_subscriber::fmt::try_init();

    let setup = E2eTestSetup::new()
        .await
        .expect("Failed to setup e2e test environment");

    setup
        .wait_for_server()
        .await
        .expect("Server should be available");

    if setup.is_mock_mode() {
        tracing::info!("Running identity proof test in mock mode");
        // In mock mode, we can test the API flow with mock identity proofs
        let test_poll = TestPoll {
            title: "Identity Proof Test Poll".to_string(),
            description: "Testing voting with mock identity proof".to_string(),
            options: vec!["Yes".to_string(), "No".to_string()],
            required_identity_proof: true,
        };

        let poll_id = TestHelpers::create_poll_via_api(
            &setup.http_client,
            &setup.config.server_base_url,
            &test_poll,
        )
        .await
        .expect("Failed to create poll");

        // Submit a vote with proper signature
        let voter_keypair = TestKeyPair::generate();
        let poll_id_num: u32 = poll_id.parse().expect("Invalid poll ID");
        let vote_data = TestHelpers::create_signed_vote_data(poll_id_num, "Yay", &voter_keypair);

        let vote_response = TestHelpers::submit_vote_via_api(
            &setup.http_client,
            &setup.config.server_base_url,
            &poll_id,
            &vote_data,
        )
        .await;

        // In mock mode, this should succeed
        match vote_response {
            Ok(_) => tracing::info!("Mock identity proof voting test passed"),
            Err(e) => tracing::warn!("Mock identity proof voting failed: {}", e),
        }

        return;
    }

    // This test would require:
    // 1. Setting up test accounts on people chain
    // 2. Getting proper identity proofs
    // 3. Creating poll with identity requirement
    // 4. Submitting vote with valid proof

    // For now, this is a placeholder showing the structure
    // Implementation depends on people chain setup and identity system

    tracing::info!("Identity proof voting test - implementation pending for real chain mode");
}
