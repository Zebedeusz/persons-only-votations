use crate::chain::subxt_generated_people_apis::api as people_api;
use crate::crypto::hex_to_bytes32;
use async_trait::async_trait;
use people_api::runtime_types::pallet_people_multi::types::RingPosition;
use scale_codec::Decode;
use subxt::{OnlineClient, PolkadotConfig};
use thiserror::Error;
use verifiable::{
    GenerateVerifiable,
    ring_vrf_impl::{BandersnatchVrfVerifiable, EncodedPublicKey, MembersCommitment},
};

#[derive(Error, Debug, Clone)]
pub enum ChainError {
    #[error("Connection to chain failed: {0}")]
    ConnectionFailed(String),
    #[error("Invalid signature: {0}")]
    InvalidSignature(String),
    #[error("Chain query failed: {0}")]
    QueryFailed(String),
    #[error("Invalid public key format: {0}")]
    InvalidPublicKey(String),
    #[error("Storage decoding failed: {0}")]
    DecodingFailed(String),
}

pub type Result<T> = std::result::Result<T, ChainError>;

/// Read-only view of the People chain needed to admit a person to a poll.
#[async_trait]
pub trait ChainClient: Send + Sync {
    /// Checks the member's plain (non-ring) signature over `message`.
    async fn validate_signature(
        &self,
        member_hex: &str,
        signature_hex: &str,
        message: &[u8],
    ) -> Result<bool>;
    /// Ring the member is included in; `None` while onboarding, suspended or unknown.
    async fn ring_index(&self, member_hex: &str) -> Result<Option<u16>>;
    /// Members commitment of the ring; `None` if the ring has no root yet.
    async fn ring_root(&self, ring_index: u16) -> Result<Option<MembersCommitment>>;
}

type Storage = subxt::storage::Storage<PolkadotConfig, OnlineClient<PolkadotConfig>>;

#[derive(Debug)]
pub struct PeopleMultiClient {
    client: OnlineClient<PolkadotConfig>,
}

impl PeopleMultiClient {
    /// Connects with exponential backoff (1s, 2s, 4s, ...) between attempts.
    pub async fn new_with_retry(endpoint: &str, max_retries: u32) -> Result<Self> {
        let mut last_error = None;
        for attempt in 0..=max_retries {
            match OnlineClient::<PolkadotConfig>::from_url(endpoint).await {
                Ok(client) => return Ok(Self { client }),
                Err(e) => {
                    last_error = Some(e);
                    if attempt < max_retries {
                        tokio::time::sleep(std::time::Duration::from_secs(1 << attempt)).await;
                    }
                }
            }
        }
        Err(ChainError::ConnectionFailed(format!(
            "Failed to connect to {} after {} attempts: {}",
            endpoint,
            max_retries + 1,
            last_error.expect("at least one attempt was made")
        )))
    }

    pub fn client(&self) -> &OnlineClient<PolkadotConfig> {
        &self.client
    }

    async fn storage(&self) -> Result<Storage> {
        self.client
            .storage()
            .at_latest()
            .await
            .map_err(|e| ChainError::QueryFailed(format!("Failed to get storage: {e}")))
    }
}

fn member_key(member_hex: &str) -> Result<[u8; 32]> {
    hex_to_bytes32(member_hex).map_err(|e| {
        ChainError::InvalidPublicKey(format!("Member key must be 32 bytes of hex: {e}"))
    })
}

fn query_failed(what: &str) -> impl Fn(subxt::Error) -> ChainError + '_ {
    move |e| ChainError::QueryFailed(format!("Failed to query {what}: {e}"))
}

#[async_trait]
impl ChainClient for PeopleMultiClient {
    async fn validate_signature(
        &self,
        member_hex: &str,
        signature_hex: &str,
        message: &[u8],
    ) -> Result<bool> {
        let member = EncodedPublicKey(member_key(member_hex)?);
        let signature_bytes = hex::decode(signature_hex)
            .map_err(|e| ChainError::InvalidSignature(format!("Invalid signature hex: {e}")))?;
        let signature: [u8; 96] = signature_bytes.as_slice().try_into().map_err(|_| {
            ChainError::InvalidSignature("Verifiable signature must be 96 bytes".to_string())
        })?;
        Ok(BandersnatchVrfVerifiable::verify_signature(
            &signature, message, &member,
        ))
    }

    async fn ring_index(&self, member_hex: &str) -> Result<Option<u16>> {
        let key = people_api::runtime_types::verifiable::ring_vrf_impl::EncodedPublicKey(
            member_key(member_hex)?,
        );
        let storage = self.storage().await?;
        let Some(personal_id) = storage
            .fetch(&people_api::storage().people().keys(&key))
            .await
            .map_err(query_failed("Keys"))?
        else {
            return Ok(None);
        };
        let record = storage
            .fetch(&people_api::storage().people().people(personal_id))
            .await
            .map_err(query_failed("People"))?;
        Ok(match record.map(|r| r.position) {
            Some(RingPosition::Included { ring_index, .. }) => Some(ring_index as u16),
            _ => None,
        })
    }

    async fn ring_root(&self, ring_index: u16) -> Result<Option<MembersCommitment>> {
        let root = self
            .storage()
            .await?
            .fetch(&people_api::storage().people().root(ring_index as u32))
            .await
            .map_err(query_failed("Root"))?;
        root.map(|root| {
            MembersCommitment::decode(&mut &root.root[..]).map_err(|e| {
                ChainError::DecodingFailed(format!("Failed to decode members commitment: {e}"))
            })
        })
        .transpose()
    }
}
