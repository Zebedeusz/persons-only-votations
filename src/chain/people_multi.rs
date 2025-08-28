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
        // Parse the hex member key
        let member_bytes = hex::decode(member_hex)
            .map_err(|e| ChainError::InvalidPublicKey(format!("Invalid hex format: {e}")))?;

        if member_bytes.len() != 32 {
            return Err(ChainError::InvalidPublicKey(
                "Member key must be 32 bytes".to_string(),
            ));
        }

        let client = &self.client;

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
        let client = &self.client;

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

    async fn is_ring_active(&self, ring_index: u16) -> Result<bool> {
        let client = &self.client;

        // Query the Root storage map for the specific ring
        let storage_query = crate::chain::subxt_generated_people_apis::api::storage()
            .people()
            .root(ring_index as u32);

        match client.storage().at_latest().await {
            Ok(storage) => {
                match storage.fetch(&storage_query).await {
                    Ok(Some(_)) => Ok(true), // Ring exists and is active
                    Ok(None) => Ok(false),   // Ring not found or inactive
                    Err(e) => Err(ChainError::QueryFailed(format!(
                        "Failed to query ring root: {e}"
                    ))),
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
        tracing::debug!("Using real chain client for personhood verification");

        // 1. Get PersonalId from member key
        let personal_id = match self.get_personal_id_by_key(member_hex).await? {
            Some(id) => id,
            None => {
                return Ok(false);
            }
        };

        // 2. Get person record
        let person_record = match self.get_person_record(personal_id).await? {
            Some(record) => record,
            None => {
                return Ok(false);
            }
        };

        // 3. Check if person is included in a ring (not just onboarding)
        let ring_index = match person_record.position.ring_index() {
            Some(index) => index,
            None => {
                return Ok(false); // Person is still onboarding
            }
        };

        // 4. Verify the ring is active
        if !self.is_ring_active(ring_index).await? {
            return Ok(false);
        }

        Ok(true)
    }

    /// Get the ring root for a specific ring index
    /// This is needed for ring proof validation
    async fn get_ring_root(
        &self,
        ring_index: u16,
    ) -> Result<Option<crate::chain::runtime::RingRoot>> {
        let client = &self.client;

        // Query the Root storage map for the specific ring
        let storage_query = crate::chain::subxt_generated_people_apis::api::storage()
            .people()
            .root(ring_index as u32);

        match client.storage().at_latest().await {
            Ok(storage) => {
                match storage.fetch(&storage_query).await {
                    Ok(Some(ring_root)) => {
                        use scale_codec::Decode;

                        // Convert from generated types to our types
                        let members_commitment =
                            verifiable::ring_vrf_impl::MembersCommitment::decode(
                                &mut &ring_root.root[..],
                            )
                            .map_err(|e| {
                                ChainError::DecodingFailed(format!(
                                    "Failed to decode members commitment: {e}"
                                ))
                            })?;
                        let members_set = verifiable::ring_vrf_impl::MembersSet::decode(
                            &mut &ring_root.intermediate[..],
                        )
                        .map_err(|e| {
                            ChainError::DecodingFailed(format!("Failed to decode members set: {e}"))
                        })?;

                        let converted_root = crate::chain::runtime::RingRoot {
                            root: members_commitment,
                            revision: ring_root.revision,
                            intermediate: members_set,
                        };
                        Ok(Some(converted_root))
                    }
                    Ok(None) => Ok(None), // Ring not found
                    Err(e) => Err(ChainError::QueryFailed(format!(
                        "Failed to query ring root: {e}"
                    ))),
                }
            }
            Err(e) => Err(ChainError::QueryFailed(format!(
                "Failed to get storage: {e}"
            ))),
        }
    }

    async fn validate_signature(
        &self,
        member_hex: &str,
        signature: &str,
        message: &[u8],
    ) -> Result<bool> {
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
        let is_valid =
            BandersnatchVrfVerifiable::verify_signature(&signature_array, message, &encoded_member);

        Ok(is_valid)
    }
}
