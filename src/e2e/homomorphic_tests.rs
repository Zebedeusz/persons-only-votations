use super::{crypto_helpers::TestKeyPair, setup::E2eTestSetup, test_helpers::*, utils::TestPoll};
use crate::crypto::contexts::{context_to_string, generate_poll_context};
use crate::crypto::homomorphic::encrypt_vote_with_public_key;
use crate::storage::models::vote::VoteType;

#[tokio::test]
async fn test_encrypted_vote_casting_with_chain_validation() {
    let setup = setup_test_environment().await;
    let (creator_user, voter_user) = get_test_users(&setup);
    let poll_id = create_poll(&setup, &encrypted_voting_poll(), &creator_user).await;
    let public_key = fetch_server_public_key(&setup).await;

    let ballot = encrypt_vote_with_public_key(&public_key, &VoteType::Yay).unwrap();
    let response = cast_encrypted_vote(&setup, &poll_id, &voter_user, &ballot, None).await;

    assert!(
        response.status().is_success(),
        "Encrypted vote casting failed: HTTP {} - {}",
        response.status(),
        response.text().await.unwrap_or_default()
    );
    assert_eq!(get_encrypted_votes_via_api(&setup, &poll_id).await.len(), 1);
}

#[tokio::test]
async fn test_homomorphic_tally_with_multiple_encrypted_votes() {
    let setup = setup_test_environment().await;
    let (creator_user, _) = get_test_users(&setup);
    let votes = [VoteType::Yay, VoteType::Nay, VoteType::Yay];

    let (yay, total) = run_encrypted_poll(&setup, &creator_user, &votes).await;

    assert_eq!(yay, 2);
    assert_eq!(total, 3);
}

#[tokio::test]
async fn test_encrypted_voting_with_malformed_ballots() {
    let setup = setup_test_environment().await;
    let (creator_user, voter_user) = get_test_users(&setup);
    let poll_id = create_poll(&setup, &encrypted_voting_poll(), &creator_user).await;
    let public_key = fetch_server_public_key(&setup).await;

    // Ballot that is not a ciphertext at all
    let response = cast_encrypted_vote(&setup, &poll_id, &voter_user, "deadbeef", None).await;
    assert!(
        response.status().is_client_error(),
        "Malformed ciphertext should be rejected"
    );

    // Valid ballot with a bogus ring proof
    let ballot = encrypt_vote_with_public_key(&public_key, &VoteType::Nay).unwrap();
    let response = cast_encrypted_vote(
        &setup,
        &poll_id,
        &voter_user,
        &ballot,
        Some("invalid_ring_proof".to_string()),
    )
    .await;
    assert!(
        response.status().is_client_error(),
        "Invalid ring proof should be rejected"
    );
}

#[tokio::test]
async fn test_encrypted_voting_duplicate_prevention() {
    let setup = setup_test_environment().await;
    let (creator_user, voter_user) = get_test_users(&setup);
    let poll_id = create_poll(&setup, &encrypted_voting_poll(), &creator_user).await;
    let public_key = fetch_server_public_key(&setup).await;

    let first = encrypt_vote_with_public_key(&public_key, &VoteType::Yay).unwrap();
    let response = cast_encrypted_vote(&setup, &poll_id, &voter_user, &first, None).await;
    assert!(
        response.status().is_success(),
        "First vote failed: HTTP {} - {}",
        response.status(),
        response.text().await.unwrap_or_default()
    );

    let second = encrypt_vote_with_public_key(&public_key, &VoteType::Nay).unwrap();
    let response = cast_encrypted_vote(&setup, &poll_id, &voter_user, &second, None).await;
    assert!(
        response.status().is_client_error(),
        "Duplicate vote should be rejected"
    );
    assert_eq!(get_encrypted_votes_via_api(&setup, &poll_id).await.len(), 1);
}

#[tokio::test]
async fn test_homomorphic_properties_end_to_end() {
    let setup = setup_test_environment().await;
    let (creator_user, _) = get_test_users(&setup);
    let votes = [
        VoteType::Yay,
        VoteType::Nay,
        VoteType::Yay,
        VoteType::Yay,
        VoteType::Nay,
    ];

    let (yay, total) = run_encrypted_poll(&setup, &creator_user, &votes).await;

    assert_eq!(yay, 3);
    assert_eq!(total, 5);
}

fn encrypted_voting_poll() -> TestPoll {
    TestPoll {
        title: "Encrypted Voting Test Poll".to_string(),
        description: "Testing homomorphic encryption voting with chain validation".to_string(),
    }
}

/// Recognized users 1.. each cast one ballot from `votes` on a 60s poll; returns (yay, ballots stored).
async fn run_encrypted_poll(
    setup: &E2eTestSetup,
    creator: &TestKeyPair,
    votes: &[VoteType],
) -> (u64, usize) {
    let public_key = fetch_server_public_key(setup).await;
    let ballots: Vec<(TestKeyPair, String)> = votes
        .iter()
        .enumerate()
        .map(|(i, vote)| {
            let voter = recognized_keypair(setup, i + 1);
            (voter, encrypt_vote_with_public_key(&public_key, vote).unwrap())
        })
        .collect();

    let poll_id = create_timed_poll(setup, &encrypted_voting_poll(), creator, 60).await;

    for (voter, ballot) in &ballots {
        let response = cast_encrypted_vote(setup, &poll_id, voter, ballot, None).await;
        assert!(
            response.status().is_success(),
            "Vote casting failed: HTTP {} - {}",
            response.status(),
            response.text().await.unwrap_or_default()
        );
    }

    tokio::time::sleep(tokio::time::Duration::from_secs(61)).await;

    let tally_response = setup
        .http_client
        .get(format!(
            "{}/polls/{}/homomorphic-tally",
            setup.config.server_base_url, poll_id
        ))
        .send()
        .await
        .expect("Failed to get homomorphic tally");
    assert!(
        tally_response.status().is_success(),
        "Tally computation should succeed"
    );
    let tally: serde_json::Value = tally_response.json().await.expect("Failed to parse tally");
    let yay = tally["total_yay_votes"]
        .as_u64()
        .expect("Tally should contain total_yay_votes");

    (yay, get_encrypted_votes_via_api(setup, &poll_id).await.len())
}

async fn fetch_server_public_key(setup: &E2eTestSetup) -> String {
    let response = setup
        .http_client
        .get(format!("{}/encryption-key", setup.config.server_base_url))
        .send()
        .await
        .expect("Failed to fetch encryption key from server");
    assert!(
        response.status().is_success(),
        "Failed to get encryption key: HTTP {}",
        response.status()
    );
    let body: serde_json::Value = response.json().await.expect("Failed to parse key response");
    body["public_key"]
        .as_str()
        .expect("Public key not found in response")
        .to_string()
}

/// Signs the ballot and submits it; `ring_proof` of `None` generates a real proof from chain data.
async fn cast_encrypted_vote(
    setup: &E2eTestSetup,
    poll_id: &str,
    voter: &TestKeyPair,
    encrypted_vote: &str,
    ring_proof: Option<String>,
) -> reqwest::Response {
    let poll_id_num: u32 = poll_id.parse().expect("Invalid poll_id");
    let context_bytes = generate_poll_context(poll_id_num);
    let message = serde_json::to_string(&serde_json::json!({
        "poll_id": poll_id_num,
        "encrypted_vote": encrypted_vote,
        "context": context_to_string(&context_bytes)
    }))
    .unwrap();

    let ring_proof = match ring_proof {
        Some(proof) => proof,
        None => ring_proof_for(setup, voter, &context_bytes, message.as_bytes()).await,
    };
    let voter_signature = voter
        .sign_message(message.as_bytes())
        .expect("Failed to sign message");

    setup
        .http_client
        .post(format!(
            "{}/polls/{}/cast-encrypted",
            setup.config.server_base_url, poll_id
        ))
        .json(&serde_json::json!({
            "poll_id": poll_id_num,
            "voter_member_hex": voter.member_hex,
            "encrypted_vote": encrypted_vote,
            "ring_proof": ring_proof,
            "voter_signature": voter_signature
        }))
        .send()
        .await
        .expect("Failed to send encrypted vote request")
}

async fn get_encrypted_votes_via_api(setup: &E2eTestSetup, poll_id: &str) -> Vec<serde_json::Value> {
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
    let data: serde_json::Value = response.json().await.expect("Failed to parse votes");
    data["encrypted_votes"]
        .as_array()
        .expect("Response should contain encrypted_votes array")
        .clone()
}
