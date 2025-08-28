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
            PollServiceError::ChainError(e) => {
                ApiError::InternalError(format!("Chain error: {}", e))
            }
            PollServiceError::RepositoryError(e) => {
                ApiError::InternalError(format!("Storage error: {}", e))
            }
            PollServiceError::SystemTimeError => {
                ApiError::InternalError("System time error".to_string())
            }
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
        tracing::debug!("Poll creation request received: {}", json_body);
        
        // Parse request
        let request: CreatePollRequest = serde_json::from_str(json_body)
            .map_err(|e| ApiError::BadRequest(format!("Invalid JSON: {}", e)))?;

        tracing::debug!("Parsed request - Member: {}", request.creator_member_hex);

        // Call service
        let response = self.poll_service.create_poll(request).await?;

        // Return response as JSON
        let json_response = serde_json::to_string(&response)
            .map_err(|e| ApiError::InternalError(format!("JSON serialization failed: {}", e)))?;

        Ok(json_response)
    }

    pub async fn get_poll(&self, poll_id: u32) -> Result<String, ApiError> {
        let poll = self.poll_service.get_poll(poll_id).await?;

        match poll {
            Some(poll) => {
                let json_response = serde_json::to_string(&poll).map_err(|e| {
                    ApiError::InternalError(format!("JSON serialization failed: {}", e))
                })?;
                Ok(json_response)
            }
            None => Err(ApiError::NotFound),
        }
    }

    pub async fn get_polls_by_creator(&self, creator_key: &str) -> Result<String, ApiError> {
        let polls = self.poll_service.get_polls_by_creator(creator_key).await?;

        let json_response = serde_json::to_string(&polls)
            .map_err(|e| ApiError::InternalError(format!("JSON serialization failed: {}", e)))?;

        Ok(json_response)
    }

    pub async fn get_all_polls(&self) -> Result<String, ApiError> {
        let polls = self.poll_service.get_all_polls().await?;

        let json_response = serde_json::to_string(&polls)
            .map_err(|e| ApiError::InternalError(format!("JSON serialization failed: {}", e)))?;

        Ok(json_response)
    }
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
    use crate::chain::people_multi::PeopleMultiClient;
    use crate::storage::repositories::poll_repository::PollRepository;

    #[tokio::test]
    async fn test_create_poll_invalid_json() {
        let repo = Arc::new(PollRepository::new());
        let chain = Arc::new(PeopleMultiClient::new_mock());
        let service = Arc::new(PollService::new(repo, chain));
        let handler = PollHandler::new(service);

        let result = handler.create_poll("invalid json").await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ApiError::BadRequest(_)));
    }
}
