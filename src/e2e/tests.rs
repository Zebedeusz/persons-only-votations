use super::{setup::E2eTestSetup, test_helpers::*};

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
async fn test_poll_voting_without_identity_proof() {
    let setup = setup_test_environment().await;

    let (creator_user, voter_user) = get_test_users(&setup);
    let test_poll = prepare_voting_poll_data();

    let poll_id = create_poll(&setup, &test_poll, &creator_user).await;
    assert_poll_created_correctly(&setup, &poll_id, &test_poll, &creator_user).await;

    submit_vote(&setup, &poll_id, &voter_user, "Yay").await;
    assert_vote_recorded(&setup, &poll_id, &test_poll).await;
}

#[tokio::test]
async fn test_recognized_users_poll_creation_and_voting() {
    let setup = setup_test_environment().await;

    let (creator_user, voter_user) = get_test_users(&setup);

    let test_poll = prepare_recognized_users_poll_data();

    let poll_id = create_timed_poll(&setup, &test_poll, &creator_user, 5).await;
    assert_poll_created_correctly(&setup, &poll_id, &test_poll, &creator_user).await;

    let reveal_data = commit_vote_and_store_reveal_data(&setup, &poll_id, &voter_user, "Yay").await;
    assert_vote_counts_hidden_during_voting(&setup, &poll_id).await;
    wait_for_poll_to_end_and_reveal(&setup, &poll_id, &reveal_data, 6).await;
    finalize_poll_and_assert_votes_counted(&setup, &poll_id).await;

    test_onboarding_user_vote_rejection(&setup, &poll_id).await;

    assert_final_poll_state(&setup, &poll_id, &test_poll).await;
}

#[tokio::test]
async fn test_impersonation_attack_prevention() {
    let setup = setup_test_environment().await;

    let (legitimate_keypair, fake_impersonation_keypair) = get_legitimate_and_fake_keypairs(&setup);
    let test_poll = prepare_impersonation_test_poll_data();

    attempt_malicious_poll_creation(&setup, &test_poll, &fake_impersonation_keypair).await;

    let poll_id = create_poll(&setup, &test_poll, &legitimate_keypair).await;
    attempt_malicious_vote(&setup, &poll_id, &fake_impersonation_keypair).await;

    verify_legitimate_user_can_vote(&setup, &poll_id, &legitimate_keypair).await;
}
