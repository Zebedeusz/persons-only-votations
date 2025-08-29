use crate::chain::runtime::{IndividualityConfig, PersonRecord, PersonalId};
use async_trait::async_trait;
use sp_core::sr25519;
use subxt::OnlineClient;
use thiserror::Error;
use verifiable::{
    GenerateVerifiable,
    ring_vrf_impl::{BandersnatchVrfVerifiable, EncodedPublicKey},
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
    async fn verify_personhood(&self, member_hex: &str, signature: &str) -> Result<bool>;
    async fn validate_signature(
        &self,
        member_hex: &str,
        signature: &str,
        message: &[u8],
    ) -> Result<bool>;
    async fn get_ring_root(
        &self,
        ring_index: u16,
    ) -> Result<Option<crate::chain::runtime::RingRoot>>;
    async fn get_person_ring_index(&self, member_hex: &str) -> Result<Option<u16>>;
}

#[derive(Debug)]
pub struct PeopleMultiClient {
    client: OnlineClient<IndividualityConfig>,
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
                    return Ok(Self { client });
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

    pub async fn health_check(&self) -> Result<bool> {
        match self.client.blocks().at_latest().await {
            Ok(_) => Ok(true),
            Err(e) => Err(ChainError::ConnectionFailed(format!(
                "Health check failed: {e}"
            ))),
        }
    }

    async fn get_personal_id_by_key(&self, member_hex: &str) -> Result<Option<PersonalId>> {
        let member_bytes = hex::decode(member_hex)
            .map_err(|e| ChainError::InvalidPublicKey(format!("Invalid hex format: {e}")))?;

        if member_bytes.len() != 32 {
            return Err(ChainError::InvalidPublicKey(
                "Member key must be 32 bytes".to_string(),
            ));
        }

        use crate::chain::subxt_generated_people_apis::api::runtime_types::verifiable::ring_vrf_impl::EncodedPublicKey as ChainEncodedPublicKey;
        let mut key_array = [0u8; 32];
        key_array.copy_from_slice(&member_bytes);
        let encoded_key = ChainEncodedPublicKey(key_array);

        let storage_query = crate::chain::subxt_generated_people_apis::api::storage()
            .people()
            .keys(&encoded_key);

        let storage = self
            .client
            .storage()
            .at_latest()
            .await
            .map_err(|e| ChainError::QueryFailed(format!("Failed to get storage: {e}")))?;
        storage
            .fetch(&storage_query)
            .await
            .map_err(|e| ChainError::QueryFailed(format!("Failed to query Keys: {e}")))
    }

    async fn get_person_record(
        &self,
        personal_id: PersonalId,
    ) -> Result<Option<PersonRecord<::subxt::ext::subxt_core::utils::AccountId32>>> {
        let storage_query = crate::chain::subxt_generated_people_apis::api::storage()
            .people()
            .people(personal_id);

        let storage = self
            .client
            .storage()
            .at_latest()
            .await
            .map_err(|e| ChainError::QueryFailed(format!("Failed to get storage: {e}")))?;

        let generated_record = storage
            .fetch(&storage_query)
            .await
            .map_err(|e| ChainError::QueryFailed(format!("Failed to query People: {e}")))?;

        Ok(generated_record.map(|record| {
            let mut key_bytes = [0u8; 32];
            if record.key.0.len() >= 32 {
                key_bytes.copy_from_slice(&record.key.0[0..32]);
            }
            
            PersonRecord {
                key: sr25519::Public::from_raw(key_bytes),
                account: record.account,
                position: match record.position {
                    crate::chain::subxt_generated_people_apis::api::runtime_types::pallet_people_multi::types::RingPosition::Onboarding { .. } => 
                        crate::chain::runtime::PersonPosition::Onboarding,
                    crate::chain::subxt_generated_people_apis::api::runtime_types::pallet_people_multi::types::RingPosition::Included { ring_index, .. } => 
                        crate::chain::runtime::PersonPosition::Included(ring_index as u16),
                    _ => crate::chain::runtime::PersonPosition::Onboarding,
                },
            }
        }))
    }

    async fn is_ring_active(&self, ring_index: u16) -> Result<bool> {
        let storage_query = crate::chain::subxt_generated_people_apis::api::storage()
            .people()
            .root(ring_index as u32);

        let storage = self
            .client
            .storage()
            .at_latest()
            .await
            .map_err(|e| ChainError::QueryFailed(format!("Failed to get storage: {e}")))?;

        let result = storage
            .fetch(&storage_query)
            .await
            .map_err(|e| ChainError::QueryFailed(format!("Failed to query ring root: {e}")))?;

        Ok(result.is_some())
    }
}

#[async_trait]
impl ChainClient for PeopleMultiClient {
    async fn verify_personhood(&self, member_hex: &str, _signature: &str) -> Result<bool> {
        tracing::debug!("Using real chain client for personhood verification");

        let personal_id = match self.get_personal_id_by_key(member_hex).await? {
            Some(id) => id,
            None => {
                return Ok(false);
            }
        };

        let person_record = match self.get_person_record(personal_id).await? {
            Some(record) => record,
            None => {
                return Ok(false);
            }
        };

        let ring_index = match person_record.position.ring_index() {
            Some(index) => index,
            None => {
                return Ok(false); // Person is still onboarding
            }
        };

        if !self.is_ring_active(ring_index).await? {
            return Ok(false);
        }

        Ok(true)
    }

    async fn get_ring_root(
        &self,
        ring_index: u16,
    ) -> Result<Option<crate::chain::runtime::RingRoot>> {
        let storage_query = crate::chain::subxt_generated_people_apis::api::storage()
            .people()
            .root(ring_index as u32);

        let storage = self
            .client
            .storage()
            .at_latest()
            .await
            .map_err(|e| ChainError::QueryFailed(format!("Failed to get storage: {e}")))?;

        let ring_root = storage
            .fetch(&storage_query)
            .await
            .map_err(|e| ChainError::QueryFailed(format!("Failed to query ring root: {e}")))?;

        match ring_root {
            Some(root) => {
                use scale_codec::Decode;

                let members_commitment =
                    verifiable::ring_vrf_impl::MembersCommitment::decode(&mut &root.root[..])
                        .map_err(|e| {
                            ChainError::DecodingFailed(format!(
                                "Failed to decode members commitment: {e}"
                            ))
                        })?;

                let members_set =
                    verifiable::ring_vrf_impl::MembersSet::decode(&mut &root.intermediate[..])
                        .map_err(|e| {
                            ChainError::DecodingFailed(format!("Failed to decode members set: {e}"))
                        })?;

                Ok(Some(crate::chain::runtime::RingRoot {
                    root: members_commitment,
                    revision: root.revision,
                    intermediate: members_set,
                }))
            }
            None => Ok(None),
        }
    }

    async fn validate_signature(
        &self,
        member_hex: &str,
        signature: &str,
        message: &[u8],
    ) -> Result<bool> {
        let member_bytes = hex::decode(member_hex)
            .map_err(|e| ChainError::InvalidSignature(format!("Invalid member key hex: {e}")))?;

        if member_bytes.len() != 32 {
            return Err(ChainError::InvalidSignature(
                "Member key must be 32 bytes".to_string(),
            ));
        }

        let mut key_array = [0u8; 32];
        key_array.copy_from_slice(&member_bytes);
        let encoded_member = EncodedPublicKey(key_array);

        let signature_bytes = hex::decode(signature)
            .map_err(|e| ChainError::InvalidSignature(format!("Invalid signature hex: {e}")))?;

        if signature_bytes.len() != 96 {
            return Err(ChainError::InvalidSignature(
                "Verifiable signature must be 96 bytes".to_string(),
            ));
        }

        let mut signature_array = [0u8; 96];
        signature_array.copy_from_slice(&signature_bytes);

        let is_valid =
            BandersnatchVrfVerifiable::verify_signature(&signature_array, message, &encoded_member);

        Ok(is_valid)
    }

    async fn get_person_ring_index(&self, member_hex: &str) -> Result<Option<u16>> {
        // Get the personal ID first
        let personal_id = match self.get_personal_id_by_key(member_hex).await? {
            Some(id) => id,
            None => return Ok(None), // Person not found
        };

        // Get the person record
        let person_record = match self.get_person_record(personal_id).await? {
            Some(record) => record,
            None => return Ok(None), // Record not found
        };

        // Extract ring index from position
        Ok(person_record.position.ring_index())
    }
}
