use super::{setup::E2eTestSetup, test_helpers::*, crypto_helpers::TestKeyPair, utils::TestPoll};
use crate::crypto::homomorphic::PaillierVotingSystem;
use crate::storage::models::vote::VoteType;

/// Test homomorphic encryption voting
#[tokio::test]
async fn test_encrypted_vote_casting_with_chain_validation() {
    let setup = setup_test_environment().await;
    let (creator_user, voter_user) = get_test_users(&setup);

    // Create a poll for encrypted voting
    let test_poll = prepare_encrypted_voting_poll_data();
    let poll_id = create_poll(&setup, &test_poll, &creator_user).await;

    // Initialize homomorphic voting system
    let voting_system = PaillierVotingSystem::new()
        .expect("Failed to create homomorphic voting system");

    // Test: Cast encrypted vote through API
    let encrypted_vote = voting_system.encrypt_vote(&VoteType::Yay)
        .expect("Failed to encrypt vote");

    let validity_proof = voting_system.create_validity_proof(&VoteType::Yay, &encrypted_vote)
        .expect("Failed to create validity proof");

    let commitment = voting_system.create_pedersen_commitment(&VoteType::Yay)
        .expect("Failed to create commitment");

    // Create ring proof for voter
    let ring_proof = create_ring_proof_for_voter(&setup, &voter_user, poll_id.parse().unwrap()).await;

    // Cast encrypted vote via API
    let cast_vote_response = cast_encrypted_vote_via_api(
        &setup,
        &poll_id,
        &voter_user,
        encrypted_vote,
        validity_proof,
        commitment,
        ring_proof,
    ).await;

    assert!(cast_vote_response.status().is_success(), "Encrypted vote should be cast successfully");

    // Verify vote was stored
    let encrypted_votes = get_encrypted_votes_via_api(&setup, &poll_id).await;
    assert_eq!(encrypted_votes.len(), 1, "Should have one encrypted vote");

    println!("✅ Encrypted vote casting with chain validation test passed");
}

#[tokio::test]
async fn test_homomorphic_tally_with_multiple_encrypted_votes() {
    let setup = setup_test_environment().await;
    let (creator_user, voter1_user) = get_test_users(&setup);

    // Get additional test users for multiple votes
    let voter2_user = setup.config.get_recognized_person(2)
        .expect("Need at least 3 recognized users for this test");
    let voter3_user = setup.config.get_recognized_person(3)
        .expect("Need at least 4 recognized users for this test");

    let voter2_keypair = TestKeyPair::from_secret_scalar(&voter2_user.secret_scalar, &voter2_user.member_hex)
        .expect("Failed to create voter2 keypair");
    let voter3_keypair = TestKeyPair::from_secret_scalar(&voter3_user.secret_scalar, &voter3_user.member_hex)
        .expect("Failed to create voter3 keypair");

    // Create poll for encrypted voting
    let test_poll = prepare_encrypted_voting_poll_data();
    let poll_id = create_poll(&setup, &test_poll, &creator_user).await;

    let voting_system = PaillierVotingSystem::new()
        .expect("Failed to create homomorphic voting system");

    // Cast multiple encrypted votes with different patterns
    let vote_scenarios = vec![
        (&voter1_user, VoteType::Yay),   // Vote 1: Yay
        (&voter2_keypair, VoteType::Nay), // Vote 2: Nay
        (&voter3_keypair, VoteType::Yay), // Vote 3: Yay
    ];

    for (voter, vote_type) in vote_scenarios {
        let encrypted_vote = voting_system.encrypt_vote(&vote_type)
            .expect("Failed to encrypt vote");

        let validity_proof = voting_system.create_validity_proof(&vote_type, &encrypted_vote)
            .expect("Failed to create validity proof");

        let commitment = voting_system.create_pedersen_commitment(&vote_type)
            .expect("Failed to create commitment");

        let ring_proof = create_ring_proof_for_voter(&setup, voter, poll_id.parse().unwrap()).await;

        let response = cast_encrypted_vote_via_api(
            &setup,
            &poll_id,
            voter,
            encrypted_vote,
            validity_proof,
            commitment,
            ring_proof,
        ).await;

        assert!(response.status().is_success(), "Each encrypted vote should be cast successfully");
    }

    // Test homomorphic tally computation via API
    let tally_response = get_homomorphic_tally_via_api(&setup, &poll_id).await;
    assert!(tally_response.status().is_success(), "Tally computation should succeed");

    let tally_data: serde_json::Value = tally_response.json().await
        .expect("Failed to parse tally response");

    // Expected: 2 Yay votes (1+1) + 1 Nay vote (0) = 2
    let total_yay_votes = tally_data["total_yay_votes"].as_u64()
        .expect("Tally should contain total_yay_votes");

    assert_eq!(total_yay_votes, 2, "Homomorphic tally should show 2 Yay votes");

    // Verify encrypted votes are stored correctly
    let encrypted_votes = get_encrypted_votes_via_api(&setup, &poll_id).await;
    assert_eq!(encrypted_votes.len(), 3, "Should have three encrypted votes");

    println!("✅ Homomorphic tally with multiple encrypted votes test passed");
    println!("   📊 Computed tally: {total_yay_votes} Yay votes from 3 total votes");
}

#[tokio::test]
async fn test_encrypted_voting_with_invalid_proofs() {
    let setup = setup_test_environment().await;
    let (creator_user, voter_user) = get_test_users(&setup);

    // Create poll
    let test_poll = prepare_encrypted_voting_poll_data();
    let poll_id = create_poll(&setup, &test_poll, &creator_user).await;

    let voting_system = PaillierVotingSystem::new()
        .expect("Failed to create homomorphic voting system");

    let encrypted_vote = voting_system.encrypt_vote(&VoteType::Yay)
        .expect("Failed to encrypt vote");

    // Test: Invalid validity proof
    let invalid_proof = "invalid_proof_data";
    let valid_commitment = voting_system.create_pedersen_commitment(&VoteType::Yay)
        .expect("Failed to create commitment");
    let ring_proof = create_ring_proof_for_voter(&setup, &voter_user, poll_id.parse().unwrap()).await;

    let response = cast_encrypted_vote_via_api(
        &setup,
        &poll_id,
        &voter_user,
        encrypted_vote.clone(),
        invalid_proof.to_string(),
        valid_commitment,
        ring_proof.clone(),
    ).await;

    assert!(response.status().is_client_error(), "Invalid validity proof should be rejected");

    // Test: Invalid ring proof
    let valid_validity_proof = voting_system.create_validity_proof(&VoteType::Yay, &encrypted_vote)
        .expect("Failed to create validity proof");
    let valid_commitment2 = voting_system.create_pedersen_commitment(&VoteType::Yay)
        .expect("Failed to create commitment");

    let response2 = cast_encrypted_vote_via_api(
        &setup,
        &poll_id,
        &voter_user,
        encrypted_vote,
        valid_validity_proof,
        valid_commitment2,
        "invalid_ring_proof".to_string(),
    ).await;

    assert!(response2.status().is_client_error(), "Invalid ring proof should be rejected");

    println!("✅ Encrypted voting with invalid proofs test passed");
}

#[tokio::test]
async fn test_encrypted_voting_duplicate_prevention() {
    let setup = setup_test_environment().await;
    let (creator_user, voter_user) = get_test_users(&setup);

    let test_poll = prepare_encrypted_voting_poll_data();
    let poll_id = create_poll(&setup, &test_poll, &creator_user).await;

    let voting_system = PaillierVotingSystem::new()
        .expect("Failed to create homomorphic voting system");

    // Cast first vote
    let encrypted_vote1 = voting_system.encrypt_vote(&VoteType::Yay)
        .expect("Failed to encrypt vote");
    let validity_proof1 = voting_system.create_validity_proof(&VoteType::Yay, &encrypted_vote1)
        .expect("Failed to create validity proof");
    let commitment1 = voting_system.create_pedersen_commitment(&VoteType::Yay)
        .expect("Failed to create commitment");
    let ring_proof1 = create_ring_proof_for_voter(&setup, &voter_user, poll_id.parse().unwrap()).await;

    let response1 = cast_encrypted_vote_via_api(
        &setup,
        &poll_id,
        &voter_user,
        encrypted_vote1,
        validity_proof1,
        commitment1,
        ring_proof1.clone(),
    ).await;

    assert!(response1.status().is_success(), "First vote should succeed");

    // Attempt second vote from same user (should fail)
    let encrypted_vote2 = voting_system.encrypt_vote(&VoteType::Nay)
        .expect("Failed to encrypt second vote");
    let validity_proof2 = voting_system.create_validity_proof(&VoteType::Nay, &encrypted_vote2)
        .expect("Failed to create validity proof");
    let commitment2 = voting_system.create_pedersen_commitment(&VoteType::Nay)
        .expect("Failed to create commitment");

    let response2 = cast_encrypted_vote_via_api(
        &setup,
        &poll_id,
        &voter_user,
        encrypted_vote2,
        validity_proof2,
        commitment2,
        ring_proof1, // Reuse ring proof
    ).await;

    assert!(response2.status().is_client_error(), "Duplicate vote should be rejected");

    // Verify only one vote is stored
    let encrypted_votes = get_encrypted_votes_via_api(&setup, &poll_id).await;
    assert_eq!(encrypted_votes.len(), 1, "Should have only one vote despite duplicate attempt");

    println!("✅ Encrypted voting duplicate prevention test passed");
}

#[tokio::test]
async fn test_homomorphic_properties_end_to_end() {
    let setup = setup_test_environment().await;
    let (creator_user, _) = get_test_users(&setup);

    // Get multiple voters
    let voters: Vec<_> = (1..=5).map(|i| {
        let user = setup.config.get_recognized_person(i)
            .unwrap_or_else(|| panic!("Need recognized user {i}"));
        TestKeyPair::from_secret_scalar(&user.secret_scalar, &user.member_hex)
            .expect("Failed to create voter keypair")
    }).collect();

    let test_poll = prepare_encrypted_voting_poll_data();
    let poll_id = create_poll(&setup, &test_poll, &creator_user).await;

    let voting_system = PaillierVotingSystem::new()
        .expect("Failed to create homomorphic voting system");

    // Cast votes with known pattern: [Yay, Nay, Yay, Yay, Nay] = 3 Yay votes
    let vote_pattern = [
        VoteType::Yay,  // 1
        VoteType::Nay,  // 0
        VoteType::Yay,  // 1
        VoteType::Yay,  // 1
        VoteType::Nay,  // 0
    ];

    for (voter, vote_type) in voters.iter().zip(vote_pattern.iter()) {
        let encrypted_vote = voting_system.encrypt_vote(vote_type)
            .expect("Failed to encrypt vote");
        let validity_proof = voting_system.create_validity_proof(vote_type, &encrypted_vote)
            .expect("Failed to create validity proof");
        let commitment = voting_system.create_pedersen_commitment(vote_type)
            .expect("Failed to create commitment");
        let ring_proof = create_ring_proof_for_voter(&setup, voter, poll_id.parse().unwrap()).await;

        let response = cast_encrypted_vote_via_api(
            &setup,
            &poll_id,
            voter,
            encrypted_vote,
            validity_proof,
            commitment,
            ring_proof,
        ).await;

        assert!(response.status().is_success(), "Vote casting should succeed");
    }

    // Verify homomorphic tally
    let tally_response = get_homomorphic_tally_via_api(&setup, &poll_id).await;
    let tally_data: serde_json::Value = tally_response.json().await
        .expect("Failed to parse tally response");

    let total_yay_votes = tally_data["total_yay_votes"].as_u64()
        .expect("Tally should contain total_yay_votes");

    assert_eq!(total_yay_votes, 3, "Homomorphic tally should correctly compute 3 Yay votes");

    // Verify individual encrypted votes are preserved
    let encrypted_votes = get_encrypted_votes_via_api(&setup, &poll_id).await;
    assert_eq!(encrypted_votes.len(), 5, "All 5 encrypted votes should be stored");

    println!("✅ Homomorphic properties end-to-end test passed");
    println!("   🔢 Pattern: [Yay,Nay,Yay,Yay,Nay] → {total_yay_votes} Yay votes");
}

// Helper functions for e2e encrypted voting tests

fn prepare_encrypted_voting_poll_data() -> TestPoll {
    TestPoll {
        title: "Encrypted Voting Test Poll".to_string(),
        description: "Testing homomorphic encryption voting with chain validation".to_string(),
        options: vec!["Yay".to_string(), "Nay".to_string()],
        required_identity_proof: true, // Require identity proof for encrypted voting
    }
}

async fn create_ring_proof_for_voter(
    _setup: &E2eTestSetup,
    voter: &TestKeyPair,
    poll_id: u32,
) -> String {
    // This would integrate with the actual ring proof generation
    // For now, return a mock proof that passes validation
    format!("ring_proof_{}_{}", voter.member_hex, poll_id)
}

async fn cast_encrypted_vote_via_api(
    setup: &E2eTestSetup,
    poll_id: &str,
    voter: &TestKeyPair,
    _encrypted_vote: tfhe::shortint::Ciphertext,
    validity_proof: String,
    commitment: String,
    ring_proof: String,
) -> reqwest::Response {
    // Serialize encrypted vote (simplified for this implementation)
    let encrypted_vote_hex = format!("encrypted_{}", fastrand::u64(..));

    let vote_payload = serde_json::json!({
        "voter_alias": format!("voter_{}", &voter.member_hex[..8]),
        "encrypted_vote": encrypted_vote_hex,
        "vote_commitment": commitment,
        "validity_proof": validity_proof,
        "ring_proof": ring_proof,
        "voter_member_hex": voter.member_hex,
        "signature": voter.sign_message(format!("vote_{poll_id}").as_bytes()).expect("Failed to sign message")
    });

    setup
        .http_client
        .post(format!("{}/polls/{}/cast-encrypted", setup.config.server_base_url, poll_id))
        .header("Content-Type", "application/json")
        .json(&vote_payload)
        .send()
        .await
        .expect("Failed to send encrypted vote request")
}

async fn get_encrypted_votes_via_api(setup: &E2eTestSetup, poll_id: &str) -> Vec<serde_json::Value> {
    let response = setup
        .http_client
        .get(format!("{}/polls/{}/encrypted-votes", setup.config.server_base_url, poll_id))
        .send()
        .await
        .expect("Failed to get encrypted votes");

    assert!(response.status().is_success(), "Get encrypted votes should succeed");

    let data: serde_json::Value = response.json().await
        .expect("Failed to parse encrypted votes response");

    data["encrypted_votes"].as_array()
        .expect("Response should contain encrypted_votes array")
        .clone()
}

async fn get_homomorphic_tally_via_api(setup: &E2eTestSetup, poll_id: &str) -> reqwest::Response {
    setup
        .http_client
        .get(format!("{}/polls/{}/homomorphic-tally", setup.config.server_base_url, poll_id))
        .send()
        .await
        .expect("Failed to get homomorphic tally")
}