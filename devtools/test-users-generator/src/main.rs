use anyhow::{Context, Result};
use codec::Encode;
use scale_info::PortableRegistry;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use subxt::{
    client::ClientState,
    config::{
        substrate::MultiAddress,
        transaction_extensions,
        ExtrinsicParams, ExtrinsicParamsEncoder,
        ExtrinsicParamsError, TransactionExtension,
    },
    Config, OnlineClient, SubstrateConfig,
};
use subxt_signer::sr25519::dev;
use verifiable::{ring_vrf_impl::BandersnatchVrfVerifiable as Crypto, GenerateVerifiable};

// Generate the runtime API from metadata
// Metadata should be fetched using: subxt metadata --url ws://127.0.0.1:9910 -f bytes > people-metadata.scale
#[subxt::subxt(runtime_metadata_path = "people-metadata.scale")]
pub mod people_chain {}

// Define custom transaction extensions required by the People chain
macro_rules! define_simple_signed_extension {
    ($ext_name:ident, $ext_identifier:expr, $value:expr) => {
        pub struct $ext_name;

        impl<T: Config> ExtrinsicParams<T> for $ext_name {
            type Params = ();
            fn new(
                _client: &ClientState<T>,
                _params: Self::Params,
            ) -> Result<Self, ExtrinsicParamsError> {
                Ok($ext_name)
            }
        }

        impl ExtrinsicParamsEncoder for $ext_name {
            fn encode_value_to(&self, v: &mut Vec<u8>) {
                $value.encode_to(v);
            }
        }

        impl<T: Config> TransactionExtension<T> for $ext_name {
            type Decoded = u8;
            fn matches(identifier: &str, _type_id: u32, _types: &PortableRegistry) -> bool {
                identifier == $ext_identifier
            }
        }
    };
}

define_simple_signed_extension!(AsPerson, "AsPerson", 0u8);
define_simple_signed_extension!(ProvideForReferral, "ProvideForReferral", 0u8);
define_simple_signed_extension!(VerifyMultiSignature, "VerifyMultiSignature", 1u8);
define_simple_signed_extension!(ProvideForVoucherClaimer, "ProvideForVoucherClaimer", ());
define_simple_signed_extension!(ScoreAsParticipant, "ScoreAsParticipant", 0u8);
define_simple_signed_extension!(GameAsInvited, "GameAsInvited", 0u8);
define_simple_signed_extension!(
    AsProofOfInkParticipant,
    "AsProofOfInkParticipant",
    Option::<u64>::None
);
define_simple_signed_extension!(PeopleLiteAuth, "PeopleLiteAuth", Option::<u32>::None);
define_simple_signed_extension!(RestrictOrigin, "RestrictOrigins", false);

pub type PeopleChainExtrinsicParam<T> = transaction_extensions::AnyOf<
    T,
    (
        VerifyMultiSignature,
        AsPerson,
        AsProofOfInkParticipant,
        ProvideForVoucherClaimer,
        ScoreAsParticipant,
        GameAsInvited,
        PeopleLiteAuth,
        RestrictOrigin,
        transaction_extensions::CheckMetadataHash,
        transaction_extensions::CheckSpecVersion,
        transaction_extensions::CheckTxVersion,
        transaction_extensions::CheckNonce,
        transaction_extensions::CheckGenesis<T>,
        transaction_extensions::CheckMortality<T>,
        transaction_extensions::ChargeAssetTxPayment<T>,
    ),
>;

struct PeopleChainConfig {}

impl Config for PeopleChainConfig {
    type AccountId = <SubstrateConfig as Config>::AccountId;
    type Address = MultiAddress<Self::AccountId, ()>;
    type Signature = <SubstrateConfig as Config>::Signature;
    type Hasher = <SubstrateConfig as Config>::Hasher;
    type Header = <SubstrateConfig as Config>::Header;
    type ExtrinsicParams = PeopleChainExtrinsicParam<Self>;
    type AssetId = u32;
}

// Type alias for RuntimeCall
type RuntimeCall = people_chain::runtime_types::people_rococo_runtime::RuntimeCall;

// Weight struct for sudo calls
type Weight = people_chain::runtime_types::sp_weights::weight_v2::Weight;

#[derive(Debug, Serialize, Deserialize)]
struct RecognizedPerson {
    id: u64,
    secret_scalar: String,
    member_bytes: Vec<u8>,
    member_hex: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct TestUsers {
    recognized_persons: Vec<RecognizedPerson>,
    onboarding_queue: Vec<RecognizedPerson>,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .init();

    let chain_endpoint = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "ws://127.0.0.1:9910".to_string());
    let num_recognized = std::env::args()
        .nth(2)
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(6);
    let num_onboarding = std::env::args()
        .nth(3)
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(1);

    tracing::info!("Connecting to chain at {}", chain_endpoint);
    let api = OnlineClient::<PeopleChainConfig>::from_url(&chain_endpoint)
        .await
        .context("Failed to connect to chain")?;

    tracing::info!(
        "Connected! Generating {} recognized + {} onboarding users",
        num_recognized,
        num_onboarding
    );

    let total_users = num_recognized + num_onboarding;
    let mut test_users = TestUsers {
        recognized_persons: Vec::new(),
        onboarding_queue: Vec::new(),
    };

    // Use Alice as the sudo account
    let alice = dev::alice();

    // Reserve IDs for all users
    tracing::info!("Reserving {} personal IDs...", total_users);

    let reserve_ids_call = RuntimeCall::DummyDim(
        people_chain::runtime_types::indiv_pallet_dummy_dim::pallet::Call::reserve_ids {
            count: total_users as u32,
        },
    );

    call_as_root(&api, reserve_ids_call, &alice).await?;
    tracing::info!("IDs reserved successfully");

    // Get the next available ID to determine what was reserved
    let next_id_addr = people_chain::storage().people().next_personal_id();
    let next_id = api
        .storage()
        .at_latest()
        .await?
        .fetch(&next_id_addr)
        .await?
        .context("Next personal ID not found")?;

    let start_id = next_id - total_users as u64;
    tracing::info!("Reserved IDs: {} to {}", start_id, next_id - 1);

    // Generate keys and recognize personhood for all users
    for i in 0..total_users {
        let personal_id = start_id + i as u64;

        // Generate unique Bandersnatch key pair
        let secret = Crypto::new_secret([personal_id as u8; 32]);
        let member = Crypto::member_from_secret(&secret);

        // Serialize the member key
        let member_bytes = member.0.to_vec();
        let member_hex = hex::encode(&member_bytes);

        // Extract secret scalar as string (for JSON storage)
        let secret_scalar = secret.scalar.to_string();

        let person = RecognizedPerson {
            id: personal_id,
            secret_scalar,
            member_bytes: member_bytes.clone(),
            member_hex: member_hex.clone(),
        };

        if i < num_recognized {
            test_users.recognized_persons.push(person);
        } else {
            test_users.onboarding_queue.push(person);
        }

        tracing::info!(
            "Generated user {} (ID: {}): {}",
            i + 1,
            personal_id,
            member_hex
        );
    }

    // Recognize personhood for all users (batch call)
    tracing::info!("Recognizing personhood for all {} users...", total_users);

    // Create the recognition data
    let ids_and_keys: Vec<_> = (0..total_users)
        .map(|i| {
            let id = start_id + i as u64;
            let secret = Crypto::new_secret([id as u8; 32]);
            let member = Crypto::member_from_secret(&secret);

            // Convert to runtime type
            let runtime_member = people_chain::runtime_types::verifiable::ring_vrf_impl::EncodedPublicKey(
                member.0
            );

            (id, runtime_member)
        })
        .collect();

    let ids_and_keys_bounded = people_chain::runtime_types::bounded_collections::bounded_vec::BoundedVec(ids_and_keys);

    let recognize_call = RuntimeCall::DummyDim(
        people_chain::runtime_types::indiv_pallet_dummy_dim::pallet::Call::recognize_personhood {
            ids_and_keys: ids_and_keys_bounded,
        },
    );

    call_as_root(&api, recognize_call, &alice).await?;
    tracing::info!("Personhood recognized successfully");

    // Write output to test_users.json
    let output_path = PathBuf::from("../../src/e2e/test_users.json");
    let json_output = serde_json::to_string_pretty(&test_users)?;
    std::fs::write(&output_path, json_output)
        .context("Failed to write test_users.json")?;

    tracing::info!("Test users written to {:?}", output_path.canonicalize()?);

    println!("\n✅ Test users generated successfully!");
    println!("   - {} recognized persons", test_users.recognized_persons.len());
    println!("   - {} in onboarding queue", test_users.onboarding_queue.len());
    println!("\nOutput: {:?}", output_path.canonicalize()?);

    Ok(())
}

// Helper function to execute a call as root using sudo
async fn call_as_root(
    api: &OnlineClient<PeopleChainConfig>,
    call: RuntimeCall,
    caller: &subxt_signer::sr25519::Keypair,
) -> Result<()> {
    let weight = Weight {
        ref_time: 1_000_000_000_000,
        proof_size: 3_000_000,
    };

    let sudo_call = people_chain::tx()
        .sudo()
        .sudo_unchecked_weight(call, weight);

    let _events = api
        .tx()
        .sign_and_submit_then_watch_default(&sudo_call, caller)
        .await?
        .wait_for_finalized_success()
        .await?;

    Ok(())
}
