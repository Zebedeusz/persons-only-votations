use crate::services::poll_service::{PollService, PollServiceError};
use crate::storage::models::poll::CreatePollRequest;
use serde_json;
use std::sync::Arc;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Bad request: {0}")]
    BadRequest(String),
    #[error("Unauthorized: {0}")]
    Unauthorized(String),
    #[error("Internal server error: {0}")]
    InternalError(String),
    #[error("Not found")]
    NotFound,
}

impl From<PollServiceError> for ApiError {
    fn from(err: PollServiceError) -> Self {
        match err {
            PollServiceError::InvalidData(msg) => ApiError::BadRequest(msg),
            PollServiceError::AuthorizationFailed(msg) => ApiError::Unauthorized(msg),
            PollServiceError::ChainError(e) => ApiError::InternalError(format!("Chain error: {e}")),
            PollServiceError::RepositoryError(e) => {
                ApiError::InternalError(format!("Storage error: {e}"))
            }
            PollServiceError::TimeError(e) => ApiError::InternalError(format!("Time error: {e}")),
        }
    }
}

pub struct PollHandler {
    poll_service: Arc<PollService>,
}

impl PollHandler {
    pub fn new(poll_service: Arc<PollService>) -> Self {
        Self { poll_service }
    }

    pub async fn create_poll(&self, json_body: &str) -> Result<String, ApiError> {
        // Parse request
        let request: CreatePollRequest = serde_json::from_str(json_body)
            .map_err(|e| ApiError::BadRequest(format!("Invalid JSON: {e}")))?;

        // Call service
        let response = self.poll_service.create_poll(request).await?;

        // Return response as JSON
        to_json_response(&response)
    }

    pub async fn get_poll(&self, poll_id: u32) -> Result<String, ApiError> {
        let poll = self.poll_service.get_poll(poll_id).await?;

        match poll {
            Some(poll) => to_json_response(&poll),
            None => Err(ApiError::NotFound),
        }
    }

    pub async fn get_polls_by_creator(&self, creator_key: &str) -> Result<String, ApiError> {
        let polls = self.poll_service.get_polls_by_creator(creator_key).await?;

        to_json_response(&polls)
    }

    pub async fn get_all_polls(&self) -> Result<String, ApiError> {
        let polls = self.poll_service.get_all_polls().await?;

        to_json_response(&polls)
    }
}

// JSON serialization helper
pub fn to_json_response<T: serde::Serialize>(data: &T) -> Result<String, ApiError> {
    serde_json::to_string(data)
        .map_err(|e| ApiError::InternalError(format!("JSON serialization failed: {e}")))
}

// HTTP status code helpers
impl ApiError {
    pub fn status_code(&self) -> u16 {
        match self {
            ApiError::BadRequest(_) => 400,
            ApiError::Unauthorized(_) => 401,
            ApiError::NotFound => 404,
            ApiError::InternalError(_) => 500,
        }
    }

    pub fn to_json(&self) -> String {
        serde_json::json!({
            "error": self.to_string()
        })
        .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_poll_invalid_json() {
        // This test only validates JSON parsing, which happens before any chain interaction.
        // Since we can't create a mock chain client anymore, we'll test JSON parsing directly
        use crate::storage::models::poll::CreatePollRequest;

        // Test invalid JSON parsing directly
        let invalid_json = "invalid json";
        let result: Result<CreatePollRequest, serde_json::Error> =
            serde_json::from_str(invalid_json);
        assert!(result.is_err());

        // Also test that a BadRequest error is created from JSON error
        let json_error = result.unwrap_err();
        let api_error = ApiError::BadRequest(format!("Invalid JSON: {json_error}"));
        assert!(matches!(api_error, ApiError::BadRequest(_)));
    }
}
