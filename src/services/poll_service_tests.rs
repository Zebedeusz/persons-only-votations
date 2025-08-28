use crate::chain::people_multi::{ChainClient, ChainError};
use crate::services::poll_service::{PollService, PollServiceError};
use crate::storage::models::poll::{CreatePollRequest, Poll, PollData};
use crate::storage::repositories::poll_repository::{PollRepositoryError, Repository};
use async_trait::async_trait;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct MockChainClient {
    pub verify_personhood_result: Arc<Mutex<Result<bool, ChainError>>>,
    pub validate_signature_result: Arc<Mutex<Result<bool, ChainError>>>,
}

impl Default for MockChainClient {
    fn default() -> Self {
        Self::new()
    }
}

impl MockChainClient {
    pub fn new() -> Self {
        Self {
            verify_personhood_result: Arc::new(Mutex::new(Ok(true))),
            validate_signature_result: Arc::new(Mutex::new(Ok(true))),
        }
    }

    pub fn set_verify_personhood_result(&self, result: Result<bool, ChainError>) {
        *self.verify_personhood_result.lock().unwrap() = result;
    }

    pub fn set_validate_signature_result(&self, result: Result<bool, ChainError>) {
        *self.validate_signature_result.lock().unwrap() = result;
    }
}

#[async_trait]
impl ChainClient for MockChainClient {
    async fn verify_personhood(
        &self,
        _public_key: &str,
        _signature: &str,
    ) -> Result<bool, ChainError> {
        self.verify_personhood_result.lock().unwrap().clone()
    }

    async fn validate_signature(
        &self,
        _public_key: &str,
        _signature: &str,
        _message: &[u8],
    ) -> Result<bool, ChainError> {
        self.validate_signature_result.lock().unwrap().clone()
    }
}

pub struct MockRepository {
    pub create_poll_result: Arc<Mutex<Result<u32, PollRepositoryError>>>,
    pub update_poll_result: Arc<Mutex<Result<(), PollRepositoryError>>>,
    pub get_poll_result: Arc<Mutex<Result<Option<Poll>, PollRepositoryError>>>,
    pub get_polls_by_creator_result: Arc<Mutex<Result<Vec<Poll>, PollRepositoryError>>>,
}

impl Default for MockRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl MockRepository {
    pub fn new() -> Self {
        Self {
            create_poll_result: Arc::new(Mutex::new(Ok(1))),
            update_poll_result: Arc::new(Mutex::new(Ok(()))),
            get_poll_result: Arc::new(Mutex::new(Ok(None))),
            get_polls_by_creator_result: Arc::new(Mutex::new(Ok(vec![]))),
        }
    }

    pub fn set_create_poll_result(&self, result: Result<u32, PollRepositoryError>) {
        *self.create_poll_result.lock().unwrap() = result;
    }

    pub fn set_update_poll_result(&self, result: Result<(), PollRepositoryError>) {
        *self.update_poll_result.lock().unwrap() = result;
    }
}

impl Repository for MockRepository {
    fn create_poll(&self, _poll: Poll) -> Result<u32, PollRepositoryError> {
        self.create_poll_result.lock().unwrap().clone()
    }

    fn get_poll_by_id(&self, _id: u32) -> Result<Option<Poll>, PollRepositoryError> {
        self.get_poll_result.lock().unwrap().clone()
    }

    fn get_polls_by_creator(&self, _creator_key: &str) -> Result<Vec<Poll>, PollRepositoryError> {
        self.get_polls_by_creator_result.lock().unwrap().clone()
    }

    fn get_all_polls(&self) -> Result<Vec<Poll>, PollRepositoryError> {
        // Return empty vec for tests - can be customized per test if needed
        Ok(vec![])
    }

    fn update_poll(&self, _poll: Poll) -> Result<(), PollRepositoryError> {
        self.update_poll_result.lock().unwrap().clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_valid_poll_data() -> PollData {
        let future_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64
            + 3600; // 1 hour from now

        PollData {
            name: "Test Poll".to_string(),
            description: "Test Description".to_string(),
            ends_at: future_time,
        }
    }

    fn create_valid_request() -> CreatePollRequest {
        CreatePollRequest {
            creator_member_hex: "xoxo".to_string(),
            creator_signature: "abcd1234".to_string(),
            poll_data: create_valid_poll_data(),
        }
    }

    #[tokio::test]
    async fn test_create_poll_success() {
        let mock_repo = Arc::new(MockRepository::new());
        let mock_chain = Arc::new(MockChainClient::new());

        mock_repo.set_create_poll_result(Ok(123));
        mock_repo.set_update_poll_result(Ok(()));
        mock_chain.set_validate_signature_result(Ok(true));
        mock_chain.set_verify_personhood_result(Ok(true));

        let service = PollService::new(mock_repo, mock_chain);

        let request = create_valid_request();
        let result = service.create_poll(request).await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.poll_id, 123);
        assert!(!response.context.is_empty());
    }

    #[tokio::test]
    async fn test_create_poll_empty_name() {
        let mock_repo = Arc::new(MockRepository::new());
        let mock_chain = Arc::new(MockChainClient::new());
        let service = PollService::new(mock_repo, mock_chain);

        let mut request = create_valid_request();
        request.poll_data.name = "".to_string();

        let result = service.create_poll(request).await;

        assert!(result.is_err());
        match result.unwrap_err() {
            PollServiceError::InvalidData(msg) => assert_eq!(msg, "Name cannot be empty"),
            _ => panic!("Expected InvalidData error"),
        }
    }

    #[tokio::test]
    async fn test_create_poll_name_too_long() {
        let mock_repo = Arc::new(MockRepository::new());
        let mock_chain = Arc::new(MockChainClient::new());
        let service = PollService::new(mock_repo, mock_chain);

        let mut request = create_valid_request();
        request.poll_data.name = "a".repeat(201);

        let result = service.create_poll(request).await;

        assert!(result.is_err());
        match result.unwrap_err() {
            PollServiceError::InvalidData(msg) => assert_eq!(msg, "Name too long"),
            _ => panic!("Expected InvalidData error"),
        }
    }

    #[tokio::test]
    async fn test_create_poll_description_too_long() {
        let mock_repo = Arc::new(MockRepository::new());
        let mock_chain = Arc::new(MockChainClient::new());
        let service = PollService::new(mock_repo, mock_chain);

        let mut request = create_valid_request();
        request.poll_data.description = "a".repeat(2001);

        let result = service.create_poll(request).await;

        assert!(result.is_err());
        match result.unwrap_err() {
            PollServiceError::InvalidData(msg) => assert_eq!(msg, "Description too long"),
            _ => panic!("Expected InvalidData error"),
        }
    }

    #[tokio::test]
    async fn test_create_poll_end_time_in_past() {
        let mock_repo = Arc::new(MockRepository::new());
        let mock_chain = Arc::new(MockChainClient::new());
        let service = PollService::new(mock_repo, mock_chain);

        let mut request = create_valid_request();
        request.poll_data.ends_at = 0; // Way in the past

        let result = service.create_poll(request).await;

        assert!(result.is_err());
        match result.unwrap_err() {
            PollServiceError::InvalidData(msg) => assert_eq!(msg, "End time must be in the future"),
            _ => panic!("Expected InvalidData error"),
        }
    }

    #[tokio::test]
    async fn test_create_poll_duration_too_long() {
        let mock_repo = Arc::new(MockRepository::new());
        let mock_chain = Arc::new(MockChainClient::new());
        let service = PollService::new(mock_repo, mock_chain);

        let mut request = create_valid_request();
        // 31 days from now
        let far_future = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64
            + (31 * 24 * 60 * 60);
        request.poll_data.ends_at = far_future;

        let result = service.create_poll(request).await;

        assert!(result.is_err());
        match result.unwrap_err() {
            PollServiceError::InvalidData(msg) => {
                assert_eq!(msg, "Poll duration cannot exceed 30 days")
            }
            _ => panic!("Expected InvalidData error"),
        }
    }

    #[tokio::test]
    async fn test_create_poll_invalid_signature() {
        let mock_repo = Arc::new(MockRepository::new());
        let mock_chain = Arc::new(MockChainClient::new());

        mock_chain.set_validate_signature_result(Ok(false));

        let service = PollService::new(mock_repo, mock_chain);

        let request = create_valid_request();
        let result = service.create_poll(request).await;

        assert!(result.is_err());
        match result.unwrap_err() {
            PollServiceError::AuthorizationFailed(msg) => assert_eq!(msg, "Invalid signature"),
            _ => panic!("Expected AuthorizationFailed error"),
        }
    }

    #[tokio::test]
    async fn test_create_poll_signature_validation_chain_error() {
        let mock_repo = Arc::new(MockRepository::new());
        let mock_chain = Arc::new(MockChainClient::new());

        mock_chain.set_validate_signature_result(Err(ChainError::ConnectionFailed(
            "Network error".to_string(),
        )));

        let service = PollService::new(mock_repo, mock_chain);

        let request = create_valid_request();
        let result = service.create_poll(request).await;

        assert!(result.is_err());
        match result.unwrap_err() {
            PollServiceError::ChainError(ChainError::ConnectionFailed(msg)) => {
                assert_eq!(msg, "Network error");
            }
            _ => panic!("Expected ChainError::ConnectionFailed"),
        }
    }

    #[tokio::test]
    async fn test_create_poll_personhood_not_verified() {
        let mock_repo = Arc::new(MockRepository::new());
        let mock_chain = Arc::new(MockChainClient::new());

        mock_chain.set_validate_signature_result(Ok(true));
        mock_chain.set_verify_personhood_result(Ok(false));

        let service = PollService::new(mock_repo, mock_chain);

        let request = create_valid_request();
        let result = service.create_poll(request).await;

        assert!(result.is_err());
        match result.unwrap_err() {
            PollServiceError::AuthorizationFailed(msg) => assert_eq!(msg, "Person not verified"),
            _ => panic!("Expected AuthorizationFailed error"),
        }
    }

    #[tokio::test]
    async fn test_create_poll_personhood_verification_chain_error() {
        let mock_repo = Arc::new(MockRepository::new());
        let mock_chain = Arc::new(MockChainClient::new());

        mock_chain.set_validate_signature_result(Ok(true));
        mock_chain.set_verify_personhood_result(Err(ChainError::QueryFailed(
            "Query timeout".to_string(),
        )));

        let service = PollService::new(mock_repo, mock_chain);

        let request = create_valid_request();
        let result = service.create_poll(request).await;

        assert!(result.is_err());
        match result.unwrap_err() {
            PollServiceError::ChainError(ChainError::QueryFailed(msg)) => {
                assert_eq!(msg, "Query timeout");
            }
            _ => panic!("Expected ChainError::QueryFailed"),
        }
    }

    #[tokio::test]
    async fn test_create_poll_repository_create_error() {
        let mock_repo = Arc::new(MockRepository::new());
        let mock_chain = Arc::new(MockChainClient::new());

        mock_chain.set_validate_signature_result(Ok(true));
        mock_chain.set_verify_personhood_result(Ok(true));
        mock_repo.set_create_poll_result(Err(PollRepositoryError::LockError));

        let service = PollService::new(mock_repo, mock_chain);

        let request = create_valid_request();
        let result = service.create_poll(request).await;

        assert!(result.is_err());
        match result.unwrap_err() {
            PollServiceError::RepositoryError(PollRepositoryError::LockError) => {}
            _ => panic!("Expected RepositoryError::LockError"),
        }
    }

    #[tokio::test]
    async fn test_create_poll_repository_update_error() {
        let mock_repo = Arc::new(MockRepository::new());
        let mock_chain = Arc::new(MockChainClient::new());

        mock_chain.set_validate_signature_result(Ok(true));
        mock_chain.set_verify_personhood_result(Ok(true));
        mock_repo.set_create_poll_result(Ok(456));
        mock_repo.set_update_poll_result(Err(PollRepositoryError::PollNotFound(456)));

        let service = PollService::new(mock_repo, mock_chain);

        let request = create_valid_request();
        let result = service.create_poll(request).await;

        assert!(result.is_err());
        match result.unwrap_err() {
            PollServiceError::RepositoryError(PollRepositoryError::PollNotFound(id)) => {
                assert_eq!(id, 456);
            }
            _ => panic!("Expected RepositoryError::PollNotFound"),
        }
    }

    #[tokio::test]
    async fn test_create_poll_whitespace_only_name() {
        let mock_repo = Arc::new(MockRepository::new());
        let mock_chain = Arc::new(MockChainClient::new());
        let service = PollService::new(mock_repo, mock_chain);

        let mut request = create_valid_request();
        request.poll_data.name = "   \t\n  ".to_string(); // Only whitespace

        let result = service.create_poll(request).await;

        assert!(result.is_err());
        match result.unwrap_err() {
            PollServiceError::InvalidData(msg) => assert_eq!(msg, "Name cannot be empty"),
            _ => panic!("Expected InvalidData error"),
        }
    }

    #[tokio::test]
    async fn test_create_poll_exactly_30_days_duration() {
        let mock_repo = Arc::new(MockRepository::new());
        let mock_chain = Arc::new(MockChainClient::new());

        mock_chain.set_validate_signature_result(Ok(true));
        mock_chain.set_verify_personhood_result(Ok(true));
        mock_repo.set_create_poll_result(Ok(789));
        mock_repo.set_update_poll_result(Ok(()));

        let service = PollService::new(mock_repo, mock_chain);

        let mut request = create_valid_request();
        // 30 days from now
        let exactly_30_days = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64
            + (30 * 24 * 60 * 60);
        request.poll_data.ends_at = exactly_30_days;

        let result = service.create_poll(request).await;
        assert!(result.is_ok());
    }
}
