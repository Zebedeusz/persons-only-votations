use crate::chain::people_multi::{ChainClient, ChainError};
use crate::crypto::contexts::{context_to_string, generate_poll_context};
use crate::storage::models::poll::{CreatePollRequest, Poll, PollData};
use crate::storage::repositories::poll_repository::{PollRepositoryError, Repository};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PollServiceError {
    #[error("Invalid poll data: {0}")]
    InvalidData(String),
    #[error("Authorization failed: {0}")]
    AuthorizationFailed(String),
    #[error("Chain error: {0}")]
    ChainError(#[from] ChainError),
    #[error("Repository error: {0}")]
    RepositoryError(#[from] PollRepositoryError),
    #[error("System time error")]
    SystemTimeError,
}

pub type Result<T> = std::result::Result<T, PollServiceError>;

#[derive(Debug, Clone, serde::Serialize)]
pub struct CreatePollResponse {
    pub poll_id: u32,
    pub context: String,
}

pub struct PollService {
    repository: Arc<dyn Repository>,
    chain_client: Arc<dyn ChainClient>,
}

impl PollService {
    pub fn new(repository: Arc<dyn Repository>, chain_client: Arc<dyn ChainClient>) -> Self {
        Self {
            repository,
            chain_client,
        }
    }

    pub async fn create_poll(&self, request: CreatePollRequest) -> Result<CreatePollResponse> {
        self.validate_poll_data(&request.poll_data)?;

        let message = self.create_signature_message(&request)?;
        let signature_valid = self
            .chain_client
            .validate_signature(
                &request.creator_public_key,
                &request.creator_signature,
                &message,
            )
            .await?;

        if !signature_valid {
            return Err(PollServiceError::AuthorizationFailed(
                "Invalid signature".to_string(),
            ));
        }

        let person_verified = self
            .chain_client
            .verify_personhood(&request.creator_public_key, &request.creator_signature)
            .await?;

        if !person_verified {
            return Err(PollServiceError::AuthorizationFailed(
                "Person not verified".to_string(),
            ));
        }

        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| PollServiceError::SystemTimeError)?
            .as_secs() as i64;

        let poll = Poll::new(
            0,
            request.creator_public_key,
            request.poll_data,
            String::new(),
            current_time,
        );

        let poll_id = self.repository.create_poll(poll.clone())?;

        let context_bytes = generate_poll_context(poll_id);
        let context_string = context_to_string(&context_bytes);

        let mut updated_poll = poll;
        updated_poll.id = poll_id;
        updated_poll.context = context_string.clone();

        self.repository.update_poll(updated_poll)?;

        Ok(CreatePollResponse {
            poll_id,
            context: context_string,
        })
    }

    pub async fn get_poll(&self, poll_id: u32) -> Result<Option<Poll>> {
        Ok(self.repository.get_poll_by_id(poll_id)?)
    }

    pub async fn get_polls_by_creator(&self, creator_key: &str) -> Result<Vec<Poll>> {
        Ok(self.repository.get_polls_by_creator(creator_key)?)
    }

    pub async fn get_all_polls(&self) -> Result<Vec<Poll>> {
        Ok(self.repository.get_all_polls()?)
    }

    fn validate_poll_data(&self, poll_data: &PollData) -> Result<()> {
        if poll_data.name.trim().is_empty() {
            return Err(PollServiceError::InvalidData(
                "Name cannot be empty".to_string(),
            ));
        }

        if poll_data.name.len() > 200 {
            return Err(PollServiceError::InvalidData("Name too long".to_string()));
        }

        if poll_data.description.len() > 2000 {
            return Err(PollServiceError::InvalidData(
                "Description too long".to_string(),
            ));
        }

        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| PollServiceError::SystemTimeError)?
            .as_secs() as i64;

        if poll_data.ends_at <= current_time {
            return Err(PollServiceError::InvalidData(
                "End time must be in the future".to_string(),
            ));
        }

        // Max poll duration: 30 days
        if poll_data.ends_at - current_time > 30 * 24 * 60 * 60 {
            return Err(PollServiceError::InvalidData(
                "Poll duration cannot exceed 30 days".to_string(),
            ));
        }

        Ok(())
    }

    fn create_signature_message(&self, request: &CreatePollRequest) -> Result<Vec<u8>> {
        let message_data = serde_json::to_string(&request.poll_data).map_err(|e| {
            PollServiceError::InvalidData(format!("JSON serialization failed: {}", e))
        })?;
        Ok(message_data.as_bytes().to_vec())
    }
}
