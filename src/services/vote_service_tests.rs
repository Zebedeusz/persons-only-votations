use crate::chain::people_multi::{ChainClient, ChainError};
use crate::services::vote_service::{VoteService, VoteServiceError};
use crate::storage::models::poll::{Poll, PollData};
use crate::storage::models::vote::{CastVoteRequest, Vote, VoteType};
use crate::storage::repositories::poll_repository::{PollRepositoryError, Repository};
use crate::storage::repositories::vote_repository::{VoteRepository, VoteRepositoryError};
use async_trait::async_trait;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct MockVoteRepository {
    pub create_vote_result: Arc<Mutex<Result<(), VoteRepositoryError>>>,
    pub get_vote_by_poll_and_voter_result: Arc<Mutex<Result<Option<Vote>, VoteRepositoryError>>>,
    pub get_votes_by_poll_result: Arc<Mutex<Result<Vec<Vote>, VoteRepositoryError>>>,
    pub count_votes_by_poll_result: Arc<Mutex<Result<(u32, u32), VoteRepositoryError>>>,
}

impl Default for MockVoteRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl MockVoteRepository {
    pub fn new() -> Self {
        Self {
            create_vote_result: Arc::new(Mutex::new(Ok(()))),
            get_vote_by_poll_and_voter_result: Arc::new(Mutex::new(Ok(None))),
            get_votes_by_poll_result: Arc::new(Mutex::new(Ok(vec![]))),
            count_votes_by_poll_result: Arc::new(Mutex::new(Ok((1, 0)))),
        }
    }

    pub fn set_create_vote_result(&self, result: Result<(), VoteRepositoryError>) {
        *self.create_vote_result.lock().unwrap() = result;
    }

    pub fn set_get_vote_by_poll_and_voter_result(
        &self,
        result: Result<Option<Vote>, VoteRepositoryError>,
    ) {
        *self.get_vote_by_poll_and_voter_result.lock().unwrap() = result;
    }

    pub fn set_count_votes_by_poll_result(&self, result: Result<(u32, u32), VoteRepositoryError>) {
        *self.count_votes_by_poll_result.lock().unwrap() = result;
    }
}

impl VoteRepository for MockVoteRepository {
    fn create_vote(&self, _vote: Vote) -> Result<(), VoteRepositoryError> {
        self.create_vote_result.lock().unwrap().clone()
    }

    fn get_votes_by_poll(&self, _poll_id: u32) -> Result<Vec<Vote>, VoteRepositoryError> {
        self.get_votes_by_poll_result.lock().unwrap().clone()
    }

    fn get_vote_by_poll_and_voter(
        &self,
        _poll_id: u32,
        _voter_member_hex: &str,
    ) -> Result<Option<Vote>, VoteRepositoryError> {
        self.get_vote_by_poll_and_voter_result
            .lock()
            .unwrap()
            .clone()
    }

    fn get_vote_by_poll_and_alias(
        &self,
        _poll_id: u32,
        _voter_alias: &str,
    ) -> Result<Option<Vote>, VoteRepositoryError> {
        // Return None for tests - can be customized per test if needed
        Ok(None)
    }

    fn count_votes_by_poll(&self, _poll_id: u32) -> Result<(u32, u32), VoteRepositoryError> {
        self.count_votes_by_poll_result.lock().unwrap().clone()
    }
}

pub struct MockPollRepository {
    pub get_poll_result: Arc<Mutex<Result<Option<Poll>, PollRepositoryError>>>,
    pub update_poll_result: Arc<Mutex<Result<(), PollRepositoryError>>>,
}

impl Default for MockPollRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl MockPollRepository {
    pub fn new() -> Self {
        Self {
            get_poll_result: Arc::new(Mutex::new(Ok(None))),
            update_poll_result: Arc::new(Mutex::new(Ok(()))),
        }
    }

    pub fn set_get_poll_result(&self, result: Result<Option<Poll>, PollRepositoryError>) {
        *self.get_poll_result.lock().unwrap() = result;
    }
}

impl Repository for MockPollRepository {
    fn create_poll(&self, _poll: Poll) -> Result<u32, PollRepositoryError> {
        Ok(1)
    }

    fn get_poll_by_id(&self, _id: u32) -> Result<Option<Poll>, PollRepositoryError> {
        self.get_poll_result.lock().unwrap().clone()
    }

    fn get_polls_by_creator(&self, _creator_key: &str) -> Result<Vec<Poll>, PollRepositoryError> {
        Ok(vec![])
    }

    fn get_all_polls(&self) -> Result<Vec<Poll>, PollRepositoryError> {
        Ok(vec![])
    }

    fn update_poll(&self, _poll: Poll) -> Result<(), PollRepositoryError> {
        self.update_poll_result.lock().unwrap().clone()
    }
}

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

    async fn get_ring_root(
        &self,
        _ring_index: u16,
    ) -> Result<Option<crate::chain::runtime::RingRoot>, ChainError> {
        use crate::crypto::test_helpers::create_mock_ring_root;
        Ok(Some(create_mock_ring_root()))
    }

    async fn get_person_ring_index(&self, _member_hex: &str) -> Result<Option<u16>, ChainError> {
        // For testing, return ring index 0 for verified persons
        Ok(Some(0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_active_poll() -> Poll {
        let future_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64
            + 3600; // 1 hour from now

        let poll_data = PollData {
            name: "Test Poll".to_string(),
            description: "Test Description".to_string(),
            ends_at: future_time,
        };

        Poll::new(
            1,
            "creator_key".to_string(),
            poll_data,
            "0000000000000000000000000000000000000000000000000000000000000001".to_string(), // 32-byte context for poll ID 1
            0, // created_at doesn't matter for these tests
        )
    }

    fn create_valid_vote_request() -> CastVoteRequest {
        CastVoteRequest {
            poll_id: 1,
            voter_member_hex: "voter_key".to_string(),
            vote_type: VoteType::Yay,
            voter_signature: "signature".to_string(),
            ring_proof: "0123456789abcdef".to_string(),
        }
    }

    #[tokio::test]
    async fn test_cast_vote_success() {
        let mock_vote_repo = Arc::new(MockVoteRepository::new());
        let mock_poll_repo = Arc::new(MockPollRepository::new());
        let mock_chain = Arc::new(MockChainClient::new());

        let active_poll = create_active_poll();
        mock_poll_repo.set_get_poll_result(Ok(Some(active_poll)));
        mock_vote_repo.set_create_vote_result(Ok(()));
        mock_vote_repo.set_get_vote_by_poll_and_voter_result(Ok(None)); // No existing vote
        mock_vote_repo.set_count_votes_by_poll_result(Ok((1, 0)));
        mock_chain.set_verify_personhood_result(Ok(true));
        mock_chain.set_validate_signature_result(Ok(true));

        let service = VoteService::new(mock_vote_repo, mock_poll_repo, mock_chain);
        let request = create_valid_vote_request();
        let result = service.cast_vote(request).await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.poll_id, 1);
        assert_eq!(response.message, "Yay vote cast successfully");
    }

    #[tokio::test]
    async fn test_cast_vote_poll_not_found() {
        let mock_vote_repo = Arc::new(MockVoteRepository::new());
        let mock_poll_repo = Arc::new(MockPollRepository::new());
        let mock_chain = Arc::new(MockChainClient::new());

        mock_poll_repo.set_get_poll_result(Ok(None)); // Poll doesn't exist

        let service = VoteService::new(mock_vote_repo, mock_poll_repo, mock_chain);
        let request = create_valid_vote_request();
        let result = service.cast_vote(request).await;

        assert!(result.is_err());
        match result.unwrap_err() {
            VoteServiceError::PollNotFound(id) => assert_eq!(id, 1),
            _ => panic!("Expected PollNotFound error"),
        }
    }

    #[tokio::test]
    async fn test_cast_vote_poll_not_active() {
        let mock_vote_repo = Arc::new(MockVoteRepository::new());
        let mock_poll_repo = Arc::new(MockPollRepository::new());
        let mock_chain = Arc::new(MockChainClient::new());

        let mut inactive_poll = create_active_poll();
        inactive_poll.is_active = false;
        mock_poll_repo.set_get_poll_result(Ok(Some(inactive_poll)));

        let service = VoteService::new(mock_vote_repo, mock_poll_repo, mock_chain);
        let request = create_valid_vote_request();
        let result = service.cast_vote(request).await;

        assert!(result.is_err());
        match result.unwrap_err() {
            VoteServiceError::PollNotActive => {}
            _ => panic!("Expected PollNotActive error"),
        }
    }

    #[tokio::test]
    async fn test_cast_vote_poll_expired() {
        let mock_vote_repo = Arc::new(MockVoteRepository::new());
        let mock_poll_repo = Arc::new(MockPollRepository::new());
        let mock_chain = Arc::new(MockChainClient::new());

        let mut expired_poll = create_active_poll();
        expired_poll.ends_at = 0; // Past timestamp
        mock_poll_repo.set_get_poll_result(Ok(Some(expired_poll)));

        let service = VoteService::new(mock_vote_repo, mock_poll_repo, mock_chain);
        let request = create_valid_vote_request();
        let result = service.cast_vote(request).await;

        assert!(result.is_err());
        match result.unwrap_err() {
            VoteServiceError::PollNotActive => {}
            _ => panic!("Expected PollNotActive error"),
        }
    }

    #[tokio::test]
    async fn test_cast_vote_duplicate_vote() {
        let mock_vote_repo = Arc::new(MockVoteRepository::new());
        let mock_poll_repo = Arc::new(MockPollRepository::new());
        let mock_chain = Arc::new(MockChainClient::new());

        let active_poll = create_active_poll();
        mock_poll_repo.set_get_poll_result(Ok(Some(active_poll)));

        // Simulate existing vote
        let existing_vote = Vote::new(
            1,
            "voter_key".to_string(),
            "alias_hex".to_string(),
            VoteType::Yay,
            "sig".to_string(),
            "ring_proof_hex".to_string(),
            0,
        );
        mock_vote_repo.set_get_vote_by_poll_and_voter_result(Ok(Some(existing_vote)));

        let service = VoteService::new(mock_vote_repo, mock_poll_repo, mock_chain);
        let request = create_valid_vote_request();
        let result = service.cast_vote(request).await;

        assert!(result.is_err());
        match result.unwrap_err() {
            VoteServiceError::DuplicateVote => {}
            _ => panic!("Expected DuplicateVote error"),
        }
    }

    #[tokio::test]
    async fn test_cast_vote_personhood_not_verified() {
        let mock_vote_repo = Arc::new(MockVoteRepository::new());
        let mock_poll_repo = Arc::new(MockPollRepository::new());
        let mock_chain = Arc::new(MockChainClient::new());

        let active_poll = create_active_poll();
        mock_poll_repo.set_get_poll_result(Ok(Some(active_poll)));
        mock_vote_repo.set_get_vote_by_poll_and_voter_result(Ok(None));
        mock_chain.set_verify_personhood_result(Ok(false)); // Not verified

        let service = VoteService::new(mock_vote_repo, mock_poll_repo, mock_chain);
        let request = create_valid_vote_request();
        let result = service.cast_vote(request).await;

        assert!(result.is_err());
        match result.unwrap_err() {
            VoteServiceError::AuthorizationFailed(msg) => {
                assert_eq!(msg, "Voter personhood not verified");
            }
            _ => panic!("Expected AuthorizationFailed error"),
        }
    }

    #[tokio::test]
    async fn test_cast_vote_invalid_signature() {
        let mock_vote_repo = Arc::new(MockVoteRepository::new());
        let mock_poll_repo = Arc::new(MockPollRepository::new());
        let mock_chain = Arc::new(MockChainClient::new());

        let active_poll = create_active_poll();
        mock_poll_repo.set_get_poll_result(Ok(Some(active_poll)));
        mock_vote_repo.set_get_vote_by_poll_and_voter_result(Ok(None));
        mock_chain.set_verify_personhood_result(Ok(true));
        mock_chain.set_validate_signature_result(Ok(false)); // Invalid signature

        let service = VoteService::new(mock_vote_repo, mock_poll_repo, mock_chain);
        let request = create_valid_vote_request();
        let result = service.cast_vote(request).await;

        assert!(result.is_err());
        match result.unwrap_err() {
            VoteServiceError::AuthorizationFailed(msg) => {
                assert_eq!(msg, "Invalid vote signature");
            }
            _ => panic!("Expected AuthorizationFailed error"),
        }
    }

    #[tokio::test]
    async fn test_cast_vote_chain_error_personhood() {
        let mock_vote_repo = Arc::new(MockVoteRepository::new());
        let mock_poll_repo = Arc::new(MockPollRepository::new());
        let mock_chain = Arc::new(MockChainClient::new());

        let active_poll = create_active_poll();
        mock_poll_repo.set_get_poll_result(Ok(Some(active_poll)));
        mock_vote_repo.set_get_vote_by_poll_and_voter_result(Ok(None));
        mock_chain.set_verify_personhood_result(Err(ChainError::ConnectionFailed(
            "Network error".to_string(),
        )));

        let service = VoteService::new(mock_vote_repo, mock_poll_repo, mock_chain);
        let request = create_valid_vote_request();
        let result = service.cast_vote(request).await;

        assert!(result.is_err());
        match result.unwrap_err() {
            VoteServiceError::ChainError(ChainError::ConnectionFailed(msg)) => {
                assert_eq!(msg, "Network error");
            }
            _ => panic!("Expected ChainError::ConnectionFailed"),
        }
    }

    #[tokio::test]
    async fn test_cast_vote_repository_error() {
        let mock_vote_repo = Arc::new(MockVoteRepository::new());
        let mock_poll_repo = Arc::new(MockPollRepository::new());
        let mock_chain = Arc::new(MockChainClient::new());

        let active_poll = create_active_poll();
        mock_poll_repo.set_get_poll_result(Ok(Some(active_poll)));
        mock_vote_repo.set_get_vote_by_poll_and_voter_result(Ok(None));
        mock_vote_repo.set_create_vote_result(Err(VoteRepositoryError::LockError));
        mock_chain.set_verify_personhood_result(Ok(true));
        mock_chain.set_validate_signature_result(Ok(true));

        let service = VoteService::new(mock_vote_repo, mock_poll_repo, mock_chain);
        let request = create_valid_vote_request();
        let result = service.cast_vote(request).await;

        assert!(result.is_err());
        match result.unwrap_err() {
            VoteServiceError::VoteRepositoryError(VoteRepositoryError::LockError) => {}
            _ => panic!("Expected VoteRepositoryError::LockError"),
        }
    }

    #[tokio::test]
    async fn test_cast_nay_vote_success() {
        let mock_vote_repo = Arc::new(MockVoteRepository::new());
        let mock_poll_repo = Arc::new(MockPollRepository::new());
        let mock_chain = Arc::new(MockChainClient::new());

        let active_poll = create_active_poll();
        mock_poll_repo.set_get_poll_result(Ok(Some(active_poll)));
        mock_vote_repo.set_create_vote_result(Ok(()));
        mock_vote_repo.set_get_vote_by_poll_and_voter_result(Ok(None));
        mock_vote_repo.set_count_votes_by_poll_result(Ok((0, 1)));
        mock_chain.set_verify_personhood_result(Ok(true));
        mock_chain.set_validate_signature_result(Ok(true));

        let service = VoteService::new(mock_vote_repo, mock_poll_repo, mock_chain);
        let mut request = create_valid_vote_request();
        request.vote_type = VoteType::Nay;
        let result = service.cast_vote(request).await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.message, "Nay vote cast successfully");
    }

    #[tokio::test]
    async fn test_get_poll_votes_success() {
        let mock_vote_repo = Arc::new(MockVoteRepository::new());
        let mock_poll_repo = Arc::new(MockPollRepository::new());
        let mock_chain = Arc::new(MockChainClient::new());

        let active_poll = create_active_poll();
        mock_poll_repo.set_get_poll_result(Ok(Some(active_poll)));

        let service = VoteService::new(mock_vote_repo, mock_poll_repo, mock_chain);
        let result = service.get_poll_votes(1).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_poll_votes_poll_not_found() {
        let mock_vote_repo = Arc::new(MockVoteRepository::new());
        let mock_poll_repo = Arc::new(MockPollRepository::new());
        let mock_chain = Arc::new(MockChainClient::new());

        mock_poll_repo.set_get_poll_result(Ok(None));

        let service = VoteService::new(mock_vote_repo, mock_poll_repo, mock_chain);
        let result = service.get_poll_votes(1).await;

        assert!(result.is_err());
        match result.unwrap_err() {
            VoteServiceError::PollNotFound(id) => assert_eq!(id, 1),
            _ => panic!("Expected PollNotFound error"),
        }
    }
}
