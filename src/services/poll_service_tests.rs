use crate::chain::people_multi::{ChainClient, ChainError};
use crate::services::poll_service::{PollService, PollServiceError};
use crate::storage::models::poll::{CreatePollRequest, PollData};
use crate::storage::repositories::poll_repository::PollRepository;
use crate::utils::get_current_timestamp;
use async_trait::async_trait;
use std::sync::Arc;
use verifiable::{
    GenerateVerifiable,
    ring_vrf_impl::{BandersnatchVrfVerifiable, MembersCommitment},
};

/// Chain stand-in: every call returns the configured value.
struct MockChainClient {
    signature_valid: Result<bool, ChainError>,
    ring_index: Result<Option<u16>, ChainError>,
}

impl Default for MockChainClient {
    fn default() -> Self {
        Self {
            signature_valid: Ok(true),
            ring_index: Ok(Some(0)),
        }
    }
}

#[async_trait]
impl ChainClient for MockChainClient {
    async fn validate_signature(&self, _: &str, _: &str, _: &[u8]) -> Result<bool, ChainError> {
        self.signature_valid.clone()
    }

    async fn ring_index(&self, _: &str) -> Result<Option<u16>, ChainError> {
        self.ring_index.clone()
    }

    async fn ring_root(&self, _: u16) -> Result<Option<MembersCommitment>, ChainError> {
        Ok(Some(BandersnatchVrfVerifiable::finish_members(
            BandersnatchVrfVerifiable::start_members(),
        )))
    }
}

fn service(chain: MockChainClient) -> PollService {
    PollService::new(Arc::new(PollRepository::default()), Arc::new(chain))
}

fn request_ending_in(seconds: i64) -> CreatePollRequest {
    CreatePollRequest {
        creator_member_hex: "xoxo".to_string(),
        creator_signature: "abcd1234".to_string(),
        poll_data: PollData {
            name: "Test Poll".to_string(),
            description: "Test Description".to_string(),
            ends_at: get_current_timestamp() + seconds,
        },
    }
}

fn valid_request() -> CreatePollRequest {
    request_ending_in(3600)
}

async fn invalid_data_message(request: CreatePollRequest) -> String {
    match service(MockChainClient::default())
        .create_poll(request)
        .await
    {
        Err(PollServiceError::InvalidData(msg)) => msg,
        other => panic!("Expected InvalidData, got {other:?}"),
    }
}

#[tokio::test]
async fn creates_poll_with_context() {
    let response = service(MockChainClient::default())
        .create_poll(valid_request())
        .await
        .unwrap();

    assert_ne!(response.poll_id, 0);
    assert_eq!(response.context.len(), 64);
}

#[tokio::test]
async fn accepts_exactly_30_days_duration() {
    let result = service(MockChainClient::default())
        .create_poll(request_ending_in(30 * 24 * 60 * 60 - 1))
        .await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn rejects_invalid_poll_data() {
    let mut request = valid_request();
    request.poll_data.name = "   \t\n  ".to_string();
    assert_eq!(invalid_data_message(request).await, "Name cannot be empty");

    let mut request = valid_request();
    request.poll_data.name = "a".repeat(201);
    assert_eq!(invalid_data_message(request).await, "Name too long");

    let mut request = valid_request();
    request.poll_data.description = "a".repeat(2001);
    assert_eq!(invalid_data_message(request).await, "Description too long");

    assert_eq!(
        invalid_data_message(request_ending_in(-1)).await,
        "End time must be in the future"
    );
    assert_eq!(
        invalid_data_message(request_ending_in(31 * 24 * 60 * 60)).await,
        "Poll duration cannot exceed 30 days"
    );
}

#[tokio::test]
async fn rejects_invalid_signature() {
    let chain = MockChainClient {
        signature_valid: Ok(false),
        ..Default::default()
    };
    match service(chain).create_poll(valid_request()).await {
        Err(PollServiceError::AuthorizationFailed(msg)) => assert_eq!(msg, "Invalid signature"),
        other => panic!("Expected AuthorizationFailed, got {other:?}"),
    }
}

#[tokio::test]
async fn rejects_unverified_person() {
    let chain = MockChainClient {
        ring_index: Ok(None),
        ..Default::default()
    };
    match service(chain).create_poll(valid_request()).await {
        Err(PollServiceError::AuthorizationFailed(msg)) => assert_eq!(msg, "Person not verified"),
        other => panic!("Expected AuthorizationFailed, got {other:?}"),
    }
}

#[tokio::test]
async fn propagates_chain_errors() {
    let chain = MockChainClient {
        signature_valid: Err(ChainError::ConnectionFailed("Network error".to_string())),
        ..Default::default()
    };
    assert!(matches!(
        service(chain).create_poll(valid_request()).await,
        Err(PollServiceError::ChainError(ChainError::ConnectionFailed(
            _
        )))
    ));

    let chain = MockChainClient {
        ring_index: Err(ChainError::QueryFailed("Query timeout".to_string())),
        ..Default::default()
    };
    assert!(matches!(
        service(chain).create_poll(valid_request()).await,
        Err(PollServiceError::ChainError(ChainError::QueryFailed(_)))
    ));
}
