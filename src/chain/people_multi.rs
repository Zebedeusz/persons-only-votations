use crate::chain::runtime::{IndividualityConfig, PersonRecord, PersonalId, RingRecord};
use async_trait::async_trait;
use sp_core::sr25519;
use subxt::OnlineClient;
use thiserror::Error;
use verifiable::{GenerateVerifiable, ring_vrf_impl::{BandersnatchVrfVerifiable, EncodedPublicKey}};

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
    async fn verify_personhood(&self, member_hex: &str, signature: &str) -> Result<bool>;
    async fn validate_signature(
        &self,
        member_hex: &str,
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
                }
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
                Err(e) => Err(ChainError::ConnectionFailed(format!(
                    "Health check failed: {e}"
                ))),
            },
            None => Err(ChainError::ConnectionFailed(
                "No client available".to_string(),
            )),
        }
    }

    async fn get_personal_id_by_key(&self, member_hex: &str) -> Result<Option<PersonalId>> {
        // Mock client fallback for testing
        if self.endpoint == "mock" {
            return self.get_personal_id_by_key_mock(member_hex).await;
        }

        // Parse the hex member key
        let member_bytes = hex::decode(member_hex)
            .map_err(|e| ChainError::InvalidPublicKey(format!("Invalid hex format: {e}")))?;

        if member_bytes.len() != 32 {
            return Err(ChainError::InvalidPublicKey(
                "Member key must be 32 bytes".to_string(),
            ));
        }

        let client = self
            .client
            .as_ref()
            .ok_or_else(|| ChainError::ConnectionFailed("No client available".to_string()))?;

        // Create EncodedPublicKey from the member bytes
        use crate::chain::subxt_generated_people_apis::api::runtime_types::verifiable::ring_vrf_impl::EncodedPublicKey as ChainEncodedPublicKey;
        let mut key_array = [0u8; 32];
        key_array.copy_from_slice(&member_bytes);
        let encoded_key = ChainEncodedPublicKey(key_array);

        let storage_query = crate::chain::subxt_generated_people_apis::api::storage()
            .people()
            .keys(&encoded_key);

        match client.storage().at_latest().await {
            Ok(storage) => match storage.fetch(&storage_query).await {
                Ok(personal_id_opt) => Ok(personal_id_opt),
                Err(e) => Err(ChainError::QueryFailed(format!(
                    "Failed to query Keys: {e}"
                ))),
            },
            Err(e) => Err(ChainError::QueryFailed(format!(
                "Failed to get storage: {e}"
            ))),
        }
    }

    async fn get_person_record(
        &self,
        personal_id: PersonalId,
    ) -> Result<Option<PersonRecord<::subxt::ext::subxt_core::utils::AccountId32>>> {
        // Mock client fallback for testing
        if self.endpoint == "mock" {
            return self.get_person_record_mock(personal_id).await;
        }

        let client = self
            .client
            .as_ref()
            .ok_or_else(|| ChainError::ConnectionFailed("No client available".to_string()))?;

        // Query People storage map
        let storage_query = crate::chain::subxt_generated_people_apis::api::storage()
            .people()
            .people(personal_id);

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
                            }
                            None => Ok(None),
                        }
                    }
                    Err(e) => Err(ChainError::QueryFailed(format!(
                        "Failed to query People: {e}"
                    ))),
                }
            }
            Err(e) => Err(ChainError::QueryFailed(format!(
                "Failed to get storage: {e}"
            ))),
        }
    }

    async fn get_ring_record(&self, ring_index: u16) -> Result<Option<RingRecord>> {
        tracing::debug!("Getting ring record for ring index: {}", ring_index);
        
        // Mock client fallback for testing
        if self.endpoint == "mock" {
            return self.get_ring_record_mock(ring_index).await;
        }

        let client = self
            .client
            .as_ref()
            .ok_or_else(|| ChainError::ConnectionFailed("No client available".to_string()))?;

        // Query the Root storage map for the specific ring
        let storage_query = crate::chain::subxt_generated_people_apis::api::storage()
            .people()
            .root(ring_index as u32);

        match client.storage().at_latest().await {
            Ok(storage) => {
                match storage.fetch(&storage_query).await {
                    Ok(ring_root_opt) => {
                        match ring_root_opt {
                            Some(ring_root) => {
                                tracing::debug!("Found ring root for ring {}: revision {}", ring_index, ring_root.revision);
                                // The chain stores a 384-byte ring root commitment, but our RingRecord expects sr25519::Public (32 bytes)
                                // For now, we'll create a mock sr25519::Public since we just need to verify the ring exists
                                let mock_root = sr25519::Public::from_raw([1u8; 32]);
                                let ring_record = RingRecord {
                                    root: mock_root, 
                                    revision: ring_root.revision,
                                };
                                Ok(Some(ring_record))
                            }
                            None => {
                                tracing::debug!("No ring root found for ring {}", ring_index);
                                Ok(None)
                            }
                        }
                    }
                    Err(e) => {
                        tracing::error!("Failed to query ring root for ring {}: {}", ring_index, e);
                        Err(ChainError::QueryFailed(format!(
                            "Failed to query Root for ring {ring_index}: {e}"
                        )))
                    }
                }
            }
            Err(e) => Err(ChainError::QueryFailed(format!(
                "Failed to get storage: {e}"
            ))),
        }
    }
}

#[async_trait]
impl ChainClient for PeopleMultiClient {
    async fn verify_personhood(&self, member_hex: &str, _signature: &str) -> Result<bool> {
        
        // Mock client fallback for testing
        if self.endpoint == "mock" {
            tracing::debug!("Using mock client for personhood verification");
            return self.verify_personhood_mock(member_hex).await;
        }

        tracing::debug!("Using real chain client for personhood verification");

        // 1. Get PersonalId from member key
        let personal_id = match self.get_personal_id_by_key(member_hex).await? {
            Some(id) => {
                id
            },
            None => {
                return Ok(false); // Key not found in people pallet
            }
        };

        // 2. Get person record
        let person_record = match self.get_person_record(personal_id).await? {
            Some(record) => {
                record
            },
            None => {
                return Ok(false); // Person record not found
            }
        };

        // 3. Check if person is included in a ring (not just onboarding)
        let ring_index = match person_record.position.ring_index() {
            Some(index) => {
                index
            },
            None => {
                return Ok(false); // Person is still onboarding
            }
        };

        // 4. Verify the ring is active
        let _ring_record = match self.get_ring_record(ring_index).await? {
            Some(record) => {
                record
            },
            None => {
                return Ok(false); // Ring not found or inactive
            }
        };

        // If we get here, the person is verified and in an active ring
        Ok(true)
    }

    async fn validate_signature(
        &self,
        member_hex: &str,
        signature: &str,
        message: &[u8],
    ) -> Result<bool> {
        // Mock client fallback for testing
        if self.endpoint == "mock" {
            return self
                .validate_signature_mock(member_hex, signature, message)
                .await;
        }

        // Parse the member key
        let member_bytes = hex::decode(member_hex)
            .map_err(|e| ChainError::InvalidSignature(format!("Invalid member key hex: {e}")))?;

        if member_bytes.len() != 32 {
            return Err(ChainError::InvalidSignature(
                "Member key must be 32 bytes".to_string(),
            ));
        }

        // Create EncodedPublicKey from member bytes
        let mut key_array = [0u8; 32];
        key_array.copy_from_slice(&member_bytes);
        let encoded_member = EncodedPublicKey(key_array);

        // Parse the signature
        let signature_bytes = hex::decode(signature)
            .map_err(|e| ChainError::InvalidSignature(format!("Invalid signature hex: {e}")))?;

        // Verifiable crypto signatures are 96 bytes (PLAIN_VRF_SIGNATURE_SIZE)
        if signature_bytes.len() != 96 {
            return Err(ChainError::InvalidSignature(
                "Verifiable signature must be 96 bytes".to_string(),
            ));
        }

        let mut signature_array = [0u8; 96];
        signature_array.copy_from_slice(&signature_bytes);

        // Verify the signature using verifiable crypto
        let is_valid = BandersnatchVrfVerifiable::verify_signature(&signature_array, message, &encoded_member);

        Ok(is_valid)
    }
}

impl PeopleMultiClient {
    async fn get_personal_id_by_key_mock(
        &self,
        member_hex: &str,
    ) -> Result<Option<PersonalId>> {
        // Parse the hex member key
        let member_bytes = hex::decode(member_hex)
            .map_err(|e| ChainError::InvalidPublicKey(format!("Invalid hex format: {e}")))?;

        if member_bytes.len() != 32 {
            return Err(ChainError::InvalidPublicKey(
                "Member key must be 32 bytes".to_string(),
            ));
        }

        // Mock implementation - return a PersonalId based on the first 8 bytes of the key
        let personal_id = u64::from_le_bytes([
            member_bytes[0],
            member_bytes[1],
            member_bytes[2],
            member_bytes[3],
            member_bytes[4],
            member_bytes[5],
            member_bytes[6],
            member_bytes[7],
        ]);
        Ok(Some(personal_id))
    }

    async fn get_person_record_mock(
        &self,
        personal_id: PersonalId,
    ) -> Result<Option<PersonRecord<::subxt::ext::subxt_core::utils::AccountId32>>> {
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

    async fn verify_personhood_mock(&self, member_hex: &str) -> Result<bool> {
        if member_hex.is_empty() {
            return Ok(false);
        }

        // Simulate chain call delay
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        // For prototype: accept keys that look like hex (even length, valid hex chars)
        if member_hex.len() != 64 { // 32 bytes * 2 hex chars per byte
            return Ok(false);
        }

        match hex::decode(member_hex) {
            Ok(bytes) if bytes.len() == 32 => {
                // For mock mode, accept any properly formatted 32-byte hex key
                // In production, this would validate against the actual chain state
                Ok(true)
            }
            _ => Ok(false),
        }
    }

    async fn validate_signature_mock(
        &self,
        member_hex: &str,
        signature: &str,
        message: &[u8],
    ) -> Result<bool> {
        // For prototype, basic validation
        if member_hex.is_empty() || signature.is_empty() || message.is_empty() {
            return Ok(false);
        }

        // Simulate signature validation delay
        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

        // Parse and validate member key
        let member_bytes = match hex::decode(member_hex) {
            Ok(bytes) if bytes.len() == 32 => bytes,
            _ => return Ok(false),
        };

        // Parse and validate signature
        let signature_bytes = match hex::decode(signature) {
            Ok(bytes) if bytes.len() == 96 => bytes, // Verifiable signature size
            _ => return Ok(false),
        };

        // Create verifiable types
        let mut key_array = [0u8; 32];
        key_array.copy_from_slice(&member_bytes);
        let encoded_key = EncodedPublicKey(key_array);

        let mut sig_array = [0u8; 96];
        sig_array.copy_from_slice(&signature_bytes);

        // Validate member key format
        if !BandersnatchVrfVerifiable::is_member_valid(&encoded_key) {
            return Ok(false);
        }

        // For mock, we can't do real signature verification without the secret,
        // so just return true for well-formed inputs
        Ok(true)
    }
}

impl Default for PeopleMultiClient {
    fn default() -> Self {
        Self::new_mock()
    }
}
