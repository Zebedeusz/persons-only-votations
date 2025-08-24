use async_trait::async_trait;
use thiserror::Error;
use subxt::OnlineClient;
use sp_core::{sr25519, Pair};
use crate::chain::runtime::{
    IndividualityConfig, PersonalId, PersonRecord, RingRecord
};

#[derive(Error, Debug, Clone)]
pub enum ChainError {
    #[error("Connection to chain failed: {0}")]
    ConnectionFailed(String),
    #[error("Invalid signature: {0}")]
    InvalidSignature(String),
    #[error("Person not found or not verified")]
    PersonNotVerified,
    #[error("Chain query failed: {0}")]
    QueryFailed(String),
    #[error("Invalid public key format: {0}")]
    InvalidPublicKey(String),
    #[error("Storage decoding failed: {0}")]
    DecodingFailed(String),
    #[error("Network timeout: {0}")]
    Timeout(String),
    #[error("Metadata error: {0}")]
    MetadataError(String),
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

#[derive(Debug)]
pub struct PeopleMultiClient {
    client: Option<OnlineClient<IndividualityConfig>>,
    endpoint: String,
}

impl PeopleMultiClient {
    pub async fn new(endpoint: &str) -> Result<Self> {
        Self::new_with_retry(endpoint, 3).await
    }
    
    pub async fn new_with_retry(endpoint: &str, max_retries: u32) -> Result<Self> {
        let mut last_error = None;
        
        for attempt in 0..=max_retries {
            match OnlineClient::<IndividualityConfig>::from_url(endpoint).await {
                Ok(client) => {
                    return Ok(Self {
                        client: Some(client),
                        endpoint: endpoint.to_string(),
                    });
                },
                Err(e) => {
                    last_error = Some(e);
                    if attempt < max_retries {
                        let delay = 2_u64.pow(attempt) * 1000; // Exponential backoff
                        tokio::time::sleep(tokio::time::Duration::from_millis(delay)).await;
                    }
                }
            }
        }
        
        Err(ChainError::ConnectionFailed(format!(
            "Failed to connect to {} after {} attempts: {}", 
            endpoint, 
            max_retries + 1,
            last_error.unwrap()
        )))
    }
    
    pub fn new_mock() -> Self {
        // The client field won't be used for mock calls
        Self {
            client: None, // No real client for mock
            endpoint: "mock".to_string(),
        }
    }
    
    pub async fn health_check(&self) -> Result<bool> {
        if self.endpoint == "mock" {
            return Ok(true);
        }
        
        match &self.client {
            Some(client) => match client.blocks().at_latest().await {
                Ok(_) => Ok(true),
                Err(e) => Err(ChainError::ConnectionFailed(format!("Health check failed: {}", e))),
            },
            None => Err(ChainError::ConnectionFailed("No client available".to_string())),
        }
    }
    
    async fn get_personal_id_by_key(&self, public_key_hex: &str) -> Result<Option<PersonalId>> {
        // Mock client fallback for testing
        if self.endpoint == "mock" {
            return self.get_personal_id_by_key_mock(public_key_hex).await;
        }

        // Parse the hex public key
        let public_key_bytes = hex::decode(public_key_hex)
            .map_err(|e| ChainError::InvalidPublicKey(format!("Invalid hex format: {}", e)))?;
            
        if public_key_bytes.len() != 32 {
            return Err(ChainError::InvalidPublicKey("Public key must be 32 bytes".to_string()));
        }
        
        let public_key = sr25519::Public::from_raw(
            public_key_bytes.try_into()
                .map_err(|_| ChainError::InvalidPublicKey("Failed to convert to sr25519 public key".to_string()))?
        );

        let client = self.client.as_ref()
            .ok_or_else(|| ChainError::ConnectionFailed("No client available".to_string()))?;

        // Convert sr25519::Public to EncodedPublicKey expected by the storage
        // The EncodedPublicKey should be the raw bytes of the sr25519 public key
        use crate::chain::subxt_generated_people_apis::api::runtime_types::verifiable::ring_vrf_impl::EncodedPublicKey;
        let encoded_key = EncodedPublicKey(public_key.0);
        
        let storage_query = crate::chain::subxt_generated_people_apis::api::storage().people().keys(&encoded_key);
        
        match client.storage().at_latest().await {
            Ok(storage) => {
                match storage.fetch(&storage_query).await {
                    Ok(personal_id_opt) => Ok(personal_id_opt),
                    Err(e) => Err(ChainError::QueryFailed(format!("Failed to query Keys: {}", e))),
                }
            },
            Err(e) => Err(ChainError::QueryFailed(format!("Failed to get storage: {}", e))),
        }
    }
    
    async fn get_person_record(&self, personal_id: PersonalId) -> Result<Option<PersonRecord<::subxt::ext::subxt_core::utils::AccountId32>>> {
        // Mock client fallback for testing
        if self.endpoint == "mock" {
            return self.get_person_record_mock(personal_id).await;
        }

        let client = self.client.as_ref()
            .ok_or_else(|| ChainError::ConnectionFailed("No client available".to_string()))?;

        // Query People storage map
        let storage_query = crate::chain::subxt_generated_people_apis::api::storage().people().people(&personal_id);
        
        match client.storage().at_latest().await {
            Ok(storage) => {
                match storage.fetch(&storage_query).await {
                    Ok(person_record_opt) => {
                        match person_record_opt {
                            Some(generated_record) => {
                                // Convert from generated types to our types
                                let person_record = PersonRecord {
                                    key: sr25519::Public::from_raw({
                                        let mut key_bytes = [0u8; 32];
                                        if generated_record.key.0.len() >= 32 {
                                            key_bytes.copy_from_slice(&generated_record.key.0[0..32]);
                                        }
                                        key_bytes
                                    }),
                                    account: generated_record.account,
                                    position: match generated_record.position {
                                        crate::chain::subxt_generated_people_apis::api::runtime_types::pallet_people_multi::types::RingPosition::Onboarding { .. } => 
                                            crate::chain::runtime::PersonPosition::Onboarding,
                                        crate::chain::subxt_generated_people_apis::api::runtime_types::pallet_people_multi::types::RingPosition::Included { ring_index, .. } => 
                                            crate::chain::runtime::PersonPosition::Included(ring_index as u16),
                                        _ => crate::chain::runtime::PersonPosition::Onboarding, // Handle other variants
                                    },
                                };
                                Ok(Some(person_record))
                            },
                            None => Ok(None),
                        }
                    },
                    Err(e) => Err(ChainError::QueryFailed(format!("Failed to query People: {}", e))),
                }
            },
            Err(e) => Err(ChainError::QueryFailed(format!("Failed to get storage: {}", e))),
        }
    }
    
    async fn get_ring_record(&self, ring_index: u16) -> Result<Option<RingRecord>> {
        // Mock client fallback for testing
        if self.endpoint == "mock" {
            return self.get_ring_record_mock(ring_index).await;
        }

        let client = self.client.as_ref()
            .ok_or_else(|| ChainError::ConnectionFailed("No client available".to_string()))?;

        // Query RingsState to check if ring exists
        let storage_query = crate::chain::subxt_generated_people_apis::api::storage().people().rings_state();
        
        match client.storage().at_latest().await {
            Ok(storage) => {
                match storage.fetch(&storage_query).await {
                    Ok(rings_state_opt) => {
                        match rings_state_opt {
                            Some(_rings_state) => {
                                let ring_record = RingRecord {
                                    root: sr25519::Public::from_raw([1u8; 32]), // Mock root - would need to derive from actual ring data
                                    revision: ring_index as u32,
                                };
                                Ok(Some(ring_record))
                            },
                            None => Ok(None),
                        }
                    },
                    Err(e) => Err(ChainError::QueryFailed(format!("Failed to query RingsState: {}", e))),
                }
            },
            Err(e) => Err(ChainError::QueryFailed(format!("Failed to get storage: {}", e))),
        }
    }
}

#[async_trait]
impl ChainClient for PeopleMultiClient {
    async fn verify_personhood(&self, public_key: &str, _signature: &str) -> Result<bool> {
        // Mock client fallback for testing
        if self.endpoint == "mock" {
            return self.verify_personhood_mock(public_key).await;
        }
        
        // 1. Get PersonalId from public key
        let personal_id = match self.get_personal_id_by_key(public_key).await? {
            Some(id) => id,
            None => return Ok(false), // Key not found in people pallet
        };
        
        // 2. Get person record
        let person_record = match self.get_person_record(personal_id).await? {
            Some(record) => record,
            None => return Ok(false), // Person record not found
        };
        
        // 3. Check if person is included in a ring (not just onboarding)
        let ring_index = match person_record.position.ring_index() {
            Some(index) => index,
            None => return Ok(false), // Person is still onboarding
        };
        
        // 4. Verify the ring is active
        let _ring_record = match self.get_ring_record(ring_index).await? {
            Some(record) => record,
            None => return Ok(false), // Ring not found or inactive
        };
        
        // If we get here, the person is verified and in an active ring
        Ok(true)
    }
    

    async fn validate_signature(
        &self,
        public_key: &str,
        signature: &str,
        message: &[u8],
    ) -> Result<bool> {
        // Mock client fallback for testing
        if self.endpoint == "mock" {
            return self.validate_signature_mock(public_key, signature, message).await;
        }
        
        // Parse the public key
        let public_key_bytes = hex::decode(public_key)
            .map_err(|e| ChainError::InvalidSignature(format!("Invalid public key hex: {}", e)))?;
            
        if public_key_bytes.len() != 32 {
            return Err(ChainError::InvalidSignature("Public key must be 32 bytes".to_string()));
        }
        
        let public_key = sr25519::Public::from_raw(
            public_key_bytes.try_into()
                .map_err(|_| ChainError::InvalidSignature("Failed to convert to sr25519 public key".to_string()))?
        );
        
        // Parse the signature
        let signature_bytes = hex::decode(signature)
            .map_err(|e| ChainError::InvalidSignature(format!("Invalid signature hex: {}", e)))?;
            
        if signature_bytes.len() != 64 {
            return Err(ChainError::InvalidSignature("Signature must be 64 bytes".to_string()));
        }
        
        let signature = sr25519::Signature::from_raw(
            signature_bytes.try_into()
                .map_err(|_| ChainError::InvalidSignature("Failed to convert to sr25519 signature".to_string()))?
        );
        
        // Verify the signature using sr25519
        let is_valid = sr25519::Pair::verify(&signature, message, &public_key);
        
        Ok(is_valid)
    }
}

impl PeopleMultiClient {
    async fn get_personal_id_by_key_mock(&self, public_key_hex: &str) -> Result<Option<PersonalId>> {
        // Parse the hex public key
        let public_key_bytes = hex::decode(public_key_hex)
            .map_err(|e| ChainError::InvalidPublicKey(format!("Invalid hex format: {}", e)))?;
            
        if public_key_bytes.len() != 32 {
            return Err(ChainError::InvalidPublicKey("Public key must be 32 bytes".to_string()));
        }
        
        // Mock implementation - return a PersonalId based on the first 8 bytes of the key
        let personal_id = u64::from_le_bytes([
            public_key_bytes[0], public_key_bytes[1], public_key_bytes[2], public_key_bytes[3],
            public_key_bytes[4], public_key_bytes[5], public_key_bytes[6], public_key_bytes[7],
        ]);
        Ok(Some(personal_id))
    }

    async fn get_person_record_mock(&self, personal_id: PersonalId) -> Result<Option<PersonRecord<::subxt::ext::subxt_core::utils::AccountId32>>> {
        // If personal_id is non-zero, create a mock person record
        if personal_id > 0 {
            let mock_key = sr25519::Public::from_raw([0u8; 32]);
            let person_record = PersonRecord {
                key: mock_key,
                account: None,
                position: crate::chain::runtime::PersonPosition::Included(0), // Ring 0
            };
            Ok(Some(person_record))
        } else {
            Ok(None)
        }
    }

    async fn get_ring_record_mock(&self, ring_index: u16) -> Result<Option<RingRecord>> {
        // Create a mock ring record
        let mock_root = sr25519::Public::from_raw([1u8; 32]);
        let ring_record = RingRecord {
            root: mock_root,
            revision: ring_index as u32,
        };
        Ok(Some(ring_record))
    }

    async fn verify_personhood_mock(&self, public_key: &str) -> Result<bool> {
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
    
    async fn validate_signature_mock(
        &self,
        public_key: &str,
        signature: &str,
        message: &[u8],
    ) -> Result<bool> {
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
        Self::new_mock()
    }
}