use crate::chain::people_multi::{ChainClient, ChainError};
use crate::crypto::contexts::generate_poll_context;
use crate::storage::models::poll::{CreatePollRequest, Poll, PollData};
use crate::storage::repositories::poll_repository::{PollRepository, PollRepositoryError};
use crate::utils::get_current_timestamp;
use std::sync::Arc;
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
}

pub type Result<T> = std::result::Result<T, PollServiceError>;

#[derive(Debug, Clone, serde::Serialize)]
pub struct CreatePollResponse {
    pub poll_id: u32,
    pub context: String,
}

/// Message a creator signs: the poll data as JSON.
pub fn poll_message(poll_data: &PollData) -> Vec<u8> {
    serde_json::to_vec(poll_data).expect("PollData always serializes")
}

pub struct PollService {
    repository: Arc<PollRepository>,
    chain_client: Arc<dyn ChainClient>,
}

impl PollService {
    pub fn new(repository: Arc<PollRepository>, chain_client: Arc<dyn ChainClient>) -> Self {
        Self {
            repository,
            chain_client,
        }
    }

    pub async fn create_poll(&self, request: CreatePollRequest) -> Result<CreatePollResponse> {
        validate_poll_data(&request.poll_data)?;

        let signature_valid = self
            .chain_client
            .validate_signature(
                &request.creator_member_hex,
                &request.creator_signature,
                &poll_message(&request.poll_data),
            )
            .await?;
        if !signature_valid {
            tracing::error!(
                "Signature validation failed for member: {}",
                request.creator_member_hex
            );
            return Err(PollServiceError::AuthorizationFailed(
                "Invalid signature".to_string(),
            ));
        }

        // A person counts as verified once included in a ring that has a root
        let ring_index = self
            .chain_client
            .ring_index(&request.creator_member_hex)
            .await?;
        let verified = match ring_index {
            Some(index) => self.chain_client.ring_root(index).await?.is_some(),
            None => false,
        };
        if !verified {
            tracing::error!(
                "Personhood verification failed for member: {}",
                request.creator_member_hex
            );
            return Err(PollServiceError::AuthorizationFailed(
                "Person not verified".to_string(),
            ));
        }

        let mut poll = Poll::new(
            0,
            request.creator_member_hex,
            request.poll_data,
            String::new(),
            get_current_timestamp(),
        );
        // The context depends on the id, which the repository assigns
        poll.id = self.repository.create_poll(poll.clone())?;
        poll.context = hex::encode(generate_poll_context(poll.id));
        self.repository.update_poll(poll.clone())?;

        Ok(CreatePollResponse {
            poll_id: poll.id,
            context: poll.context,
        })
    }

    pub async fn get_poll(&self, poll_id: u32) -> Result<Option<Poll>> {
        Ok(self.repository.get_poll_by_id(poll_id)?)
    }

    pub async fn get_all_polls(&self) -> Result<Vec<Poll>> {
        Ok(self.repository.get_all_polls()?)
    }
}

fn validate_poll_data(poll_data: &PollData) -> Result<()> {
    const MAX_NAME_LENGTH: usize = 200;
    const MAX_DESCRIPTION_LENGTH: usize = 2000;
    const MAX_DURATION_SECONDS: i64 = 30 * 24 * 60 * 60;

    let invalid = |msg: &str| Err(PollServiceError::InvalidData(msg.to_string()));
    if poll_data.name.trim().is_empty() {
        return invalid("Name cannot be empty");
    }
    if poll_data.name.len() > MAX_NAME_LENGTH {
        return invalid("Name too long");
    }
    if poll_data.description.len() > MAX_DESCRIPTION_LENGTH {
        return invalid("Description too long");
    }
    let duration = poll_data.ends_at - get_current_timestamp();
    if duration <= 0 {
        return invalid("End time must be in the future");
    }
    if duration > MAX_DURATION_SECONDS {
        return invalid("Poll duration cannot exceed 30 days");
    }
    Ok(())
}
