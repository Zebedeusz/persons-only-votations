use super::{crypto_helpers::TestKeyPair, setup::E2eTestSetup, test_helpers::*, utils::TestPoll};
use crate::crypto::homomorphic::PaillierVotingSystem;
use crate::crypto::contexts::{context_to_string, generate_poll_context};
use crate::storage::models::vote::VoteType;

#[tokio::test]
async fn test_encrypted_vote_casting_with_chain_validation() {
    let setup = setup_test_environment().await;
    let (creator_user, voter_user) = get_test_users(&setup);

    let test_poll = prepare_encrypted_voting_poll_data();
    let poll_id = create_poll(&setup, &test_poll, &creator_user).await;

    let voting_system = fetch_server_public_key(&setup).await;

    let encrypted_vote = voting_system
        .encrypt_vote(&VoteType::Yay)
        .expect("Failed to encrypt vote");

    let validity_proof = voting_system
        .create_validity_proof(&VoteType::Yay, &encrypted_vote)
        .expect("Failed to create validity proof");

    let commitment = voting_system
        .create_pedersen_commitment(&VoteType::Yay)
        .expect("Failed to create commitment");

    let encrypted_vote_hex = serialize_tfhe_ciphertext_to_hex(&encrypted_vote);

    let ring_proof = create_ring_proof_for_voter(
        &setup,
        &voter_user,
        poll_id.parse().unwrap(),
        &encrypted_vote_hex,
        &validity_proof,
        &commitment,
    )
    .await;

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

    assert!(
        cast_vote_response.status().is_success(),
        "Encrypted vote casting failed: HTTP {} - {}",
        cast_vote_response.status(),
        cast_vote_response.text().await.unwrap_or_default()
    );

    let encrypted_votes = get_encrypted_votes_via_api(&setup, &poll_id).await;
    assert_eq!(encrypted_votes.len(), 1);
}

#[tokio::test]
async fn test_homomorphic_tally_with_multiple_encrypted_votes() {
    let setup = setup_test_environment().await;
    let (creator_user, voter1_user) = get_test_users(&setup);

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

    let shared_voting_system = fetch_server_public_key(&setup).await;

    let vote_scenarios = vec![
        (&voter1_user, VoteType::Yay),
        (&voter2_keypair, VoteType::Nay),
        (&voter3_keypair, VoteType::Yay),
    ];

    let mut pre_generated_crypto = Vec::new();

    for (_voter, vote_type) in vote_scenarios.iter() {
        let (encrypted_vote_hex, validity_proof, commitment) =
            pre_generate_encrypted_vote_data_with_system(&shared_voting_system, vote_type).await;
        pre_generated_crypto.push((vote_type, encrypted_vote_hex, validity_proof, commitment));
    }

    let test_poll = prepare_encrypted_voting_poll_data();
    let poll_id = create_timed_poll(&setup, &test_poll, &creator_user, 60).await;

    for ((voter, _), (_vote_type, encrypted_vote_hex, validity_proof, commitment)) in
        vote_scenarios.iter().zip(pre_generated_crypto.into_iter()) {

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

        assert!(
            response.status().is_success(),
            "Vote casting failed: HTTP {} - {}",
            response.status(),
            response.text().await.unwrap_or_default()
        );
    }

    tokio::time::sleep(tokio::time::Duration::from_secs(61)).await;

    let tally_response = get_homomorphic_tally_via_api(&setup, &poll_id).await;
    assert!(
        tally_response.status().is_success(),
        "Tally computation should succeed"
    );

    let tally_data: serde_json::Value = tally_response
        .json()
        .await
        .expect("Failed to parse tally response");

    let total_yay_votes = tally_data["total_yay_votes"]
        .as_u64()
        .expect("Tally should contain total_yay_votes");

    let encrypted_votes = get_encrypted_votes_via_api(&setup, &poll_id).await;

    assert_eq!(total_yay_votes, 2);
    assert_eq!(encrypted_votes.len(), 3);
}

#[tokio::test]
async fn test_encrypted_voting_with_invalid_proofs() {
    let setup = setup_test_environment().await;
    let (creator_user, voter_user) = get_test_users(&setup);

    let test_poll = prepare_encrypted_voting_poll_data();
    let poll_id = create_poll(&setup, &test_poll, &creator_user).await;

    let voting_system = fetch_server_public_key(&setup).await;

    let encrypted_vote = voting_system
        .encrypt_vote(&VoteType::Yay)
        .expect("Failed to encrypt vote");

    let invalid_proof = "invalid_proof_data";
    let valid_commitment = voting_system
        .create_pedersen_commitment(&VoteType::Yay)
        .expect("Failed to create commitment");

    let encrypted_vote_hex = serialize_tfhe_ciphertext_to_hex(&encrypted_vote);

    let ring_proof = create_ring_proof_for_voter(
        &setup,
        &voter_user,
        poll_id.parse().unwrap(),
        &encrypted_vote_hex,
        invalid_proof,
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

    let encrypted_vote2 = voting_system
        .encrypt_vote(&VoteType::Nay)
        .expect("Failed to encrypt second vote");
    let valid_validity_proof = voting_system
        .create_validity_proof(&VoteType::Nay, &encrypted_vote2)
        .expect("Failed to create validity proof");
    let valid_commitment2 = voting_system
        .create_pedersen_commitment(&VoteType::Nay)
        .expect("Failed to create commitment");

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
}

#[tokio::test]
async fn test_encrypted_voting_duplicate_prevention() {
    let setup = setup_test_environment().await;
    let (creator_user, voter_user) = get_test_users(&setup);

    let test_poll = prepare_encrypted_voting_poll_data();
    let poll_id = create_poll(&setup, &test_poll, &creator_user).await;

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

    assert!(
        response1.status().is_success(),
        "First vote failed: HTTP {} - {}",
        response1.status(),
        response1.text().await.unwrap_or_default()
    );

    let (encrypted_vote_hex2, validity_proof2, commitment2, _ring_proof2) =
        generate_coordinated_encrypted_vote_data(&setup, &voter_user, poll_id.parse().unwrap(), &VoteType::Nay).await;

    let response2 = cast_encrypted_vote_via_api(
        &setup,
        &poll_id,
        &voter_user,
        encrypted_vote_hex2,
        validity_proof2,
        commitment2,
        ring_proof1,
    )
    .await;

    assert!(
        response2.status().is_client_error(),
        "Duplicate vote should be rejected"
    );

    let encrypted_votes = get_encrypted_votes_via_api(&setup, &poll_id).await;
    assert_eq!(encrypted_votes.len(), 1);
}

#[tokio::test]
async fn test_homomorphic_properties_end_to_end() {
    let setup = setup_test_environment().await;
    let (creator_user, _) = get_test_users(&setup);

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

    let shared_voting_system = fetch_server_public_key(&setup).await;

    let vote_pattern = [
        VoteType::Yay,
        VoteType::Nay,
        VoteType::Yay,
        VoteType::Yay,
        VoteType::Nay,
    ];

    let mut pre_generated_crypto = Vec::new();

    for (voter, vote_type) in voters.iter().zip(vote_pattern.iter()) {
        let (encrypted_vote_hex, validity_proof, commitment) =
            pre_generate_encrypted_vote_data_with_system(&shared_voting_system, vote_type).await;
        pre_generated_crypto.push((voter, vote_type, encrypted_vote_hex, validity_proof, commitment));
    }

    let test_poll = prepare_encrypted_voting_poll_data();
    let poll_id = create_timed_poll(&setup, &test_poll, &creator_user, 60).await;

    for (voter, _vote_type, encrypted_vote_hex, validity_proof, commitment) in pre_generated_crypto.into_iter() {
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

        assert!(
            response.status().is_success(),
            "Vote casting failed: HTTP {} - {}",
            response.status(),
            response.text().await.unwrap_or_default()
        );
    }

    tokio::time::sleep(tokio::time::Duration::from_secs(61)).await;

    let tally_response = get_homomorphic_tally_via_api(&setup, &poll_id).await;
    let tally_data: serde_json::Value = tally_response
        .json()
        .await
        .expect("Failed to parse tally response");

    let total_yay_votes = tally_data["total_yay_votes"]
        .as_u64()
        .expect("Tally should contain total_yay_votes");

    let encrypted_votes = get_encrypted_votes_via_api(&setup, &poll_id).await;

    assert_eq!(total_yay_votes, 3);
    assert_eq!(encrypted_votes.len(), 5);
}

fn prepare_encrypted_voting_poll_data() -> TestPoll {
    TestPoll {
        title: "Encrypted Voting Test Poll".to_string(),
        description: "Testing homomorphic encryption voting with chain validation".to_string(),
        options: vec!["Yay".to_string(), "Nay".to_string()],
        required_identity_proof: true,
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

    let context_bytes = generate_poll_context(poll_id);
    let poll_context = context_to_string(&context_bytes);

    let signature_data = serde_json::json!({
        "poll_id": poll_id,
        "encrypted_vote": encrypted_vote,
        "vote_commitment": commitment,
        "validity_proof": validity_proof,
        "context": poll_context
    });

    let signature_message = serde_json::to_string(&signature_data)
        .expect("Failed to serialize signature data");

    let voter_ring_index = TestHelpers::get_voter_ring_index_from_chain(&voter.member_hex, setup)
        .await
        .unwrap_or_else(|e| {
            panic!("Failed to get ring index for voter {}: {}", &voter.member_hex[0..8], e)
        });

    let ring_keys = TestHelpers::get_ring_keys_from_chain(voter_ring_index, setup)
        .await
        .unwrap_or_else(|e| {
            panic!("Failed to get ring keys for ring {}: {}", voter_ring_index, e)
        });

    voter.generate_ring_proof(ring_keys.into_iter(), &context_bytes, signature_message.as_bytes())
        .unwrap_or_else(|e| {
            panic!("Ring proof generation failed for voter {}: {}", &voter.member_hex[0..8], e)
        })
}

async fn fetch_server_public_key(setup: &E2eTestSetup) -> PaillierVotingSystem {
    let response = setup
        .http_client
        .get(format!("{}/encryption-key", setup.config.server_base_url))
        .send()
        .await
        .expect("Failed to fetch encryption key from server");

    assert!(
        response.status().is_success(),
        "Failed to get encryption key: HTTP {} - {}",
        response.status(),
        response.text().await.unwrap_or_default()
    );

    let key_response: serde_json::Value = response
        .json()
        .await
        .expect("Failed to parse encryption key response");

    let public_key_hex = key_response["public_key"]
        .as_str()
        .expect("Public key not found in response");

    PaillierVotingSystem::from_public_key_hex(public_key_hex)
        .expect("Failed to create voting system from server's public key")
}

async fn pre_generate_encrypted_vote_data_with_system(
    voting_system: &PaillierVotingSystem,
    vote_type: &VoteType,
) -> (String, String, String) {
    let encrypted_vote = voting_system.encrypt_vote(vote_type).expect("Failed to encrypt vote");
    let encrypted_vote_hex = serialize_tfhe_ciphertext_to_hex(&encrypted_vote);
    let validity_proof = voting_system.create_validity_proof(vote_type, &encrypted_vote).expect("Failed to create validity proof");
    let commitment = voting_system.create_pedersen_commitment(vote_type).expect("Failed to create commitment");

    (encrypted_vote_hex, validity_proof, commitment)
}

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

fn serialize_tfhe_ciphertext_to_hex(ciphertext: &tfhe::shortint::Ciphertext) -> String {
    let serialized_bytes = bincode::serialize(ciphertext)
        .expect("Failed to serialize TFHE ciphertext");
    hex::encode(serialized_bytes)
}

async fn generate_coordinated_encrypted_vote_data(
    setup: &E2eTestSetup,
    voter: &TestKeyPair,
    poll_id: u32,
    vote_type: &VoteType,
) -> (String, String, String, String) {
    let voting_system = fetch_server_public_key(setup).await;

    let encrypted_vote = voting_system.encrypt_vote(vote_type).expect("Failed to encrypt vote");
    let validity_proof = voting_system.create_validity_proof(vote_type, &encrypted_vote).expect("Failed to create validity proof");
    let commitment = voting_system.create_pedersen_commitment(vote_type).expect("Failed to create commitment");

    let encrypted_vote_hex = serialize_tfhe_ciphertext_to_hex(&encrypted_vote);

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
    let poll_id_num = poll_id.parse::<u32>().expect("Invalid poll_id");
    let context_bytes = generate_poll_context(poll_id_num);
    let poll_context = context_to_string(&context_bytes);

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
