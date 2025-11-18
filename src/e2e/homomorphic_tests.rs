use super::{crypto_helpers::TestKeyPair, setup::E2eTestSetup, test_helpers::*, utils::TestPoll};
use crate::crypto::homomorphic::PaillierVotingSystem;
use crate::crypto::contexts::{context_to_string, generate_poll_context};
use crate::storage::models::vote::VoteType;

/// Test homomorphic encryption voting
#[tokio::test]
async fn test_encrypted_vote_casting_with_chain_validation() {
    let setup = setup_test_environment().await;
    let (creator_user, voter_user) = get_test_users(&setup);

    // Create a poll for encrypted voting
    let test_poll = prepare_encrypted_voting_poll_data();
    let poll_id = create_poll(&setup, &test_poll, &creator_user).await;

    // Fetch server's public encryption key (simulates real voter behavior)
    let voting_system = fetch_server_public_key(&setup).await;

    // Test: Cast encrypted vote through API
    let encrypted_vote = voting_system
        .encrypt_vote(&VoteType::Yay)
        .expect("Failed to encrypt vote");

    let validity_proof = voting_system
        .create_validity_proof(&VoteType::Yay, &encrypted_vote)
        .expect("Failed to create validity proof");

    let commitment = voting_system
        .create_pedersen_commitment(&VoteType::Yay)
        .expect("Failed to create commitment");

    // Serialize the encrypted vote to hex
    let encrypted_vote_hex = serialize_tfhe_ciphertext_to_hex(&encrypted_vote);

    // Create ring proof for voter (using the encrypted vote data that will be sent to the API)
    let ring_proof = create_ring_proof_for_voter(
        &setup,
        &voter_user,
        poll_id.parse().unwrap(),
        &encrypted_vote_hex,
        &validity_proof,
        &commitment,
    )
    .await;

    // Cast encrypted vote via API (using the same encrypted_vote_hex for ring proof validation)
    let cast_vote_response = cast_encrypted_vote_via_api(
        &setup,
        &poll_id,
        &voter_user,
        encrypted_vote_hex,
        validity_proof,
        commitment,
        ring_proof,
    )
    .await;

    if !cast_vote_response.status().is_success() {
        let status = cast_vote_response.status();
        let error_body = cast_vote_response.text().await.unwrap_or_default();
        tracing::error!("❌ Encrypted vote casting failed for poll {}", poll_id);
        tracing::error!("❌ HTTP Status: {}", status);
        tracing::error!("❌ Error Response: {}", error_body);
        tracing::error!("❌ Voter: {}", &voter_user.member_hex[0..8]);
        panic!("Encrypted vote casting failed: HTTP {} - {}", status, error_body);
    }

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
    let voter2_user = setup
        .config
        .get_recognized_person(2)
        .expect("Need at least 3 recognized users for this test");
    let voter3_user = setup
        .config
        .get_recognized_person(3)
        .expect("Need at least 4 recognized users for this test");

    let voter2_keypair =
        TestKeyPair::from_secret_scalar(&voter2_user.secret_scalar, &voter2_user.member_hex)
            .expect("Failed to create voter2 keypair");
    let voter3_keypair =
        TestKeyPair::from_secret_scalar(&voter3_user.secret_scalar, &voter3_user.member_hex)
            .expect("Failed to create voter3 keypair");

    // Fetch server's public encryption key (simulates real voter behavior)
    println!("🔑 Fetching server's public encryption key...");
    let shared_voting_system = fetch_server_public_key(&setup).await;
    println!("✅ Using server's encryption key for votes");

    // Pre-generate all encrypted vote data before creating poll (this takes the long time)
    println!("🔄 Pre-generating encrypted vote data for 3 voters...");
    let vote_scenarios = vec![
        (&voter1_user, VoteType::Yay),    // Vote 1: Yay
        (&voter2_keypair, VoteType::Nay), // Vote 2: Nay
        (&voter3_keypair, VoteType::Yay), // Vote 3: Yay
    ];

    let mut pre_generated_crypto = Vec::new();

    for (i, (voter, vote_type)) in vote_scenarios.iter().enumerate() {
        println!("🔄 Generating TFHE encryption {} of 3 for {:?} vote...", i + 1, vote_type);
        let (encrypted_vote_hex, validity_proof, commitment) =
            pre_generate_encrypted_vote_data_with_system(&shared_voting_system, vote_type).await;
        pre_generated_crypto.push((voter, vote_type, encrypted_vote_hex, validity_proof, commitment));
    }

    println!("✅ All TFHE crypto data pre-generated!");

    // Now create poll with shorter duration since crypto is pre-generated
    let test_poll = prepare_encrypted_voting_poll_data();
    let poll_id = create_timed_poll(&setup, &test_poll, &creator_user, 60).await;

    // Generate ring proofs and cast votes with pre-generated crypto
    println!("🗳️ Generating ring proofs and casting {} votes...", pre_generated_crypto.len());
    for (i, (voter, _vote_type, encrypted_vote_hex, validity_proof, commitment)) in pre_generated_crypto.into_iter().enumerate() {
        println!("🗳️ Generating ring proof and casting vote {} for voter: {}...", i + 1, &voter.member_hex[0..8]);

        // Generate ring proof with real poll ID
        let ring_proof = generate_ring_proof_for_pre_generated_data(
            &setup,
            voter,
            poll_id.parse().unwrap(),
            &encrypted_vote_hex,
            &validity_proof,
            &commitment,
        ).await;

        let response = cast_encrypted_vote_via_api(
            &setup,
            &poll_id,
            voter,
            encrypted_vote_hex,
            validity_proof,
            commitment,
            ring_proof,
        )
        .await;

        if !response.status().is_success() {
            let status = response.status();
            let error_body = response.text().await.unwrap_or_default();
            panic!("Encrypted vote casting failed for voter: HTTP {} - {}", status, error_body);
        }
    }
    println!("✅ All votes cast successfully!");

    // Wait for poll to expire before computing tally
    tokio::time::sleep(tokio::time::Duration::from_secs(61)).await;

    // Test homomorphic tally computation via API
    let tally_response = get_homomorphic_tally_via_api(&setup, &poll_id).await;
    assert!(
        tally_response.status().is_success(),
        "Tally computation should succeed"
    );

    let tally_data: serde_json::Value = tally_response
        .json()
        .await
        .expect("Failed to parse tally response");

    println!("🔍 Debug tally response: {}", serde_json::to_string_pretty(&tally_data).unwrap_or_default());

    // Expected: 2 Yay votes (1+1) + 1 Nay vote (0) = 2
    let total_yay_votes = tally_data["total_yay_votes"]
        .as_u64()
        .expect("Tally should contain total_yay_votes");

    // Verify encrypted votes are stored correctly
    let encrypted_votes = get_encrypted_votes_via_api(&setup, &poll_id).await;
    println!("🔍 Debug: {} encrypted votes stored", encrypted_votes.len());

    assert_eq!(
        total_yay_votes, 2,
        "Homomorphic tally should show 2 Yay votes but got {}. Tally data: {}",
        total_yay_votes,
        serde_json::to_string_pretty(&tally_data).unwrap_or_default()
    );

    assert_eq!(
        encrypted_votes.len(),
        3,
        "Should have three encrypted votes"
    );

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

    // Fetch server's public encryption key
    let voting_system = fetch_server_public_key(&setup).await;

    let encrypted_vote = voting_system
        .encrypt_vote(&VoteType::Yay)
        .expect("Failed to encrypt vote");

    // Test: Invalid validity proof
    let invalid_proof = "invalid_proof_data";
    let valid_commitment = voting_system
        .create_pedersen_commitment(&VoteType::Yay)
        .expect("Failed to create commitment");

    // Serialize encrypted vote to hex
    let encrypted_vote_hex = serialize_tfhe_ciphertext_to_hex(&encrypted_vote);

    // Generate coordinated data but use invalid proof
    let ring_proof = create_ring_proof_for_voter(
        &setup,
        &voter_user,
        poll_id.parse().unwrap(),
        &encrypted_vote_hex,
        invalid_proof,  // Use invalid proof in ring proof generation too
        &valid_commitment,
    )
    .await;

    let response = cast_encrypted_vote_via_api(
        &setup,
        &poll_id,
        &voter_user,
        encrypted_vote_hex.clone(),
        invalid_proof.to_string(),
        valid_commitment,
        ring_proof.clone(),
    )
    .await;

    assert!(
        response.status().is_client_error(),
        "Invalid validity proof should be rejected"
    );

    // Test: Invalid ring proof
    let encrypted_vote2 = voting_system
        .encrypt_vote(&VoteType::Nay)
        .expect("Failed to encrypt second vote");
    let valid_validity_proof = voting_system
        .create_validity_proof(&VoteType::Nay, &encrypted_vote2)
        .expect("Failed to create validity proof");
    let valid_commitment2 = voting_system
        .create_pedersen_commitment(&VoteType::Nay)
        .expect("Failed to create commitment");

    // Use a different encrypted vote hex for the second test
    let encrypted_vote_hex2 = serialize_tfhe_ciphertext_to_hex(&encrypted_vote2);

    let response2 = cast_encrypted_vote_via_api(
        &setup,
        &poll_id,
        &voter_user,
        encrypted_vote_hex2,
        valid_validity_proof,
        valid_commitment2,
        "invalid_ring_proof".to_string(),
    )
    .await;

    assert!(
        response2.status().is_client_error(),
        "Invalid ring proof should be rejected"
    );

    println!("✅ Encrypted voting with invalid proofs test passed");
}

#[tokio::test]
async fn test_encrypted_voting_duplicate_prevention() {
    let setup = setup_test_environment().await;
    let (creator_user, voter_user) = get_test_users(&setup);

    let test_poll = prepare_encrypted_voting_poll_data();
    let poll_id = create_poll(&setup, &test_poll, &creator_user).await;

    let _voting_system =
        PaillierVotingSystem::new().expect("Failed to create homomorphic voting system");

    // Cast first vote with coordinated data
    let (encrypted_vote_hex1, validity_proof1, commitment1, ring_proof1) =
        generate_coordinated_encrypted_vote_data(&setup, &voter_user, poll_id.parse().unwrap(), &VoteType::Yay).await;

    let response1 = cast_encrypted_vote_via_api(
        &setup,
        &poll_id,
        &voter_user,
        encrypted_vote_hex1,
        validity_proof1,
        commitment1,
        ring_proof1.clone(),
    )
    .await;

    if !response1.status().is_success() {
        let status = response1.status();
        let error_body = response1.text().await.unwrap_or_default();
        panic!("First vote should succeed: HTTP {} - {}", status, error_body);
    }

    // Attempt second vote from same user (should fail)
    let (encrypted_vote_hex2, validity_proof2, commitment2, _ring_proof2) =
        generate_coordinated_encrypted_vote_data(&setup, &voter_user, poll_id.parse().unwrap(), &VoteType::Nay).await;

    let response2 = cast_encrypted_vote_via_api(
        &setup,
        &poll_id,
        &voter_user,
        encrypted_vote_hex2,
        validity_proof2,
        commitment2,
        ring_proof1, // Reuse ring proof from first vote
    )
    .await;

    assert!(
        response2.status().is_client_error(),
        "Duplicate vote should be rejected"
    );

    // Verify only one vote is stored
    let encrypted_votes = get_encrypted_votes_via_api(&setup, &poll_id).await;
    assert_eq!(
        encrypted_votes.len(),
        1,
        "Should have only one vote despite duplicate attempt"
    );

    println!("✅ Encrypted voting duplicate prevention test passed");
}

#[tokio::test]
async fn test_homomorphic_properties_end_to_end() {
    let setup = setup_test_environment().await;
    let (creator_user, _) = get_test_users(&setup);

    // Get multiple voters (indices 1-5 for 5 voters total)
    let voters: Vec<_> = (1..6)
        .map(|i| {
            let user = setup
                .config
                .get_recognized_person(i)
                .unwrap_or_else(|| panic!("Need recognized user at index {}", i));
            TestKeyPair::from_secret_scalar(&user.secret_scalar, &user.member_hex)
                .expect("Failed to create voter keypair")
        })
        .collect();

    // Fetch server's public encryption key (simulates real voter behavior)
    println!("🔑 Fetching server's public encryption key...");
    let shared_voting_system = fetch_server_public_key(&setup).await;
    println!("✅ Using server's encryption key for votes");

    // Pre-generate all encrypted vote data before creating poll (this takes the long time)
    println!("🔄 Pre-generating encrypted vote data for 5 voters...");
    let vote_pattern = [
        VoteType::Yay, // 1
        VoteType::Nay, // 0
        VoteType::Yay, // 1
        VoteType::Yay, // 1
        VoteType::Nay, // 0
    ];

    let mut pre_generated_crypto = Vec::new();

    for (i, (voter, vote_type)) in voters.iter().zip(vote_pattern.iter()).enumerate() {
        println!("🔄 Generating TFHE encryption {} of 5 for voter: {} ({:?})...", i + 1, &voter.member_hex[0..8], vote_type);
        let (encrypted_vote_hex, validity_proof, commitment) =
            pre_generate_encrypted_vote_data_with_system(&shared_voting_system, vote_type).await;
        pre_generated_crypto.push((voter, vote_type, encrypted_vote_hex, validity_proof, commitment));
    }

    println!("✅ All TFHE crypto data pre-generated!");

    // Now create poll with shorter duration since crypto is pre-generated
    let test_poll = prepare_encrypted_voting_poll_data();
    let poll_id = create_timed_poll(&setup, &test_poll, &creator_user, 60).await;

    // Generate ring proofs and cast votes with pre-generated crypto
    println!("🗳️ Generating ring proofs and casting {} votes...", pre_generated_crypto.len());
    for (i, (voter, _vote_type, encrypted_vote_hex, validity_proof, commitment)) in pre_generated_crypto.into_iter().enumerate() {
        println!("🗳️ Generating ring proof and casting vote {} for voter: {}...", i + 1, &voter.member_hex[0..8]);

        // Generate ring proof with real poll ID
        let ring_proof = generate_ring_proof_for_pre_generated_data(
            &setup,
            voter,
            poll_id.parse().unwrap(),
            &encrypted_vote_hex,
            &validity_proof,
            &commitment,
        ).await;

        let response = cast_encrypted_vote_via_api(
            &setup,
            &poll_id,
            voter,
            encrypted_vote_hex,
            validity_proof,
            commitment,
            ring_proof,
        )
        .await;

        if !response.status().is_success() {
            let status = response.status();
            let error_body = response.text().await.unwrap_or_default();
            panic!("Vote casting failed: HTTP {} - {}", status, error_body);
        }
    }
    println!("✅ All votes cast successfully!");

    // Wait for poll to expire before computing tally
    tokio::time::sleep(tokio::time::Duration::from_secs(61)).await;

    // Verify homomorphic tally
    let tally_response = get_homomorphic_tally_via_api(&setup, &poll_id).await;
    let tally_data: serde_json::Value = tally_response
        .json()
        .await
        .expect("Failed to parse tally response");

    let total_yay_votes = tally_data["total_yay_votes"]
        .as_u64()
        .expect("Tally should contain total_yay_votes");

    assert_eq!(
        total_yay_votes, 3,
        "Homomorphic tally should correctly compute 3 Yay votes"
    );

    // Verify individual encrypted votes are preserved
    let encrypted_votes = get_encrypted_votes_via_api(&setup, &poll_id).await;
    assert_eq!(
        encrypted_votes.len(),
        5,
        "All 5 encrypted votes should be stored"
    );

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
    setup: &E2eTestSetup,
    voter: &TestKeyPair,
    poll_id: u32,
    encrypted_vote: &str,
    validity_proof: &str,
    commitment: &str,
) -> String {
    use super::utils::TestHelpers;
    use crate::crypto::contexts::{generate_poll_context, context_to_string};

    // Generate the correct poll context
    let context_bytes = generate_poll_context(poll_id);
    let poll_context = context_to_string(&context_bytes);

    // Create the signature message that matches what the server expects (same as in cast_encrypted_vote_via_api)
    let signature_data = serde_json::json!({
        "poll_id": poll_id,
        "encrypted_vote": encrypted_vote,
        "vote_commitment": commitment,
        "validity_proof": validity_proof,
        "context": poll_context
    });

    let signature_message = serde_json::to_string(&signature_data)
        .expect("Failed to serialize signature data");

    // Get the voter's ring index from the chain
    let voter_ring_index = match TestHelpers::get_voter_ring_index_from_chain(&voter.member_hex, setup).await {
        Ok(index) => {
            tracing::debug!("✅ Found voter {} in ring {}", &voter.member_hex[0..8], index);
            index
        }
        Err(e) => {
            tracing::error!("❌ Failed to get ring index for voter {}: {}", &voter.member_hex[0..8], e);
            if e.to_string().contains("Person is not included in any ring") {
                panic!("❌ Voter {} is not included in any ring (may be onboarding). Ensure test users are properly registered in active rings.", &voter.member_hex[0..8]);
            } else {
                panic!("❌ Ring index lookup failed for voter {}: {}", &voter.member_hex[0..8], e);
            }
        }
    };

    // Get the individual ring keys from the chain
    let ring_keys = match TestHelpers::get_ring_keys_from_chain(voter_ring_index, setup).await {
        Ok(keys) => {
            tracing::debug!("✅ Retrieved {} ring keys for ring {}", keys.len(), voter_ring_index);
            if keys.len() < 2 {
                tracing::warn!("⚠️  Ring {} only has {} members, ring proofs may not work properly", voter_ring_index, keys.len());
            }
            keys
        }
        Err(e) => {
            tracing::error!("❌ Failed to get ring keys for ring {}: {}", voter_ring_index, e);
            panic!("❌ Ring keys retrieval failed for ring {}: {}", voter_ring_index, e);
        }
    };

    // Generate the real ring proof using the exact same message format that the server will validate
    let ring_keys_len = ring_keys.len();
    match voter.generate_ring_proof(ring_keys.into_iter(), &context_bytes, signature_message.as_bytes()) {
        Ok(proof) => {
            tracing::debug!("✅ Generated ring proof for voter {} (length: {} chars)", &voter.member_hex[0..8], proof.len());
            proof
        }
        Err(e) => {
            tracing::error!("❌ Failed to generate ring proof for voter {}: {}", &voter.member_hex[0..8], e);
            panic!("❌ Ring proof generation failed for voter {} in ring {} with {} members: {}", &voter.member_hex[0..8], voter_ring_index, ring_keys_len, e);
        }
    }
}

// Helper to fetch the server's public encryption key
async fn fetch_server_public_key(setup: &E2eTestSetup) -> PaillierVotingSystem {
    let response = setup
        .http_client
        .get(format!("{}/encryption-key", setup.config.server_base_url))
        .send()
        .await
        .expect("Failed to fetch encryption key from server");

    if !response.status().is_success() {
        let status = response.status();
        let error_body = response.text().await.unwrap_or_default();
        panic!("Failed to get encryption key: HTTP {} - {}", status, error_body);
    }

    let key_response: serde_json::Value = response
        .json()
        .await
        .expect("Failed to parse encryption key response");

    let public_key_hex = key_response["public_key"]
        .as_str()
        .expect("Public key not found in response");

    tracing::debug!("✅ Fetched server's public encryption key ({} bytes)", public_key_hex.len() / 2);

    PaillierVotingSystem::from_public_key_hex(public_key_hex)
        .expect("Failed to create voting system from server's public key")
}

// Helper to pre-generate poll-agnostic encrypted vote data using a shared voting system
async fn pre_generate_encrypted_vote_data_with_system(
    voting_system: &PaillierVotingSystem,
    vote_type: &VoteType,
) -> (String, String, String) {
    // Use real TFHE encryption for true e2e testing
    let encrypted_vote = voting_system.encrypt_vote(vote_type).expect("Failed to encrypt vote");

    // Serialize the real TFHE ciphertext to hex string
    let encrypted_vote_hex = serialize_tfhe_ciphertext_to_hex(&encrypted_vote);

    // Generate validity proof and commitment
    let validity_proof = voting_system.create_validity_proof(vote_type, &encrypted_vote).expect("Failed to create validity proof");
    let commitment = voting_system.create_pedersen_commitment(vote_type).expect("Failed to create commitment");

    tracing::debug!("Pre-generated real TFHE encrypted vote for {:?}: {} bytes", vote_type, encrypted_vote_hex.len() / 2);

    (encrypted_vote_hex, validity_proof, commitment)
}

// Helper to generate poll-specific ring proof using pre-generated data
async fn generate_ring_proof_for_pre_generated_data(
    setup: &E2eTestSetup,
    voter: &TestKeyPair,
    poll_id: u32,
    encrypted_vote_hex: &str,
    validity_proof: &str,
    commitment: &str,
) -> String {
    create_ring_proof_for_voter(
        setup,
        voter,
        poll_id,
        encrypted_vote_hex,
        validity_proof,
        commitment,
    )
    .await
}

// Helper to serialize TFHE ciphertext to hex string
fn serialize_tfhe_ciphertext_to_hex(ciphertext: &tfhe::shortint::Ciphertext) -> String {
    
    // Serialize the ciphertext to bytes
    let serialized_bytes = bincode::serialize(ciphertext)
        .expect("Failed to serialize TFHE ciphertext");

    // Convert to hex string
    hex::encode(serialized_bytes)
}

// Helper to generate coordinated encrypted vote data for testing with real TFHE ciphertext
async fn generate_coordinated_encrypted_vote_data(
    setup: &E2eTestSetup,
    voter: &TestKeyPair,
    poll_id: u32,
    vote_type: &VoteType,
) -> (String, String, String, String) {
    // Fetch server's public encryption key (simulates real voter behavior)
    let voting_system = fetch_server_public_key(setup).await;

    // Generate actual cryptographic proofs
    let encrypted_vote = voting_system.encrypt_vote(vote_type).expect("Failed to encrypt vote");
    let validity_proof = voting_system.create_validity_proof(vote_type, &encrypted_vote).expect("Failed to create validity proof");
    let commitment = voting_system.create_pedersen_commitment(vote_type).expect("Failed to create commitment");

    // Serialize the real TFHE ciphertext to hex string
    let encrypted_vote_hex = serialize_tfhe_ciphertext_to_hex(&encrypted_vote);

    tracing::debug!("Generated real TFHE ciphertext for {:?}: {} bytes", vote_type, encrypted_vote_hex.len() / 2);

    // Generate ring proof using coordinated data
    let ring_proof = create_ring_proof_for_voter(
        setup,
        voter,
        poll_id,
        &encrypted_vote_hex,
        &validity_proof,
        &commitment,
    )
    .await;

    (encrypted_vote_hex, validity_proof, commitment, ring_proof)
}

async fn cast_encrypted_vote_via_api(
    setup: &E2eTestSetup,
    poll_id: &str,
    voter: &TestKeyPair,
    encrypted_vote_hex: String,  // Accept as parameter instead of generating
    validity_proof: String,
    commitment: String,
    ring_proof: String,
) -> reqwest::Response {
    cast_encrypted_vote_via_api_with_data(
        setup,
        poll_id,
        voter,
        &encrypted_vote_hex,
        validity_proof,
        commitment,
        ring_proof,
    )
    .await
}

async fn cast_encrypted_vote_via_api_with_data(
    setup: &E2eTestSetup,
    poll_id: &str,
    voter: &TestKeyPair,
    encrypted_vote_hex: &str,
    validity_proof: String,
    commitment: String,
    ring_proof: String,
) -> reqwest::Response {

    // Generate the correct poll context (must match server expectations)
    let poll_id_num = poll_id.parse::<u32>().expect("Invalid poll_id");
    let context_bytes = generate_poll_context(poll_id_num);
    let poll_context = context_to_string(&context_bytes);

    // Create the signature message that matches server expectations
    let signature_data = serde_json::json!({
        "poll_id": poll_id_num,
        "encrypted_vote": encrypted_vote_hex,
        "vote_commitment": commitment,
        "validity_proof": validity_proof,
        "context": poll_context
    });

    let signature_message = serde_json::to_string(&signature_data).expect("Failed to serialize signature data");
    let voter_signature = voter.sign_message(signature_message.as_bytes()).expect("Failed to sign message");

    let vote_payload = serde_json::json!({
        "poll_id": poll_id_num,
        "voter_member_hex": voter.member_hex,
        "encrypted_vote": encrypted_vote_hex,
        "vote_commitment": commitment,
        "validity_proof": validity_proof,
        "ring_proof": ring_proof,
        "voter_signature": voter_signature
    });

    setup
        .http_client
        .post(format!(
            "{}/polls/{}/cast-encrypted",
            setup.config.server_base_url, poll_id
        ))
        .header("Content-Type", "application/json")
        .json(&vote_payload)
        .send()
        .await
        .expect("Failed to send encrypted vote request")
}

async fn get_encrypted_votes_via_api(
    setup: &E2eTestSetup,
    poll_id: &str,
) -> Vec<serde_json::Value> {
    let response = setup
        .http_client
        .get(format!(
            "{}/polls/{}/encrypted-votes",
            setup.config.server_base_url, poll_id
        ))
        .send()
        .await
        .expect("Failed to get encrypted votes");

    assert!(
        response.status().is_success(),
        "Get encrypted votes should succeed"
    );

    let data: serde_json::Value = response
        .json()
        .await
        .expect("Failed to parse encrypted votes response");

    data["encrypted_votes"]
        .as_array()
        .expect("Response should contain encrypted_votes array")
        .clone()
}

async fn get_homomorphic_tally_via_api(setup: &E2eTestSetup, poll_id: &str) -> reqwest::Response {
    setup
        .http_client
        .get(format!(
            "{}/polls/{}/homomorphic-tally",
            setup.config.server_base_url, poll_id
        ))
        .send()
        .await
        .expect("Failed to get homomorphic tally")
}
