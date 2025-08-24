use async_trait::async_trait;
use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum ChainError {
    #[error("Connection to chain failed: {0}")]
    ConnectionFailed(String),
    #[error("Invalid signature")]
    InvalidSignature,
    #[error("Person not found or not verified")]
    PersonNotVerified,
    #[error("Chain query failed: {0}")]
    QueryFailed(String),
}

pub type Result<T> = std::result::Result<T, ChainError>;

#[async_trait]
pub trait ChainClient: Send + Sync {
    async fn verify_personhood(&self, public_key: &str, signature: &str) -> Result<bool>;
    async fn validate_signature(
        &self,
        public_key: &str,
        signature: &str,
        message: &[u8],
    ) -> Result<bool>;
}

pub struct PeopleMultiClient {
    // TODO: Add subxt client when we have actual chain connection
}

impl PeopleMultiClient {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl ChainClient for PeopleMultiClient {
    async fn verify_personhood(&self, public_key: &str, _signature: &str) -> Result<bool> {
        // TODO: Implement actual chain verification
        // For prototype, accept any non-empty public key
        if public_key.is_empty() {
            return Ok(false);
        }
        
        // Simulate chain call delay
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        
        // For prototype: accept keys that look like hex (even length, valid hex chars)
        if public_key.len() % 2 != 0 {
            return Ok(false);
        }
        
        match hex::decode(public_key) {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    async fn validate_signature(
        &self,
        public_key: &str,
        signature: &str,
        message: &[u8],
    ) -> Result<bool> {
        // TODO: Implement actual signature validation with subxt
        // For prototype, basic validation
        if public_key.is_empty() || signature.is_empty() || message.is_empty() {
            return Ok(false);
        }
        
        // Simulate signature validation delay
        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
        
        // For prototype: accept if both key and signature are valid hex
        let key_valid = hex::decode(public_key).is_ok();
        let sig_valid = hex::decode(signature).is_ok();
        
        Ok(key_valid && sig_valid)
    }
}

impl Default for PeopleMultiClient {
    fn default() -> Self {
        Self::new()
    }
}