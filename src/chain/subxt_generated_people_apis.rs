#[allow(dead_code, unused_imports, non_camel_case_types, unreachable_patterns)]
#[allow(clippy::all)]
#[allow(rustdoc::broken_intra_doc_links)]
pub mod api {
    #[allow(unused_imports)]
    mod root_mod {
        pub use super::*;
    }
    pub static PALLETS: [&str; 2usize] = ["System", "People"];
    pub static RUNTIME_APIS: [&str; 0usize] = [];
    #[doc = r" The error type that is returned when there is a runtime issue."]
    pub type DispatchError = runtime_types::sp_runtime::DispatchError;
    #[doc = r" The outer event enum."]
    pub type Event = runtime_types::people_rococo_runtime::RuntimeEvent;
    #[doc = r" The outer extrinsic enum."]
    pub type Call = runtime_types::people_rococo_runtime::RuntimeCall;
    #[doc = r" The outer error enum represents the DispatchError's Module variant."]
    pub type Error = runtime_types::people_rococo_runtime::RuntimeError;
    pub fn constants() -> ConstantsApi {
        ConstantsApi
    }
    pub fn storage() -> StorageApi {
        StorageApi
    }
    pub fn tx() -> TransactionApi {
        TransactionApi
    }
    pub fn apis() -> runtime_apis::RuntimeApi {
        runtime_apis::RuntimeApi
    }
    pub mod runtime_apis {
        use super::root_mod;
        use super::runtime_types;
        use ::subxt::ext::subxt_core::ext::codec::Encode;
        pub struct RuntimeApi;
        impl RuntimeApi {}
    }
    pub fn view_functions() -> ViewFunctionsApi {
        ViewFunctionsApi
    }
    pub fn custom() -> CustomValuesApi {
        CustomValuesApi
    }
    pub struct CustomValuesApi;
    impl CustomValuesApi {}
    pub struct ConstantsApi;
    impl ConstantsApi {
        pub fn people(&self) -> people::constants::ConstantsApi {
            people::constants::ConstantsApi
        }
    }
    pub struct StorageApi;
    impl StorageApi {
        pub fn people(&self) -> people::storage::StorageApi {
            people::storage::StorageApi
        }
    }
    pub struct TransactionApi;
    impl TransactionApi {
        pub fn people(&self) -> people::calls::TransactionApi {
            people::calls::TransactionApi
        }
    }
    pub struct ViewFunctionsApi;
    impl ViewFunctionsApi {}
    #[doc = r" check whether the metadata provided is aligned with this statically generated code."]
    pub fn is_codegen_valid_for(metadata: &::subxt::ext::subxt_core::Metadata) -> bool {
        let runtime_metadata_hash = metadata
            .hasher()
            .only_these_pallets(&PALLETS)
            .only_these_runtime_apis(&RUNTIME_APIS)
            .hash();
        runtime_metadata_hash
            == [
                84u8, 203u8, 170u8, 225u8, 72u8, 122u8, 94u8, 120u8, 46u8, 40u8, 227u8, 48u8, 33u8,
                219u8, 91u8, 125u8, 96u8, 210u8, 27u8, 46u8, 76u8, 80u8, 36u8, 142u8, 99u8, 101u8,
                230u8, 186u8, 59u8, 100u8, 68u8, 51u8,
            ]
    }
    pub mod system {
        use super::root_mod;
        use super::runtime_types;
    }
    pub mod people {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "The `Error` enum of this pallet."]
        pub type Error = runtime_types::pallet_people_multi::pallet::Error;
        #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
        pub type Call = runtime_types::pallet_people_multi::pallet::Call;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Build a ring root by including registered people."]
                #[doc = ""]
                #[doc = "This task is performed automatically by the pallet through the `on_idle` hook whenever"]
                #[doc = "there is leftover weight in a block. This call is meant to be a backup in case of"]
                #[doc = "extreme congestion and should be submitted by signed origins."]
                pub struct BuildRingManual {
                    pub ring_index: build_ring_manual::RingIndex,
                    pub limit: build_ring_manual::Limit,
                }
                pub mod build_ring_manual {
                    use super::runtime_types;
                    pub type RingIndex = ::core::primitive::u32;
                    pub type Limit = ::core::option::Option<::core::primitive::u32>;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for BuildRingManual {
                    const PALLET: &'static str = "People";
                    const CALL: &'static str = "build_ring_manual";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Onboard people into a ring by taking their keys from the onboarding queue and"]
                #[doc = "registering them into the ring. This does not compute the root, that is done using"]
                #[doc = "`build_ring`."]
                #[doc = ""]
                #[doc = "This task is performed automatically by the pallet through the `on_idle` hook whenever"]
                #[doc = "there is leftover weight in a block. This call is meant to be a backup in case of"]
                #[doc = "extreme congestion and should be submitted by signed origins."]
                pub struct OnboardPeopleManual;
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for OnboardPeopleManual {
                    const PALLET: &'static str = "People";
                    const CALL: &'static str = "onboard_people_manual";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Merge the people in two rings into a single, new ring. In order for the rings to be"]
                #[doc = "eligible for merging, they must be below 1/2 of max capacity, have no pending"]
                #[doc = "suspensions and not be the top ring used for onboarding."]
                pub struct MergeRings {
                    pub base_ring_index: merge_rings::BaseRingIndex,
                    pub target_ring_index: merge_rings::TargetRingIndex,
                }
                pub mod merge_rings {
                    use super::runtime_types;
                    pub type BaseRingIndex = ::core::primitive::u32;
                    pub type TargetRingIndex = ::core::primitive::u32;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for MergeRings {
                    const PALLET: &'static str = "People";
                    const CALL: &'static str = "merge_rings";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Dispatch a call under an alias using the `account <-> alias` mapping."]
                #[doc = ""]
                #[doc = "This is a call version of the transaction extension `AsPersonalAliasWithAccount`."]
                #[doc = "It is recommended to use the transaction extension instead when suitable."]
                pub struct UnderAlias {
                    pub call: ::subxt::ext::subxt_core::alloc::boxed::Box<under_alias::Call>,
                }
                pub mod under_alias {
                    use super::runtime_types;
                    pub type Call = runtime_types::people_rococo_runtime::RuntimeCall;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for UnderAlias {
                    const PALLET: &'static str = "People";
                    const CALL: &'static str = "under_alias";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "This transaction is refunded if successful and no alias was previously set."]
                #[doc = ""]
                #[doc = "The call is valid from `call_valid_at` until"]
                #[doc = "`call_valid_at + account_setup_time_tolerance`."]
                #[doc = "`account_setup_time_tolerance` is a constant available in the metadata."]
                #[doc = ""]
                #[doc = "Parameters:"]
                #[doc = "- `account`: The account to set the alias for."]
                #[doc = "- `call_valid_at`: The block number when the call becomes valid."]
                pub struct SetAliasAccount {
                    pub account: set_alias_account::Account,
                    pub call_valid_at: set_alias_account::CallValidAt,
                }
                pub mod set_alias_account {
                    use super::runtime_types;
                    pub type Account = ::subxt::ext::subxt_core::utils::AccountId32;
                    pub type CallValidAt = ::core::primitive::u32;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for SetAliasAccount {
                    const PALLET: &'static str = "People";
                    const CALL: &'static str = "set_alias_account";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Remove the mapping from a particular alias to its registered account."]
                pub struct UnsetAliasAccount;
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for UnsetAliasAccount {
                    const PALLET: &'static str = "People";
                    const CALL: &'static str = "unset_alias_account";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Recognize a set of people without any additional checks."]
                #[doc = ""]
                #[doc = "The people are identified by the provided list of keys and will each be assigned, in"]
                #[doc = "order, the next available personal ID."]
                #[doc = ""]
                #[doc = "The origin for this call must have root privileges."]
                pub struct ForceRecognizePersonhood {
                    pub people: force_recognize_personhood::People,
                }
                pub mod force_recognize_personhood {
                    use super::runtime_types;
                    pub type People = ::subxt::ext::subxt_core::alloc::vec::Vec<
                        runtime_types::verifiable::ring_vrf_impl::EncodedPublicKey,
                    >;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for ForceRecognizePersonhood {
                    const PALLET: &'static str = "People";
                    const CALL: &'static str = "force_recognize_personhood";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Set a personal id account."]
                #[doc = ""]
                #[doc = "The account can then be used to sign transactions on behalf of the personal id, and"]
                #[doc = "provide replay protection with the nonce."]
                #[doc = ""]
                #[doc = "This transaction is refunded if successful and no account was previously set for the"]
                #[doc = "personal id."]
                #[doc = ""]
                #[doc = "The call is valid from `call_valid_at` until"]
                #[doc = "`call_valid_at + account_setup_time_tolerance`."]
                #[doc = "`account_setup_time_tolerance` is a constant available in the metadata."]
                #[doc = ""]
                #[doc = "Parameters:"]
                #[doc = "- `account`: The account to set the alias for."]
                #[doc = "- `call_valid_at`: The block number when the call becomes valid."]
                pub struct SetPersonalIdAccount {
                    pub account: set_personal_id_account::Account,
                    pub call_valid_at: set_personal_id_account::CallValidAt,
                }
                pub mod set_personal_id_account {
                    use super::runtime_types;
                    pub type Account = ::subxt::ext::subxt_core::utils::AccountId32;
                    pub type CallValidAt = ::core::primitive::u32;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for SetPersonalIdAccount {
                    const PALLET: &'static str = "People";
                    const CALL: &'static str = "set_personal_id_account";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Unset the personal id account."]
                pub struct UnsetPersonalIdAccount;
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for UnsetPersonalIdAccount {
                    const PALLET: &'static str = "People";
                    const CALL: &'static str = "unset_personal_id_account";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Migrate the key for a person who was onboarded and is currently included in a ring. The"]
                #[doc = "migration is not instant as the key replacement and subsequent inclusion in a new ring"]
                #[doc = "root will happen only after the next mutation session."]
                pub struct MigrateIncludedKey {
                    pub new_key: migrate_included_key::NewKey,
                }
                pub mod migrate_included_key {
                    use super::runtime_types;
                    pub type NewKey = runtime_types::verifiable::ring_vrf_impl::EncodedPublicKey;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for MigrateIncludedKey {
                    const PALLET: &'static str = "People";
                    const CALL: &'static str = "migrate_included_key";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Migrate the key for a person who is currently onboarding. The operation is instant,"]
                #[doc = "replacing the old key in the onboarding queue."]
                pub struct MigrateOnboardingKey {
                    pub new_key: migrate_onboarding_key::NewKey,
                }
                pub mod migrate_onboarding_key {
                    use super::runtime_types;
                    pub type NewKey = runtime_types::verifiable::ring_vrf_impl::EncodedPublicKey;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for MigrateOnboardingKey {
                    const PALLET: &'static str = "People";
                    const CALL: &'static str = "migrate_onboarding_key";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Force set the onboarding size for new people. This call requires root privileges."]
                pub struct SetOnboardingSize {
                    pub onboarding_size: set_onboarding_size::OnboardingSize,
                }
                pub mod set_onboarding_size {
                    use super::runtime_types;
                    pub type OnboardingSize = ::core::primitive::u32;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for SetOnboardingSize {
                    const PALLET: &'static str = "People";
                    const CALL: &'static str = "set_onboarding_size";
                }
            }
            pub struct TransactionApi;
            impl TransactionApi {
                #[doc = "Build a ring root by including registered people."]
                #[doc = ""]
                #[doc = "This task is performed automatically by the pallet through the `on_idle` hook whenever"]
                #[doc = "there is leftover weight in a block. This call is meant to be a backup in case of"]
                #[doc = "extreme congestion and should be submitted by signed origins."]
                pub fn build_ring_manual(
                    &self,
                    ring_index: types::build_ring_manual::RingIndex,
                    limit: types::build_ring_manual::Limit,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::BuildRingManual>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "People",
                        "build_ring_manual",
                        types::BuildRingManual { ring_index, limit },
                        [
                            242u8, 179u8, 91u8, 143u8, 145u8, 78u8, 142u8, 162u8, 85u8, 104u8,
                            89u8, 113u8, 73u8, 78u8, 220u8, 38u8, 136u8, 58u8, 104u8, 29u8, 134u8,
                            236u8, 2u8, 189u8, 210u8, 43u8, 27u8, 235u8, 246u8, 88u8, 179u8, 207u8,
                        ],
                    )
                }
                #[doc = "Onboard people into a ring by taking their keys from the onboarding queue and"]
                #[doc = "registering them into the ring. This does not compute the root, that is done using"]
                #[doc = "`build_ring`."]
                #[doc = ""]
                #[doc = "This task is performed automatically by the pallet through the `on_idle` hook whenever"]
                #[doc = "there is leftover weight in a block. This call is meant to be a backup in case of"]
                #[doc = "extreme congestion and should be submitted by signed origins."]
                pub fn onboard_people_manual(
                    &self,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::OnboardPeopleManual>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "People",
                        "onboard_people_manual",
                        types::OnboardPeopleManual {},
                        [
                            212u8, 178u8, 163u8, 119u8, 201u8, 229u8, 24u8, 212u8, 71u8, 51u8,
                            220u8, 40u8, 238u8, 142u8, 77u8, 100u8, 170u8, 210u8, 137u8, 16u8,
                            15u8, 168u8, 137u8, 254u8, 219u8, 226u8, 21u8, 68u8, 42u8, 108u8, 29u8,
                            105u8,
                        ],
                    )
                }
                #[doc = "Merge the people in two rings into a single, new ring. In order for the rings to be"]
                #[doc = "eligible for merging, they must be below 1/2 of max capacity, have no pending"]
                #[doc = "suspensions and not be the top ring used for onboarding."]
                pub fn merge_rings(
                    &self,
                    base_ring_index: types::merge_rings::BaseRingIndex,
                    target_ring_index: types::merge_rings::TargetRingIndex,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::MergeRings>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "People",
                        "merge_rings",
                        types::MergeRings {
                            base_ring_index,
                            target_ring_index,
                        },
                        [
                            115u8, 255u8, 185u8, 6u8, 254u8, 225u8, 196u8, 128u8, 248u8, 146u8,
                            175u8, 176u8, 89u8, 103u8, 76u8, 137u8, 83u8, 95u8, 77u8, 242u8, 238u8,
                            42u8, 20u8, 255u8, 28u8, 52u8, 59u8, 210u8, 126u8, 188u8, 14u8, 159u8,
                        ],
                    )
                }
                #[doc = "Dispatch a call under an alias using the `account <-> alias` mapping."]
                #[doc = ""]
                #[doc = "This is a call version of the transaction extension `AsPersonalAliasWithAccount`."]
                #[doc = "It is recommended to use the transaction extension instead when suitable."]
                pub fn under_alias(
                    &self,
                    call: types::under_alias::Call,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::UnderAlias>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "People",
                        "under_alias",
                        types::UnderAlias {
                            call: ::subxt::ext::subxt_core::alloc::boxed::Box::new(call),
                        },
                        [
                            156u8, 19u8, 169u8, 103u8, 115u8, 235u8, 29u8, 120u8, 244u8, 19u8,
                            204u8, 232u8, 249u8, 140u8, 97u8, 52u8, 121u8, 17u8, 228u8, 49u8,
                            195u8, 28u8, 173u8, 65u8, 90u8, 145u8, 234u8, 60u8, 191u8, 96u8, 117u8,
                            14u8,
                        ],
                    )
                }
                #[doc = "This transaction is refunded if successful and no alias was previously set."]
                #[doc = ""]
                #[doc = "The call is valid from `call_valid_at` until"]
                #[doc = "`call_valid_at + account_setup_time_tolerance`."]
                #[doc = "`account_setup_time_tolerance` is a constant available in the metadata."]
                #[doc = ""]
                #[doc = "Parameters:"]
                #[doc = "- `account`: The account to set the alias for."]
                #[doc = "- `call_valid_at`: The block number when the call becomes valid."]
                pub fn set_alias_account(
                    &self,
                    account: types::set_alias_account::Account,
                    call_valid_at: types::set_alias_account::CallValidAt,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::SetAliasAccount>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "People",
                        "set_alias_account",
                        types::SetAliasAccount {
                            account,
                            call_valid_at,
                        },
                        [
                            175u8, 205u8, 31u8, 47u8, 33u8, 180u8, 30u8, 33u8, 233u8, 204u8, 63u8,
                            2u8, 155u8, 1u8, 236u8, 87u8, 77u8, 216u8, 32u8, 210u8, 12u8, 225u8,
                            195u8, 176u8, 12u8, 73u8, 95u8, 0u8, 161u8, 231u8, 74u8, 120u8,
                        ],
                    )
                }
                #[doc = "Remove the mapping from a particular alias to its registered account."]
                pub fn unset_alias_account(
                    &self,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::UnsetAliasAccount>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "People",
                        "unset_alias_account",
                        types::UnsetAliasAccount {},
                        [
                            238u8, 203u8, 129u8, 32u8, 116u8, 108u8, 5u8, 153u8, 79u8, 113u8,
                            139u8, 101u8, 204u8, 50u8, 47u8, 94u8, 61u8, 144u8, 163u8, 83u8, 11u8,
                            167u8, 153u8, 90u8, 204u8, 20u8, 23u8, 133u8, 64u8, 247u8, 194u8,
                            213u8,
                        ],
                    )
                }
                #[doc = "Recognize a set of people without any additional checks."]
                #[doc = ""]
                #[doc = "The people are identified by the provided list of keys and will each be assigned, in"]
                #[doc = "order, the next available personal ID."]
                #[doc = ""]
                #[doc = "The origin for this call must have root privileges."]
                pub fn force_recognize_personhood(
                    &self,
                    people: types::force_recognize_personhood::People,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<
                    types::ForceRecognizePersonhood,
                > {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "People",
                        "force_recognize_personhood",
                        types::ForceRecognizePersonhood { people },
                        [
                            223u8, 139u8, 140u8, 122u8, 41u8, 98u8, 10u8, 27u8, 62u8, 131u8, 201u8,
                            167u8, 96u8, 55u8, 97u8, 220u8, 114u8, 149u8, 19u8, 208u8, 99u8, 97u8,
                            155u8, 87u8, 138u8, 0u8, 12u8, 197u8, 61u8, 174u8, 226u8, 43u8,
                        ],
                    )
                }
                #[doc = "Set a personal id account."]
                #[doc = ""]
                #[doc = "The account can then be used to sign transactions on behalf of the personal id, and"]
                #[doc = "provide replay protection with the nonce."]
                #[doc = ""]
                #[doc = "This transaction is refunded if successful and no account was previously set for the"]
                #[doc = "personal id."]
                #[doc = ""]
                #[doc = "The call is valid from `call_valid_at` until"]
                #[doc = "`call_valid_at + account_setup_time_tolerance`."]
                #[doc = "`account_setup_time_tolerance` is a constant available in the metadata."]
                #[doc = ""]
                #[doc = "Parameters:"]
                #[doc = "- `account`: The account to set the alias for."]
                #[doc = "- `call_valid_at`: The block number when the call becomes valid."]
                pub fn set_personal_id_account(
                    &self,
                    account: types::set_personal_id_account::Account,
                    call_valid_at: types::set_personal_id_account::CallValidAt,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::SetPersonalIdAccount>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "People",
                        "set_personal_id_account",
                        types::SetPersonalIdAccount {
                            account,
                            call_valid_at,
                        },
                        [
                            53u8, 54u8, 18u8, 202u8, 174u8, 253u8, 23u8, 201u8, 106u8, 37u8, 153u8,
                            69u8, 136u8, 3u8, 163u8, 244u8, 121u8, 168u8, 222u8, 159u8, 247u8,
                            165u8, 243u8, 53u8, 170u8, 227u8, 221u8, 182u8, 155u8, 168u8, 32u8,
                            62u8,
                        ],
                    )
                }
                #[doc = "Unset the personal id account."]
                pub fn unset_personal_id_account(
                    &self,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<
                    types::UnsetPersonalIdAccount,
                > {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "People",
                        "unset_personal_id_account",
                        types::UnsetPersonalIdAccount {},
                        [
                            224u8, 104u8, 118u8, 192u8, 154u8, 92u8, 42u8, 230u8, 52u8, 23u8,
                            166u8, 59u8, 72u8, 102u8, 90u8, 205u8, 121u8, 51u8, 91u8, 250u8, 225u8,
                            208u8, 182u8, 126u8, 159u8, 44u8, 15u8, 37u8, 254u8, 220u8, 84u8,
                            100u8,
                        ],
                    )
                }
                #[doc = "Migrate the key for a person who was onboarded and is currently included in a ring. The"]
                #[doc = "migration is not instant as the key replacement and subsequent inclusion in a new ring"]
                #[doc = "root will happen only after the next mutation session."]
                pub fn migrate_included_key(
                    &self,
                    new_key: types::migrate_included_key::NewKey,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::MigrateIncludedKey>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "People",
                        "migrate_included_key",
                        types::MigrateIncludedKey { new_key },
                        [
                            190u8, 235u8, 199u8, 9u8, 236u8, 29u8, 82u8, 169u8, 254u8, 173u8,
                            180u8, 211u8, 113u8, 221u8, 226u8, 79u8, 163u8, 191u8, 139u8, 135u8,
                            4u8, 25u8, 126u8, 238u8, 196u8, 124u8, 207u8, 45u8, 90u8, 198u8, 37u8,
                            53u8,
                        ],
                    )
                }
                #[doc = "Migrate the key for a person who is currently onboarding. The operation is instant,"]
                #[doc = "replacing the old key in the onboarding queue."]
                pub fn migrate_onboarding_key(
                    &self,
                    new_key: types::migrate_onboarding_key::NewKey,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::MigrateOnboardingKey>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "People",
                        "migrate_onboarding_key",
                        types::MigrateOnboardingKey { new_key },
                        [
                            56u8, 223u8, 173u8, 148u8, 216u8, 7u8, 100u8, 22u8, 206u8, 232u8, 17u8,
                            73u8, 192u8, 244u8, 30u8, 5u8, 220u8, 20u8, 218u8, 240u8, 93u8, 211u8,
                            160u8, 160u8, 30u8, 19u8, 222u8, 173u8, 94u8, 136u8, 60u8, 60u8,
                        ],
                    )
                }
                #[doc = "Force set the onboarding size for new people. This call requires root privileges."]
                pub fn set_onboarding_size(
                    &self,
                    onboarding_size: types::set_onboarding_size::OnboardingSize,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::SetOnboardingSize>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "People",
                        "set_onboarding_size",
                        types::SetOnboardingSize { onboarding_size },
                        [
                            123u8, 227u8, 36u8, 7u8, 162u8, 229u8, 183u8, 243u8, 137u8, 78u8, 1u8,
                            186u8, 184u8, 35u8, 186u8, 148u8, 45u8, 165u8, 126u8, 106u8, 117u8,
                            120u8, 202u8, 48u8, 83u8, 46u8, 4u8, 170u8, 138u8, 50u8, 162u8, 237u8,
                        ],
                    )
                }
            }
        }
        #[doc = "The `Event` enum of this pallet"]
        pub type Event = runtime_types::pallet_people_multi::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[codec(dumb_trait_bound)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "An individual has had their personhood recognised and indexed."]
            pub struct PersonhoodRecognized {
                pub who: personhood_recognized::Who,
                pub key: personhood_recognized::Key,
            }
            pub mod personhood_recognized {
                use super::runtime_types;
                pub type Who = ::core::primitive::u64;
                pub type Key = runtime_types::verifiable::ring_vrf_impl::EncodedPublicKey;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for PersonhoodRecognized {
                const PALLET: &'static str = "People";
                const EVENT: &'static str = "PersonhoodRecognized";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[codec(dumb_trait_bound)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "An individual has had their personhood recognised again and indexed."]
            pub struct PersonOnboarding {
                pub who: person_onboarding::Who,
                pub key: person_onboarding::Key,
            }
            pub mod person_onboarding {
                use super::runtime_types;
                pub type Who = ::core::primitive::u64;
                pub type Key = runtime_types::verifiable::ring_vrf_impl::EncodedPublicKey;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for PersonOnboarding {
                const PALLET: &'static str = "People";
                const EVENT: &'static str = "PersonOnboarding";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub mod types {
                use super::runtime_types;
                pub mod root {
                    use super::runtime_types;
                    pub type Root = runtime_types::pallet_people_multi::types::RingRoot;
                    pub type Param0 = ::core::primitive::u32;
                }
                pub mod current_ring_index {
                    use super::runtime_types;
                    pub type CurrentRingIndex = ::core::primitive::u32;
                }
                pub mod onboarding_size {
                    use super::runtime_types;
                    pub type OnboardingSize = ::core::primitive::u32;
                }
                pub mod ring_building_people_limit {
                    use super::runtime_types;
                    pub type RingBuildingPeopleLimit = ::core::primitive::u32;
                }
                pub mod ring_keys {
                    use super::runtime_types;
                    pub type RingKeys = runtime_types::bounded_collections::bounded_vec::BoundedVec<
                        runtime_types::verifiable::ring_vrf_impl::EncodedPublicKey,
                    >;
                    pub type Param0 = ::core::primitive::u32;
                }
                pub mod ring_keys_status {
                    use super::runtime_types;
                    pub type RingKeysStatus = runtime_types::pallet_people_multi::types::RingStatus;
                    pub type Param0 = ::core::primitive::u32;
                }
                pub mod pending_suspensions {
                    use super::runtime_types;
                    pub type PendingSuspensions =
                        runtime_types::bounded_collections::bounded_vec::BoundedVec<
                            ::core::primitive::u32,
                        >;
                    pub type Param0 = ::core::primitive::u32;
                }
                pub mod active_members {
                    use super::runtime_types;
                    pub type ActiveMembers = ::core::primitive::u32;
                }
                pub mod keys {
                    use super::runtime_types;
                    pub type Keys = ::core::primitive::u64;
                    pub type Param0 = runtime_types::verifiable::ring_vrf_impl::EncodedPublicKey;
                }
                pub mod counter_for_keys {
                    use super::runtime_types;
                    pub type CounterForKeys = ::core::primitive::u32;
                }
                pub mod key_migration_queue {
                    use super::runtime_types;
                    pub type KeyMigrationQueue =
                        runtime_types::verifiable::ring_vrf_impl::EncodedPublicKey;
                    pub type Param0 = ::core::primitive::u64;
                }
                pub mod people {
                    use super::runtime_types;
                    pub type People = runtime_types::pallet_people_multi::types::PersonRecord<
                        runtime_types::verifiable::ring_vrf_impl::EncodedPublicKey,
                        ::subxt::ext::subxt_core::utils::AccountId32,
                    >;
                    pub type Param0 = ::core::primitive::u64;
                }
                pub mod alias_to_account {
                    use super::runtime_types;
                    pub type AliasToAccount = ::subxt::ext::subxt_core::utils::AccountId32;
                    pub type Param0 =
                        runtime_types::individuality_support::traits::reality::ContextualAlias;
                }
                pub mod account_to_alias {
                    use super::runtime_types;
                    pub type AccountToAlias =
                        runtime_types::pallet_people_multi::types::RevisedContextualAlias;
                    pub type Param0 = ::subxt::ext::subxt_core::utils::AccountId32;
                }
                pub mod account_to_personal_id {
                    use super::runtime_types;
                    pub type AccountToPersonalId = ::core::primitive::u64;
                    pub type Param0 = ::subxt::ext::subxt_core::utils::AccountId32;
                }
                pub mod chunks {
                    use super::runtime_types;
                    pub type Chunks = runtime_types::bounded_collections::bounded_vec::BoundedVec<
                        [::core::primitive::u8; 48usize],
                    >;
                    pub type Param0 = ::core::primitive::u32;
                }
                pub mod next_personal_id {
                    use super::runtime_types;
                    pub type NextPersonalId = ::core::primitive::u64;
                }
                pub mod rings_state {
                    use super::runtime_types;
                    pub type RingsState =
                        runtime_types::pallet_people_multi::types::RingMembersState;
                }
                pub mod reserved_personal_id {
                    use super::runtime_types;
                    pub type ReservedPersonalId = ();
                    pub type Param0 = ::core::primitive::u64;
                }
                pub mod queue_page_indices {
                    use super::runtime_types;
                    pub type QueuePageIndices = (::core::primitive::u32, ::core::primitive::u32);
                }
                pub mod onboarding_queue {
                    use super::runtime_types;
                    pub type OnboardingQueue =
                        runtime_types::bounded_collections::bounded_vec::BoundedVec<
                            runtime_types::verifiable::ring_vrf_impl::EncodedPublicKey,
                        >;
                    pub type Param0 = ::core::primitive::u32;
                }
            }
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " The current individuals we recognise."]
                pub fn root_iter(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::root::Root,
                    (),
                    (),
                    ::subxt::ext::subxt_core::utils::Yes,
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "People",
                        "Root",
                        (),
                        [
                            47u8, 176u8, 75u8, 166u8, 36u8, 98u8, 227u8, 159u8, 61u8, 141u8, 194u8,
                            84u8, 58u8, 236u8, 5u8, 22u8, 113u8, 222u8, 129u8, 189u8, 131u8, 202u8,
                            253u8, 114u8, 24u8, 137u8, 122u8, 75u8, 139u8, 62u8, 113u8, 240u8,
                        ],
                    )
                }
                #[doc = " The current individuals we recognise."]
                pub fn root(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::root::Param0>,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    ::subxt::ext::subxt_core::storage::address::StaticStorageKey<
                        types::root::Param0,
                    >,
                    types::root::Root,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "People",
                        "Root",
                        ::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(
                            _0.borrow(),
                        ),
                        [
                            47u8, 176u8, 75u8, 166u8, 36u8, 98u8, 227u8, 159u8, 61u8, 141u8, 194u8,
                            84u8, 58u8, 236u8, 5u8, 22u8, 113u8, 222u8, 129u8, 189u8, 131u8, 202u8,
                            253u8, 114u8, 24u8, 137u8, 122u8, 75u8, 139u8, 62u8, 113u8, 240u8,
                        ],
                    )
                }
                #[doc = " Keeps track of the ring index currently being populated."]
                pub fn current_ring_index(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::current_ring_index::CurrentRingIndex,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "People",
                        "CurrentRingIndex",
                        (),
                        [
                            195u8, 10u8, 27u8, 214u8, 6u8, 251u8, 203u8, 34u8, 110u8, 94u8, 111u8,
                            16u8, 191u8, 180u8, 239u8, 178u8, 88u8, 120u8, 81u8, 96u8, 174u8,
                            179u8, 200u8, 176u8, 10u8, 31u8, 148u8, 48u8, 101u8, 123u8, 39u8,
                            186u8,
                        ],
                    )
                }
                #[doc = " Maximum number of people queued before onboarding to a ring."]
                pub fn onboarding_size(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::onboarding_size::OnboardingSize,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "People",
                        "OnboardingSize",
                        (),
                        [
                            221u8, 131u8, 171u8, 83u8, 228u8, 31u8, 207u8, 228u8, 211u8, 76u8,
                            90u8, 29u8, 242u8, 193u8, 253u8, 21u8, 31u8, 191u8, 204u8, 195u8,
                            167u8, 53u8, 4u8, 181u8, 51u8, 27u8, 140u8, 22u8, 239u8, 87u8, 138u8,
                            233u8,
                        ],
                    )
                }
                #[doc = " Hint for the maximum number of people that can be included in a ring through a single root"]
                #[doc = " building call. If no value is set, then the onboarding size will be used instead."]
                pub fn ring_building_people_limit(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::ring_building_people_limit::RingBuildingPeopleLimit,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "People",
                        "RingBuildingPeopleLimit",
                        (),
                        [
                            128u8, 181u8, 186u8, 114u8, 40u8, 175u8, 71u8, 196u8, 51u8, 185u8,
                            57u8, 32u8, 191u8, 19u8, 252u8, 127u8, 136u8, 157u8, 183u8, 17u8, 64u8,
                            82u8, 143u8, 152u8, 238u8, 161u8, 24u8, 224u8, 1u8, 111u8, 94u8, 213u8,
                        ],
                    )
                }
                #[doc = " Both the keys that are included in built rings"]
                #[doc = " and the keys that will be used in future rings."]
                pub fn ring_keys_iter(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::ring_keys::RingKeys,
                    (),
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "People",
                        "RingKeys",
                        (),
                        [
                            177u8, 111u8, 50u8, 210u8, 228u8, 29u8, 17u8, 20u8, 196u8, 99u8, 146u8,
                            96u8, 85u8, 114u8, 2u8, 71u8, 94u8, 100u8, 66u8, 96u8, 90u8, 72u8,
                            11u8, 49u8, 103u8, 177u8, 108u8, 241u8, 69u8, 240u8, 191u8, 166u8,
                        ],
                    )
                }
                #[doc = " Both the keys that are included in built rings"]
                #[doc = " and the keys that will be used in future rings."]
                pub fn ring_keys(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::ring_keys::Param0>,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    ::subxt::ext::subxt_core::storage::address::StaticStorageKey<
                        types::ring_keys::Param0,
                    >,
                    types::ring_keys::RingKeys,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "People",
                        "RingKeys",
                        ::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(
                            _0.borrow(),
                        ),
                        [
                            177u8, 111u8, 50u8, 210u8, 228u8, 29u8, 17u8, 20u8, 196u8, 99u8, 146u8,
                            96u8, 85u8, 114u8, 2u8, 71u8, 94u8, 100u8, 66u8, 96u8, 90u8, 72u8,
                            11u8, 49u8, 103u8, 177u8, 108u8, 241u8, 69u8, 240u8, 191u8, 166u8,
                        ],
                    )
                }
                #[doc = " Stores the meta information for each ring, the number of keys and how many are actually"]
                #[doc = " included in the root."]
                pub fn ring_keys_status_iter(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::ring_keys_status::RingKeysStatus,
                    (),
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "People",
                        "RingKeysStatus",
                        (),
                        [
                            214u8, 62u8, 237u8, 236u8, 76u8, 92u8, 12u8, 232u8, 21u8, 130u8, 57u8,
                            230u8, 86u8, 112u8, 86u8, 21u8, 32u8, 5u8, 148u8, 222u8, 133u8, 160u8,
                            7u8, 214u8, 245u8, 138u8, 100u8, 30u8, 152u8, 79u8, 123u8, 96u8,
                        ],
                    )
                }
                #[doc = " Stores the meta information for each ring, the number of keys and how many are actually"]
                #[doc = " included in the root."]
                pub fn ring_keys_status(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::ring_keys_status::Param0>,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    ::subxt::ext::subxt_core::storage::address::StaticStorageKey<
                        types::ring_keys_status::Param0,
                    >,
                    types::ring_keys_status::RingKeysStatus,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "People",
                        "RingKeysStatus",
                        ::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(
                            _0.borrow(),
                        ),
                        [
                            214u8, 62u8, 237u8, 236u8, 76u8, 92u8, 12u8, 232u8, 21u8, 130u8, 57u8,
                            230u8, 86u8, 112u8, 86u8, 21u8, 32u8, 5u8, 148u8, 222u8, 133u8, 160u8,
                            7u8, 214u8, 245u8, 138u8, 100u8, 30u8, 152u8, 79u8, 123u8, 96u8,
                        ],
                    )
                }
                #[doc = " A map of all rings which currently have pending suspensions and need cleaning, along with"]
                #[doc = " their respective number of suspended keys which need to be removed."]
                pub fn pending_suspensions_iter(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::pending_suspensions::PendingSuspensions,
                    (),
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "People",
                        "PendingSuspensions",
                        (),
                        [
                            101u8, 81u8, 247u8, 158u8, 2u8, 188u8, 225u8, 205u8, 10u8, 130u8,
                            136u8, 211u8, 108u8, 221u8, 250u8, 213u8, 150u8, 60u8, 0u8, 0u8, 123u8,
                            47u8, 203u8, 121u8, 114u8, 199u8, 106u8, 213u8, 85u8, 225u8, 221u8,
                            254u8,
                        ],
                    )
                }
                #[doc = " A map of all rings which currently have pending suspensions and need cleaning, along with"]
                #[doc = " their respective number of suspended keys which need to be removed."]
                pub fn pending_suspensions(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::pending_suspensions::Param0>,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    ::subxt::ext::subxt_core::storage::address::StaticStorageKey<
                        types::pending_suspensions::Param0,
                    >,
                    types::pending_suspensions::PendingSuspensions,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "People",
                        "PendingSuspensions",
                        ::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(
                            _0.borrow(),
                        ),
                        [
                            101u8, 81u8, 247u8, 158u8, 2u8, 188u8, 225u8, 205u8, 10u8, 130u8,
                            136u8, 211u8, 108u8, 221u8, 250u8, 213u8, 150u8, 60u8, 0u8, 0u8, 123u8,
                            47u8, 203u8, 121u8, 114u8, 199u8, 106u8, 213u8, 85u8, 225u8, 221u8,
                            254u8,
                        ],
                    )
                }
                #[doc = " The number of people currently included in a ring."]
                pub fn active_members(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::active_members::ActiveMembers,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "People",
                        "ActiveMembers",
                        (),
                        [
                            128u8, 89u8, 240u8, 65u8, 232u8, 158u8, 242u8, 184u8, 156u8, 132u8,
                            65u8, 201u8, 168u8, 184u8, 153u8, 240u8, 145u8, 155u8, 127u8, 246u8,
                            120u8, 65u8, 167u8, 115u8, 44u8, 88u8, 253u8, 137u8, 255u8, 109u8,
                            218u8, 49u8,
                        ],
                    )
                }
                #[doc = " The current individuals we recognise, but not necessarily yet included in a ring."]
                #[doc = ""]
                #[doc = " Look-up from the crypto (public) key to the immutable ID of the individual (`PersonalId`). A"]
                #[doc = " person can have two different entries in this map if they queued a key migration which"]
                #[doc = " hasn't been enacted yet."]
                pub fn keys_iter(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::keys::Keys,
                    (),
                    (),
                    ::subxt::ext::subxt_core::utils::Yes,
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "People",
                        "Keys",
                        (),
                        [
                            211u8, 61u8, 71u8, 182u8, 110u8, 24u8, 1u8, 211u8, 58u8, 171u8, 210u8,
                            85u8, 103u8, 93u8, 49u8, 8u8, 217u8, 214u8, 179u8, 235u8, 19u8, 105u8,
                            50u8, 124u8, 65u8, 171u8, 23u8, 109u8, 218u8, 213u8, 215u8, 213u8,
                        ],
                    )
                }
                #[doc = " The current individuals we recognise, but not necessarily yet included in a ring."]
                #[doc = ""]
                #[doc = " Look-up from the crypto (public) key to the immutable ID of the individual (`PersonalId`). A"]
                #[doc = " person can have two different entries in this map if they queued a key migration which"]
                #[doc = " hasn't been enacted yet."]
                pub fn keys(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::keys::Param0>,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    ::subxt::ext::subxt_core::storage::address::StaticStorageKey<
                        types::keys::Param0,
                    >,
                    types::keys::Keys,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "People",
                        "Keys",
                        ::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(
                            _0.borrow(),
                        ),
                        [
                            211u8, 61u8, 71u8, 182u8, 110u8, 24u8, 1u8, 211u8, 58u8, 171u8, 210u8,
                            85u8, 103u8, 93u8, 49u8, 8u8, 217u8, 214u8, 179u8, 235u8, 19u8, 105u8,
                            50u8, 124u8, 65u8, 171u8, 23u8, 109u8, 218u8, 213u8, 215u8, 213u8,
                        ],
                    )
                }
                #[doc = "Counter for the related counted storage map"]
                pub fn counter_for_keys(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::counter_for_keys::CounterForKeys,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "People",
                        "CounterForKeys",
                        (),
                        [
                            232u8, 254u8, 255u8, 46u8, 239u8, 137u8, 115u8, 135u8, 18u8, 120u8,
                            200u8, 159u8, 36u8, 34u8, 181u8, 98u8, 150u8, 81u8, 246u8, 48u8, 104u8,
                            9u8, 123u8, 120u8, 247u8, 28u8, 121u8, 235u8, 225u8, 248u8, 186u8,
                            106u8,
                        ],
                    )
                }
                #[doc = " A map of all the people who have declared their intent to migrate their keys and are waiting"]
                #[doc = " for the next mutation session."]
                pub fn key_migration_queue_iter(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::key_migration_queue::KeyMigrationQueue,
                    (),
                    (),
                    ::subxt::ext::subxt_core::utils::Yes,
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "People",
                        "KeyMigrationQueue",
                        (),
                        [
                            141u8, 61u8, 186u8, 201u8, 96u8, 58u8, 159u8, 101u8, 228u8, 178u8, 9u8,
                            7u8, 72u8, 181u8, 25u8, 239u8, 160u8, 169u8, 73u8, 127u8, 195u8, 248u8,
                            7u8, 151u8, 184u8, 216u8, 36u8, 38u8, 49u8, 87u8, 132u8, 11u8,
                        ],
                    )
                }
                #[doc = " A map of all the people who have declared their intent to migrate their keys and are waiting"]
                #[doc = " for the next mutation session."]
                pub fn key_migration_queue(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::key_migration_queue::Param0>,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    ::subxt::ext::subxt_core::storage::address::StaticStorageKey<
                        types::key_migration_queue::Param0,
                    >,
                    types::key_migration_queue::KeyMigrationQueue,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "People",
                        "KeyMigrationQueue",
                        ::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(
                            _0.borrow(),
                        ),
                        [
                            141u8, 61u8, 186u8, 201u8, 96u8, 58u8, 159u8, 101u8, 228u8, 178u8, 9u8,
                            7u8, 72u8, 181u8, 25u8, 239u8, 160u8, 169u8, 73u8, 127u8, 195u8, 248u8,
                            7u8, 151u8, 184u8, 216u8, 36u8, 38u8, 49u8, 87u8, 132u8, 11u8,
                        ],
                    )
                }
                #[doc = " The current individuals we recognise, but not necessarily yet included in a ring."]
                #[doc = ""]
                #[doc = " Immutable ID of the individual (`PersonalId`) to information about their key and status."]
                pub fn people_iter(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::people::People,
                    (),
                    (),
                    ::subxt::ext::subxt_core::utils::Yes,
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "People",
                        "People",
                        (),
                        [
                            68u8, 243u8, 73u8, 52u8, 211u8, 197u8, 236u8, 58u8, 152u8, 46u8, 121u8,
                            227u8, 83u8, 252u8, 160u8, 252u8, 74u8, 169u8, 142u8, 122u8, 128u8,
                            6u8, 107u8, 233u8, 53u8, 186u8, 140u8, 158u8, 186u8, 29u8, 69u8, 180u8,
                        ],
                    )
                }
                #[doc = " The current individuals we recognise, but not necessarily yet included in a ring."]
                #[doc = ""]
                #[doc = " Immutable ID of the individual (`PersonalId`) to information about their key and status."]
                pub fn people(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::people::Param0>,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    ::subxt::ext::subxt_core::storage::address::StaticStorageKey<
                        types::people::Param0,
                    >,
                    types::people::People,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "People",
                        "People",
                        ::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(
                            _0.borrow(),
                        ),
                        [
                            68u8, 243u8, 73u8, 52u8, 211u8, 197u8, 236u8, 58u8, 152u8, 46u8, 121u8,
                            227u8, 83u8, 252u8, 160u8, 252u8, 74u8, 169u8, 142u8, 122u8, 128u8,
                            6u8, 107u8, 233u8, 53u8, 186u8, 140u8, 158u8, 186u8, 29u8, 69u8, 180u8,
                        ],
                    )
                }
                #[doc = " Conversion of a contextual alias to an account ID."]
                pub fn alias_to_account_iter(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::alias_to_account::AliasToAccount,
                    (),
                    (),
                    ::subxt::ext::subxt_core::utils::Yes,
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "People",
                        "AliasToAccount",
                        (),
                        [
                            90u8, 243u8, 169u8, 254u8, 140u8, 222u8, 35u8, 176u8, 45u8, 7u8, 254u8,
                            197u8, 234u8, 166u8, 253u8, 47u8, 5u8, 126u8, 86u8, 21u8, 170u8, 229u8,
                            197u8, 183u8, 207u8, 27u8, 117u8, 196u8, 23u8, 230u8, 230u8, 27u8,
                        ],
                    )
                }
                #[doc = " Conversion of a contextual alias to an account ID."]
                pub fn alias_to_account(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::alias_to_account::Param0>,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    ::subxt::ext::subxt_core::storage::address::StaticStorageKey<
                        types::alias_to_account::Param0,
                    >,
                    types::alias_to_account::AliasToAccount,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "People",
                        "AliasToAccount",
                        ::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(
                            _0.borrow(),
                        ),
                        [
                            90u8, 243u8, 169u8, 254u8, 140u8, 222u8, 35u8, 176u8, 45u8, 7u8, 254u8,
                            197u8, 234u8, 166u8, 253u8, 47u8, 5u8, 126u8, 86u8, 21u8, 170u8, 229u8,
                            197u8, 183u8, 207u8, 27u8, 117u8, 196u8, 23u8, 230u8, 230u8, 27u8,
                        ],
                    )
                }
                #[doc = " Conversion of an account ID to a contextual alias."]
                pub fn account_to_alias_iter(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::account_to_alias::AccountToAlias,
                    (),
                    (),
                    ::subxt::ext::subxt_core::utils::Yes,
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "People",
                        "AccountToAlias",
                        (),
                        [
                            72u8, 246u8, 128u8, 68u8, 76u8, 69u8, 119u8, 113u8, 17u8, 95u8, 227u8,
                            172u8, 201u8, 236u8, 137u8, 185u8, 42u8, 139u8, 194u8, 101u8, 200u8,
                            64u8, 25u8, 93u8, 52u8, 90u8, 75u8, 114u8, 118u8, 92u8, 52u8, 61u8,
                        ],
                    )
                }
                #[doc = " Conversion of an account ID to a contextual alias."]
                pub fn account_to_alias(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::account_to_alias::Param0>,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    ::subxt::ext::subxt_core::storage::address::StaticStorageKey<
                        types::account_to_alias::Param0,
                    >,
                    types::account_to_alias::AccountToAlias,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "People",
                        "AccountToAlias",
                        ::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(
                            _0.borrow(),
                        ),
                        [
                            72u8, 246u8, 128u8, 68u8, 76u8, 69u8, 119u8, 113u8, 17u8, 95u8, 227u8,
                            172u8, 201u8, 236u8, 137u8, 185u8, 42u8, 139u8, 194u8, 101u8, 200u8,
                            64u8, 25u8, 93u8, 52u8, 90u8, 75u8, 114u8, 118u8, 92u8, 52u8, 61u8,
                        ],
                    )
                }
                #[doc = " Association of an account ID to a personal ID."]
                #[doc = ""]
                #[doc = " Managed with `set_personal_id_account` and `unset_personal_id_account`."]
                #[doc = " Reverse lookup is inside `People` storage, inside the record."]
                pub fn account_to_personal_id_iter(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::account_to_personal_id::AccountToPersonalId,
                    (),
                    (),
                    ::subxt::ext::subxt_core::utils::Yes,
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "People",
                        "AccountToPersonalId",
                        (),
                        [
                            236u8, 149u8, 24u8, 190u8, 99u8, 144u8, 149u8, 212u8, 247u8, 172u8,
                            75u8, 9u8, 160u8, 192u8, 10u8, 126u8, 153u8, 186u8, 26u8, 184u8, 25u8,
                            235u8, 143u8, 63u8, 154u8, 170u8, 60u8, 159u8, 75u8, 11u8, 222u8,
                            134u8,
                        ],
                    )
                }
                #[doc = " Association of an account ID to a personal ID."]
                #[doc = ""]
                #[doc = " Managed with `set_personal_id_account` and `unset_personal_id_account`."]
                #[doc = " Reverse lookup is inside `People` storage, inside the record."]
                pub fn account_to_personal_id(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::account_to_personal_id::Param0>,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    ::subxt::ext::subxt_core::storage::address::StaticStorageKey<
                        types::account_to_personal_id::Param0,
                    >,
                    types::account_to_personal_id::AccountToPersonalId,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "People",
                        "AccountToPersonalId",
                        ::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(
                            _0.borrow(),
                        ),
                        [
                            236u8, 149u8, 24u8, 190u8, 99u8, 144u8, 149u8, 212u8, 247u8, 172u8,
                            75u8, 9u8, 160u8, 192u8, 10u8, 126u8, 153u8, 186u8, 26u8, 184u8, 25u8,
                            235u8, 143u8, 63u8, 154u8, 170u8, 60u8, 159u8, 75u8, 11u8, 222u8,
                            134u8,
                        ],
                    )
                }
                #[doc = " Paginated collection of static chunks used by the verifiable crypto."]
                pub fn chunks_iter(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::chunks::Chunks,
                    (),
                    (),
                    ::subxt::ext::subxt_core::utils::Yes,
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "People",
                        "Chunks",
                        (),
                        [
                            29u8, 81u8, 81u8, 23u8, 157u8, 34u8, 110u8, 250u8, 88u8, 191u8, 160u8,
                            107u8, 14u8, 104u8, 31u8, 11u8, 218u8, 198u8, 254u8, 254u8, 252u8,
                            24u8, 110u8, 139u8, 100u8, 172u8, 170u8, 61u8, 149u8, 187u8, 180u8,
                            104u8,
                        ],
                    )
                }
                #[doc = " Paginated collection of static chunks used by the verifiable crypto."]
                pub fn chunks(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::chunks::Param0>,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    ::subxt::ext::subxt_core::storage::address::StaticStorageKey<
                        types::chunks::Param0,
                    >,
                    types::chunks::Chunks,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "People",
                        "Chunks",
                        ::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(
                            _0.borrow(),
                        ),
                        [
                            29u8, 81u8, 81u8, 23u8, 157u8, 34u8, 110u8, 250u8, 88u8, 191u8, 160u8,
                            107u8, 14u8, 104u8, 31u8, 11u8, 218u8, 198u8, 254u8, 254u8, 252u8,
                            24u8, 110u8, 139u8, 100u8, 172u8, 170u8, 61u8, 149u8, 187u8, 180u8,
                            104u8,
                        ],
                    )
                }
                #[doc = " The next free and never reserved personal ID."]
                pub fn next_personal_id(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::next_personal_id::NextPersonalId,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "People",
                        "NextPersonalId",
                        (),
                        [
                            118u8, 50u8, 83u8, 177u8, 33u8, 162u8, 0u8, 175u8, 92u8, 96u8, 168u8,
                            137u8, 205u8, 182u8, 198u8, 15u8, 46u8, 101u8, 249u8, 199u8, 178u8,
                            56u8, 187u8, 53u8, 233u8, 148u8, 89u8, 198u8, 134u8, 188u8, 200u8,
                            71u8,
                        ],
                    )
                }
                #[doc = " The state of the pallet regarding the actions that are currently allowed to be performed on"]
                #[doc = " all existing rings."]
                pub fn rings_state(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::rings_state::RingsState,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "People",
                        "RingsState",
                        (),
                        [
                            68u8, 242u8, 167u8, 185u8, 178u8, 0u8, 45u8, 83u8, 60u8, 121u8, 253u8,
                            1u8, 72u8, 27u8, 178u8, 141u8, 1u8, 175u8, 120u8, 231u8, 216u8, 37u8,
                            95u8, 66u8, 90u8, 202u8, 176u8, 7u8, 35u8, 27u8, 170u8, 111u8,
                        ],
                    )
                }
                #[doc = " Candidates' reserved identities which we track."]
                pub fn reserved_personal_id_iter(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::reserved_personal_id::ReservedPersonalId,
                    (),
                    (),
                    ::subxt::ext::subxt_core::utils::Yes,
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "People",
                        "ReservedPersonalId",
                        (),
                        [
                            229u8, 248u8, 49u8, 240u8, 83u8, 23u8, 109u8, 246u8, 77u8, 215u8,
                            221u8, 23u8, 237u8, 180u8, 36u8, 128u8, 217u8, 218u8, 192u8, 159u8,
                            160u8, 149u8, 1u8, 57u8, 99u8, 185u8, 190u8, 123u8, 50u8, 222u8, 33u8,
                            251u8,
                        ],
                    )
                }
                #[doc = " Candidates' reserved identities which we track."]
                pub fn reserved_personal_id(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::reserved_personal_id::Param0>,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    ::subxt::ext::subxt_core::storage::address::StaticStorageKey<
                        types::reserved_personal_id::Param0,
                    >,
                    types::reserved_personal_id::ReservedPersonalId,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "People",
                        "ReservedPersonalId",
                        ::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(
                            _0.borrow(),
                        ),
                        [
                            229u8, 248u8, 49u8, 240u8, 83u8, 23u8, 109u8, 246u8, 77u8, 215u8,
                            221u8, 23u8, 237u8, 180u8, 36u8, 128u8, 217u8, 218u8, 192u8, 159u8,
                            160u8, 149u8, 1u8, 57u8, 99u8, 185u8, 190u8, 123u8, 50u8, 222u8, 33u8,
                            251u8,
                        ],
                    )
                }
                #[doc = " Keeps track of the page indices of the head and tail of the onboarding queue."]
                pub fn queue_page_indices(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::queue_page_indices::QueuePageIndices,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "People",
                        "QueuePageIndices",
                        (),
                        [
                            228u8, 106u8, 36u8, 72u8, 176u8, 161u8, 16u8, 203u8, 254u8, 245u8,
                            203u8, 207u8, 15u8, 10u8, 25u8, 190u8, 166u8, 9u8, 43u8, 88u8, 124u8,
                            135u8, 9u8, 251u8, 110u8, 148u8, 75u8, 255u8, 226u8, 109u8, 74u8,
                            180u8,
                        ],
                    )
                }
                #[doc = " Paginated collection of people public keys ready to be included in a ring."]
                pub fn onboarding_queue_iter(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::onboarding_queue::OnboardingQueue,
                    (),
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "People",
                        "OnboardingQueue",
                        (),
                        [
                            251u8, 126u8, 237u8, 25u8, 3u8, 253u8, 13u8, 186u8, 181u8, 136u8, 55u8,
                            241u8, 158u8, 73u8, 209u8, 79u8, 173u8, 69u8, 237u8, 207u8, 22u8, 25u8,
                            162u8, 131u8, 110u8, 19u8, 138u8, 93u8, 88u8, 177u8, 229u8, 194u8,
                        ],
                    )
                }
                #[doc = " Paginated collection of people public keys ready to be included in a ring."]
                pub fn onboarding_queue(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::onboarding_queue::Param0>,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    ::subxt::ext::subxt_core::storage::address::StaticStorageKey<
                        types::onboarding_queue::Param0,
                    >,
                    types::onboarding_queue::OnboardingQueue,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "People",
                        "OnboardingQueue",
                        ::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(
                            _0.borrow(),
                        ),
                        [
                            251u8, 126u8, 237u8, 25u8, 3u8, 253u8, 13u8, 186u8, 181u8, 136u8, 55u8,
                            241u8, 158u8, 73u8, 209u8, 79u8, 173u8, 69u8, 237u8, 207u8, 22u8, 25u8,
                            162u8, 131u8, 110u8, 19u8, 138u8, 93u8, 88u8, 177u8, 229u8, 194u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                #[doc = " Number of chunks per page."]
                pub fn chunk_page_size(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    ::core::primitive::u32,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "People",
                        "ChunkPageSize",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                #[doc = " Maximum number of people included in a ring before a new one is created."]
                pub fn max_ring_size(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    ::core::primitive::u32,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "People",
                        "MaxRingSize",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                #[doc = " Maximum number of people included in an onboarding queue page before a new one is"]
                #[doc = " created."]
                pub fn onboarding_queue_page_size(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    ::core::primitive::u32,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "People",
                        "OnboardingQueuePageSize",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                #[doc = " The amount of block number tolerance we allow for a setup account transaction."]
                #[doc = ""]
                #[doc = " `set_alias_account` and `set_personal_id_account` calls contains"]
                #[doc = " `call_valid_at` as a parameter, those calls are valid if the block number is within"]
                #[doc = " the tolerance period."]
                pub fn account_setup_time_tolerance(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    ::core::primitive::u32,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "People",
                        "account_setup_time_tolerance",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod runtime_types {
        use super::runtime_types;
        pub mod bounded_collections {
            use super::runtime_types;
            pub mod bounded_vec {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct BoundedVec<_0>(pub ::subxt::ext::subxt_core::alloc::vec::Vec<_0>);
            }
        }
        pub mod cumulus_pallet_parachain_system {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
                pub enum Call {
                    # [codec (index = 0)] # [doc = "Set the current validation data."] # [doc = ""] # [doc = "This should be invoked exactly once per block. It will panic at the finalization"] # [doc = "phase if the call was not invoked."] # [doc = ""] # [doc = "The dispatch origin for this call must be `Inherent`"] # [doc = ""] # [doc = "As a side effect, this function upgrades the current validation function"] # [doc = "if the appropriate time has come."] set_validation_data { data : runtime_types :: cumulus_primitives_parachain_inherent :: ParachainInherentData , } , # [codec (index = 1)] sudo_send_upward_message { message : :: subxt :: ext :: subxt_core :: alloc :: vec :: Vec < :: core :: primitive :: u8 > , } , }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "The `Error` enum of this pallet."]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "Attempt to upgrade validation function while existing upgrade pending."]
                    OverlappingUpgrades,
                    #[codec(index = 1)]
                    #[doc = "Polkadot currently prohibits this parachain from upgrading its validation function."]
                    ProhibitedByPolkadot,
                    #[codec(index = 2)]
                    #[doc = "The supplied validation function has compiled into a blob larger than Polkadot is"]
                    #[doc = "willing to run."]
                    TooBig,
                    #[codec(index = 3)]
                    #[doc = "The inherent which supplies the validation data did not run this block."]
                    ValidationDataNotAvailable,
                    #[codec(index = 4)]
                    #[doc = "The inherent which supplies the host configuration did not run this block."]
                    HostConfigurationNotAvailable,
                    #[codec(index = 5)]
                    #[doc = "No validation function upgrade is currently scheduled."]
                    NotScheduled,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "The `Event` enum of this pallet"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "The validation function has been scheduled to apply."]
                    ValidationFunctionStored,
                    #[codec(index = 1)]
                    #[doc = "The validation function was applied as of the contained relay chain block number."]
                    ValidationFunctionApplied {
                        relay_chain_block_num: ::core::primitive::u32,
                    },
                    #[codec(index = 2)]
                    #[doc = "The relay-chain aborted the upgrade process."]
                    ValidationFunctionDiscarded,
                    #[codec(index = 3)]
                    #[doc = "Some downward messages have been received and will be processed."]
                    DownwardMessagesReceived { count: ::core::primitive::u32 },
                    #[codec(index = 4)]
                    #[doc = "Downward messages were processed using the given weight."]
                    DownwardMessagesProcessed {
                        weight_used: runtime_types::sp_weights::weight_v2::Weight,
                        dmq_head: ::subxt::ext::subxt_core::utils::H256,
                    },
                    #[codec(index = 5)]
                    #[doc = "An upward message was sent to the relay chain."]
                    UpwardMessageSent {
                        message_hash: ::core::option::Option<[::core::primitive::u8; 32usize]>,
                    },
                }
            }
        }
        pub mod cumulus_pallet_weight_reclaim {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[codec(dumb_trait_bound)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct StorageWeightReclaim<_1>(pub _1);
        }
        pub mod cumulus_pallet_xcm {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
                pub enum Call {}
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "The `Event` enum of this pallet"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "Downward message is invalid XCM."]
                    #[doc = "\\[ id \\]"]
                    InvalidFormat([::core::primitive::u8; 32usize]),
                    #[codec(index = 1)]
                    #[doc = "Downward message is unsupported version of XCM."]
                    #[doc = "\\[ id \\]"]
                    UnsupportedVersion([::core::primitive::u8; 32usize]),
                    #[codec(index = 2)]
                    #[doc = "Downward message executed with the given outcome."]
                    #[doc = "\\[ id, outcome \\]"]
                    ExecutedDownward(
                        [::core::primitive::u8; 32usize],
                        runtime_types::staging_xcm::v5::traits::Outcome,
                    ),
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub enum Origin {
                    #[codec(index = 0)]
                    Relay,
                    #[codec(index = 1)]
                    SiblingParachain(runtime_types::polkadot_parachain_primitives::primitives::Id),
                }
            }
        }
        pub mod cumulus_pallet_xcmp_queue {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
                pub enum Call {
                    #[codec(index = 1)]
                    #[doc = "Suspends all XCM executions for the XCMP queue, regardless of the sender's origin."]
                    #[doc = ""]
                    #[doc = "- `origin`: Must pass `ControllerOrigin`."]
                    suspend_xcm_execution,
                    #[codec(index = 2)]
                    #[doc = "Resumes all XCM executions for the XCMP queue."]
                    #[doc = ""]
                    #[doc = "Note that this function doesn't change the status of the in/out bound channels."]
                    #[doc = ""]
                    #[doc = "- `origin`: Must pass `ControllerOrigin`."]
                    resume_xcm_execution,
                    #[codec(index = 3)]
                    #[doc = "Overwrites the number of pages which must be in the queue for the other side to be"]
                    #[doc = "told to suspend their sending."]
                    #[doc = ""]
                    #[doc = "- `origin`: Must pass `Root`."]
                    #[doc = "- `new`: Desired value for `QueueConfigData.suspend_value`"]
                    update_suspend_threshold { new: ::core::primitive::u32 },
                    #[codec(index = 4)]
                    #[doc = "Overwrites the number of pages which must be in the queue after which we drop any"]
                    #[doc = "further messages from the channel."]
                    #[doc = ""]
                    #[doc = "- `origin`: Must pass `Root`."]
                    #[doc = "- `new`: Desired value for `QueueConfigData.drop_threshold`"]
                    update_drop_threshold { new: ::core::primitive::u32 },
                    #[codec(index = 5)]
                    #[doc = "Overwrites the number of pages which the queue must be reduced to before it signals"]
                    #[doc = "that message sending may recommence after it has been suspended."]
                    #[doc = ""]
                    #[doc = "- `origin`: Must pass `Root`."]
                    #[doc = "- `new`: Desired value for `QueueConfigData.resume_threshold`"]
                    update_resume_threshold { new: ::core::primitive::u32 },
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "The `Error` enum of this pallet."]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "Setting the queue config failed since one of its values was invalid."]
                    BadQueueConfig,
                    #[codec(index = 1)]
                    #[doc = "The execution is already suspended."]
                    AlreadySuspended,
                    #[codec(index = 2)]
                    #[doc = "The execution is already resumed."]
                    AlreadyResumed,
                    #[codec(index = 3)]
                    #[doc = "There are too many active outbound channels."]
                    TooManyActiveOutboundChannels,
                    #[codec(index = 4)]
                    #[doc = "The message is too big."]
                    TooBig,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "The `Event` enum of this pallet"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "An HRMP message was sent to a sibling parachain."]
                    XcmpMessageSent {
                        message_hash: [::core::primitive::u8; 32usize],
                    },
                }
            }
        }
        pub mod cumulus_primitives_core {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[codec(dumb_trait_bound)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub enum AggregateMessageOrigin {
                #[codec(index = 0)]
                Here,
                #[codec(index = 1)]
                Parent,
                #[codec(index = 2)]
                Sibling(runtime_types::polkadot_parachain_primitives::primitives::Id),
            }
        }
        pub mod cumulus_primitives_parachain_inherent {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[codec(dumb_trait_bound)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct ParachainInherentData {
                pub validation_data:
                    runtime_types::polkadot_primitives::v8::PersistedValidationData<
                        ::subxt::ext::subxt_core::utils::H256,
                        ::core::primitive::u32,
                    >,
                pub relay_chain_state: runtime_types::sp_trie::storage_proof::StorageProof,
                pub downward_messages: ::subxt::ext::subxt_core::alloc::vec::Vec<
                    runtime_types::polkadot_core_primitives::InboundDownwardMessage<
                        ::core::primitive::u32,
                    >,
                >,
                pub horizontal_messages: ::subxt::ext::subxt_core::utils::KeyedVec<
                    runtime_types::polkadot_parachain_primitives::primitives::Id,
                    ::subxt::ext::subxt_core::alloc::vec::Vec<
                        runtime_types::polkadot_core_primitives::InboundHrmpMessage<
                            ::core::primitive::u32,
                        >,
                    >,
                >,
                pub relay_parent_descendants: ::subxt::ext::subxt_core::alloc::vec::Vec<
                    runtime_types::sp_runtime::generic::header::Header<::core::primitive::u32>,
                >,
                pub collator_peer_id: ::core::option::Option<
                    runtime_types::bounded_collections::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >,
                >,
            }
        }
        pub mod frame_support {
            use super::runtime_types;
            pub mod dispatch {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub enum DispatchClass {
                    #[codec(index = 0)]
                    Normal,
                    #[codec(index = 1)]
                    Operational,
                    #[codec(index = 2)]
                    Mandatory,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub enum Pays {
                    #[codec(index = 0)]
                    Yes,
                    #[codec(index = 1)]
                    No,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub enum RawOrigin<_0> {
                    #[codec(index = 0)]
                    Root,
                    #[codec(index = 1)]
                    Signed(_0),
                    #[codec(index = 2)]
                    None,
                    #[codec(index = 3)]
                    Authorized,
                }
            }
            pub mod traits {
                use super::runtime_types;
                pub mod messages {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                        :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                    #[codec(dumb_trait_bound)]
                    #[decode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                    )]
                    #[encode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                    )]
                    pub enum ProcessMessageError {
                        #[codec(index = 0)]
                        BadFormat,
                        #[codec(index = 1)]
                        Corrupt,
                        #[codec(index = 2)]
                        Unsupported,
                        #[codec(index = 3)]
                        Overweight(runtime_types::sp_weights::weight_v2::Weight),
                        #[codec(index = 4)]
                        Yield,
                        #[codec(index = 5)]
                        StackLimitReached,
                    }
                }
                pub mod tokens {
                    use super::runtime_types;
                    pub mod misc {
                        use super::runtime_types;
                        #[derive(
                            :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                            :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                            :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                            :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                            Debug,
                        )]
                        # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                        #[codec(dumb_trait_bound)]
                        #[decode_as_type(
                            crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                        )]
                        #[encode_as_type(
                            crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                        )]
                        pub enum BalanceStatus {
                            #[codec(index = 0)]
                            Free,
                            #[codec(index = 1)]
                            Reserved,
                        }
                    }
                }
            }
        }
        pub mod frame_system {
            use super::runtime_types;
            pub mod extensions {
                use super::runtime_types;
                pub mod check_genesis {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                        :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                    #[codec(dumb_trait_bound)]
                    #[decode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                    )]
                    #[encode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                    )]
                    pub struct CheckGenesis;
                }
                pub mod check_mortality {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                        :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                    #[codec(dumb_trait_bound)]
                    #[decode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                    )]
                    #[encode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                    )]
                    pub struct CheckMortality(pub runtime_types::sp_runtime::generic::era::Era);
                }
                pub mod check_non_zero_sender {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                        :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                    #[codec(dumb_trait_bound)]
                    #[decode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                    )]
                    #[encode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                    )]
                    pub struct CheckNonZeroSender;
                }
                pub mod check_nonce {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                        :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                    #[codec(dumb_trait_bound)]
                    #[decode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                    )]
                    #[encode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                    )]
                    pub struct CheckNonce(#[codec(compact)] pub ::core::primitive::u32);
                }
                pub mod check_spec_version {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                        :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                    #[codec(dumb_trait_bound)]
                    #[decode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                    )]
                    #[encode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                    )]
                    pub struct CheckSpecVersion;
                }
                pub mod check_tx_version {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                        :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                    #[codec(dumb_trait_bound)]
                    #[decode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                    )]
                    #[encode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                    )]
                    pub struct CheckTxVersion;
                }
                pub mod check_weight {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                        :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                    #[codec(dumb_trait_bound)]
                    #[decode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                    )]
                    #[encode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                    )]
                    pub struct CheckWeight;
                }
            }
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Make some on-chain remark."]
                    #[doc = ""]
                    #[doc = "Can be executed by every `origin`."]
                    remark {
                        remark: ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 1)]
                    #[doc = "Set the number of pages in the WebAssembly environment's heap."]
                    set_heap_pages { pages: ::core::primitive::u64 },
                    #[codec(index = 2)]
                    #[doc = "Set the new runtime code."]
                    set_code {
                        code: ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 3)]
                    #[doc = "Set the new runtime code without doing any checks of the given `code`."]
                    #[doc = ""]
                    #[doc = "Note that runtime upgrades will not run if this is called with a not-increasing spec"]
                    #[doc = "version!"]
                    set_code_without_checks {
                        code: ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 4)]
                    #[doc = "Set some items of storage."]
                    set_storage {
                        items: ::subxt::ext::subxt_core::alloc::vec::Vec<(
                            ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                            ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                        )>,
                    },
                    #[codec(index = 5)]
                    #[doc = "Kill some items from storage."]
                    kill_storage {
                        keys: ::subxt::ext::subxt_core::alloc::vec::Vec<
                            ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                        >,
                    },
                    #[codec(index = 6)]
                    #[doc = "Kill all storage items with a key that starts with the given prefix."]
                    #[doc = ""]
                    #[doc = "**NOTE:** We rely on the Root origin to provide us the number of subkeys under"]
                    #[doc = "the prefix we are removing to accurately calculate the weight of this function."]
                    kill_prefix {
                        prefix: ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                        subkeys: ::core::primitive::u32,
                    },
                    #[codec(index = 7)]
                    #[doc = "Make some on-chain remark and emit event."]
                    remark_with_event {
                        remark: ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 9)]
                    #[doc = "Authorize an upgrade to a given `code_hash` for the runtime. The runtime can be supplied"]
                    #[doc = "later."]
                    #[doc = ""]
                    #[doc = "This call requires Root origin."]
                    authorize_upgrade {
                        code_hash: ::subxt::ext::subxt_core::utils::H256,
                    },
                    #[codec(index = 10)]
                    #[doc = "Authorize an upgrade to a given `code_hash` for the runtime. The runtime can be supplied"]
                    #[doc = "later."]
                    #[doc = ""]
                    #[doc = "WARNING: This authorizes an upgrade that will take place without any safety checks, for"]
                    #[doc = "example that the spec name remains the same and that the version number increases. Not"]
                    #[doc = "recommended for normal use. Use `authorize_upgrade` instead."]
                    #[doc = ""]
                    #[doc = "This call requires Root origin."]
                    authorize_upgrade_without_checks {
                        code_hash: ::subxt::ext::subxt_core::utils::H256,
                    },
                    #[codec(index = 11)]
                    #[doc = "Provide the preimage (runtime binary) `code` for an upgrade that has been authorized."]
                    #[doc = ""]
                    #[doc = "If the authorization required a version check, this call will ensure the spec name"]
                    #[doc = "remains unchanged and that the spec version has increased."]
                    #[doc = ""]
                    #[doc = "Depending on the runtime's `OnSetCode` configuration, this function may directly apply"]
                    #[doc = "the new `code` in the same block or attempt to schedule the upgrade."]
                    #[doc = ""]
                    #[doc = "All origins are allowed."]
                    apply_authorized_upgrade {
                        code: ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                    },
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Error for the System pallet"]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "The name of specification does not match between the current runtime"]
                    #[doc = "and the new runtime."]
                    InvalidSpecName,
                    #[codec(index = 1)]
                    #[doc = "The specification version is not allowed to decrease between the current runtime"]
                    #[doc = "and the new runtime."]
                    SpecVersionNeedsToIncrease,
                    #[codec(index = 2)]
                    #[doc = "Failed to extract the runtime version from the new runtime."]
                    #[doc = ""]
                    #[doc = "Either calling `Core_version` or decoding `RuntimeVersion` failed."]
                    FailedToExtractRuntimeVersion,
                    #[codec(index = 3)]
                    #[doc = "Suicide called when the account has non-default composite data."]
                    NonDefaultComposite,
                    #[codec(index = 4)]
                    #[doc = "There is a non-zero reference count preventing the account from being purged."]
                    NonZeroRefCount,
                    #[codec(index = 5)]
                    #[doc = "The origin filter prevent the call to be dispatched."]
                    CallFiltered,
                    #[codec(index = 6)]
                    #[doc = "A multi-block migration is ongoing and prevents the current code from being replaced."]
                    MultiBlockMigrationsOngoing,
                    #[codec(index = 7)]
                    #[doc = "No upgrade authorized."]
                    NothingAuthorized,
                    #[codec(index = 8)]
                    #[doc = "The submitted code is not authorized."]
                    Unauthorized,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Event for the System pallet."]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "An extrinsic completed successfully."]
                    ExtrinsicSuccess {
                        dispatch_info: runtime_types::frame_system::DispatchEventInfo,
                    },
                    #[codec(index = 1)]
                    #[doc = "An extrinsic failed."]
                    ExtrinsicFailed {
                        dispatch_error: runtime_types::sp_runtime::DispatchError,
                        dispatch_info: runtime_types::frame_system::DispatchEventInfo,
                    },
                    #[codec(index = 2)]
                    #[doc = "`:code` was updated."]
                    CodeUpdated,
                    #[codec(index = 3)]
                    #[doc = "A new account was created."]
                    NewAccount {
                        account: ::subxt::ext::subxt_core::utils::AccountId32,
                    },
                    #[codec(index = 4)]
                    #[doc = "An account was reaped."]
                    KilledAccount {
                        account: ::subxt::ext::subxt_core::utils::AccountId32,
                    },
                    #[codec(index = 5)]
                    #[doc = "On on-chain remark happened."]
                    Remarked {
                        sender: ::subxt::ext::subxt_core::utils::AccountId32,
                        hash: ::subxt::ext::subxt_core::utils::H256,
                    },
                    #[codec(index = 6)]
                    #[doc = "An upgrade was authorized."]
                    UpgradeAuthorized {
                        code_hash: ::subxt::ext::subxt_core::utils::H256,
                        check_version: ::core::primitive::bool,
                    },
                    #[codec(index = 7)]
                    #[doc = "An invalid authorized upgrade was rejected while trying to apply it."]
                    RejectedInvalidAuthorizedUpgrade {
                        code_hash: ::subxt::ext::subxt_core::utils::H256,
                        error: runtime_types::sp_runtime::DispatchError,
                    },
                }
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[codec(dumb_trait_bound)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct DispatchEventInfo {
                pub weight: runtime_types::sp_weights::weight_v2::Weight,
                pub class: runtime_types::frame_support::dispatch::DispatchClass,
                pub pays_fee: runtime_types::frame_support::dispatch::Pays,
            }
        }
        pub mod individuality_support {
            use super::runtime_types;
            pub mod traits {
                use super::runtime_types;
                pub mod reality {
                    use super::runtime_types;
                    pub mod identity {
                        use super::runtime_types;
                        #[derive(
                            :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                            :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                            :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                            :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                            Debug,
                        )]
                        # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                        #[codec(dumb_trait_bound)]
                        #[decode_as_type(
                            crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                        )]
                        #[encode_as_type(
                            crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                        )]
                        pub enum Social {
                            #[codec(index = 0)]
                            Twitter {
                                username:
                                    runtime_types::bounded_collections::bounded_vec::BoundedVec<
                                        ::core::primitive::u8,
                                    >,
                            },
                            #[codec(index = 1)]
                            Github {
                                username:
                                    runtime_types::bounded_collections::bounded_vec::BoundedVec<
                                        ::core::primitive::u8,
                                    >,
                            },
                            #[codec(index = 2)]
                            Discord {
                                display_and_tag:
                                    runtime_types::bounded_collections::bounded_vec::BoundedVec<
                                        ::core::primitive::u8,
                                    >,
                            },
                        }
                    }
                    #[derive(
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                        :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                    #[codec(dumb_trait_bound)]
                    #[decode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                    )]
                    #[encode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                    )]
                    pub struct ContextualAlias {
                        pub alias: [::core::primitive::u8; 32usize],
                        pub context: [::core::primitive::u8; 32usize],
                    }
                    #[derive(
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                        :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                    #[codec(dumb_trait_bound)]
                    #[decode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                    )]
                    #[encode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                    )]
                    pub enum Judgement {
                        #[codec(index = 0)]
                        Truth(runtime_types::individuality_support::traits::reality::Truth),
                        #[codec(index = 1)]
                        Contempt,
                    }
                    #[derive(
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                        :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                    #[codec(dumb_trait_bound)]
                    #[decode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                    )]
                    #[encode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                    )]
                    pub enum Truth {
                        #[codec(index = 0)]
                        True,
                        #[codec(index = 1)]
                        False,
                    }
                }
            }
        }
        pub mod pallet_assets {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Issue a new class of fungible assets from a public origin."]
                    #[doc = ""]
                    #[doc = "This new asset class has no assets initially and its owner is the origin."]
                    #[doc = ""]
                    #[doc = "The origin must conform to the configured `CreateOrigin` and have sufficient funds free."]
                    #[doc = ""]
                    #[doc = "Funds of sender are reserved by `AssetDeposit`."]
                    #[doc = ""]
                    #[doc = "Parameters:"]
                    #[doc = "- `id`: The identifier of the new asset. This must not be currently in use to identify"]
                    #[doc = "an existing asset. If [`NextAssetId`] is set, then this must be equal to it."]
                    #[doc = "- `admin`: The admin of this class of assets. The admin is the initial address of each"]
                    #[doc = "member of the asset class's admin team."]
                    #[doc = "- `min_balance`: The minimum balance of this new asset that any single account must"]
                    #[doc = "have. If an account's balance is reduced below this, then it collapses to zero."]
                    #[doc = ""]
                    #[doc = "Emits `Created` event when successful."]
                    #[doc = ""]
                    #[doc = "Weight: `O(1)`"]
                    create {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        admin: ::subxt::ext::subxt_core::utils::MultiAddress<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            (),
                        >,
                        min_balance: ::core::primitive::u128,
                    },
                    #[codec(index = 1)]
                    #[doc = "Issue a new class of fungible assets from a privileged origin."]
                    #[doc = ""]
                    #[doc = "This new asset class has no assets initially."]
                    #[doc = ""]
                    #[doc = "The origin must conform to `ForceOrigin`."]
                    #[doc = ""]
                    #[doc = "Unlike `create`, no funds are reserved."]
                    #[doc = ""]
                    #[doc = "- `id`: The identifier of the new asset. This must not be currently in use to identify"]
                    #[doc = "an existing asset. If [`NextAssetId`] is set, then this must be equal to it."]
                    #[doc = "- `owner`: The owner of this class of assets. The owner has full superuser permissions"]
                    #[doc = "over this asset, but may later change and configure the permissions using"]
                    #[doc = "`transfer_ownership` and `set_team`."]
                    #[doc = "- `min_balance`: The minimum balance of this new asset that any single account must"]
                    #[doc = "have. If an account's balance is reduced below this, then it collapses to zero."]
                    #[doc = ""]
                    #[doc = "Emits `ForceCreated` event when successful."]
                    #[doc = ""]
                    #[doc = "Weight: `O(1)`"]
                    force_create {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        owner: ::subxt::ext::subxt_core::utils::MultiAddress<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            (),
                        >,
                        is_sufficient: ::core::primitive::bool,
                        #[codec(compact)]
                        min_balance: ::core::primitive::u128,
                    },
                    #[codec(index = 2)]
                    #[doc = "Start the process of destroying a fungible asset class."]
                    #[doc = ""]
                    #[doc = "`start_destroy` is the first in a series of extrinsics that should be called, to allow"]
                    #[doc = "destruction of an asset class."]
                    #[doc = ""]
                    #[doc = "The origin must conform to `ForceOrigin` or must be `Signed` by the asset's `owner`."]
                    #[doc = ""]
                    #[doc = "- `id`: The identifier of the asset to be destroyed. This must identify an existing"]
                    #[doc = "  asset."]
                    #[doc = ""]
                    #[doc = "It will fail with either [`Error::ContainsHolds`] or [`Error::ContainsFreezes`] if"]
                    #[doc = "an account contains holds or freezes in place."]
                    start_destroy {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                    },
                    #[codec(index = 3)]
                    #[doc = "Destroy all accounts associated with a given asset."]
                    #[doc = ""]
                    #[doc = "`destroy_accounts` should only be called after `start_destroy` has been called, and the"]
                    #[doc = "asset is in a `Destroying` state."]
                    #[doc = ""]
                    #[doc = "Due to weight restrictions, this function may need to be called multiple times to fully"]
                    #[doc = "destroy all accounts. It will destroy `RemoveItemsLimit` accounts at a time."]
                    #[doc = ""]
                    #[doc = "- `id`: The identifier of the asset to be destroyed. This must identify an existing"]
                    #[doc = "  asset."]
                    #[doc = ""]
                    #[doc = "Each call emits the `Event::DestroyedAccounts` event."]
                    destroy_accounts {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                    },
                    #[codec(index = 4)]
                    #[doc = "Destroy all approvals associated with a given asset up to the max (T::RemoveItemsLimit)."]
                    #[doc = ""]
                    #[doc = "`destroy_approvals` should only be called after `start_destroy` has been called, and the"]
                    #[doc = "asset is in a `Destroying` state."]
                    #[doc = ""]
                    #[doc = "Due to weight restrictions, this function may need to be called multiple times to fully"]
                    #[doc = "destroy all approvals. It will destroy `RemoveItemsLimit` approvals at a time."]
                    #[doc = ""]
                    #[doc = "- `id`: The identifier of the asset to be destroyed. This must identify an existing"]
                    #[doc = "  asset."]
                    #[doc = ""]
                    #[doc = "Each call emits the `Event::DestroyedApprovals` event."]
                    destroy_approvals {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                    },
                    #[codec(index = 5)]
                    #[doc = "Complete destroying asset and unreserve currency."]
                    #[doc = ""]
                    #[doc = "`finish_destroy` should only be called after `start_destroy` has been called, and the"]
                    #[doc = "asset is in a `Destroying` state. All accounts or approvals should be destroyed before"]
                    #[doc = "hand."]
                    #[doc = ""]
                    #[doc = "- `id`: The identifier of the asset to be destroyed. This must identify an existing"]
                    #[doc = "  asset."]
                    #[doc = ""]
                    #[doc = "Each successful call emits the `Event::Destroyed` event."]
                    finish_destroy {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                    },
                    #[codec(index = 6)]
                    #[doc = "Mint assets of a particular class."]
                    #[doc = ""]
                    #[doc = "The origin must be Signed and the sender must be the Issuer of the asset `id`."]
                    #[doc = ""]
                    #[doc = "- `id`: The identifier of the asset to have some amount minted."]
                    #[doc = "- `beneficiary`: The account to be credited with the minted assets."]
                    #[doc = "- `amount`: The amount of the asset to be minted."]
                    #[doc = ""]
                    #[doc = "Emits `Issued` event when successful."]
                    #[doc = ""]
                    #[doc = "Weight: `O(1)`"]
                    #[doc = "Modes: Pre-existing balance of `beneficiary`; Account pre-existence of `beneficiary`."]
                    mint {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        beneficiary: ::subxt::ext::subxt_core::utils::MultiAddress<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            (),
                        >,
                        #[codec(compact)]
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 7)]
                    #[doc = "Reduce the balance of `who` by as much as possible up to `amount` assets of `id`."]
                    #[doc = ""]
                    #[doc = "Origin must be Signed and the sender should be the Manager of the asset `id`."]
                    #[doc = ""]
                    #[doc = "Bails with `NoAccount` if the `who` is already dead."]
                    #[doc = ""]
                    #[doc = "- `id`: The identifier of the asset to have some amount burned."]
                    #[doc = "- `who`: The account to be debited from."]
                    #[doc = "- `amount`: The maximum amount by which `who`'s balance should be reduced."]
                    #[doc = ""]
                    #[doc = "Emits `Burned` with the actual amount burned. If this takes the balance to below the"]
                    #[doc = "minimum for the asset, then the amount burned is increased to take it to zero."]
                    #[doc = ""]
                    #[doc = "Weight: `O(1)`"]
                    #[doc = "Modes: Post-existence of `who`; Pre & post Zombie-status of `who`."]
                    burn {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        who: ::subxt::ext::subxt_core::utils::MultiAddress<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            (),
                        >,
                        #[codec(compact)]
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 8)]
                    #[doc = "Move some assets from the sender account to another."]
                    #[doc = ""]
                    #[doc = "Origin must be Signed."]
                    #[doc = ""]
                    #[doc = "- `id`: The identifier of the asset to have some amount transferred."]
                    #[doc = "- `target`: The account to be credited."]
                    #[doc = "- `amount`: The amount by which the sender's balance of assets should be reduced and"]
                    #[doc = "`target`'s balance increased. The amount actually transferred may be slightly greater in"]
                    #[doc = "the case that the transfer would otherwise take the sender balance above zero but below"]
                    #[doc = "the minimum balance. Must be greater than zero."]
                    #[doc = ""]
                    #[doc = "Emits `Transferred` with the actual amount transferred. If this takes the source balance"]
                    #[doc = "to below the minimum for the asset, then the amount transferred is increased to take it"]
                    #[doc = "to zero."]
                    #[doc = ""]
                    #[doc = "Weight: `O(1)`"]
                    #[doc = "Modes: Pre-existence of `target`; Post-existence of sender; Account pre-existence of"]
                    #[doc = "`target`."]
                    transfer {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        target: ::subxt::ext::subxt_core::utils::MultiAddress<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            (),
                        >,
                        #[codec(compact)]
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 9)]
                    #[doc = "Move some assets from the sender account to another, keeping the sender account alive."]
                    #[doc = ""]
                    #[doc = "Origin must be Signed."]
                    #[doc = ""]
                    #[doc = "- `id`: The identifier of the asset to have some amount transferred."]
                    #[doc = "- `target`: The account to be credited."]
                    #[doc = "- `amount`: The amount by which the sender's balance of assets should be reduced and"]
                    #[doc = "`target`'s balance increased. The amount actually transferred may be slightly greater in"]
                    #[doc = "the case that the transfer would otherwise take the sender balance above zero but below"]
                    #[doc = "the minimum balance. Must be greater than zero."]
                    #[doc = ""]
                    #[doc = "Emits `Transferred` with the actual amount transferred. If this takes the source balance"]
                    #[doc = "to below the minimum for the asset, then the amount transferred is increased to take it"]
                    #[doc = "to zero."]
                    #[doc = ""]
                    #[doc = "Weight: `O(1)`"]
                    #[doc = "Modes: Pre-existence of `target`; Post-existence of sender; Account pre-existence of"]
                    #[doc = "`target`."]
                    transfer_keep_alive {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        target: ::subxt::ext::subxt_core::utils::MultiAddress<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            (),
                        >,
                        #[codec(compact)]
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 10)]
                    #[doc = "Move some assets from one account to another."]
                    #[doc = ""]
                    #[doc = "Origin must be Signed and the sender should be the Admin of the asset `id`."]
                    #[doc = ""]
                    #[doc = "- `id`: The identifier of the asset to have some amount transferred."]
                    #[doc = "- `source`: The account to be debited."]
                    #[doc = "- `dest`: The account to be credited."]
                    #[doc = "- `amount`: The amount by which the `source`'s balance of assets should be reduced and"]
                    #[doc = "`dest`'s balance increased. The amount actually transferred may be slightly greater in"]
                    #[doc = "the case that the transfer would otherwise take the `source` balance above zero but"]
                    #[doc = "below the minimum balance. Must be greater than zero."]
                    #[doc = ""]
                    #[doc = "Emits `Transferred` with the actual amount transferred. If this takes the source balance"]
                    #[doc = "to below the minimum for the asset, then the amount transferred is increased to take it"]
                    #[doc = "to zero."]
                    #[doc = ""]
                    #[doc = "Weight: `O(1)`"]
                    #[doc = "Modes: Pre-existence of `dest`; Post-existence of `source`; Account pre-existence of"]
                    #[doc = "`dest`."]
                    force_transfer {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        source: ::subxt::ext::subxt_core::utils::MultiAddress<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            (),
                        >,
                        dest: ::subxt::ext::subxt_core::utils::MultiAddress<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            (),
                        >,
                        #[codec(compact)]
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 11)]
                    #[doc = "Disallow further unprivileged transfers of an asset `id` from an account `who`. `who`"]
                    #[doc = "must already exist as an entry in `Account`s of the asset. If you want to freeze an"]
                    #[doc = "account that does not have an entry, use `touch_other` first."]
                    #[doc = ""]
                    #[doc = "Origin must be Signed and the sender should be the Freezer of the asset `id`."]
                    #[doc = ""]
                    #[doc = "- `id`: The identifier of the asset to be frozen."]
                    #[doc = "- `who`: The account to be frozen."]
                    #[doc = ""]
                    #[doc = "Emits `Frozen`."]
                    #[doc = ""]
                    #[doc = "Weight: `O(1)`"]
                    freeze {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        who: ::subxt::ext::subxt_core::utils::MultiAddress<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            (),
                        >,
                    },
                    #[codec(index = 12)]
                    #[doc = "Allow unprivileged transfers to and from an account again."]
                    #[doc = ""]
                    #[doc = "Origin must be Signed and the sender should be the Admin of the asset `id`."]
                    #[doc = ""]
                    #[doc = "- `id`: The identifier of the asset to be frozen."]
                    #[doc = "- `who`: The account to be unfrozen."]
                    #[doc = ""]
                    #[doc = "Emits `Thawed`."]
                    #[doc = ""]
                    #[doc = "Weight: `O(1)`"]
                    thaw {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        who: ::subxt::ext::subxt_core::utils::MultiAddress<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            (),
                        >,
                    },
                    #[codec(index = 13)]
                    #[doc = "Disallow further unprivileged transfers for the asset class."]
                    #[doc = ""]
                    #[doc = "Origin must be Signed and the sender should be the Freezer of the asset `id`."]
                    #[doc = ""]
                    #[doc = "- `id`: The identifier of the asset to be frozen."]
                    #[doc = ""]
                    #[doc = "Emits `Frozen`."]
                    #[doc = ""]
                    #[doc = "Weight: `O(1)`"]
                    freeze_asset {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                    },
                    #[codec(index = 14)]
                    #[doc = "Allow unprivileged transfers for the asset again."]
                    #[doc = ""]
                    #[doc = "Origin must be Signed and the sender should be the Admin of the asset `id`."]
                    #[doc = ""]
                    #[doc = "- `id`: The identifier of the asset to be thawed."]
                    #[doc = ""]
                    #[doc = "Emits `Thawed`."]
                    #[doc = ""]
                    #[doc = "Weight: `O(1)`"]
                    thaw_asset {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                    },
                    #[codec(index = 15)]
                    #[doc = "Change the Owner of an asset."]
                    #[doc = ""]
                    #[doc = "Origin must be Signed and the sender should be the Owner of the asset `id`."]
                    #[doc = ""]
                    #[doc = "- `id`: The identifier of the asset."]
                    #[doc = "- `owner`: The new Owner of this asset."]
                    #[doc = ""]
                    #[doc = "Emits `OwnerChanged`."]
                    #[doc = ""]
                    #[doc = "Weight: `O(1)`"]
                    transfer_ownership {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        owner: ::subxt::ext::subxt_core::utils::MultiAddress<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            (),
                        >,
                    },
                    #[codec(index = 16)]
                    #[doc = "Change the Issuer, Admin and Freezer of an asset."]
                    #[doc = ""]
                    #[doc = "Origin must be Signed and the sender should be the Owner of the asset `id`."]
                    #[doc = ""]
                    #[doc = "- `id`: The identifier of the asset to be frozen."]
                    #[doc = "- `issuer`: The new Issuer of this asset."]
                    #[doc = "- `admin`: The new Admin of this asset."]
                    #[doc = "- `freezer`: The new Freezer of this asset."]
                    #[doc = ""]
                    #[doc = "Emits `TeamChanged`."]
                    #[doc = ""]
                    #[doc = "Weight: `O(1)`"]
                    set_team {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        issuer: ::subxt::ext::subxt_core::utils::MultiAddress<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            (),
                        >,
                        admin: ::subxt::ext::subxt_core::utils::MultiAddress<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            (),
                        >,
                        freezer: ::subxt::ext::subxt_core::utils::MultiAddress<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            (),
                        >,
                    },
                    #[codec(index = 17)]
                    #[doc = "Set the metadata for an asset."]
                    #[doc = ""]
                    #[doc = "Origin must be Signed and the sender should be the Owner of the asset `id`."]
                    #[doc = ""]
                    #[doc = "Funds of sender are reserved according to the formula:"]
                    #[doc = "`MetadataDepositBase + MetadataDepositPerByte * (name.len + symbol.len)` taking into"]
                    #[doc = "account any already reserved funds."]
                    #[doc = ""]
                    #[doc = "- `id`: The identifier of the asset to update."]
                    #[doc = "- `name`: The user friendly name of this asset. Limited in length by `StringLimit`."]
                    #[doc = "- `symbol`: The exchange symbol for this asset. Limited in length by `StringLimit`."]
                    #[doc = "- `decimals`: The number of decimals this asset uses to represent one unit."]
                    #[doc = ""]
                    #[doc = "Emits `MetadataSet`."]
                    #[doc = ""]
                    #[doc = "Weight: `O(1)`"]
                    set_metadata {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        name: ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                        symbol: ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                        decimals: ::core::primitive::u8,
                    },
                    #[codec(index = 18)]
                    #[doc = "Clear the metadata for an asset."]
                    #[doc = ""]
                    #[doc = "Origin must be Signed and the sender should be the Owner of the asset `id`."]
                    #[doc = ""]
                    #[doc = "Any deposit is freed for the asset owner."]
                    #[doc = ""]
                    #[doc = "- `id`: The identifier of the asset to clear."]
                    #[doc = ""]
                    #[doc = "Emits `MetadataCleared`."]
                    #[doc = ""]
                    #[doc = "Weight: `O(1)`"]
                    clear_metadata {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                    },
                    #[codec(index = 19)]
                    #[doc = "Force the metadata for an asset to some value."]
                    #[doc = ""]
                    #[doc = "Origin must be ForceOrigin."]
                    #[doc = ""]
                    #[doc = "Any deposit is left alone."]
                    #[doc = ""]
                    #[doc = "- `id`: The identifier of the asset to update."]
                    #[doc = "- `name`: The user friendly name of this asset. Limited in length by `StringLimit`."]
                    #[doc = "- `symbol`: The exchange symbol for this asset. Limited in length by `StringLimit`."]
                    #[doc = "- `decimals`: The number of decimals this asset uses to represent one unit."]
                    #[doc = ""]
                    #[doc = "Emits `MetadataSet`."]
                    #[doc = ""]
                    #[doc = "Weight: `O(N + S)` where N and S are the length of the name and symbol respectively."]
                    force_set_metadata {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        name: ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                        symbol: ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                        decimals: ::core::primitive::u8,
                        is_frozen: ::core::primitive::bool,
                    },
                    #[codec(index = 20)]
                    #[doc = "Clear the metadata for an asset."]
                    #[doc = ""]
                    #[doc = "Origin must be ForceOrigin."]
                    #[doc = ""]
                    #[doc = "Any deposit is returned."]
                    #[doc = ""]
                    #[doc = "- `id`: The identifier of the asset to clear."]
                    #[doc = ""]
                    #[doc = "Emits `MetadataCleared`."]
                    #[doc = ""]
                    #[doc = "Weight: `O(1)`"]
                    force_clear_metadata {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                    },
                    #[codec(index = 21)]
                    #[doc = "Alter the attributes of a given asset."]
                    #[doc = ""]
                    #[doc = "Origin must be `ForceOrigin`."]
                    #[doc = ""]
                    #[doc = "- `id`: The identifier of the asset."]
                    #[doc = "- `owner`: The new Owner of this asset."]
                    #[doc = "- `issuer`: The new Issuer of this asset."]
                    #[doc = "- `admin`: The new Admin of this asset."]
                    #[doc = "- `freezer`: The new Freezer of this asset."]
                    #[doc = "- `min_balance`: The minimum balance of this new asset that any single account must"]
                    #[doc = "have. If an account's balance is reduced below this, then it collapses to zero."]
                    #[doc = "- `is_sufficient`: Whether a non-zero balance of this asset is deposit of sufficient"]
                    #[doc = "value to account for the state bloat associated with its balance storage. If set to"]
                    #[doc = "`true`, then non-zero balances may be stored without a `consumer` reference (and thus"]
                    #[doc = "an ED in the Balances pallet or whatever else is used to control user-account state"]
                    #[doc = "growth)."]
                    #[doc = "- `is_frozen`: Whether this asset class is frozen except for permissioned/admin"]
                    #[doc = "instructions."]
                    #[doc = ""]
                    #[doc = "Emits `AssetStatusChanged` with the identity of the asset."]
                    #[doc = ""]
                    #[doc = "Weight: `O(1)`"]
                    force_asset_status {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        owner: ::subxt::ext::subxt_core::utils::MultiAddress<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            (),
                        >,
                        issuer: ::subxt::ext::subxt_core::utils::MultiAddress<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            (),
                        >,
                        admin: ::subxt::ext::subxt_core::utils::MultiAddress<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            (),
                        >,
                        freezer: ::subxt::ext::subxt_core::utils::MultiAddress<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            (),
                        >,
                        #[codec(compact)]
                        min_balance: ::core::primitive::u128,
                        is_sufficient: ::core::primitive::bool,
                        is_frozen: ::core::primitive::bool,
                    },
                    #[codec(index = 22)]
                    #[doc = "Approve an amount of asset for transfer by a delegated third-party account."]
                    #[doc = ""]
                    #[doc = "Origin must be Signed."]
                    #[doc = ""]
                    #[doc = "Ensures that `ApprovalDeposit` worth of `Currency` is reserved from signing account"]
                    #[doc = "for the purpose of holding the approval. If some non-zero amount of assets is already"]
                    #[doc = "approved from signing account to `delegate`, then it is topped up or unreserved to"]
                    #[doc = "meet the right value."]
                    #[doc = ""]
                    #[doc = "NOTE: The signing account does not need to own `amount` of assets at the point of"]
                    #[doc = "making this call."]
                    #[doc = ""]
                    #[doc = "- `id`: The identifier of the asset."]
                    #[doc = "- `delegate`: The account to delegate permission to transfer asset."]
                    #[doc = "- `amount`: The amount of asset that may be transferred by `delegate`. If there is"]
                    #[doc = "already an approval in place, then this acts additively."]
                    #[doc = ""]
                    #[doc = "Emits `ApprovedTransfer` on success."]
                    #[doc = ""]
                    #[doc = "Weight: `O(1)`"]
                    approve_transfer {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        delegate: ::subxt::ext::subxt_core::utils::MultiAddress<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            (),
                        >,
                        #[codec(compact)]
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 23)]
                    #[doc = "Cancel all of some asset approved for delegated transfer by a third-party account."]
                    #[doc = ""]
                    #[doc = "Origin must be Signed and there must be an approval in place between signer and"]
                    #[doc = "`delegate`."]
                    #[doc = ""]
                    #[doc = "Unreserves any deposit previously reserved by `approve_transfer` for the approval."]
                    #[doc = ""]
                    #[doc = "- `id`: The identifier of the asset."]
                    #[doc = "- `delegate`: The account delegated permission to transfer asset."]
                    #[doc = ""]
                    #[doc = "Emits `ApprovalCancelled` on success."]
                    #[doc = ""]
                    #[doc = "Weight: `O(1)`"]
                    cancel_approval {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        delegate: ::subxt::ext::subxt_core::utils::MultiAddress<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            (),
                        >,
                    },
                    #[codec(index = 24)]
                    #[doc = "Cancel all of some asset approved for delegated transfer by a third-party account."]
                    #[doc = ""]
                    #[doc = "Origin must be either ForceOrigin or Signed origin with the signer being the Admin"]
                    #[doc = "account of the asset `id`."]
                    #[doc = ""]
                    #[doc = "Unreserves any deposit previously reserved by `approve_transfer` for the approval."]
                    #[doc = ""]
                    #[doc = "- `id`: The identifier of the asset."]
                    #[doc = "- `delegate`: The account delegated permission to transfer asset."]
                    #[doc = ""]
                    #[doc = "Emits `ApprovalCancelled` on success."]
                    #[doc = ""]
                    #[doc = "Weight: `O(1)`"]
                    force_cancel_approval {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        owner: ::subxt::ext::subxt_core::utils::MultiAddress<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            (),
                        >,
                        delegate: ::subxt::ext::subxt_core::utils::MultiAddress<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            (),
                        >,
                    },
                    #[codec(index = 25)]
                    #[doc = "Transfer some asset balance from a previously delegated account to some third-party"]
                    #[doc = "account."]
                    #[doc = ""]
                    #[doc = "Origin must be Signed and there must be an approval in place by the `owner` to the"]
                    #[doc = "signer."]
                    #[doc = ""]
                    #[doc = "If the entire amount approved for transfer is transferred, then any deposit previously"]
                    #[doc = "reserved by `approve_transfer` is unreserved."]
                    #[doc = ""]
                    #[doc = "- `id`: The identifier of the asset."]
                    #[doc = "- `owner`: The account which previously approved for a transfer of at least `amount` and"]
                    #[doc = "from which the asset balance will be withdrawn."]
                    #[doc = "- `destination`: The account to which the asset balance of `amount` will be transferred."]
                    #[doc = "- `amount`: The amount of assets to transfer."]
                    #[doc = ""]
                    #[doc = "Emits `TransferredApproved` on success."]
                    #[doc = ""]
                    #[doc = "Weight: `O(1)`"]
                    transfer_approved {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        owner: ::subxt::ext::subxt_core::utils::MultiAddress<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            (),
                        >,
                        destination: ::subxt::ext::subxt_core::utils::MultiAddress<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            (),
                        >,
                        #[codec(compact)]
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 26)]
                    #[doc = "Create an asset account for non-provider assets."]
                    #[doc = ""]
                    #[doc = "A deposit will be taken from the signer account."]
                    #[doc = ""]
                    #[doc = "- `origin`: Must be Signed; the signer account must have sufficient funds for a deposit"]
                    #[doc = "  to be taken."]
                    #[doc = "- `id`: The identifier of the asset for the account to be created."]
                    #[doc = ""]
                    #[doc = "Emits `Touched` event when successful."]
                    touch {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                    },
                    #[codec(index = 27)]
                    #[doc = "Return the deposit (if any) of an asset account or a consumer reference (if any) of an"]
                    #[doc = "account."]
                    #[doc = ""]
                    #[doc = "The origin must be Signed."]
                    #[doc = ""]
                    #[doc = "- `id`: The identifier of the asset for which the caller would like the deposit"]
                    #[doc = "  refunded."]
                    #[doc = "- `allow_burn`: If `true` then assets may be destroyed in order to complete the refund."]
                    #[doc = ""]
                    #[doc = "It will fail with either [`Error::ContainsHolds`] or [`Error::ContainsFreezes`] if"]
                    #[doc = "the asset account contains holds or freezes in place."]
                    #[doc = ""]
                    #[doc = "Emits `Refunded` event when successful."]
                    refund {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        allow_burn: ::core::primitive::bool,
                    },
                    #[codec(index = 28)]
                    #[doc = "Sets the minimum balance of an asset."]
                    #[doc = ""]
                    #[doc = "Only works if there aren't any accounts that are holding the asset or if"]
                    #[doc = "the new value of `min_balance` is less than the old one."]
                    #[doc = ""]
                    #[doc = "Origin must be Signed and the sender has to be the Owner of the"]
                    #[doc = "asset `id`."]
                    #[doc = ""]
                    #[doc = "- `id`: The identifier of the asset."]
                    #[doc = "- `min_balance`: The new value of `min_balance`."]
                    #[doc = ""]
                    #[doc = "Emits `AssetMinBalanceChanged` event when successful."]
                    set_min_balance {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        min_balance: ::core::primitive::u128,
                    },
                    #[codec(index = 29)]
                    #[doc = "Create an asset account for `who`."]
                    #[doc = ""]
                    #[doc = "A deposit will be taken from the signer account."]
                    #[doc = ""]
                    #[doc = "- `origin`: Must be Signed by `Freezer` or `Admin` of the asset `id`; the signer account"]
                    #[doc = "  must have sufficient funds for a deposit to be taken."]
                    #[doc = "- `id`: The identifier of the asset for the account to be created."]
                    #[doc = "- `who`: The account to be created."]
                    #[doc = ""]
                    #[doc = "Emits `Touched` event when successful."]
                    touch_other {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        who: ::subxt::ext::subxt_core::utils::MultiAddress<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            (),
                        >,
                    },
                    #[codec(index = 30)]
                    #[doc = "Return the deposit (if any) of a target asset account. Useful if you are the depositor."]
                    #[doc = ""]
                    #[doc = "The origin must be Signed and either the account owner, depositor, or asset `Admin`. In"]
                    #[doc = "order to burn a non-zero balance of the asset, the caller must be the account and should"]
                    #[doc = "use `refund`."]
                    #[doc = ""]
                    #[doc = "- `id`: The identifier of the asset for the account holding a deposit."]
                    #[doc = "- `who`: The account to refund."]
                    #[doc = ""]
                    #[doc = "It will fail with either [`Error::ContainsHolds`] or [`Error::ContainsFreezes`] if"]
                    #[doc = "the asset account contains holds or freezes in place."]
                    #[doc = ""]
                    #[doc = "Emits `Refunded` event when successful."]
                    refund_other {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        who: ::subxt::ext::subxt_core::utils::MultiAddress<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            (),
                        >,
                    },
                    #[codec(index = 31)]
                    #[doc = "Disallow further unprivileged transfers of an asset `id` to and from an account `who`."]
                    #[doc = ""]
                    #[doc = "Origin must be Signed and the sender should be the Freezer of the asset `id`."]
                    #[doc = ""]
                    #[doc = "- `id`: The identifier of the account's asset."]
                    #[doc = "- `who`: The account to be unblocked."]
                    #[doc = ""]
                    #[doc = "Emits `Blocked`."]
                    #[doc = ""]
                    #[doc = "Weight: `O(1)`"]
                    block {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        who: ::subxt::ext::subxt_core::utils::MultiAddress<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            (),
                        >,
                    },
                    #[codec(index = 32)]
                    #[doc = "Transfer the entire transferable balance from the caller asset account."]
                    #[doc = ""]
                    #[doc = "NOTE: This function only attempts to transfer _transferable_ balances. This means that"]
                    #[doc = "any held, frozen, or minimum balance (when `keep_alive` is `true`), will not be"]
                    #[doc = "transferred by this function. To ensure that this function results in a killed account,"]
                    #[doc = "you might need to prepare the account by removing any reference counters, storage"]
                    #[doc = "deposits, etc..."]
                    #[doc = ""]
                    #[doc = "The dispatch origin of this call must be Signed."]
                    #[doc = ""]
                    #[doc = "- `id`: The identifier of the asset for the account holding a deposit."]
                    #[doc = "- `dest`: The recipient of the transfer."]
                    #[doc = "- `keep_alive`: A boolean to determine if the `transfer_all` operation should send all"]
                    #[doc = "  of the funds the asset account has, causing the sender asset account to be killed"]
                    #[doc = "  (false), or transfer everything except at least the minimum balance, which will"]
                    #[doc = "  guarantee to keep the sender asset account alive (true)."]
                    transfer_all {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        dest: ::subxt::ext::subxt_core::utils::MultiAddress<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            (),
                        >,
                        keep_alive: ::core::primitive::bool,
                    },
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "The `Error` enum of this pallet."]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "Account balance must be greater than or equal to the transfer amount."]
                    BalanceLow,
                    #[codec(index = 1)]
                    #[doc = "The account to alter does not exist."]
                    NoAccount,
                    #[codec(index = 2)]
                    #[doc = "The signing account has no permission to do the operation."]
                    NoPermission,
                    #[codec(index = 3)]
                    #[doc = "The given asset ID is unknown."]
                    Unknown,
                    #[codec(index = 4)]
                    #[doc = "The origin account is frozen."]
                    Frozen,
                    #[codec(index = 5)]
                    #[doc = "The asset ID is already taken."]
                    InUse,
                    #[codec(index = 6)]
                    #[doc = "Invalid witness data given."]
                    BadWitness,
                    #[codec(index = 7)]
                    #[doc = "Minimum balance should be non-zero."]
                    MinBalanceZero,
                    #[codec(index = 8)]
                    #[doc = "Unable to increment the consumer reference counters on the account. Either no provider"]
                    #[doc = "reference exists to allow a non-zero balance of a non-self-sufficient asset, or one"]
                    #[doc = "fewer then the maximum number of consumers has been reached."]
                    UnavailableConsumer,
                    #[codec(index = 9)]
                    #[doc = "Invalid metadata given."]
                    BadMetadata,
                    #[codec(index = 10)]
                    #[doc = "No approval exists that would allow the transfer."]
                    Unapproved,
                    #[codec(index = 11)]
                    #[doc = "The source account would not survive the transfer and it needs to stay alive."]
                    WouldDie,
                    #[codec(index = 12)]
                    #[doc = "The asset-account already exists."]
                    AlreadyExists,
                    #[codec(index = 13)]
                    #[doc = "The asset-account doesn't have an associated deposit."]
                    NoDeposit,
                    #[codec(index = 14)]
                    #[doc = "The operation would result in funds being burned."]
                    WouldBurn,
                    #[codec(index = 15)]
                    #[doc = "The asset is a live asset and is actively being used. Usually emit for operations such"]
                    #[doc = "as `start_destroy` which require the asset to be in a destroying state."]
                    LiveAsset,
                    #[codec(index = 16)]
                    #[doc = "The asset is not live, and likely being destroyed."]
                    AssetNotLive,
                    #[codec(index = 17)]
                    #[doc = "The asset status is not the expected status."]
                    IncorrectStatus,
                    #[codec(index = 18)]
                    #[doc = "The asset should be frozen before the given operation."]
                    NotFrozen,
                    #[codec(index = 19)]
                    #[doc = "Callback action resulted in error"]
                    CallbackFailed,
                    #[codec(index = 20)]
                    #[doc = "The asset ID must be equal to the [`NextAssetId`]."]
                    BadAssetId,
                    #[codec(index = 21)]
                    #[doc = "The asset cannot be destroyed because some accounts for this asset contain freezes."]
                    ContainsFreezes,
                    #[codec(index = 22)]
                    #[doc = "The asset cannot be destroyed because some accounts for this asset contain holds."]
                    ContainsHolds,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "The `Event` enum of this pallet"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "Some asset class was created."]
                    Created {
                        asset_id: ::core::primitive::u32,
                        creator: ::subxt::ext::subxt_core::utils::AccountId32,
                        owner: ::subxt::ext::subxt_core::utils::AccountId32,
                    },
                    #[codec(index = 1)]
                    #[doc = "Some assets were issued."]
                    Issued {
                        asset_id: ::core::primitive::u32,
                        owner: ::subxt::ext::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 2)]
                    #[doc = "Some assets were transferred."]
                    Transferred {
                        asset_id: ::core::primitive::u32,
                        from: ::subxt::ext::subxt_core::utils::AccountId32,
                        to: ::subxt::ext::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 3)]
                    #[doc = "Some assets were destroyed."]
                    Burned {
                        asset_id: ::core::primitive::u32,
                        owner: ::subxt::ext::subxt_core::utils::AccountId32,
                        balance: ::core::primitive::u128,
                    },
                    #[codec(index = 4)]
                    #[doc = "The management team changed."]
                    TeamChanged {
                        asset_id: ::core::primitive::u32,
                        issuer: ::subxt::ext::subxt_core::utils::AccountId32,
                        admin: ::subxt::ext::subxt_core::utils::AccountId32,
                        freezer: ::subxt::ext::subxt_core::utils::AccountId32,
                    },
                    #[codec(index = 5)]
                    #[doc = "The owner changed."]
                    OwnerChanged {
                        asset_id: ::core::primitive::u32,
                        owner: ::subxt::ext::subxt_core::utils::AccountId32,
                    },
                    #[codec(index = 6)]
                    #[doc = "Some account `who` was frozen."]
                    Frozen {
                        asset_id: ::core::primitive::u32,
                        who: ::subxt::ext::subxt_core::utils::AccountId32,
                    },
                    #[codec(index = 7)]
                    #[doc = "Some account `who` was thawed."]
                    Thawed {
                        asset_id: ::core::primitive::u32,
                        who: ::subxt::ext::subxt_core::utils::AccountId32,
                    },
                    #[codec(index = 8)]
                    #[doc = "Some asset `asset_id` was frozen."]
                    AssetFrozen { asset_id: ::core::primitive::u32 },
                    #[codec(index = 9)]
                    #[doc = "Some asset `asset_id` was thawed."]
                    AssetThawed { asset_id: ::core::primitive::u32 },
                    #[codec(index = 10)]
                    #[doc = "Accounts were destroyed for given asset."]
                    AccountsDestroyed {
                        asset_id: ::core::primitive::u32,
                        accounts_destroyed: ::core::primitive::u32,
                        accounts_remaining: ::core::primitive::u32,
                    },
                    #[codec(index = 11)]
                    #[doc = "Approvals were destroyed for given asset."]
                    ApprovalsDestroyed {
                        asset_id: ::core::primitive::u32,
                        approvals_destroyed: ::core::primitive::u32,
                        approvals_remaining: ::core::primitive::u32,
                    },
                    #[codec(index = 12)]
                    #[doc = "An asset class is in the process of being destroyed."]
                    DestructionStarted { asset_id: ::core::primitive::u32 },
                    #[codec(index = 13)]
                    #[doc = "An asset class was destroyed."]
                    Destroyed { asset_id: ::core::primitive::u32 },
                    #[codec(index = 14)]
                    #[doc = "Some asset class was force-created."]
                    ForceCreated {
                        asset_id: ::core::primitive::u32,
                        owner: ::subxt::ext::subxt_core::utils::AccountId32,
                    },
                    #[codec(index = 15)]
                    #[doc = "New metadata has been set for an asset."]
                    MetadataSet {
                        asset_id: ::core::primitive::u32,
                        name: ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                        symbol: ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                        decimals: ::core::primitive::u8,
                        is_frozen: ::core::primitive::bool,
                    },
                    #[codec(index = 16)]
                    #[doc = "Metadata has been cleared for an asset."]
                    MetadataCleared { asset_id: ::core::primitive::u32 },
                    #[codec(index = 17)]
                    #[doc = "(Additional) funds have been approved for transfer to a destination account."]
                    ApprovedTransfer {
                        asset_id: ::core::primitive::u32,
                        source: ::subxt::ext::subxt_core::utils::AccountId32,
                        delegate: ::subxt::ext::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 18)]
                    #[doc = "An approval for account `delegate` was cancelled by `owner`."]
                    ApprovalCancelled {
                        asset_id: ::core::primitive::u32,
                        owner: ::subxt::ext::subxt_core::utils::AccountId32,
                        delegate: ::subxt::ext::subxt_core::utils::AccountId32,
                    },
                    #[codec(index = 19)]
                    #[doc = "An `amount` was transferred in its entirety from `owner` to `destination` by"]
                    #[doc = "the approved `delegate`."]
                    TransferredApproved {
                        asset_id: ::core::primitive::u32,
                        owner: ::subxt::ext::subxt_core::utils::AccountId32,
                        delegate: ::subxt::ext::subxt_core::utils::AccountId32,
                        destination: ::subxt::ext::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 20)]
                    #[doc = "An asset has had its attributes changed by the `Force` origin."]
                    AssetStatusChanged { asset_id: ::core::primitive::u32 },
                    #[codec(index = 21)]
                    #[doc = "The min_balance of an asset has been updated by the asset owner."]
                    AssetMinBalanceChanged {
                        asset_id: ::core::primitive::u32,
                        new_min_balance: ::core::primitive::u128,
                    },
                    #[codec(index = 22)]
                    #[doc = "Some account `who` was created with a deposit from `depositor`."]
                    Touched {
                        asset_id: ::core::primitive::u32,
                        who: ::subxt::ext::subxt_core::utils::AccountId32,
                        depositor: ::subxt::ext::subxt_core::utils::AccountId32,
                    },
                    #[codec(index = 23)]
                    #[doc = "Some account `who` was blocked."]
                    Blocked {
                        asset_id: ::core::primitive::u32,
                        who: ::subxt::ext::subxt_core::utils::AccountId32,
                    },
                    #[codec(index = 24)]
                    #[doc = "Some assets were deposited (e.g. for transaction fees)."]
                    Deposited {
                        asset_id: ::core::primitive::u32,
                        who: ::subxt::ext::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 25)]
                    #[doc = "Some assets were withdrawn from the account (e.g. for transaction fees)."]
                    Withdrawn {
                        asset_id: ::core::primitive::u32,
                        who: ::subxt::ext::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                }
            }
        }
        pub mod pallet_assets_holder {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "The `Error` enum of this pallet."]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "Number of holds on an account would exceed the count of `RuntimeHoldReason`."]
                    TooManyHolds,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "The `Event` enum of this pallet"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "`who`s balance on hold was increased by `amount`."]
                    Held {
                        who: ::subxt::ext::subxt_core::utils::AccountId32,
                        asset_id: ::core::primitive::u32,
                        reason: runtime_types::people_rococo_runtime::RuntimeHoldReason,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 1)]
                    #[doc = "`who`s balance on hold was decreased by `amount`."]
                    Released {
                        who: ::subxt::ext::subxt_core::utils::AccountId32,
                        asset_id: ::core::primitive::u32,
                        reason: runtime_types::people_rococo_runtime::RuntimeHoldReason,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 2)]
                    #[doc = "`who`s balance on hold was burned by `amount`."]
                    Burned {
                        who: ::subxt::ext::subxt_core::utils::AccountId32,
                        asset_id: ::core::primitive::u32,
                        reason: runtime_types::people_rococo_runtime::RuntimeHoldReason,
                        amount: ::core::primitive::u128,
                    },
                }
            }
        }
        pub mod pallet_balances {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Transfer some liquid free balance to another account."]
                    #[doc = ""]
                    #[doc = "`transfer_allow_death` will set the `FreeBalance` of the sender and receiver."]
                    #[doc = "If the sender's account is below the existential deposit as a result"]
                    #[doc = "of the transfer, the account will be reaped."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be `Signed` by the transactor."]
                    transfer_allow_death {
                        dest: ::subxt::ext::subxt_core::utils::MultiAddress<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            (),
                        >,
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                    },
                    #[codec(index = 2)]
                    #[doc = "Exactly as `transfer_allow_death`, except the origin must be root and the source account"]
                    #[doc = "may be specified."]
                    force_transfer {
                        source: ::subxt::ext::subxt_core::utils::MultiAddress<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            (),
                        >,
                        dest: ::subxt::ext::subxt_core::utils::MultiAddress<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            (),
                        >,
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                    },
                    #[codec(index = 3)]
                    #[doc = "Same as the [`transfer_allow_death`] call, but with a check that the transfer will not"]
                    #[doc = "kill the origin account."]
                    #[doc = ""]
                    #[doc = "99% of the time you want [`transfer_allow_death`] instead."]
                    #[doc = ""]
                    #[doc = "[`transfer_allow_death`]: struct.Pallet.html#method.transfer"]
                    transfer_keep_alive {
                        dest: ::subxt::ext::subxt_core::utils::MultiAddress<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            (),
                        >,
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                    },
                    #[codec(index = 4)]
                    #[doc = "Transfer the entire transferable balance from the caller account."]
                    #[doc = ""]
                    #[doc = "NOTE: This function only attempts to transfer _transferable_ balances. This means that"]
                    #[doc = "any locked, reserved, or existential deposits (when `keep_alive` is `true`), will not be"]
                    #[doc = "transferred by this function. To ensure that this function results in a killed account,"]
                    #[doc = "you might need to prepare the account by removing any reference counters, storage"]
                    #[doc = "deposits, etc..."]
                    #[doc = ""]
                    #[doc = "The dispatch origin of this call must be Signed."]
                    #[doc = ""]
                    #[doc = "- `dest`: The recipient of the transfer."]
                    #[doc = "- `keep_alive`: A boolean to determine if the `transfer_all` operation should send all"]
                    #[doc = "  of the funds the account has, causing the sender account to be killed (false), or"]
                    #[doc = "  transfer everything except at least the existential deposit, which will guarantee to"]
                    #[doc = "  keep the sender account alive (true)."]
                    transfer_all {
                        dest: ::subxt::ext::subxt_core::utils::MultiAddress<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            (),
                        >,
                        keep_alive: ::core::primitive::bool,
                    },
                    #[codec(index = 5)]
                    #[doc = "Unreserve some balance from a user by force."]
                    #[doc = ""]
                    #[doc = "Can only be called by ROOT."]
                    force_unreserve {
                        who: ::subxt::ext::subxt_core::utils::MultiAddress<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            (),
                        >,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 6)]
                    #[doc = "Upgrade a specified account."]
                    #[doc = ""]
                    #[doc = "- `origin`: Must be `Signed`."]
                    #[doc = "- `who`: The account to be upgraded."]
                    #[doc = ""]
                    #[doc = "This will waive the transaction fee if at least all but 10% of the accounts needed to"]
                    #[doc = "be upgraded. (We let some not have to be upgraded just in order to allow for the"]
                    #[doc = "possibility of churn)."]
                    upgrade_accounts {
                        who: ::subxt::ext::subxt_core::alloc::vec::Vec<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                        >,
                    },
                    #[codec(index = 8)]
                    #[doc = "Set the regular balance of a given account."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call is `root`."]
                    force_set_balance {
                        who: ::subxt::ext::subxt_core::utils::MultiAddress<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            (),
                        >,
                        #[codec(compact)]
                        new_free: ::core::primitive::u128,
                    },
                    #[codec(index = 9)]
                    #[doc = "Adjust the total issuance in a saturating way."]
                    #[doc = ""]
                    #[doc = "Can only be called by root and always needs a positive `delta`."]
                    #[doc = ""]
                    #[doc = "# Example"]
                    force_adjust_total_issuance {
                        direction: runtime_types::pallet_balances::types::AdjustmentDirection,
                        #[codec(compact)]
                        delta: ::core::primitive::u128,
                    },
                    #[codec(index = 10)]
                    #[doc = "Burn the specified liquid free balance from the origin account."]
                    #[doc = ""]
                    #[doc = "If the origin's account ends up below the existential deposit as a result"]
                    #[doc = "of the burn and `keep_alive` is false, the account will be reaped."]
                    #[doc = ""]
                    #[doc = "Unlike sending funds to a _burn_ address, which merely makes the funds inaccessible,"]
                    #[doc = "this `burn` operation will reduce total issuance by the amount _burned_."]
                    burn {
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                        keep_alive: ::core::primitive::bool,
                    },
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "The `Error` enum of this pallet."]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "Vesting balance too high to send value."]
                    VestingBalance,
                    #[codec(index = 1)]
                    #[doc = "Account liquidity restrictions prevent withdrawal."]
                    LiquidityRestrictions,
                    #[codec(index = 2)]
                    #[doc = "Balance too low to send value."]
                    InsufficientBalance,
                    #[codec(index = 3)]
                    #[doc = "Value too low to create account due to existential deposit."]
                    ExistentialDeposit,
                    #[codec(index = 4)]
                    #[doc = "Transfer/payment would kill account."]
                    Expendability,
                    #[codec(index = 5)]
                    #[doc = "A vesting schedule already exists for this account."]
                    ExistingVestingSchedule,
                    #[codec(index = 6)]
                    #[doc = "Beneficiary account must pre-exist."]
                    DeadAccount,
                    #[codec(index = 7)]
                    #[doc = "Number of named reserves exceed `MaxReserves`."]
                    TooManyReserves,
                    #[codec(index = 8)]
                    #[doc = "Number of holds exceed `VariantCountOf<T::RuntimeHoldReason>`."]
                    TooManyHolds,
                    #[codec(index = 9)]
                    #[doc = "Number of freezes exceed `MaxFreezes`."]
                    TooManyFreezes,
                    #[codec(index = 10)]
                    #[doc = "The issuance cannot be modified since it is already deactivated."]
                    IssuanceDeactivated,
                    #[codec(index = 11)]
                    #[doc = "The delta cannot be zero."]
                    DeltaZero,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "The `Event` enum of this pallet"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "An account was created with some free balance."]
                    Endowed {
                        account: ::subxt::ext::subxt_core::utils::AccountId32,
                        free_balance: ::core::primitive::u128,
                    },
                    #[codec(index = 1)]
                    #[doc = "An account was removed whose balance was non-zero but below ExistentialDeposit,"]
                    #[doc = "resulting in an outright loss."]
                    DustLost {
                        account: ::subxt::ext::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 2)]
                    #[doc = "Transfer succeeded."]
                    Transfer {
                        from: ::subxt::ext::subxt_core::utils::AccountId32,
                        to: ::subxt::ext::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 3)]
                    #[doc = "A balance was set by root."]
                    BalanceSet {
                        who: ::subxt::ext::subxt_core::utils::AccountId32,
                        free: ::core::primitive::u128,
                    },
                    #[codec(index = 4)]
                    #[doc = "Some balance was reserved (moved from free to reserved)."]
                    Reserved {
                        who: ::subxt::ext::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 5)]
                    #[doc = "Some balance was unreserved (moved from reserved to free)."]
                    Unreserved {
                        who: ::subxt::ext::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 6)]
                    #[doc = "Some balance was moved from the reserve of the first account to the second account."]
                    #[doc = "Final argument indicates the destination balance type."]
                    ReserveRepatriated {
                        from: ::subxt::ext::subxt_core::utils::AccountId32,
                        to: ::subxt::ext::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u128,
                        destination_status:
                            runtime_types::frame_support::traits::tokens::misc::BalanceStatus,
                    },
                    #[codec(index = 7)]
                    #[doc = "Some amount was deposited (e.g. for transaction fees)."]
                    Deposit {
                        who: ::subxt::ext::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 8)]
                    #[doc = "Some amount was withdrawn from the account (e.g. for transaction fees)."]
                    Withdraw {
                        who: ::subxt::ext::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 9)]
                    #[doc = "Some amount was removed from the account (e.g. for misbehavior)."]
                    Slashed {
                        who: ::subxt::ext::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 10)]
                    #[doc = "Some amount was minted into an account."]
                    Minted {
                        who: ::subxt::ext::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 11)]
                    #[doc = "Some amount was burned from an account."]
                    Burned {
                        who: ::subxt::ext::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 12)]
                    #[doc = "Some amount was suspended from an account (it can be restored later)."]
                    Suspended {
                        who: ::subxt::ext::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 13)]
                    #[doc = "Some amount was restored into an account."]
                    Restored {
                        who: ::subxt::ext::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 14)]
                    #[doc = "An account was upgraded."]
                    Upgraded {
                        who: ::subxt::ext::subxt_core::utils::AccountId32,
                    },
                    #[codec(index = 15)]
                    #[doc = "Total issuance was increased by `amount`, creating a credit to be balanced."]
                    Issued { amount: ::core::primitive::u128 },
                    #[codec(index = 16)]
                    #[doc = "Total issuance was decreased by `amount`, creating a debt to be balanced."]
                    Rescinded { amount: ::core::primitive::u128 },
                    #[codec(index = 17)]
                    #[doc = "Some balance was locked."]
                    Locked {
                        who: ::subxt::ext::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 18)]
                    #[doc = "Some balance was unlocked."]
                    Unlocked {
                        who: ::subxt::ext::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 19)]
                    #[doc = "Some balance was frozen."]
                    Frozen {
                        who: ::subxt::ext::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 20)]
                    #[doc = "Some balance was thawed."]
                    Thawed {
                        who: ::subxt::ext::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 21)]
                    #[doc = "The `TotalIssuance` was forcefully changed."]
                    TotalIssuanceForced {
                        old: ::core::primitive::u128,
                        new: ::core::primitive::u128,
                    },
                }
            }
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct AccountData<_0> {
                    pub free: _0,
                    pub reserved: _0,
                    pub frozen: _0,
                    pub flags: runtime_types::pallet_balances::types::ExtraFlags,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub enum AdjustmentDirection {
                    #[codec(index = 0)]
                    Increase,
                    #[codec(index = 1)]
                    Decrease,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: CompactAs,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct ExtraFlags(pub ::core::primitive::u128);
            }
        }
        pub mod pallet_collator_selection {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Set the list of invulnerable (fixed) collators. These collators must do some"]
                    #[doc = "preparation, namely to have registered session keys."]
                    #[doc = ""]
                    #[doc = "The call will remove any accounts that have not registered keys from the set. That is,"]
                    #[doc = "it is non-atomic; the caller accepts all `AccountId`s passed in `new` _individually_ as"]
                    #[doc = "acceptable Invulnerables, and is not proposing a _set_ of new Invulnerables."]
                    #[doc = ""]
                    #[doc = "This call does not maintain mutual exclusivity of `Invulnerables` and `Candidates`. It"]
                    #[doc = "is recommended to use a batch of `add_invulnerable` and `remove_invulnerable` instead. A"]
                    #[doc = "`batch_all` can also be used to enforce atomicity. If any candidates are included in"]
                    #[doc = "`new`, they should be removed with `remove_invulnerable_candidate` after execution."]
                    #[doc = ""]
                    #[doc = "Must be called by the `UpdateOrigin`."]
                    set_invulnerables {
                        new: ::subxt::ext::subxt_core::alloc::vec::Vec<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                        >,
                    },
                    #[codec(index = 1)]
                    #[doc = "Set the ideal number of non-invulnerable collators. If lowering this number, then the"]
                    #[doc = "number of running collators could be higher than this figure. Aside from that edge case,"]
                    #[doc = "there should be no other way to have more candidates than the desired number."]
                    #[doc = ""]
                    #[doc = "The origin for this call must be the `UpdateOrigin`."]
                    set_desired_candidates { max: ::core::primitive::u32 },
                    #[codec(index = 2)]
                    #[doc = "Set the candidacy bond amount."]
                    #[doc = ""]
                    #[doc = "If the candidacy bond is increased by this call, all current candidates which have a"]
                    #[doc = "deposit lower than the new bond will be kicked from the list and get their deposits"]
                    #[doc = "back."]
                    #[doc = ""]
                    #[doc = "The origin for this call must be the `UpdateOrigin`."]
                    set_candidacy_bond { bond: ::core::primitive::u128 },
                    #[codec(index = 3)]
                    #[doc = "Register this account as a collator candidate. The account must (a) already have"]
                    #[doc = "registered session keys and (b) be able to reserve the `CandidacyBond`."]
                    #[doc = ""]
                    #[doc = "This call is not available to `Invulnerable` collators."]
                    register_as_candidate,
                    #[codec(index = 4)]
                    #[doc = "Deregister `origin` as a collator candidate. Note that the collator can only leave on"]
                    #[doc = "session change. The `CandidacyBond` will be unreserved immediately."]
                    #[doc = ""]
                    #[doc = "This call will fail if the total number of candidates would drop below"]
                    #[doc = "`MinEligibleCollators`."]
                    leave_intent,
                    #[codec(index = 5)]
                    #[doc = "Add a new account `who` to the list of `Invulnerables` collators. `who` must have"]
                    #[doc = "registered session keys. If `who` is a candidate, they will be removed."]
                    #[doc = ""]
                    #[doc = "The origin for this call must be the `UpdateOrigin`."]
                    add_invulnerable {
                        who: ::subxt::ext::subxt_core::utils::AccountId32,
                    },
                    #[codec(index = 6)]
                    #[doc = "Remove an account `who` from the list of `Invulnerables` collators. `Invulnerables` must"]
                    #[doc = "be sorted."]
                    #[doc = ""]
                    #[doc = "The origin for this call must be the `UpdateOrigin`."]
                    remove_invulnerable {
                        who: ::subxt::ext::subxt_core::utils::AccountId32,
                    },
                    #[codec(index = 7)]
                    #[doc = "Update the candidacy bond of collator candidate `origin` to a new amount `new_deposit`."]
                    #[doc = ""]
                    #[doc = "Setting a `new_deposit` that is lower than the current deposit while `origin` is"]
                    #[doc = "occupying a top-`DesiredCandidates` slot is not allowed."]
                    #[doc = ""]
                    #[doc = "This call will fail if `origin` is not a collator candidate, the updated bond is lower"]
                    #[doc = "than the minimum candidacy bond, and/or the amount cannot be reserved."]
                    update_bond {
                        new_deposit: ::core::primitive::u128,
                    },
                    #[codec(index = 8)]
                    #[doc = "The caller `origin` replaces a candidate `target` in the collator candidate list by"]
                    #[doc = "reserving `deposit`. The amount `deposit` reserved by the caller must be greater than"]
                    #[doc = "the existing bond of the target it is trying to replace."]
                    #[doc = ""]
                    #[doc = "This call will fail if the caller is already a collator candidate or invulnerable, the"]
                    #[doc = "caller does not have registered session keys, the target is not a collator candidate,"]
                    #[doc = "and/or the `deposit` amount cannot be reserved."]
                    take_candidate_slot {
                        deposit: ::core::primitive::u128,
                        target: ::subxt::ext::subxt_core::utils::AccountId32,
                    },
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "The `Error` enum of this pallet."]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "The pallet has too many candidates."]
                    TooManyCandidates,
                    #[codec(index = 1)]
                    #[doc = "Leaving would result in too few candidates."]
                    TooFewEligibleCollators,
                    #[codec(index = 2)]
                    #[doc = "Account is already a candidate."]
                    AlreadyCandidate,
                    #[codec(index = 3)]
                    #[doc = "Account is not a candidate."]
                    NotCandidate,
                    #[codec(index = 4)]
                    #[doc = "There are too many Invulnerables."]
                    TooManyInvulnerables,
                    #[codec(index = 5)]
                    #[doc = "Account is already an Invulnerable."]
                    AlreadyInvulnerable,
                    #[codec(index = 6)]
                    #[doc = "Account is not an Invulnerable."]
                    NotInvulnerable,
                    #[codec(index = 7)]
                    #[doc = "Account has no associated validator ID."]
                    NoAssociatedValidatorId,
                    #[codec(index = 8)]
                    #[doc = "Validator ID is not yet registered."]
                    ValidatorNotRegistered,
                    #[codec(index = 9)]
                    #[doc = "Could not insert in the candidate list."]
                    InsertToCandidateListFailed,
                    #[codec(index = 10)]
                    #[doc = "Could not remove from the candidate list."]
                    RemoveFromCandidateListFailed,
                    #[codec(index = 11)]
                    #[doc = "New deposit amount would be below the minimum candidacy bond."]
                    DepositTooLow,
                    #[codec(index = 12)]
                    #[doc = "Could not update the candidate list."]
                    UpdateCandidateListFailed,
                    #[codec(index = 13)]
                    #[doc = "Deposit amount is too low to take the target's slot in the candidate list."]
                    InsufficientBond,
                    #[codec(index = 14)]
                    #[doc = "The target account to be replaced in the candidate list is not a candidate."]
                    TargetIsNotCandidate,
                    #[codec(index = 15)]
                    #[doc = "The updated deposit amount is equal to the amount already reserved."]
                    IdenticalDeposit,
                    #[codec(index = 16)]
                    #[doc = "Cannot lower candidacy bond while occupying a future collator slot in the list."]
                    InvalidUnreserve,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "The `Event` enum of this pallet"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "New Invulnerables were set."]
                    NewInvulnerables {
                        invulnerables: ::subxt::ext::subxt_core::alloc::vec::Vec<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                        >,
                    },
                    #[codec(index = 1)]
                    #[doc = "A new Invulnerable was added."]
                    InvulnerableAdded {
                        account_id: ::subxt::ext::subxt_core::utils::AccountId32,
                    },
                    #[codec(index = 2)]
                    #[doc = "An Invulnerable was removed."]
                    InvulnerableRemoved {
                        account_id: ::subxt::ext::subxt_core::utils::AccountId32,
                    },
                    #[codec(index = 3)]
                    #[doc = "The number of desired candidates was set."]
                    NewDesiredCandidates {
                        desired_candidates: ::core::primitive::u32,
                    },
                    #[codec(index = 4)]
                    #[doc = "The candidacy bond was set."]
                    NewCandidacyBond {
                        bond_amount: ::core::primitive::u128,
                    },
                    #[codec(index = 5)]
                    #[doc = "A new candidate joined."]
                    CandidateAdded {
                        account_id: ::subxt::ext::subxt_core::utils::AccountId32,
                        deposit: ::core::primitive::u128,
                    },
                    #[codec(index = 6)]
                    #[doc = "Bond of a candidate updated."]
                    CandidateBondUpdated {
                        account_id: ::subxt::ext::subxt_core::utils::AccountId32,
                        deposit: ::core::primitive::u128,
                    },
                    #[codec(index = 7)]
                    #[doc = "A candidate was removed."]
                    CandidateRemoved {
                        account_id: ::subxt::ext::subxt_core::utils::AccountId32,
                    },
                    #[codec(index = 8)]
                    #[doc = "An account was replaced in the candidate list by another one."]
                    CandidateReplaced {
                        old: ::subxt::ext::subxt_core::utils::AccountId32,
                        new: ::subxt::ext::subxt_core::utils::AccountId32,
                        deposit: ::core::primitive::u128,
                    },
                    #[codec(index = 9)]
                    #[doc = "An account was unable to be added to the Invulnerables because they did not have keys"]
                    #[doc = "registered. Other Invulnerables may have been set."]
                    InvalidInvulnerableSkipped {
                        account_id: ::subxt::ext::subxt_core::utils::AccountId32,
                    },
                }
            }
        }
        pub mod pallet_dummy_dim {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Reserve a number of personal IDs."]
                    reserve_ids { count: ::core::primitive::u32 },
                    #[codec(index = 1)]
                    #[doc = "Renew a personal ID. The ID must not be in use."]
                    renew_id_reservation { id: ::core::primitive::u64 },
                    #[codec(index = 2)]
                    #[doc = "Cancel a personal ID reservation."]
                    cancel_id_reservation { id: ::core::primitive::u64 },
                    #[codec(index = 3)]
                    #[doc = "Grant personhood for a list of candidates that have reserved personal IDs."]
                    recognize_personhood {
                        ids_and_keys:
                            runtime_types::bounded_collections::bounded_vec::BoundedVec<(
                                ::core::primitive::u64,
                                runtime_types::verifiable::ring_vrf_impl::EncodedPublicKey,
                            )>,
                    },
                    #[codec(index = 4)]
                    #[doc = "Suspend the personhood of a list of recognized people. The people must not currently be"]
                    #[doc = "suspended."]
                    suspend_personhood {
                        ids: runtime_types::bounded_collections::bounded_vec::BoundedVec<
                            ::core::primitive::u64,
                        >,
                    },
                    #[codec(index = 5)]
                    #[doc = "Resume someone's personhood. The person must currently be suspended."]
                    resume_personhood { id: ::core::primitive::u64 },
                    #[codec(index = 6)]
                    #[doc = "Start a mutation session in the underlying `People` interface. This call does not check"]
                    #[doc = "whether a mutation session is already ongoing and can start new sessions."]
                    start_mutation_session,
                    #[codec(index = 7)]
                    #[doc = "End a mutation session in the underlying `People` interface. This call can end multiple"]
                    #[doc = "mutation sessions, even ones not started by this pallet."]
                    #[doc = ""]
                    #[doc = "This call will fail if no mutation session is ongoing."]
                    end_mutation_session,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "The `Error` enum of this pallet."]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "The personal ID does not belong to a recognized person."]
                    NotPerson,
                    #[codec(index = 1)]
                    #[doc = "The personal ID does not belong to a suspended person."]
                    NotSuspended,
                    #[codec(index = 2)]
                    #[doc = "The personal ID is not reserved and awaiting recognition."]
                    NotReserved,
                    #[codec(index = 3)]
                    #[doc = "The operation does not support this many people."]
                    TooManyPeople,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "The `Event` enum of this pallet"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "A number of IDs was reserved."]
                    IdsReserved { count: ::core::primitive::u32 },
                    #[codec(index = 1)]
                    #[doc = "An ID was renewed."]
                    IdRenewed { id: ::core::primitive::u64 },
                    #[codec(index = 2)]
                    #[doc = "A reserved ID was removed."]
                    IdUnreserved { id: ::core::primitive::u64 },
                    #[codec(index = 3)]
                    #[doc = "Register multiple people."]
                    PeopleRegistered { count: ::core::primitive::u32 },
                    #[codec(index = 4)]
                    #[doc = "Suspend a number of people."]
                    PeopleSuspended { count: ::core::primitive::u32 },
                    #[codec(index = 5)]
                    #[doc = "Someone's personhood was resumed."]
                    PersonhoodResumed { id: ::core::primitive::u64 },
                    #[codec(index = 6)]
                    #[doc = "The pallet enabled suspensions."]
                    SuspensionsStarted,
                    #[codec(index = 7)]
                    #[doc = "The pallet disabled suspensions."]
                    SuspensionsEnded,
                }
            }
        }
        pub mod pallet_game {
            use super::runtime_types;
            pub mod extension {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct GameAsInvited(
                    pub  ::core::option::Option<
                        runtime_types::pallet_game::extension::GameAsInvitedData,
                    >,
                );
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct GameAsInvitedData {
                    pub nonce: ::core::primitive::u32,
                    pub inviter: ::subxt::ext::subxt_core::utils::AccountId32,
                    pub ticket: ::subxt::ext::subxt_core::utils::AccountId32,
                    pub signature: runtime_types::sp_runtime::MultiSignature,
                }
            }
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
                pub enum Call {
                    #[codec(index = 14)]
                    #[doc = "Sign up for the game using an account and an invite."]
                    #[doc = ""]
                    #[doc = "This is for new players, player not new or archived should use"]
                    #[doc = "[`Pallet::sign_up_with_account`]."]
                    #[doc = ""]
                    #[doc = "A game must be ongoing and in its registration phase."]
                    #[doc = ""]
                    #[doc = "The origin must be a signed by an account and use the `GameAsInvited` extension."]
                    sign_up_with_invite,
                    #[codec(index = 1)]
                    #[doc = "Sign up for the game using an account."]
                    #[doc = ""]
                    #[doc = "If the player is new or archived, then a deposit will be taken from the signer."]
                    #[doc = "Otherwise the call is free."]
                    #[doc = ""]
                    #[doc = "A game must be ongoing and in its registration phase."]
                    #[doc = ""]
                    #[doc = "The origin must be signed by an account, or, be signed by an account and use"]
                    #[doc = "`ScoreAsParticipant` extension."]
                    sign_up_with_account,
                    #[codec(index = 20)]
                    #[doc = "Sign up for the game."]
                    #[doc = ""]
                    #[doc = "If a player is already recognized by another DIM, they can sign using their alias and"]
                    #[doc = "don't need any deposit or invite to prove their initial credibility."]
                    #[doc = "On top of this their score is never going below personhood threshold and the player will"]
                    #[doc = "never get archived."]
                    #[doc = ""]
                    #[doc = "A game must be ongoing and in its registration phase."]
                    #[doc = ""]
                    #[doc = "The origin must be a personal alias."]
                    #[doc = ""]
                    #[doc = "Parameters:"]
                    #[doc = "- `statement_account`: the account id to use to interact with the statement store during"]
                    #[doc = "  the game."]
                    #[doc = "- `sig`: the proof of ownership of the account `statement_account` by an alias, it is"]
                    #[doc = "  the signature of the message `\"pop:game:stmt_account_for_alias:\"` concatenated to the"]
                    #[doc = "  alias. The base of the message can be found in the constant:"]
                    #[doc = "  `proof_of_ownership_msg_base`."]
                    sign_up_with_alias {
                        statement_account: ::subxt::ext::subxt_core::utils::AccountId32,
                        sig: runtime_types::sp_runtime::MultiSignature,
                    },
                    #[codec(index = 4)]
                    #[doc = "After the game, send the full report."]
                    #[doc = ""]
                    #[doc = "The game must be ongoing and in its reporting phase."]
                    #[doc = ""]
                    #[doc = "The origin must be an alias, or signed by an account, or signed by an account and use"]
                    #[doc = "`ScoreAsParticipant` extension."]
                    report {
                        full_report: runtime_types::bounded_collections::bounded_vec::BoundedVec<
                            runtime_types::bounded_collections::bounded_vec::BoundedVec<
                                runtime_types::pallet_game::types::Report,
                            >,
                        >,
                    },
                    #[codec(index = 6)]
                    #[doc = "Offboard a player from the game."]
                    #[doc = ""]
                    #[doc = "The origin must be an alias, or signed by an account, or signed by an account and use"]
                    #[doc = "`ScoreAsParticipant` extension."]
                    #[doc = ""]
                    #[doc = "There must be no game or the existing game must be in registration phase and the player"]
                    #[doc = "must have not signed up for the game."]
                    offboard,
                    #[codec(index = 9)]
                    #[doc = "Kickout a kickable player that is not playing after `NonPlayingKickoutTime`."]
                    #[doc = ""]
                    #[doc = "The origin must be signed by an account."]
                    #[doc = ""]
                    #[doc = "- `player`: The player to kickout. It must be archived and kickable with"]
                    #[doc = "  `archived_since` older than `NonPlayingKickoutTime`."]
                    kickout {
                        player: ::subxt::ext::subxt_core::utils::AccountId32,
                    },
                    #[codec(index = 10)]
                    #[doc = "Grant some invites to an account so they can distribute them."]
                    #[doc = ""]
                    #[doc = "The origin must be `InviteIssuer`."]
                    #[doc = ""]
                    #[doc = "- `account`: The account to grant invites to."]
                    #[doc = "- `count`: The number of invites to grant."]
                    grant_invites {
                        account: ::subxt::ext::subxt_core::utils::AccountId32,
                        count: ::core::primitive::u32,
                    },
                    #[codec(index = 11)]
                    #[doc = "Clear all invites given to an account."]
                    #[doc = ""]
                    #[doc = "The origin must be `InviteIssuer`."]
                    #[doc = ""]
                    #[doc = "- `account`: The account to remove all invites from."]
                    #[doc = "- `limit`: The maximum number of pending invites to remove."]
                    remove_available_and_pending_invites {
                        account: ::subxt::ext::subxt_core::utils::AccountId32,
                        limit: ::core::primitive::u32,
                    },
                    #[codec(index = 12)]
                    #[doc = "Invite an account."]
                    #[doc = ""]
                    #[doc = "The origin must be signed by an account and have some invites left."]
                    #[doc = ""]
                    #[doc = "- `ticket`: The invite ticket to set."]
                    set_invite_ticket {
                        ticket: ::subxt::ext::subxt_core::utils::AccountId32,
                    },
                    #[codec(index = 13)]
                    #[doc = "Cancel an invite."]
                    #[doc = ""]
                    #[doc = "The origin must be signed by the account that owns the ticket to cancel."]
                    #[doc = ""]
                    #[doc = "- `ticket`: The invite ticket to cancel."]
                    cancel_invite_ticket {
                        ticket: ::subxt::ext::subxt_core::utils::AccountId32,
                    },
                    #[codec(index = 15)]
                    #[doc = "Schedules new games according to provided schedules."]
                    #[doc = "Schedules must be in chronological order, and after the ongoing game (if there is any)."]
                    schedule_games {
                        games_schedules: ::subxt::ext::subxt_core::alloc::vec::Vec<
                            runtime_types::pallet_game::types::GameSchedule,
                        >,
                    },
                    #[codec(index = 16)]
                    remove_scheduled_game {
                        game_play_time: ::core::primitive::u32,
                    },
                    #[codec(index = 101)]
                    #[doc = "Force start the shuffle before its normal start time."]
                    #[doc = ""]
                    #[doc = "This action can only be performed by the root origin and is only meant for testing."]
                    testnet_force_start_shuffle,
                    #[codec(index = 102)]
                    #[doc = "Force end a game's reporting phase before its normal end time."]
                    #[doc = ""]
                    #[doc = "This action can only be performed by the root origin and is only meant for testing."]
                    testnet_force_end_reporting,
                    #[codec(index = 103)]
                    #[doc = "Force set the game phase periods."]
                    #[doc = ""]
                    #[doc = "This action can only be performed by the root origin and is only meant for testing."]
                    testnet_set_game_phases {
                        phases: runtime_types::pallet_game::types::PhaseDurationValues,
                    },
                    #[codec(index = 104)]
                    #[doc = "Kill the current game. Whatever the phase it is in."]
                    testnet_kill_current_game,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "The `Error` enum of this pallet."]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "Game ongoing."]
                    GameOngoing,
                    #[codec(index = 1)]
                    #[doc = "No registration phase ongoing."]
                    NoRegistration,
                    #[codec(index = 2)]
                    #[doc = "The setup is outdated."]
                    OutdatedGameSetup,
                    #[codec(index = 3)]
                    #[doc = "Invalid setup."]
                    InvalidGameSetup,
                    #[codec(index = 4)]
                    #[doc = "Invalid report."]
                    InvalidReport,
                    #[codec(index = 5)]
                    #[doc = "No game ongoing."]
                    NoGame,
                    #[codec(index = 6)]
                    #[doc = "No report phase ongoing."]
                    NoReporting,
                    #[codec(index = 7)]
                    #[doc = "Not registered."]
                    NotRegistered,
                    #[codec(index = 8)]
                    #[doc = "Player already registered."]
                    AlreadyRegistered,
                    #[codec(index = 9)]
                    #[doc = "Report already sent."]
                    ReportAlreadySent,
                    #[codec(index = 10)]
                    #[doc = "Operation is not valid yet."]
                    Early,
                    #[codec(index = 11)]
                    #[doc = "The operation expect a player account."]
                    NotKickablePlayer,
                    #[codec(index = 12)]
                    #[doc = "No archived player found."]
                    NoArchivedPlayer,
                    #[codec(index = 13)]
                    #[doc = "No ticket found."]
                    NoTicket,
                    #[codec(index = 14)]
                    #[doc = "No invite available."]
                    NoInvites,
                    #[codec(index = 15)]
                    #[doc = "Invite is already set."]
                    AlreadyInvited,
                    #[codec(index = 16)]
                    #[doc = "Not an account based player, expected an account based player."]
                    NotAccountPlayer,
                    #[codec(index = 17)]
                    #[doc = "The player can't use an invite if already playing."]
                    UseInviteButAlreadyPlaying,
                    #[codec(index = 18)]
                    #[doc = "The number of existing schedules and new schedules exceeds the configured limit."]
                    TooManyGameSchedules,
                    #[codec(index = 19)]
                    #[doc = "The game that was supposed to be removed was not found in scheduled games."]
                    NoSuchGameScheduled,
                    #[codec(index = 20)]
                    #[doc = "The statement account signature is invalid."]
                    InvalidStatementAccountSignature,
                    #[codec(index = 21)]
                    #[doc = "The statement account is already in used by another player."]
                    StatementAccountAlreadyInUse,
                    #[codec(index = 22)]
                    #[doc = "Internal error invalid state."]
                    InternalErrorInvalidState,
                    #[codec(index = 23)]
                    #[doc = "The operation cannot be performed in the current game state."]
                    InvalidGameState,
                    #[codec(index = 24)]
                    #[doc = "No player found."]
                    NoPlayer,
                    #[codec(index = 25)]
                    #[doc = "The player cannot offboard while registered for a game."]
                    CannotOffboardWhileRegisteredForGame,
                    #[codec(index = 26)]
                    #[doc = "Invalid state"]
                    InvalidState,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "The `Event` enum of this pallet"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "A new game is starting."]
                    NewGame {
                        registration_ends: ::core::primitive::u32,
                        game_date: ::core::primitive::u32,
                        report_ends: ::core::primitive::u32,
                    },
                    #[codec(index = 1)]
                    #[doc = "The game and its post-process has ended."]
                    GameEnded { index: ::core::primitive::u32 },
                    #[codec(index = 2)]
                    #[doc = "All invites have been removed for the inviter."]
                    AllInvitesRemoved {
                        inviter: ::subxt::ext::subxt_core::utils::AccountId32,
                    },
                    #[codec(index = 3)]
                    #[doc = "Some invites have been removed for the inviter, some are remaining."]
                    SomeInvitesRemoved {
                        inviter: ::subxt::ext::subxt_core::utils::AccountId32,
                    },
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub enum HoldReason {
                    #[codec(index = 0)]
                    PlayGame,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub enum Origin {
                    #[codec(index = 0)]
                    Invited(::subxt::ext::subxt_core::utils::AccountId32),
                }
            }
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct GameSchedule {
                    pub game_play_time: ::core::primitive::u32,
                    pub rounds: ::core::primitive::u8,
                    pub max_group_size: ::core::primitive::u32,
                    pub boost: ::core::primitive::u32,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct PhaseDurationValues {
                    pub registration: ::core::primitive::u32,
                    pub shuffle: ::core::primitive::u32,
                    pub post_shuffle_margin: ::core::primitive::u32,
                    pub reporting: ::core::primitive::u32,
                    pub player_process: ::core::primitive::u32,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub enum Report {
                    #[codec(index = 0)]
                    Person,
                    #[codec(index = 1)]
                    NotPerson,
                }
            }
        }
        pub mod pallet_identity {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Identity pallet declaration."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Add a registrar to the system."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be `T::RegistrarOrigin`."]
                    #[doc = ""]
                    #[doc = "- `account`: the account of the registrar."]
                    #[doc = ""]
                    #[doc = "Emits `RegistrarAdded` if successful."]
                    add_registrar {
                        account: ::subxt::ext::subxt_core::utils::MultiAddress<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            (),
                        >,
                    },
                    #[codec(index = 1)]
                    #[doc = "Set an account's identity information and reserve the appropriate deposit."]
                    #[doc = ""]
                    #[doc = "If the account already has identity information, the deposit is taken as part payment"]
                    #[doc = "for the new deposit."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_."]
                    #[doc = ""]
                    #[doc = "- `info`: The identity information."]
                    #[doc = ""]
                    #[doc = "Emits `IdentitySet` if successful."]
                    set_identity {
                        info: ::subxt::ext::subxt_core::alloc::boxed::Box<
                            runtime_types::people_rococo_runtime::people::IdentityInfo,
                        >,
                    },
                    #[codec(index = 2)]
                    #[doc = "Set the sub-accounts of the sender."]
                    #[doc = ""]
                    #[doc = "Payment: Any aggregate balance reserved by previous `set_subs` calls will be returned"]
                    #[doc = "and an amount `SubAccountDeposit` will be reserved for each item in `subs`."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_ and the sender must have a registered"]
                    #[doc = "identity."]
                    #[doc = ""]
                    #[doc = "- `subs`: The identity's (new) sub-accounts."]
                    set_subs {
                        subs: ::subxt::ext::subxt_core::alloc::vec::Vec<(
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            runtime_types::pallet_identity::types::Data,
                        )>,
                    },
                    #[codec(index = 3)]
                    #[doc = "Clear an account's identity info and all sub-accounts and return all deposits."]
                    #[doc = ""]
                    #[doc = "Payment: All reserved balances on the account are returned."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_ and the sender must have a registered"]
                    #[doc = "identity."]
                    #[doc = ""]
                    #[doc = "Emits `IdentityCleared` if successful."]
                    clear_identity,
                    #[codec(index = 4)]
                    #[doc = "Request a judgement from a registrar."]
                    #[doc = ""]
                    #[doc = "Payment: At most `max_fee` will be reserved for payment to the registrar if judgement"]
                    #[doc = "given."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_ and the sender must have a"]
                    #[doc = "registered identity."]
                    #[doc = ""]
                    #[doc = "- `reg_index`: The index of the registrar whose judgement is requested."]
                    #[doc = "- `max_fee`: The maximum fee that may be paid. This should just be auto-populated as:"]
                    #[doc = ""]
                    #[doc = "```nocompile"]
                    #[doc = "Self::registrars().get(reg_index).unwrap().fee"]
                    #[doc = "```"]
                    #[doc = ""]
                    #[doc = "Emits `JudgementRequested` if successful."]
                    request_judgement {
                        #[codec(compact)]
                        reg_index: ::core::primitive::u32,
                        #[codec(compact)]
                        max_fee: ::core::primitive::u128,
                    },
                    #[codec(index = 5)]
                    #[doc = "Cancel a previous request."]
                    #[doc = ""]
                    #[doc = "Payment: A previously reserved deposit is returned on success."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_ and the sender must have a"]
                    #[doc = "registered identity."]
                    #[doc = ""]
                    #[doc = "- `reg_index`: The index of the registrar whose judgement is no longer requested."]
                    #[doc = ""]
                    #[doc = "Emits `JudgementUnrequested` if successful."]
                    cancel_request { reg_index: ::core::primitive::u32 },
                    #[codec(index = 6)]
                    #[doc = "Set the fee required for a judgement to be requested from a registrar."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_ and the sender must be the account"]
                    #[doc = "of the registrar whose index is `index`."]
                    #[doc = ""]
                    #[doc = "- `index`: the index of the registrar whose fee is to be set."]
                    #[doc = "- `fee`: the new fee."]
                    set_fee {
                        #[codec(compact)]
                        index: ::core::primitive::u32,
                        #[codec(compact)]
                        fee: ::core::primitive::u128,
                    },
                    #[codec(index = 7)]
                    #[doc = "Change the account associated with a registrar."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_ and the sender must be the account"]
                    #[doc = "of the registrar whose index is `index`."]
                    #[doc = ""]
                    #[doc = "- `index`: the index of the registrar whose fee is to be set."]
                    #[doc = "- `new`: the new account ID."]
                    set_account_id {
                        #[codec(compact)]
                        index: ::core::primitive::u32,
                        new: ::subxt::ext::subxt_core::utils::MultiAddress<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            (),
                        >,
                    },
                    #[codec(index = 8)]
                    #[doc = "Set the field information for a registrar."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_ and the sender must be the account"]
                    #[doc = "of the registrar whose index is `index`."]
                    #[doc = ""]
                    #[doc = "- `index`: the index of the registrar whose fee is to be set."]
                    #[doc = "- `fields`: the fields that the registrar concerns themselves with."]
                    set_fields {
                        #[codec(compact)]
                        index: ::core::primitive::u32,
                        fields: ::core::primitive::u64,
                    },
                    #[codec(index = 9)]
                    #[doc = "Provide a judgement for an account's identity."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_ and the sender must be the account"]
                    #[doc = "of the registrar whose index is `reg_index`."]
                    #[doc = ""]
                    #[doc = "- `reg_index`: the index of the registrar whose judgement is being made."]
                    #[doc = "- `target`: the account whose identity the judgement is upon. This must be an account"]
                    #[doc = "  with a registered identity."]
                    #[doc = "- `judgement`: the judgement of the registrar of index `reg_index` about `target`."]
                    #[doc = "- `identity`: The hash of the [`IdentityInformationProvider`] for that the judgement is"]
                    #[doc = "  provided."]
                    #[doc = ""]
                    #[doc = "Note: Judgements do not apply to a username."]
                    #[doc = ""]
                    #[doc = "Emits `JudgementGiven` if successful."]
                    provide_judgement {
                        #[codec(compact)]
                        reg_index: ::core::primitive::u32,
                        target: ::subxt::ext::subxt_core::utils::MultiAddress<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            (),
                        >,
                        judgement: runtime_types::pallet_identity::types::Judgement<
                            ::core::primitive::u128,
                        >,
                        identity: ::subxt::ext::subxt_core::utils::H256,
                    },
                    #[codec(index = 10)]
                    #[doc = "Remove an account's identity and sub-account information and slash the deposits."]
                    #[doc = ""]
                    #[doc = "Payment: Reserved balances from `set_subs` and `set_identity` are slashed and handled by"]
                    #[doc = "`Slash`. Verification request deposits are not returned; they should be cancelled"]
                    #[doc = "manually using `cancel_request`."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must match `T::ForceOrigin`."]
                    #[doc = ""]
                    #[doc = "- `target`: the account whose identity the judgement is upon. This must be an account"]
                    #[doc = "  with a registered identity."]
                    #[doc = ""]
                    #[doc = "Emits `IdentityKilled` if successful."]
                    kill_identity {
                        target: ::subxt::ext::subxt_core::utils::MultiAddress<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            (),
                        >,
                    },
                    #[codec(index = 11)]
                    #[doc = "Add the given account to the sender's subs."]
                    #[doc = ""]
                    #[doc = "Payment: Balance reserved by a previous `set_subs` call for one sub will be repatriated"]
                    #[doc = "to the sender."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_ and the sender must have a registered"]
                    #[doc = "sub identity of `sub`."]
                    add_sub {
                        sub: ::subxt::ext::subxt_core::utils::MultiAddress<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            (),
                        >,
                        data: runtime_types::pallet_identity::types::Data,
                    },
                    #[codec(index = 12)]
                    #[doc = "Alter the associated name of the given sub-account."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_ and the sender must have a registered"]
                    #[doc = "sub identity of `sub`."]
                    rename_sub {
                        sub: ::subxt::ext::subxt_core::utils::MultiAddress<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            (),
                        >,
                        data: runtime_types::pallet_identity::types::Data,
                    },
                    #[codec(index = 13)]
                    #[doc = "Remove the given account from the sender's subs."]
                    #[doc = ""]
                    #[doc = "Payment: Balance reserved by a previous `set_subs` call for one sub will be repatriated"]
                    #[doc = "to the sender."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_ and the sender must have a registered"]
                    #[doc = "sub identity of `sub`."]
                    remove_sub {
                        sub: ::subxt::ext::subxt_core::utils::MultiAddress<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            (),
                        >,
                    },
                    #[codec(index = 14)]
                    #[doc = "Remove the sender as a sub-account."]
                    #[doc = ""]
                    #[doc = "Payment: Balance reserved by a previous `set_subs` call for one sub will be repatriated"]
                    #[doc = "to the sender (*not* the original depositor)."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_ and the sender must have a registered"]
                    #[doc = "super-identity."]
                    #[doc = ""]
                    #[doc = "NOTE: This should not normally be used, but is provided in the case that the non-"]
                    #[doc = "controller of an account is maliciously registered as a sub-account."]
                    quit_sub,
                    #[codec(index = 15)]
                    #[doc = "Add an `AccountId` with permission to grant usernames with a given `suffix` appended."]
                    #[doc = ""]
                    #[doc = "The authority can grant up to `allocation` usernames. To top up the allocation or"]
                    #[doc = "change the account used to grant usernames, this call can be used with the updated"]
                    #[doc = "parameters to overwrite the existing configuration."]
                    add_username_authority {
                        authority: ::subxt::ext::subxt_core::utils::MultiAddress<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            (),
                        >,
                        suffix: ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                        allocation: ::core::primitive::u32,
                    },
                    #[codec(index = 16)]
                    #[doc = "Remove `authority` from the username authorities."]
                    remove_username_authority {
                        suffix: ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                        authority: ::subxt::ext::subxt_core::utils::MultiAddress<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            (),
                        >,
                    },
                    #[codec(index = 17)]
                    #[doc = "Set the username for `who`. Must be called by a username authority."]
                    #[doc = ""]
                    #[doc = "If `use_allocation` is set, the authority must have a username allocation available to"]
                    #[doc = "spend. Otherwise, the authority will need to put up a deposit for registering the"]
                    #[doc = "username."]
                    #[doc = ""]
                    #[doc = "Users can either pre-sign their usernames or"]
                    #[doc = "accept them later."]
                    #[doc = ""]
                    #[doc = "Usernames must:"]
                    #[doc = "  - Only contain lowercase ASCII characters or digits."]
                    #[doc = "  - When combined with the suffix of the issuing authority be _less than_ the"]
                    #[doc = "    `MaxUsernameLength`."]
                    set_username_for {
                        who: ::subxt::ext::subxt_core::utils::MultiAddress<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            (),
                        >,
                        username: ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                        signature:
                            ::core::option::Option<runtime_types::sp_runtime::MultiSignature>,
                        use_allocation: ::core::primitive::bool,
                    },
                    #[codec(index = 18)]
                    #[doc = "Accept a given username that an `authority` granted. The call must include the full"]
                    #[doc = "username, as in `username.suffix`. Authorities cannot grant usernames to people, only"]
                    #[doc = "to regular accounts."]
                    accept_username {
                        username: runtime_types::bounded_collections::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                    },
                    #[codec(index = 19)]
                    #[doc = "Remove an expired username approval. The username was approved by an authority but never"]
                    #[doc = "accepted by the user and must now be beyond its expiration. The call must include the"]
                    #[doc = "full username, as in `username.suffix`."]
                    remove_expired_approval {
                        username: runtime_types::bounded_collections::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                    },
                    #[codec(index = 20)]
                    #[doc = "Set a given username as the primary. The username should include the suffix. Only"]
                    #[doc = "regular accounts can set their primary username, as people can only have one username,"]
                    #[doc = "granted through a system allocation."]
                    set_primary_username {
                        username: runtime_types::bounded_collections::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                    },
                    #[codec(index = 21)]
                    #[doc = "Start the process of removing a username by placing it in the unbinding usernames map."]
                    #[doc = "Once the grace period has passed, the username can be deleted by calling"]
                    #[doc = "[remove_username](crate::Call::remove_username)."]
                    unbind_username {
                        username: runtime_types::bounded_collections::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                    },
                    #[codec(index = 22)]
                    #[doc = "Permanently delete a username which has been unbinding for longer than the grace period."]
                    #[doc = "Caller is refunded the fee if the username expired and the removal was successful."]
                    remove_username {
                        username: runtime_types::bounded_collections::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                    },
                    #[codec(index = 23)]
                    #[doc = "Call with [ForceOrigin](crate::Config::ForceOrigin) privileges which deletes a username"]
                    #[doc = "and slashes any deposit associated with it."]
                    kill_username {
                        username: runtime_types::bounded_collections::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                    },
                    #[codec(index = 24)]
                    #[doc = "Sets the username and on-chain account for a person, along with an empty identity which"]
                    #[doc = "can only be populated using an oracle through the social credential verification system."]
                    #[doc = "The chosen username must be domainless."]
                    #[doc = ""]
                    #[doc = "The sender must sign the alias using the key associated with the provided on-chain"]
                    #[doc = "account to prove ownership."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be the contextual alias of the person and the"]
                    #[doc = "sender must not have already registered their identity and username."]
                    #[doc = ""]
                    #[doc = "Emits `PersonalIdentitySet` if successful."]
                    set_personal_identity {
                        account: ::subxt::ext::subxt_core::utils::AccountId32,
                        signature: runtime_types::sp_runtime::MultiSignature,
                        username: runtime_types::bounded_collections::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                    },
                    #[codec(index = 25)]
                    #[doc = "Open a case for an oracle to judge a social credential of a person."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be the contextual alias of the person and the"]
                    #[doc = "sender must have a registered identity."]
                    #[doc = ""]
                    #[doc = "Emits `EvidenceSubmitted` if successful."]
                    submit_personal_credential_evidence {
                        credential:
                            runtime_types::individuality_support::traits::reality::identity::Social,
                    },
                    #[codec(index = 26)]
                    #[doc = "Callback to enforce the judgement of a social credential. This is to be called only by"]
                    #[doc = "the oracle that judged the case."]
                    personal_credential_judged {
                        ticket: ::core::primitive::u32,
                        context: runtime_types::bounded_collections::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                        judgement: runtime_types::individuality_support::traits::reality::Judgement,
                    },
                    #[codec(index = 27)]
                    #[doc = "Clear a person's identity info. The sender must pay a penalty through the associated"]
                    #[doc = "alias account for removing the identity."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be the contextual alias of the person and the"]
                    #[doc = "sender must have a registered identity."]
                    #[doc = ""]
                    #[doc = "Emits `IdentityPersonalCleared` if successful."]
                    clear_personal_identity,
                    #[codec(index = 28)]
                    #[doc = "Report a username as invalid/offensive/wrong/anything else."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be a signed extrinsic."]
                    #[doc = ""]
                    #[doc = "- `username`: The username to be reported."]
                    report_username {
                        username: runtime_types::bounded_collections::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                    },
                    #[codec(index = 29)]
                    #[doc = "Given judgement on a previously reported username either:"]
                    #[doc = "1. In case username was found valid:"]
                    #[doc = "- slashes the amount deposited by the reporter,"]
                    #[doc = "- does nothing to the identity behind the reported username,"]
                    #[doc = "2. In case username was found invalid:"]
                    #[doc = "- returns the deposited funds to the reporter,"]
                    #[doc = "- bans the identity behind the reported username."]
                    #[doc = "Serves as a callback used upon judgement request to the oracle."]
                    #[doc = "Must only be used by the statement oracle."]
                    reported_username_judged {
                        ticket: ::core::primitive::u32,
                        context: runtime_types::bounded_collections::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                        judgement: runtime_types::individuality_support::traits::reality::Judgement,
                    },
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "The `Error` enum of this pallet."]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "Too many subs-accounts."]
                    TooManySubAccounts,
                    #[codec(index = 1)]
                    #[doc = "No alias found for an account."]
                    NoAlias,
                    #[codec(index = 2)]
                    #[doc = "Account isn't found."]
                    NotFound,
                    #[codec(index = 3)]
                    #[doc = "Account isn't named."]
                    NotNamed,
                    #[codec(index = 4)]
                    #[doc = "Empty index."]
                    EmptyIndex,
                    #[codec(index = 5)]
                    #[doc = "Fee is changed."]
                    FeeChanged,
                    #[codec(index = 6)]
                    #[doc = "No identity found."]
                    NoIdentity,
                    #[codec(index = 7)]
                    #[doc = "Sticky judgement."]
                    StickyJudgement,
                    #[codec(index = 8)]
                    #[doc = "Judgement given."]
                    JudgementGiven,
                    #[codec(index = 9)]
                    #[doc = "Invalid judgement."]
                    InvalidJudgement,
                    #[codec(index = 10)]
                    #[doc = "The index is invalid."]
                    InvalidIndex,
                    #[codec(index = 11)]
                    #[doc = "The target is invalid."]
                    InvalidTarget,
                    #[codec(index = 12)]
                    #[doc = "Maximum amount of registrars reached. Cannot add any more."]
                    TooManyRegistrars,
                    #[codec(index = 13)]
                    #[doc = "Account ID is already named."]
                    AlreadyClaimed,
                    #[codec(index = 14)]
                    #[doc = "Username has already been reported."]
                    AlreadyReported,
                    #[codec(index = 15)]
                    #[doc = "Sender is not a sub-account."]
                    NotSub,
                    #[codec(index = 16)]
                    #[doc = "Sub-account isn't owned by sender."]
                    NotOwned,
                    #[codec(index = 17)]
                    #[doc = "The provided judgement was for a different identity."]
                    JudgementForDifferentIdentity,
                    #[codec(index = 18)]
                    #[doc = "Error that occurs when there is an issue paying for judgement."]
                    JudgementPaymentFailed,
                    #[codec(index = 19)]
                    #[doc = "The provided suffix is too long."]
                    InvalidSuffix,
                    #[codec(index = 20)]
                    #[doc = "The sender does not have permission to issue a username."]
                    NotUsernameAuthority,
                    #[codec(index = 21)]
                    #[doc = "The authority cannot allocate any more usernames."]
                    NoAllocation,
                    #[codec(index = 22)]
                    #[doc = "The signature on a username was not valid."]
                    InvalidSignature,
                    #[codec(index = 23)]
                    #[doc = "Setting this username requires a signature, but none was provided."]
                    RequiresSignature,
                    #[codec(index = 24)]
                    #[doc = "The username does not meet the requirements."]
                    InvalidUsername,
                    #[codec(index = 25)]
                    #[doc = "The username is already taken."]
                    UsernameTaken,
                    #[codec(index = 26)]
                    #[doc = "The requested username does not exist."]
                    NoUsername,
                    #[codec(index = 27)]
                    #[doc = "The reported username was not provided by the system."]
                    NotSystemProvidedUsername,
                    #[codec(index = 28)]
                    #[doc = "The username cannot be forcefully removed because it can still be accepted."]
                    NotExpired,
                    #[codec(index = 29)]
                    #[doc = "The username cannot be removed because it's still in the grace period."]
                    TooEarly,
                    #[codec(index = 30)]
                    #[doc = "The username cannot be removed because it is not unbinding."]
                    NotUnbinding,
                    #[codec(index = 31)]
                    #[doc = "The username cannot be unbound because it is already unbinding."]
                    AlreadyUnbinding,
                    #[codec(index = 32)]
                    #[doc = "The action cannot be performed because of insufficient privileges (e.g. authority"]
                    #[doc = "trying to unbind a username provided by the system)."]
                    InsufficientPrivileges,
                    #[codec(index = 33)]
                    #[doc = "The context in which the alias was used is not supported."]
                    BadContext,
                    #[codec(index = 34)]
                    #[doc = "No associated request for the judgement received."]
                    UnexpectedJudgement,
                    #[codec(index = 35)]
                    #[doc = "The social credential is not supported by the configured identity information provider."]
                    NotSupported,
                    #[codec(index = 36)]
                    #[doc = "The person is banned and cannot perform the operation."]
                    Banned,
                    #[codec(index = 37)]
                    #[doc = "The person already has a personal identity associated with their alias."]
                    AlreadyRegistered,
                    #[codec(index = 38)]
                    #[doc = "The list of judgements ongoing on a personal identity is full."]
                    JudgementListFull,
                    #[codec(index = 39)]
                    #[doc = "The username has been reported too recently."]
                    LastUsernameReportTooRecent,
                    #[codec(index = 40)]
                    #[doc = "The username has been reported and is undergoing validity judgement."]
                    UsernameJudgementOngoing,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "The `Event` enum of this pallet"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "A name was set or reset (which will remove all judgements)."]
                    IdentitySet {
                        who: ::subxt::ext::subxt_core::utils::AccountId32,
                    },
                    #[codec(index = 1)]
                    #[doc = "A name was cleared, and the given balance returned."]
                    IdentityCleared {
                        who: ::subxt::ext::subxt_core::utils::AccountId32,
                        deposit: ::core::primitive::u128,
                    },
                    #[codec(index = 2)]
                    #[doc = "A name was removed and the given balance slashed."]
                    IdentityKilled {
                        who: ::subxt::ext::subxt_core::utils::AccountId32,
                        deposit: ::core::primitive::u128,
                    },
                    #[codec(index = 3)]
                    #[doc = "A judgement was asked from a registrar."]
                    JudgementRequested {
                        who: ::subxt::ext::subxt_core::utils::AccountId32,
                        registrar_index: ::core::primitive::u32,
                    },
                    #[codec(index = 4)]
                    #[doc = "A judgement request was retracted."]
                    JudgementUnrequested {
                        who: ::subxt::ext::subxt_core::utils::AccountId32,
                        registrar_index: ::core::primitive::u32,
                    },
                    #[codec(index = 5)]
                    #[doc = "A judgement was given by a registrar."]
                    JudgementGiven {
                        target: ::subxt::ext::subxt_core::utils::AccountId32,
                        registrar_index: ::core::primitive::u32,
                    },
                    #[codec(index = 6)]
                    #[doc = "A registrar was added."]
                    RegistrarAdded {
                        registrar_index: ::core::primitive::u32,
                    },
                    #[codec(index = 7)]
                    #[doc = "A sub-identity was added to an identity and the deposit paid."]
                    SubIdentityAdded {
                        sub: ::subxt::ext::subxt_core::utils::AccountId32,
                        main: ::subxt::ext::subxt_core::utils::AccountId32,
                        deposit: ::core::primitive::u128,
                    },
                    #[codec(index = 8)]
                    #[doc = "An account's sub-identities were set (in bulk)."]
                    SubIdentitiesSet {
                        main: ::subxt::ext::subxt_core::utils::AccountId32,
                        number_of_subs: ::core::primitive::u32,
                        new_deposit: ::core::primitive::u128,
                    },
                    #[codec(index = 9)]
                    #[doc = "A given sub-account's associated name was changed by its super-identity."]
                    SubIdentityRenamed {
                        sub: ::subxt::ext::subxt_core::utils::AccountId32,
                        main: ::subxt::ext::subxt_core::utils::AccountId32,
                    },
                    #[codec(index = 10)]
                    #[doc = "A sub-identity was removed from an identity and the deposit freed."]
                    SubIdentityRemoved {
                        sub: ::subxt::ext::subxt_core::utils::AccountId32,
                        main: ::subxt::ext::subxt_core::utils::AccountId32,
                        deposit: ::core::primitive::u128,
                    },
                    #[codec(index = 11)]
                    #[doc = "A sub-identity was cleared, and the given deposit repatriated from the"]
                    #[doc = "main identity account to the sub-identity account."]
                    SubIdentityRevoked {
                        sub: ::subxt::ext::subxt_core::utils::AccountId32,
                        main: ::subxt::ext::subxt_core::utils::AccountId32,
                        deposit: ::core::primitive::u128,
                    },
                    #[codec(index = 12)]
                    #[doc = "A username authority was added."]
                    AuthorityAdded {
                        authority: ::subxt::ext::subxt_core::utils::AccountId32,
                    },
                    #[codec(index = 13)]
                    #[doc = "A username authority was removed."]
                    AuthorityRemoved {
                        authority: ::subxt::ext::subxt_core::utils::AccountId32,
                    },
                    #[codec(index = 14)]
                    #[doc = "A username was set for `who`."]
                    UsernameSet {
                        who: ::subxt::ext::subxt_core::utils::AccountId32,
                        username: runtime_types::bounded_collections::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                    },
                    #[codec(index = 15)]
                    #[doc = "A username was queued, but `who` must accept it prior to `expiration`."]
                    UsernameQueued {
                        who: ::subxt::ext::subxt_core::utils::AccountId32,
                        username: runtime_types::bounded_collections::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                        expiration: ::core::primitive::u32,
                    },
                    #[codec(index = 16)]
                    #[doc = "A queued username passed its expiration without being claimed and was removed."]
                    PreapprovalExpired {
                        whose: ::subxt::ext::subxt_core::utils::AccountId32,
                    },
                    #[codec(index = 17)]
                    #[doc = "A username was set as a primary and can be looked up from `who`."]
                    PrimaryUsernameSet {
                        who: ::subxt::ext::subxt_core::utils::AccountId32,
                        username: runtime_types::bounded_collections::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                    },
                    #[codec(index = 18)]
                    #[doc = "A dangling username (as in, a username corresponding to an account that has removed its"]
                    #[doc = "identity) has been removed."]
                    DanglingUsernameRemoved {
                        who: ::subxt::ext::subxt_core::utils::AccountId32,
                        username: runtime_types::bounded_collections::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                    },
                    #[codec(index = 19)]
                    #[doc = "A username has been unbound."]
                    UsernameUnbound {
                        username: runtime_types::bounded_collections::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                    },
                    #[codec(index = 20)]
                    #[doc = "A username has been removed."]
                    UsernameRemoved {
                        username: runtime_types::bounded_collections::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                    },
                    #[codec(index = 21)]
                    #[doc = "A username has been killed."]
                    UsernameKilled {
                        username: runtime_types::bounded_collections::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                    },
                    #[codec(index = 22)]
                    #[doc = "A username has been reported."]
                    UsernameReported {
                        reporter: ::subxt::ext::subxt_core::utils::AccountId32,
                        username: runtime_types::bounded_collections::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                    },
                    #[codec(index = 23)]
                    #[doc = "An identity for a person has been set."]
                    PersonalIdentitySet {
                        alias: [::core::primitive::u8; 32usize],
                        account: ::subxt::ext::subxt_core::utils::AccountId32,
                        username: runtime_types::bounded_collections::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                    },
                    #[codec(index = 24)]
                    #[doc = "Evidence for a credential has beed submitted by a person."]
                    EvidenceSubmitted {
                        alias: [::core::primitive::u8; 32usize],
                    },
                    #[codec(index = 25)]
                    #[doc = "A credential was accepted for a person."]
                    CredentialAccepted {
                        alias: [::core::primitive::u8; 32usize],
                    },
                    #[codec(index = 26)]
                    #[doc = "A credential was rejected for a person."]
                    CredentialRejected {
                        alias: [::core::primitive::u8; 32usize],
                    },
                    #[codec(index = 27)]
                    #[doc = "Person was banned for sending contemptuous evidence."]
                    PersonBanned {
                        alias: [::core::primitive::u8; 32usize],
                    },
                    #[codec(index = 28)]
                    #[doc = "A personal identity was cleared."]
                    PersonalIdentityCleared {
                        alias: [::core::primitive::u8; 32usize],
                    },
                    #[codec(index = 29)]
                    #[doc = "A reported username was judged valid."]
                    ReportedUsernameJudgedValid {
                        username: runtime_types::bounded_collections::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                    },
                    #[codec(index = 30)]
                    #[doc = "A reported username was judged invalid."]
                    ReportedUsernameJudgedInvalid {
                        username: runtime_types::bounded_collections::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                    },
                    #[codec(index = 31)]
                    #[doc = "The judgment regarding a reported username was unclear or uncertain."]
                    ReportedUsernameWeakOrUnclearJudgement {
                        username: runtime_types::bounded_collections::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                        judgement: runtime_types::individuality_support::traits::reality::Judgement,
                    },
                }
            }
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub enum Data {
                    #[codec(index = 0)]
                    None,
                    #[codec(index = 1)]
                    Raw0([::core::primitive::u8; 0usize]),
                    #[codec(index = 2)]
                    Raw1([::core::primitive::u8; 1usize]),
                    #[codec(index = 3)]
                    Raw2([::core::primitive::u8; 2usize]),
                    #[codec(index = 4)]
                    Raw3([::core::primitive::u8; 3usize]),
                    #[codec(index = 5)]
                    Raw4([::core::primitive::u8; 4usize]),
                    #[codec(index = 6)]
                    Raw5([::core::primitive::u8; 5usize]),
                    #[codec(index = 7)]
                    Raw6([::core::primitive::u8; 6usize]),
                    #[codec(index = 8)]
                    Raw7([::core::primitive::u8; 7usize]),
                    #[codec(index = 9)]
                    Raw8([::core::primitive::u8; 8usize]),
                    #[codec(index = 10)]
                    Raw9([::core::primitive::u8; 9usize]),
                    #[codec(index = 11)]
                    Raw10([::core::primitive::u8; 10usize]),
                    #[codec(index = 12)]
                    Raw11([::core::primitive::u8; 11usize]),
                    #[codec(index = 13)]
                    Raw12([::core::primitive::u8; 12usize]),
                    #[codec(index = 14)]
                    Raw13([::core::primitive::u8; 13usize]),
                    #[codec(index = 15)]
                    Raw14([::core::primitive::u8; 14usize]),
                    #[codec(index = 16)]
                    Raw15([::core::primitive::u8; 15usize]),
                    #[codec(index = 17)]
                    Raw16([::core::primitive::u8; 16usize]),
                    #[codec(index = 18)]
                    Raw17([::core::primitive::u8; 17usize]),
                    #[codec(index = 19)]
                    Raw18([::core::primitive::u8; 18usize]),
                    #[codec(index = 20)]
                    Raw19([::core::primitive::u8; 19usize]),
                    #[codec(index = 21)]
                    Raw20([::core::primitive::u8; 20usize]),
                    #[codec(index = 22)]
                    Raw21([::core::primitive::u8; 21usize]),
                    #[codec(index = 23)]
                    Raw22([::core::primitive::u8; 22usize]),
                    #[codec(index = 24)]
                    Raw23([::core::primitive::u8; 23usize]),
                    #[codec(index = 25)]
                    Raw24([::core::primitive::u8; 24usize]),
                    #[codec(index = 26)]
                    Raw25([::core::primitive::u8; 25usize]),
                    #[codec(index = 27)]
                    Raw26([::core::primitive::u8; 26usize]),
                    #[codec(index = 28)]
                    Raw27([::core::primitive::u8; 27usize]),
                    #[codec(index = 29)]
                    Raw28([::core::primitive::u8; 28usize]),
                    #[codec(index = 30)]
                    Raw29([::core::primitive::u8; 29usize]),
                    #[codec(index = 31)]
                    Raw30([::core::primitive::u8; 30usize]),
                    #[codec(index = 32)]
                    Raw31([::core::primitive::u8; 31usize]),
                    #[codec(index = 33)]
                    Raw32([::core::primitive::u8; 32usize]),
                    #[codec(index = 34)]
                    BlakeTwo256([::core::primitive::u8; 32usize]),
                    #[codec(index = 35)]
                    Sha256([::core::primitive::u8; 32usize]),
                    #[codec(index = 36)]
                    Keccak256([::core::primitive::u8; 32usize]),
                    #[codec(index = 37)]
                    ShaThree256([::core::primitive::u8; 32usize]),
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub enum Judgement<_0> {
                    #[codec(index = 0)]
                    Unknown,
                    #[codec(index = 1)]
                    FeePaid(_0),
                    #[codec(index = 2)]
                    Reasonable,
                    #[codec(index = 3)]
                    KnownGood,
                    #[codec(index = 4)]
                    OutOfDate,
                    #[codec(index = 5)]
                    LowQuality,
                    #[codec(index = 6)]
                    Erroneous,
                    #[codec(index = 7)]
                    External,
                }
            }
        }
        pub mod pallet_lottery {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Reset the schedules."]
                    #[doc = ""]
                    #[doc = "The origin must be `ScheduleOrigin`."]
                    #[doc = ""]
                    #[doc = "This will remove all schedule events and release all their held funds."]
                    #[doc = "This would usually be used batched with another call `add_schedule_event` in order to"]
                    #[doc = "set new schedules."]
                    reset_schedule_event,
                    #[codec(index = 1)]
                    #[doc = "Add some schedules after the current schedules."]
                    #[doc = ""]
                    #[doc = "The origin must be `ScheduleOrigin`."]
                    #[doc = ""]
                    #[doc = "This adds schedule events after current schedules. To schedule events in between current"]
                    #[doc = "schedules, a batch call with `reset_schedule_event` and then `add_schedule_event` can be"]
                    #[doc = "used."]
                    #[doc = ""]
                    #[doc = "The schedule events must not overlap with previous schedules, their registration ends"]
                    #[doc = "must be later that previous schedule registration end."]
                    #[doc = "All fungible funds in the schedule must be non-zero, and there should be at least one"]
                    #[doc = "fungible fund."]
                    #[doc = "The distribution must not be empty, and all weights must be non-zero."]
                    #[doc = ""]
                    #[doc = "The is no strong check that the schedule events are sufficiently spaced in time, but if"]
                    #[doc = "a schedule event doesn't start `MinimumTimeToRegister` seconds before its configured end"]
                    #[doc = "of registration then it will be skipped."]
                    #[doc = "To avoid skipped events, the events should be spaced by at least:"]
                    #[doc = "```math"]
                    #[doc = "MinimumTimeToRegister + TimeToClaim + \"time to build entropy; probably less than a day\""]
                    #[doc = "```"]
                    add_schedule_event {
                        events: ::subxt::ext::subxt_core::alloc::vec::Vec<
                            runtime_types::pallet_lottery::pallet::LotteryEvent<
                                ::core::primitive::u32,
                                ::core::primitive::u128,
                            >,
                        >,
                    },
                    #[codec(index = 2)]
                    #[doc = "Register for the current event."]
                    #[doc = ""]
                    #[doc = "The origin must be the alias of a person in the context: lottery context base"]
                    #[doc = "concatenated the current lottery id in little-endian encoding in 4 bytes."]
                    #[doc = "The lottery context base can be found in the metadata."]
                    #[doc = ""]
                    #[doc = "There must be an active event, the event must be in the `Registering` phase, and the"]
                    #[doc = "alias must not be already registered."]
                    #[doc = ""]
                    #[doc = "When registering the caller is assigned an index. This index will be used to identify"]
                    #[doc = "the winners."]
                    register_for_event {
                        destination: ::subxt::ext::subxt_core::utils::AccountId32,
                    },
                    #[codec(index = 3)]
                    #[doc = "Claim prize for the current event."]
                    #[doc = ""]
                    #[doc = "The origin must be the alias of a person in the context: lottery context base"]
                    #[doc = "concatenated the current lottery id in little-endian encoding in 4 bytes."]
                    #[doc = "The lottery context base can be found in the metadata."]
                    #[doc = ""]
                    #[doc = "There must be an active event, the event must be in the `Claiming` phase, and the"]
                    #[doc = "caller index must be a winner."]
                    #[doc = ""]
                    #[doc = "NOTE: The destination account might require some minimum balance if the reward assets"]
                    #[doc = "are not sufficient and/or the reward is lower than the minimum balance."]
                    claim_prize {
                        destination: ::subxt::ext::subxt_core::utils::AccountId32,
                    },
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "The `Error` enum of this pallet."]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "No active lottery event."]
                    NoActiveEvent,
                    #[codec(index = 1)]
                    #[doc = "The origin is not a person."]
                    NotAPerson,
                    #[codec(index = 2)]
                    #[doc = "The alias is already registered."]
                    AliasAlreadyRegistered,
                    #[codec(index = 3)]
                    #[doc = "The event registration time is invalid."]
                    InvalidEventRegistrationTime,
                    #[codec(index = 4)]
                    #[doc = "Participant ID overflow."]
                    ParticipantIdOverflow,
                    #[codec(index = 5)]
                    #[doc = "Too many event schedules."]
                    TooManyEventSchedule,
                    #[codec(index = 6)]
                    #[doc = "The event registration overlaps with the previous one."]
                    EventRegistrationOverlap,
                    #[codec(index = 7)]
                    #[doc = "The scheduled event is not found."]
                    ScheduledEventNotFound,
                    #[codec(index = 8)]
                    #[doc = "The proof is invalid."]
                    InvalidProof,
                    #[codec(index = 9)]
                    #[doc = "The event is not in the claiming status."]
                    InvalidEventClaimingTime,
                    #[codec(index = 10)]
                    #[doc = "The participant is not registered."]
                    NotRegistered,
                    #[codec(index = 11)]
                    #[doc = "The participant has no prize."]
                    NoPrize,
                    #[codec(index = 12)]
                    #[doc = "Some asset in the event registration has zero amount."]
                    EventRegistrationSomeAssetWithZeroAmount,
                    #[codec(index = 13)]
                    #[doc = "The event registration has empty distribution."]
                    EventRegistrationEmptyDistribution,
                    #[codec(index = 14)]
                    #[doc = "The event registration has no funds."]
                    EventRegistrationNoFunds,
                    #[codec(index = 15)]
                    #[doc = "The event registration distribution has some zero weight."]
                    EventRegistrationDistributionWithZeroWeight,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "The `Event` enum of this pallet"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "A participant registered for an event."]
                    ParticipantRegistered {
                        participant_id: ::core::primitive::u32,
                        alias: [::core::primitive::u8; 32usize],
                        destination: ::subxt::ext::subxt_core::utils::AccountId32,
                    },
                    #[codec(index = 1)]
                    #[doc = "The schedule is updated"]
                    ScheduleUpdate {
                        events: runtime_types::bounded_collections::bounded_vec::BoundedVec<
                            runtime_types::pallet_lottery::pallet::LotteryEvent<
                                ::core::primitive::u32,
                                ::core::primitive::u128,
                            >,
                        >,
                    },
                    #[codec(index = 2)]
                    #[doc = "The event is started."]
                    EventStarted {
                        id: ::core::primitive::u32,
                        event: runtime_types::pallet_lottery::pallet::LotteryEvent<
                            ::core::primitive::u32,
                            ::core::primitive::u128,
                        >,
                    },
                    #[codec(index = 3)]
                    #[doc = "The scheduled event is skipped."]
                    EventSkipped {
                        event: runtime_types::pallet_lottery::pallet::LotteryEvent<
                            ::core::primitive::u32,
                            ::core::primitive::u128,
                        >,
                    },
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub enum HoldReason {
                    #[codec(index = 0)]
                    Lottery,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct LotteryEvent<_0, _1> {
                    pub fungibles:
                        runtime_types::bounded_collections::bounded_vec::BoundedVec<(_0, _1)>,
                    pub distribution:
                        runtime_types::bounded_collections::bounded_vec::BoundedVec<_0>,
                    pub registration_end_secs: ::core::primitive::u32,
                }
            }
        }
        pub mod pallet_message_queue {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Remove a page which has no more messages remaining to be processed or is stale."]
                    reap_page {
                        message_origin:
                            runtime_types::cumulus_primitives_core::AggregateMessageOrigin,
                        page_index: ::core::primitive::u32,
                    },
                    #[codec(index = 1)]
                    #[doc = "Execute an overweight message."]
                    #[doc = ""]
                    #[doc = "Temporary processing errors will be propagated whereas permanent errors are treated"]
                    #[doc = "as success condition."]
                    #[doc = ""]
                    #[doc = "- `origin`: Must be `Signed`."]
                    #[doc = "- `message_origin`: The origin from which the message to be executed arrived."]
                    #[doc = "- `page`: The page in the queue in which the message to be executed is sitting."]
                    #[doc = "- `index`: The index into the queue of the message to be executed."]
                    #[doc = "- `weight_limit`: The maximum amount of weight allowed to be consumed in the execution"]
                    #[doc = "  of the message."]
                    #[doc = ""]
                    #[doc = "Benchmark complexity considerations: O(index + weight_limit)."]
                    execute_overweight {
                        message_origin:
                            runtime_types::cumulus_primitives_core::AggregateMessageOrigin,
                        page: ::core::primitive::u32,
                        index: ::core::primitive::u32,
                        weight_limit: runtime_types::sp_weights::weight_v2::Weight,
                    },
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "The `Error` enum of this pallet."]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "Page is not reapable because it has items remaining to be processed and is not old"]
                    #[doc = "enough."]
                    NotReapable,
                    #[codec(index = 1)]
                    #[doc = "Page to be reaped does not exist."]
                    NoPage,
                    #[codec(index = 2)]
                    #[doc = "The referenced message could not be found."]
                    NoMessage,
                    #[codec(index = 3)]
                    #[doc = "The message was already processed and cannot be processed again."]
                    AlreadyProcessed,
                    #[codec(index = 4)]
                    #[doc = "The message is queued for future execution."]
                    Queued,
                    #[codec(index = 5)]
                    #[doc = "There is temporarily not enough weight to continue servicing messages."]
                    InsufficientWeight,
                    #[codec(index = 6)]
                    #[doc = "This message is temporarily unprocessable."]
                    #[doc = ""]
                    #[doc = "Such errors are expected, but not guaranteed, to resolve themselves eventually through"]
                    #[doc = "retrying."]
                    TemporarilyUnprocessable,
                    #[codec(index = 7)]
                    #[doc = "The queue is paused and no message can be executed from it."]
                    #[doc = ""]
                    #[doc = "This can change at any time and may resolve in the future by re-trying."]
                    QueuePaused,
                    #[codec(index = 8)]
                    #[doc = "Another call is in progress and needs to finish before this call can happen."]
                    RecursiveDisallowed,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "The `Event` enum of this pallet"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "Message discarded due to an error in the `MessageProcessor` (usually a format error)."]
                    ProcessingFailed {
                        id: ::subxt::ext::subxt_core::utils::H256,
                        origin: runtime_types::cumulus_primitives_core::AggregateMessageOrigin,
                        error: runtime_types::frame_support::traits::messages::ProcessMessageError,
                    },
                    #[codec(index = 1)]
                    #[doc = "Message is processed."]
                    Processed {
                        id: ::subxt::ext::subxt_core::utils::H256,
                        origin: runtime_types::cumulus_primitives_core::AggregateMessageOrigin,
                        weight_used: runtime_types::sp_weights::weight_v2::Weight,
                        success: ::core::primitive::bool,
                    },
                    #[codec(index = 2)]
                    #[doc = "Message placed in overweight queue."]
                    OverweightEnqueued {
                        id: [::core::primitive::u8; 32usize],
                        origin: runtime_types::cumulus_primitives_core::AggregateMessageOrigin,
                        page_index: ::core::primitive::u32,
                        message_index: ::core::primitive::u32,
                    },
                    #[codec(index = 3)]
                    #[doc = "This page was reaped."]
                    PageReaped {
                        origin: runtime_types::cumulus_primitives_core::AggregateMessageOrigin,
                        index: ::core::primitive::u32,
                    },
                }
            }
        }
        pub mod pallet_migrations {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Allows root to set a cursor to forcefully start, stop or forward the migration process."]
                    #[doc = ""]
                    #[doc = "Should normally not be needed and is only in place as emergency measure. Note that"]
                    #[doc = "restarting the migration process in this manner will not call the"]
                    #[doc = "[`MigrationStatusHandler::started`] hook or emit an `UpgradeStarted` event."]
                    force_set_cursor {
                        cursor: ::core::option::Option<
                            runtime_types::pallet_migrations::MigrationCursor<
                                runtime_types::bounded_collections::bounded_vec::BoundedVec<
                                    ::core::primitive::u8,
                                >,
                                ::core::primitive::u32,
                            >,
                        >,
                    },
                    #[codec(index = 1)]
                    #[doc = "Allows root to set an active cursor to forcefully start/forward the migration process."]
                    #[doc = ""]
                    #[doc = "This is an edge-case version of [`Self::force_set_cursor`] that allows to set the"]
                    #[doc = "`started_at` value to the next block number. Otherwise this would not be possible, since"]
                    #[doc = "`force_set_cursor` takes an absolute block number. Setting `started_at` to `None`"]
                    #[doc = "indicates that the current block number plus one should be used."]
                    force_set_active_cursor {
                        index: ::core::primitive::u32,
                        inner_cursor: ::core::option::Option<
                            runtime_types::bounded_collections::bounded_vec::BoundedVec<
                                ::core::primitive::u8,
                            >,
                        >,
                        started_at: ::core::option::Option<::core::primitive::u32>,
                    },
                    #[codec(index = 2)]
                    #[doc = "Forces the onboarding of the migrations."]
                    #[doc = ""]
                    #[doc = "This process happens automatically on a runtime upgrade. It is in place as an emergency"]
                    #[doc = "measurement. The cursor needs to be `None` for this to succeed."]
                    force_onboard_mbms,
                    #[codec(index = 3)]
                    #[doc = "Clears the `Historic` set."]
                    #[doc = ""]
                    #[doc = "`map_cursor` must be set to the last value that was returned by the"]
                    #[doc = "`HistoricCleared` event. The first time `None` can be used. `limit` must be chosen in a"]
                    #[doc = "way that will result in a sensible weight."]
                    clear_historic {
                        selector: runtime_types::pallet_migrations::HistoricCleanupSelector<
                            runtime_types::bounded_collections::bounded_vec::BoundedVec<
                                ::core::primitive::u8,
                            >,
                        >,
                    },
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "The `Error` enum of this pallet."]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "The operation cannot complete since some MBMs are ongoing."]
                    Ongoing,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "The `Event` enum of this pallet"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "A Runtime upgrade started."]
                    #[doc = ""]
                    #[doc = "Its end is indicated by `UpgradeCompleted` or `UpgradeFailed`."]
                    UpgradeStarted { migrations: ::core::primitive::u32 },
                    #[codec(index = 1)]
                    #[doc = "The current runtime upgrade completed."]
                    #[doc = ""]
                    #[doc = "This implies that all of its migrations completed successfully as well."]
                    UpgradeCompleted,
                    #[codec(index = 2)]
                    #[doc = "Runtime upgrade failed."]
                    #[doc = ""]
                    #[doc = "This is very bad and will require governance intervention."]
                    UpgradeFailed,
                    #[codec(index = 3)]
                    #[doc = "A migration was skipped since it was already executed in the past."]
                    MigrationSkipped { index: ::core::primitive::u32 },
                    #[codec(index = 4)]
                    #[doc = "A migration progressed."]
                    MigrationAdvanced {
                        index: ::core::primitive::u32,
                        took: ::core::primitive::u32,
                    },
                    #[codec(index = 5)]
                    #[doc = "A Migration completed."]
                    MigrationCompleted {
                        index: ::core::primitive::u32,
                        took: ::core::primitive::u32,
                    },
                    #[codec(index = 6)]
                    #[doc = "A Migration failed."]
                    #[doc = ""]
                    #[doc = "This implies that the whole upgrade failed and governance intervention is required."]
                    MigrationFailed {
                        index: ::core::primitive::u32,
                        took: ::core::primitive::u32,
                    },
                    #[codec(index = 7)]
                    #[doc = "The set of historical migrations has been cleared."]
                    HistoricCleared {
                        next_cursor: ::core::option::Option<
                            ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                        >,
                    },
                }
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[codec(dumb_trait_bound)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct ActiveCursor<_0, _1> {
                pub index: ::core::primitive::u32,
                pub inner_cursor: ::core::option::Option<_0>,
                pub started_at: _1,
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[codec(dumb_trait_bound)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub enum HistoricCleanupSelector<_0> {
                #[codec(index = 0)]
                Specific(::subxt::ext::subxt_core::alloc::vec::Vec<_0>),
                #[codec(index = 1)]
                Wildcard {
                    limit: ::core::option::Option<::core::primitive::u32>,
                    previous_cursor: ::core::option::Option<
                        ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                    >,
                },
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[codec(dumb_trait_bound)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub enum MigrationCursor<_0, _1> {
                #[codec(index = 0)]
                Active(runtime_types::pallet_migrations::ActiveCursor<_0, _1>),
                #[codec(index = 1)]
                Stuck,
            }
        }
        pub mod pallet_mob_rule {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Feeless on success (determined only by top three lines)."]
                    vote {
                        case_index: ::core::primitive::u32,
                        opinion: runtime_types::individuality_support::traits::reality::Judgement,
                    },
                    #[codec(index = 1)]
                    close_case { case_index: ::core::primitive::u32 },
                    #[codec(index = 2)]
                    clean_vote {
                        case_index: ::core::primitive::u32,
                        voter: [::core::primitive::u8; 32usize],
                    },
                    #[codec(index = 3)]
                    reap_case { case_index: ::core::primitive::u32 },
                    #[codec(index = 4)]
                    intervene {
                        case_index: ::core::primitive::u32,
                        verdict: runtime_types::individuality_support::traits::reality::Judgement,
                    },
                    #[codec(index = 5)]
                    #[doc = "A person claims the mob credit associated with a correct vote on a case."]
                    #[doc = "The case must be `Done`."]
                    claim_vote { case_index: ::core::primitive::u32 },
                    #[codec(index = 6)]
                    #[doc = "A person converts their claimed mob credit into rewards."]
                    payout_rewards {
                        voucher: runtime_types::verifiable::ring_vrf_impl::EncodedPublicKey,
                    },
                    #[codec(index = 7)]
                    #[doc = "A person claims multiple mob credits associated a correct vote on a case. The"]
                    #[doc = "case must be `Done`."]
                    claim_votes {
                        case_indices:
                            ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u32>,
                    },
                    #[codec(index = 8)]
                    start_payout_round,
                    #[codec(index = 9)]
                    schedule_payout_rounds {
                        amount: ::core::primitive::u128,
                        count: ::core::primitive::u32,
                        period: ::core::primitive::u32,
                    },
                    #[codec(index = 10)]
                    remove_payout_schedule { index: ::core::primitive::u32 },
                    #[codec(index = 11)]
                    claim_credit,
                    #[codec(index = 12)]
                    clean_points {
                        round: ::core::primitive::u32,
                        voter: [::core::primitive::u8; 32usize],
                    },
                    #[codec(index = 13)]
                    force_ripen_case { case_index: ::core::primitive::u32 },
                    #[codec(index = 14)]
                    touch_case { case_index: ::core::primitive::u32 },
                    #[codec(index = 15)]
                    clear_voting_penalty,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "The `Error` enum of this pallet."]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "The case does not exist."]
                    NoSuchCase,
                    #[codec(index = 1)]
                    #[doc = "The vote does not exist."]
                    NoSuchVote,
                    #[codec(index = 2)]
                    #[doc = "The case is not open."]
                    NotOpen,
                    #[codec(index = 3)]
                    #[doc = "The case is not ripe."]
                    NotRipe,
                    #[codec(index = 4)]
                    #[doc = "The case is not yet done."]
                    NotDone,
                    #[codec(index = 5)]
                    #[doc = "The decode of the call failed. Maybe there was a breaking runtime upgrade in between?"]
                    CodecError,
                    #[codec(index = 6)]
                    #[doc = "The call failed to dispatch. Maybe there was a breaking runtime upgrade in between?"]
                    DispatchError,
                    #[codec(index = 7)]
                    #[doc = "The case is too recent to be reaped."]
                    Recent,
                    #[codec(index = 8)]
                    #[doc = "Not enough credit to payout the rewards."]
                    NoCredit,
                    #[codec(index = 9)]
                    #[doc = "The configured voucher type is not supported by the voucher service."]
                    NoVoucher,
                    #[codec(index = 10)]
                    #[doc = "No mob credit distribution in place to reward voters."]
                    NoReward,
                    #[codec(index = 11)]
                    #[doc = "No points to be converted to mob credit."]
                    NoPoints,
                    #[codec(index = 12)]
                    #[doc = "Too many vote claims."]
                    TooManyClaims,
                    #[codec(index = 13)]
                    #[doc = "No payout in progress."]
                    NoPayout,
                    #[codec(index = 14)]
                    #[doc = "The point and/or credit arithmetic overflows."]
                    ArithmeticOverflow,
                    #[codec(index = 15)]
                    #[doc = "Too many payout round schedules."]
                    TooManySchedules,
                    #[codec(index = 16)]
                    #[doc = "No payout round schedule found."]
                    NoSchedule,
                    #[codec(index = 17)]
                    #[doc = "No vote penalty found."]
                    NoPenalty,
                    #[codec(index = 18)]
                    #[doc = "The vote penalty has not expired yet."]
                    Early,
                    #[codec(index = 19)]
                    #[doc = "The vote cannot be cast due to a voting penalty in effect."]
                    UnderPenalty,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "The `Event` enum of this pallet"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "A case has been created."]
                    CaseCreated { case_index: ::core::primitive::u32 },
                    #[codec(index = 1)]
                    #[doc = "A callback was triggered from mob-rule."]
                    Callback {
                        result:
                            ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
                    },
                    #[codec(index = 2)]
                    #[doc = "There was a codec error when trying to execute the callback."]
                    CallbackError,
                    #[codec(index = 3)]
                    #[doc = "The case has been closed with the following result."]
                    CaseClosed {
                        case_index: ::core::primitive::u32,
                        verdict: runtime_types::individuality_support::traits::reality::Judgement,
                    },
                    #[codec(index = 4)]
                    #[doc = "A vote has been placed on a case."]
                    Voted {
                        case_index: ::core::primitive::u32,
                        voter: [::core::primitive::u8; 32usize],
                        opinion: runtime_types::individuality_support::traits::reality::Judgement,
                    },
                    #[codec(index = 5)]
                    #[doc = "A vote has been cleaned."]
                    VoteCleaned {
                        case_index: ::core::primitive::u32,
                        voter: [::core::primitive::u8; 32usize],
                    },
                    #[codec(index = 6)]
                    #[doc = "A case has been removed."]
                    CaseRemoved { case_index: ::core::primitive::u32 },
                    #[codec(index = 7)]
                    #[doc = "A case has been intervened."]
                    CaseIntervened {
                        case_index: ::core::primitive::u32,
                        verdict: runtime_types::individuality_support::traits::reality::Judgement,
                    },
                    #[codec(index = 8)]
                    #[doc = "Credits for votes have been claimed."]
                    VotesClaimed {
                        voter: [::core::primitive::u8; 32usize],
                        case_indices:
                            ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u32>,
                    },
                    #[codec(index = 9)]
                    #[doc = "A reward voucher has been paid out."]
                    RewardPayout {
                        voter: [::core::primitive::u8; 32usize],
                        voucher: runtime_types::verifiable::ring_vrf_impl::EncodedPublicKey,
                    },
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub enum HoldReason {
                    #[codec(index = 0)]
                    Payout,
                    #[codec(index = 1)]
                    Credit,
                }
            }
        }
        pub mod pallet_multisig {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Immediately dispatch a multi-signature call using a single approval from the caller."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_."]
                    #[doc = ""]
                    #[doc = "- `other_signatories`: The accounts (other than the sender) who are part of the"]
                    #[doc = "multi-signature, but do not participate in the approval process."]
                    #[doc = "- `call`: The call to be executed."]
                    #[doc = ""]
                    #[doc = "Result is equivalent to the dispatched result."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "O(Z + C) where Z is the length of the call and C its execution weight."]
                    as_multi_threshold_1 {
                        other_signatories: ::subxt::ext::subxt_core::alloc::vec::Vec<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                        >,
                        call: ::subxt::ext::subxt_core::alloc::boxed::Box<
                            runtime_types::people_rococo_runtime::RuntimeCall,
                        >,
                    },
                    #[codec(index = 1)]
                    #[doc = "Register approval for a dispatch to be made from a deterministic composite account if"]
                    #[doc = "approved by a total of `threshold - 1` of `other_signatories`."]
                    #[doc = ""]
                    #[doc = "If there are enough, then dispatch the call."]
                    #[doc = ""]
                    #[doc = "Payment: `DepositBase` will be reserved if this is the first approval, plus"]
                    #[doc = "`threshold` times `DepositFactor`. It is returned once this dispatch happens or"]
                    #[doc = "is cancelled."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_."]
                    #[doc = ""]
                    #[doc = "- `threshold`: The total number of approvals for this dispatch before it is executed."]
                    #[doc = "- `other_signatories`: The accounts (other than the sender) who can approve this"]
                    #[doc = "dispatch. May not be empty."]
                    #[doc = "- `maybe_timepoint`: If this is the first approval, then this must be `None`. If it is"]
                    #[doc = "not the first approval, then it must be `Some`, with the timepoint (block number and"]
                    #[doc = "transaction index) of the first approval transaction."]
                    #[doc = "- `call`: The call to be executed."]
                    #[doc = ""]
                    #[doc = "NOTE: Unless this is the final approval, you will generally want to use"]
                    #[doc = "`approve_as_multi` instead, since it only requires a hash of the call."]
                    #[doc = ""]
                    #[doc = "Result is equivalent to the dispatched result if `threshold` is exactly `1`. Otherwise"]
                    #[doc = "on success, result is `Ok` and the result from the interior call, if it was executed,"]
                    #[doc = "may be found in the deposited `MultisigExecuted` event."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- `O(S + Z + Call)`."]
                    #[doc = "- Up to one balance-reserve or unreserve operation."]
                    #[doc = "- One passthrough operation, one insert, both `O(S)` where `S` is the number of"]
                    #[doc = "  signatories. `S` is capped by `MaxSignatories`, with weight being proportional."]
                    #[doc = "- One call encode & hash, both of complexity `O(Z)` where `Z` is tx-len."]
                    #[doc = "- One encode & hash, both of complexity `O(S)`."]
                    #[doc = "- Up to one binary search and insert (`O(logS + S)`)."]
                    #[doc = "- I/O: 1 read `O(S)`, up to 1 mutate `O(S)`. Up to one remove."]
                    #[doc = "- One event."]
                    #[doc = "- The weight of the `call`."]
                    #[doc = "- Storage: inserts one item, value size bounded by `MaxSignatories`, with a deposit"]
                    #[doc = "  taken for its lifetime of `DepositBase + threshold * DepositFactor`."]
                    as_multi {
                        threshold: ::core::primitive::u16,
                        other_signatories: ::subxt::ext::subxt_core::alloc::vec::Vec<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                        >,
                        maybe_timepoint: ::core::option::Option<
                            runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                        >,
                        call: ::subxt::ext::subxt_core::alloc::boxed::Box<
                            runtime_types::people_rococo_runtime::RuntimeCall,
                        >,
                        max_weight: runtime_types::sp_weights::weight_v2::Weight,
                    },
                    #[codec(index = 2)]
                    #[doc = "Register approval for a dispatch to be made from a deterministic composite account if"]
                    #[doc = "approved by a total of `threshold - 1` of `other_signatories`."]
                    #[doc = ""]
                    #[doc = "Payment: `DepositBase` will be reserved if this is the first approval, plus"]
                    #[doc = "`threshold` times `DepositFactor`. It is returned once this dispatch happens or"]
                    #[doc = "is cancelled."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_."]
                    #[doc = ""]
                    #[doc = "- `threshold`: The total number of approvals for this dispatch before it is executed."]
                    #[doc = "- `other_signatories`: The accounts (other than the sender) who can approve this"]
                    #[doc = "dispatch. May not be empty."]
                    #[doc = "- `maybe_timepoint`: If this is the first approval, then this must be `None`. If it is"]
                    #[doc = "not the first approval, then it must be `Some`, with the timepoint (block number and"]
                    #[doc = "transaction index) of the first approval transaction."]
                    #[doc = "- `call_hash`: The hash of the call to be executed."]
                    #[doc = ""]
                    #[doc = "NOTE: If this is the final approval, you will want to use `as_multi` instead."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- `O(S)`."]
                    #[doc = "- Up to one balance-reserve or unreserve operation."]
                    #[doc = "- One passthrough operation, one insert, both `O(S)` where `S` is the number of"]
                    #[doc = "  signatories. `S` is capped by `MaxSignatories`, with weight being proportional."]
                    #[doc = "- One encode & hash, both of complexity `O(S)`."]
                    #[doc = "- Up to one binary search and insert (`O(logS + S)`)."]
                    #[doc = "- I/O: 1 read `O(S)`, up to 1 mutate `O(S)`. Up to one remove."]
                    #[doc = "- One event."]
                    #[doc = "- Storage: inserts one item, value size bounded by `MaxSignatories`, with a deposit"]
                    #[doc = "  taken for its lifetime of `DepositBase + threshold * DepositFactor`."]
                    approve_as_multi {
                        threshold: ::core::primitive::u16,
                        other_signatories: ::subxt::ext::subxt_core::alloc::vec::Vec<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                        >,
                        maybe_timepoint: ::core::option::Option<
                            runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                        >,
                        call_hash: [::core::primitive::u8; 32usize],
                        max_weight: runtime_types::sp_weights::weight_v2::Weight,
                    },
                    #[codec(index = 3)]
                    #[doc = "Cancel a pre-existing, on-going multisig transaction. Any deposit reserved previously"]
                    #[doc = "for this operation will be unreserved on success."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_."]
                    #[doc = ""]
                    #[doc = "- `threshold`: The total number of approvals for this dispatch before it is executed."]
                    #[doc = "- `other_signatories`: The accounts (other than the sender) who can approve this"]
                    #[doc = "dispatch. May not be empty."]
                    #[doc = "- `timepoint`: The timepoint (block number and transaction index) of the first approval"]
                    #[doc = "transaction for this dispatch."]
                    #[doc = "- `call_hash`: The hash of the call to be executed."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- `O(S)`."]
                    #[doc = "- Up to one balance-reserve or unreserve operation."]
                    #[doc = "- One passthrough operation, one insert, both `O(S)` where `S` is the number of"]
                    #[doc = "  signatories. `S` is capped by `MaxSignatories`, with weight being proportional."]
                    #[doc = "- One encode & hash, both of complexity `O(S)`."]
                    #[doc = "- One event."]
                    #[doc = "- I/O: 1 read `O(S)`, one remove."]
                    #[doc = "- Storage: removes one item."]
                    cancel_as_multi {
                        threshold: ::core::primitive::u16,
                        other_signatories: ::subxt::ext::subxt_core::alloc::vec::Vec<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                        >,
                        timepoint:
                            runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                        call_hash: [::core::primitive::u8; 32usize],
                    },
                    #[codec(index = 4)]
                    #[doc = "Poke the deposit reserved for an existing multisig operation."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_ and must be the original depositor of"]
                    #[doc = "the multisig operation."]
                    #[doc = ""]
                    #[doc = "The transaction fee is waived if the deposit amount has changed."]
                    #[doc = ""]
                    #[doc = "- `threshold`: The total number of approvals needed for this multisig."]
                    #[doc = "- `other_signatories`: The accounts (other than the sender) who are part of the"]
                    #[doc = "  multisig."]
                    #[doc = "- `call_hash`: The hash of the call this deposit is reserved for."]
                    #[doc = ""]
                    #[doc = "Emits `DepositPoked` if successful."]
                    poke_deposit {
                        threshold: ::core::primitive::u16,
                        other_signatories: ::subxt::ext::subxt_core::alloc::vec::Vec<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                        >,
                        call_hash: [::core::primitive::u8; 32usize],
                    },
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "The `Error` enum of this pallet."]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "Threshold must be 2 or greater."]
                    MinimumThreshold,
                    #[codec(index = 1)]
                    #[doc = "Call is already approved by this signatory."]
                    AlreadyApproved,
                    #[codec(index = 2)]
                    #[doc = "Call doesn't need any (more) approvals."]
                    NoApprovalsNeeded,
                    #[codec(index = 3)]
                    #[doc = "There are too few signatories in the list."]
                    TooFewSignatories,
                    #[codec(index = 4)]
                    #[doc = "There are too many signatories in the list."]
                    TooManySignatories,
                    #[codec(index = 5)]
                    #[doc = "The signatories were provided out of order; they should be ordered."]
                    SignatoriesOutOfOrder,
                    #[codec(index = 6)]
                    #[doc = "The sender was contained in the other signatories; it shouldn't be."]
                    SenderInSignatories,
                    #[codec(index = 7)]
                    #[doc = "Multisig operation not found in storage."]
                    NotFound,
                    #[codec(index = 8)]
                    #[doc = "Only the account that originally created the multisig is able to cancel it or update"]
                    #[doc = "its deposits."]
                    NotOwner,
                    #[codec(index = 9)]
                    #[doc = "No timepoint was given, yet the multisig operation is already underway."]
                    NoTimepoint,
                    #[codec(index = 10)]
                    #[doc = "A different timepoint was given to the multisig operation that is underway."]
                    WrongTimepoint,
                    #[codec(index = 11)]
                    #[doc = "A timepoint was given, yet no multisig operation is underway."]
                    UnexpectedTimepoint,
                    #[codec(index = 12)]
                    #[doc = "The maximum weight information provided was too low."]
                    MaxWeightTooLow,
                    #[codec(index = 13)]
                    #[doc = "The data to be stored is already stored."]
                    AlreadyStored,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "The `Event` enum of this pallet"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "A new multisig operation has begun."]
                    NewMultisig {
                        approving: ::subxt::ext::subxt_core::utils::AccountId32,
                        multisig: ::subxt::ext::subxt_core::utils::AccountId32,
                        call_hash: [::core::primitive::u8; 32usize],
                    },
                    #[codec(index = 1)]
                    #[doc = "A multisig operation has been approved by someone."]
                    MultisigApproval {
                        approving: ::subxt::ext::subxt_core::utils::AccountId32,
                        timepoint:
                            runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                        multisig: ::subxt::ext::subxt_core::utils::AccountId32,
                        call_hash: [::core::primitive::u8; 32usize],
                    },
                    #[codec(index = 2)]
                    #[doc = "A multisig operation has been executed."]
                    MultisigExecuted {
                        approving: ::subxt::ext::subxt_core::utils::AccountId32,
                        timepoint:
                            runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                        multisig: ::subxt::ext::subxt_core::utils::AccountId32,
                        call_hash: [::core::primitive::u8; 32usize],
                        result:
                            ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
                    },
                    #[codec(index = 3)]
                    #[doc = "A multisig operation has been cancelled."]
                    MultisigCancelled {
                        cancelling: ::subxt::ext::subxt_core::utils::AccountId32,
                        timepoint:
                            runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                        multisig: ::subxt::ext::subxt_core::utils::AccountId32,
                        call_hash: [::core::primitive::u8; 32usize],
                    },
                    #[codec(index = 4)]
                    #[doc = "The deposit for a multisig operation has been updated/poked."]
                    DepositPoked {
                        who: ::subxt::ext::subxt_core::utils::AccountId32,
                        call_hash: [::core::primitive::u8; 32usize],
                        old_deposit: ::core::primitive::u128,
                        new_deposit: ::core::primitive::u128,
                    },
                }
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[codec(dumb_trait_bound)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct Timepoint<_0> {
                pub height: _0,
                pub index: ::core::primitive::u32,
            }
        }
        pub mod pallet_origin_restriction {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
                pub enum Call {
                    #[codec(index = 1)]
                    #[doc = "Allow to clean usage associated with an entity when it is zero or when there is no"]
                    #[doc = "longer any allowance for the origin."]
                    clean_usage {
                        entity: runtime_types::people_rococo_runtime::RestrictedEntity,
                    },
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "The `Error` enum of this pallet."]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "The origin has no usage tracked."]
                    NoUsage,
                    #[codec(index = 1)]
                    #[doc = "The usage is not zero."]
                    NotZero,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "The `Event` enum of this pallet"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "Usage for an entity is cleaned."]
                    UsageCleaned {
                        entity: runtime_types::people_rococo_runtime::RestrictedEntity,
                    },
                }
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[codec(dumb_trait_bound)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct RestrictOrigin(pub ::core::primitive::bool);
        }
        pub mod pallet_people_multi {
            use super::runtime_types;
            pub mod extension {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct AsPerson(
                    pub  ::core::option::Option<
                        runtime_types::pallet_people_multi::extension::AsPersonInfo,
                    >,
                );
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub enum AsPersonInfo {
                    #[codec(index = 0)]
                    AsPersonalAliasWithAccount(::core::primitive::u32),
                    #[codec(index = 1)]
                    AsPersonalAliasWithProof(
                        [::core::primitive::u8; 788usize],
                        ::core::primitive::u32,
                        [::core::primitive::u8; 32usize],
                    ),
                    #[codec(index = 2)]
                    AsPersonalIdentityWithProof(
                        [::core::primitive::u8; 96usize],
                        ::core::primitive::u64,
                    ),
                    #[codec(index = 3)]
                    AsPersonalIdentityWithAccount(::core::primitive::u32),
                    #[codec(index = 4)]
                    AsPersonalAliasWithAccountRevised(
                        ::core::primitive::u32,
                        [::core::primitive::u8; 788usize],
                        ::core::primitive::u32,
                        [::core::primitive::u8; 32usize],
                    ),
                }
            }
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
                pub enum Call {
                    #[codec(index = 100)]
                    #[doc = "Build a ring root by including registered people."]
                    #[doc = ""]
                    #[doc = "This task is performed automatically by the pallet through the `on_idle` hook whenever"]
                    #[doc = "there is leftover weight in a block. This call is meant to be a backup in case of"]
                    #[doc = "extreme congestion and should be submitted by signed origins."]
                    build_ring_manual {
                        ring_index: ::core::primitive::u32,
                        limit: ::core::option::Option<::core::primitive::u32>,
                    },
                    #[codec(index = 101)]
                    #[doc = "Onboard people into a ring by taking their keys from the onboarding queue and"]
                    #[doc = "registering them into the ring. This does not compute the root, that is done using"]
                    #[doc = "`build_ring`."]
                    #[doc = ""]
                    #[doc = "This task is performed automatically by the pallet through the `on_idle` hook whenever"]
                    #[doc = "there is leftover weight in a block. This call is meant to be a backup in case of"]
                    #[doc = "extreme congestion and should be submitted by signed origins."]
                    onboard_people_manual,
                    #[codec(index = 102)]
                    #[doc = "Merge the people in two rings into a single, new ring. In order for the rings to be"]
                    #[doc = "eligible for merging, they must be below 1/2 of max capacity, have no pending"]
                    #[doc = "suspensions and not be the top ring used for onboarding."]
                    merge_rings {
                        base_ring_index: ::core::primitive::u32,
                        target_ring_index: ::core::primitive::u32,
                    },
                    #[codec(index = 0)]
                    #[doc = "Dispatch a call under an alias using the `account <-> alias` mapping."]
                    #[doc = ""]
                    #[doc = "This is a call version of the transaction extension `AsPersonalAliasWithAccount`."]
                    #[doc = "It is recommended to use the transaction extension instead when suitable."]
                    under_alias {
                        call: ::subxt::ext::subxt_core::alloc::boxed::Box<
                            runtime_types::people_rococo_runtime::RuntimeCall,
                        >,
                    },
                    #[codec(index = 1)]
                    #[doc = "This transaction is refunded if successful and no alias was previously set."]
                    #[doc = ""]
                    #[doc = "The call is valid from `call_valid_at` until"]
                    #[doc = "`call_valid_at + account_setup_time_tolerance`."]
                    #[doc = "`account_setup_time_tolerance` is a constant available in the metadata."]
                    #[doc = ""]
                    #[doc = "Parameters:"]
                    #[doc = "- `account`: The account to set the alias for."]
                    #[doc = "- `call_valid_at`: The block number when the call becomes valid."]
                    set_alias_account {
                        account: ::subxt::ext::subxt_core::utils::AccountId32,
                        call_valid_at: ::core::primitive::u32,
                    },
                    #[codec(index = 2)]
                    #[doc = "Remove the mapping from a particular alias to its registered account."]
                    unset_alias_account,
                    #[codec(index = 3)]
                    #[doc = "Recognize a set of people without any additional checks."]
                    #[doc = ""]
                    #[doc = "The people are identified by the provided list of keys and will each be assigned, in"]
                    #[doc = "order, the next available personal ID."]
                    #[doc = ""]
                    #[doc = "The origin for this call must have root privileges."]
                    force_recognize_personhood {
                        people: ::subxt::ext::subxt_core::alloc::vec::Vec<
                            runtime_types::verifiable::ring_vrf_impl::EncodedPublicKey,
                        >,
                    },
                    #[codec(index = 4)]
                    #[doc = "Set a personal id account."]
                    #[doc = ""]
                    #[doc = "The account can then be used to sign transactions on behalf of the personal id, and"]
                    #[doc = "provide replay protection with the nonce."]
                    #[doc = ""]
                    #[doc = "This transaction is refunded if successful and no account was previously set for the"]
                    #[doc = "personal id."]
                    #[doc = ""]
                    #[doc = "The call is valid from `call_valid_at` until"]
                    #[doc = "`call_valid_at + account_setup_time_tolerance`."]
                    #[doc = "`account_setup_time_tolerance` is a constant available in the metadata."]
                    #[doc = ""]
                    #[doc = "Parameters:"]
                    #[doc = "- `account`: The account to set the alias for."]
                    #[doc = "- `call_valid_at`: The block number when the call becomes valid."]
                    set_personal_id_account {
                        account: ::subxt::ext::subxt_core::utils::AccountId32,
                        call_valid_at: ::core::primitive::u32,
                    },
                    #[codec(index = 5)]
                    #[doc = "Unset the personal id account."]
                    unset_personal_id_account,
                    #[codec(index = 6)]
                    #[doc = "Migrate the key for a person who was onboarded and is currently included in a ring. The"]
                    #[doc = "migration is not instant as the key replacement and subsequent inclusion in a new ring"]
                    #[doc = "root will happen only after the next mutation session."]
                    migrate_included_key {
                        new_key: runtime_types::verifiable::ring_vrf_impl::EncodedPublicKey,
                    },
                    #[codec(index = 7)]
                    #[doc = "Migrate the key for a person who is currently onboarding. The operation is instant,"]
                    #[doc = "replacing the old key in the onboarding queue."]
                    migrate_onboarding_key {
                        new_key: runtime_types::verifiable::ring_vrf_impl::EncodedPublicKey,
                    },
                    #[codec(index = 8)]
                    #[doc = "Force set the onboarding size for new people. This call requires root privileges."]
                    set_onboarding_size {
                        onboarding_size: ::core::primitive::u32,
                    },
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "The `Error` enum of this pallet."]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "The supplied identifier does not represent a person."]
                    NotPerson,
                    #[codec(index = 1)]
                    #[doc = "The given person has no associated key."]
                    NoKey,
                    #[codec(index = 2)]
                    #[doc = "The context is not a member of those allowed to have account aliases held."]
                    InvalidContext,
                    #[codec(index = 3)]
                    #[doc = "The account is not known."]
                    InvalidAccount,
                    #[codec(index = 4)]
                    #[doc = "The account is already in use under another alias."]
                    AccountInUse,
                    #[codec(index = 5)]
                    #[doc = "The proof is invalid."]
                    InvalidProof,
                    #[codec(index = 6)]
                    #[doc = "The signature is invalid."]
                    InvalidSignature,
                    #[codec(index = 7)]
                    #[doc = "There are not yet any members of our personhood set."]
                    NoMembers,
                    #[codec(index = 8)]
                    #[doc = "The root cannot be finalized as there are still unpushed members."]
                    Incomplete,
                    #[codec(index = 9)]
                    #[doc = "The root is still fresh."]
                    StillFresh,
                    #[codec(index = 10)]
                    #[doc = "Too many members have been pushed."]
                    TooManyMembers,
                    #[codec(index = 11)]
                    #[doc = "Key already in use by another person."]
                    KeyAlreadyInUse,
                    #[codec(index = 12)]
                    #[doc = "The old key was not found when expected."]
                    KeyNotFound,
                    #[codec(index = 13)]
                    #[doc = "Could not push member into the ring."]
                    CouldNotPush,
                    #[codec(index = 14)]
                    #[doc = "The record is already using this key."]
                    SameKey,
                    #[codec(index = 15)]
                    #[doc = "Personal Id was not reserved."]
                    PersonalIdNotReserved,
                    #[codec(index = 16)]
                    #[doc = "Personal Id has never been reserved."]
                    PersonalIdReservationCannotRenew,
                    #[codec(index = 17)]
                    #[doc = "Personal Id was not reserved or not already recognized."]
                    PersonalIdNotReservedOrNotRecognized,
                    #[codec(index = 18)]
                    #[doc = "Ring cannot be merged if it's the top ring."]
                    InvalidRing,
                    #[codec(index = 19)]
                    #[doc = "Ring cannot be built while there are suspensions pending."]
                    SuspensionsPending,
                    #[codec(index = 20)]
                    #[doc = "Ring cannot be merged if it's not below 1/2 capacity."]
                    RingAboveMergeThreshold,
                    #[codec(index = 21)]
                    #[doc = "Suspension indices provided are invalid."]
                    InvalidSuspensions,
                    #[codec(index = 22)]
                    #[doc = "An mutating action was queued when there was no mutation session in progress."]
                    NoMutationSession,
                    #[codec(index = 23)]
                    #[doc = "An mutating session could not be started."]
                    CouldNotStartMutationSession,
                    #[codec(index = 24)]
                    #[doc = "Cannot merge rings while a suspension session is in progress."]
                    SuspensionSessionInProgress,
                    #[codec(index = 25)]
                    #[doc = "Call is too late or too early."]
                    TimeOutOfRange,
                    #[codec(index = 26)]
                    #[doc = "Alias <-> Account is already set and up to date."]
                    AliasAccountAlreadySet,
                    #[codec(index = 27)]
                    #[doc = "Personhood cannot be resumed if it is not suspended."]
                    NotSuspended,
                    #[codec(index = 28)]
                    #[doc = "Personhood is suspended."]
                    Suspended,
                    #[codec(index = 29)]
                    #[doc = "Invalid state for attempted key migration."]
                    InvalidKeyMigration,
                    #[codec(index = 30)]
                    #[doc = "Invalid suspension of a key belonging to a person whose index in the ring has already"]
                    #[doc = "been included in the pending suspensions list."]
                    KeyAlreadySuspended,
                    #[codec(index = 31)]
                    #[doc = "The onboarding size must not exceed the maximum ring size."]
                    InvalidOnboardingSize,
                    #[codec(index = 32)]
                    #[doc = "The member key is not valid for the crypto."]
                    InvalidMemberKey,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "The `Event` enum of this pallet"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "An individual has had their personhood recognised and indexed."]
                    PersonhoodRecognized {
                        who: ::core::primitive::u64,
                        key: runtime_types::verifiable::ring_vrf_impl::EncodedPublicKey,
                    },
                    #[codec(index = 1)]
                    #[doc = "An individual has had their personhood recognised again and indexed."]
                    PersonOnboarding {
                        who: ::core::primitive::u64,
                        key: runtime_types::verifiable::ring_vrf_impl::EncodedPublicKey,
                    },
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub enum Origin {
                    #[codec(index = 0)]
                    PersonalIdentity(::core::primitive::u64),
                    #[codec(index = 1)]
                    PersonalAlias(
                        runtime_types::pallet_people_multi::types::RevisedContextualAlias,
                    ),
                }
            }
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct PersonRecord<_0, _1> {
                    pub key: _0,
                    pub position: runtime_types::pallet_people_multi::types::RingPosition,
                    pub account: ::core::option::Option<_1>,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct RevisedContextualAlias {
                    pub revision: ::core::primitive::u32,
                    pub ring: ::core::primitive::u32,
                    pub ca: runtime_types::individuality_support::traits::reality::ContextualAlias,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub enum RingMembersState {
                    #[codec(index = 0)]
                    AppendOnly,
                    #[codec(index = 1)]
                    Mutating(::core::primitive::u8),
                    #[codec(index = 2)]
                    KeyMigration,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub enum RingPosition {
                    #[codec(index = 0)]
                    Onboarding { queue_page: ::core::primitive::u32 },
                    #[codec(index = 1)]
                    Included {
                        ring_index: ::core::primitive::u32,
                        ring_position: ::core::primitive::u32,
                        scheduled_for_removal: ::core::primitive::bool,
                    },
                    #[codec(index = 2)]
                    Suspended,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct RingRoot {
                    pub root: [::core::primitive::u8; 384usize],
                    pub revision: ::core::primitive::u32,
                    pub intermediate: [::core::primitive::u8; 432usize],
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct RingStatus {
                    pub total: ::core::primitive::u32,
                    pub included: ::core::primitive::u32,
                }
            }
        }
        pub mod pallet_privacy_voucher {
            use super::runtime_types;
            pub mod extension {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct ProvideForVoucherClaimer;
            }
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Claim a private voucher. The token will be deposited in the caller account."]
                    #[doc = ""]
                    #[doc = "NOTE: This doesn't guarantee that the voucher is enough to provide for a new account."]
                    claim_voucher {
                        proof: [::core::primitive::u8; 788usize],
                        voucher_value: ::core::primitive::u128,
                        ring_index: ::core::primitive::u32,
                    },
                    #[codec(index = 1)]
                    #[doc = "Claim a private voucher into a destination account."]
                    #[doc = ""]
                    #[doc = "NOTE: This doesn't guarantee that the voucher is enough to provide for a new account."]
                    claim_voucher_into_destination {
                        proof: [::core::primitive::u8; 788usize],
                        dest: ::subxt::ext::subxt_core::utils::AccountId32,
                        voucher_value: ::core::primitive::u128,
                        ring_index: ::core::primitive::u32,
                    },
                    #[codec(index = 2)]
                    clean_ring {
                        voucher_value: ::core::primitive::u128,
                        ring_index: ::core::primitive::u32,
                    },
                    #[codec(index = 3)]
                    #[doc = "Set the voucher value for a variable voucher type. The value will take effect after the"]
                    #[doc = "building ring for the old value is fully built, if no building ring then it takes effect"]
                    #[doc = "immediately."]
                    #[doc = ""]
                    #[doc = "If the voucher id does not exist, it will be created with the new value."]
                    set_voucher_value_delayed {
                        variable_voucher_id: [::core::primitive::u8; 8usize],
                        new_value: ::core::primitive::u128,
                    },
                    #[codec(index = 4)]
                    #[doc = "Set the voucher value for a variable voucher type instantly. The value will take effect"]
                    #[doc = "instantly. If there is a building ring for the old value, and no other voucher use this"]
                    #[doc = "old value, the building ring might never become claimable."]
                    #[doc = ""]
                    #[doc = "If the voucher id does not exist, it will be created with the new value."]
                    set_voucher_value_instant {
                        variable_voucher_id: [::core::primitive::u8; 8usize],
                        new_value: ::core::primitive::u128,
                    },
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "The `Error` enum of this pallet."]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "The proof is invalid."]
                    InvalidProof,
                    #[codec(index = 1)]
                    #[doc = "The member was already added to the root."]
                    AlreadyMember,
                    #[codec(index = 2)]
                    #[doc = "The member has already been pushed into this root."]
                    AlreadyPushed,
                    #[codec(index = 3)]
                    #[doc = "Too many members have been pushed."]
                    TooManyMembers,
                    #[codec(index = 4)]
                    #[doc = "The root does not exist."]
                    NoRoot,
                    #[codec(index = 5)]
                    #[doc = "The root does not have any members."]
                    NoMembers,
                    #[codec(index = 6)]
                    #[doc = "The given index does not exist."]
                    NonExistent,
                    #[codec(index = 7)]
                    #[doc = "Voucher was already claimed."]
                    AlreadySpent,
                    #[codec(index = 8)]
                    #[doc = "The reward couldn't be paid out."]
                    PayoutFailure,
                    #[codec(index = 9)]
                    #[doc = "The deposit couldn't be received."]
                    DepositFailure,
                    #[codec(index = 10)]
                    #[doc = "The voucher value isn't set."]
                    NoVoucher,
                    #[codec(index = 11)]
                    #[doc = "Invalid ring state."]
                    InvalidRingState,
                    #[codec(index = 12)]
                    #[doc = "Clean operation is not in order."]
                    CleanNotInOrder,
                    #[codec(index = 13)]
                    #[doc = "The ring is not expired."]
                    RingNotExpired,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "The `Event` enum of this pallet"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "An individual has created a voucher."]
                    KeyAdded {
                        key: runtime_types::verifiable::ring_vrf_impl::EncodedPublicKey,
                        voucher_value: ::core::primitive::u128,
                        ring_index: ::core::primitive::u32,
                    },
                    #[codec(index = 1)]
                    #[doc = "An individual claimed a voucher."]
                    VoucherClaimed {
                        who: ::subxt::ext::subxt_core::utils::AccountId32,
                        alias: [::core::primitive::u8; 32usize],
                    },
                    #[codec(index = 2)]
                    #[doc = "An individual sent a voucher."]
                    VoucherSent {
                        alias: [::core::primitive::u8; 32usize],
                        dest: ::subxt::ext::subxt_core::utils::AccountId32,
                    },
                    #[codec(index = 3)]
                    #[doc = "Ring is claimable."]
                    ClaimableRing {
                        voucher_value: ::core::primitive::u128,
                        ring_index: ::core::primitive::u32,
                    },
                    #[codec(index = 4)]
                    #[doc = "Voucher value has changed."]
                    VoucherValueChanged {
                        variable_voucher_id: [::core::primitive::u8; 8usize],
                        new_value: ::core::primitive::u128,
                    },
                    #[codec(index = 5)]
                    #[doc = "Voucher value change is pending."]
                    VoucherValuePending {
                        variable_voucher_id: [::core::primitive::u8; 8usize],
                        new_value: ::core::primitive::u128,
                    },
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub enum Origin {
                    #[codec(index = 0)]
                    AuthorizedVoucherClaim(
                        [::core::primitive::u8; 32usize],
                        ::subxt::ext::subxt_core::utils::AccountId32,
                    ),
                }
            }
        }
        pub mod pallet_proof_of_ink {
            use super::runtime_types;
            pub mod extension {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct AsProofOfInkParticipant(
                    pub  ::core::option::Option<
                        runtime_types::pallet_proof_of_ink::extension::AsProofOfInkParticipantInfo,
                    >,
                );
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub enum AsProofOfInkParticipantInfo {
                    #[codec(index = 0)]
                    AsApplyWithSig(::core::primitive::u32),
                    #[codec(index = 1)]
                    AsReferred(::core::primitive::u32),
                    #[codec(index = 2)]
                    AsInvited(::core::primitive::u32),
                }
            }
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "- Declare intention to get a tattoo."]
                    #[doc = "- Deposit is taken."]
                    #[doc = "- `InkType` may not be `ProceduralDerivative`."]
                    apply,
                    #[codec(index = 1)]
                    #[doc = "- Open a judicial case to judge the `evidence`."]
                    #[doc = "- Calls Judicial/`JudgeStatement(POI_HASH, evidence)`"]
                    #[doc = "- Needs `SignedExtension` to avoid upfront requirement for fee if `judgements == 0`."]
                    #[doc = "- If `judgements > 0`, then an additional fee should be charged into Treasury."]
                    submit_evidence {
                        evidence: [::core::primitive::u8; 32usize],
                    },
                    #[codec(index = 2)]
                    #[doc = "Is called by the Oracle when the evidence has been judged."]
                    judged {
                        ticket: ::core::primitive::u32,
                        context: runtime_types::bounded_collections::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                        judgement: runtime_types::individuality_support::traits::reality::Judgement,
                    },
                    #[codec(index = 3)]
                    #[doc = "- Gets a new `index`"]
                    #[doc = "- Puts an entry into `People`."]
                    #[doc = "- Calls People/`newPerson(index, key)`"]
                    #[doc = ""]
                    #[doc = "# Arguments"]
                    #[doc = "* `key` - The personal identity to register."]
                    #[doc = "* `voucher_key` - The voucher key to register the voucher of the referred person."]
                    #[doc = "* `proof_of_ownerhsip` - The signature of predefined prefix `\"pop register using\"`"]
                    #[doc = "  concatenated with the sender account id."]
                    register_referred {
                        key: runtime_types::verifiable::ring_vrf_impl::EncodedPublicKey,
                        voucher_key: runtime_types::verifiable::ring_vrf_impl::EncodedPublicKey,
                        proof_of_ownership: [::core::primitive::u8; 96usize],
                    },
                    #[codec(index = 4)]
                    #[doc = "- Gets a new `index`"]
                    #[doc = "- Puts an entry into `People`."]
                    #[doc = "- Calls People/`newPerson(index, key)`"]
                    #[doc = ""]
                    #[doc = "# Arguments"]
                    #[doc = "* `key` - The personal identity to register."]
                    #[doc = "* `voucher_key` - The voucher key to register the voucher of the referred person."]
                    #[doc = "* `self_ref_voucher_key` - If the candidate is not referred then the voucher to register"]
                    #[doc = "  a self-referrer voucher."]
                    #[doc = "* `proof_of_ownerhsip` - The signature of predefined prefix `\"pop register using\"`"]
                    #[doc = "  concatenated with the sender account id."]
                    #[doc = ""]
                    #[doc = "# Warning:"]
                    #[doc = ""]
                    #[doc = "This call register 2 vouchers to the same person. If this person redeem the 2 vouchers"]
                    #[doc = "to the same account, this might compromise the anonymity of owner of the personal id."]
                    #[doc = "Because from the other side, 2 vouchers of those 2 specific value redeemed to the same"]
                    #[doc = "account means it is likely that the account is owned by a person that is not referred."]
                    #[doc = "If few vouchers in the rings come from the registration of a non-referred person then"]
                    #[doc = "the account can be linked to those few registered personal ids."]
                    register_non_referred {
                        key: runtime_types::verifiable::ring_vrf_impl::EncodedPublicKey,
                        voucher_key: runtime_types::verifiable::ring_vrf_impl::EncodedPublicKey,
                        self_ref_voucher_key:
                            runtime_types::verifiable::ring_vrf_impl::EncodedPublicKey,
                        proof_of_ownership: [::core::primitive::u8; 96usize],
                    },
                    #[codec(index = 5)]
                    reroll,
                    #[codec(index = 6)]
                    #[doc = "Commit to a design and authorize the storage for (possibly just initial) evidence."]
                    #[doc = ""]
                    #[doc = "If a specific personal identity is required, then this can be placed in `require_id`."]
                    #[doc = "This can be any unused/unreserved personal identity no greater than the `NextId`"]
                    #[doc = "counter."]
                    commit {
                        choice: runtime_types::pallet_proof_of_ink::pallet::InkChoice,
                        require_id: ::core::option::Option<::core::primitive::u64>,
                    },
                    #[codec(index = 7)]
                    allocate_full,
                    #[codec(index = 8)]
                    #[doc = "Once a timeout passes, removes a candidate who is selected but not yet proven."]
                    #[doc = "If the candidate was referred then referral is bad, if candidate applied with deposit,"]
                    #[doc = "deposit is slashed."]
                    timeout {
                        account: ::subxt::ext::subxt_core::utils::AccountId32,
                    },
                    #[codec(index = 9)]
                    #[doc = "Remove a candidate who has not yet committed."]
                    flakeout,
                    #[codec(index = 10)]
                    #[doc = "Utilize a referral signature."]
                    #[doc = ""]
                    #[doc = "The payload to be signed by the referrer using their registered key pair is the encoded"]
                    #[doc = "bytes of the account ID of the candidate."]
                    apply_with_signature {
                        referrer: ::core::primitive::u64,
                        signature: runtime_types::sp_runtime::MultiSignature,
                        ticket: ::subxt::ext::subxt_core::utils::AccountId32,
                    },
                    #[codec(index = 19)]
                    #[doc = "Apply to the Proof-of-Ink process with an invitation."]
                    #[doc = ""]
                    #[doc = "The payload to be signed by the inviter using their registered key pair is the encoded"]
                    #[doc = "bytes of the account ID of the candidate."]
                    apply_with_invitation {
                        inviter: ::subxt::ext::subxt_core::utils::AccountId32,
                        ticket: ::subxt::ext::subxt_core::utils::AccountId32,
                        signature: runtime_types::sp_runtime::MultiSignature,
                    },
                    #[codec(index = 11)]
                    #[doc = "Add a design family. Must have privileged access to do this."]
                    add_design_family {
                        index: ::core::primitive::u16,
                        kind: runtime_types::pallet_proof_of_ink::pallet::FamilyKind,
                        id: [::core::primitive::u8; 32usize],
                    },
                    #[codec(index = 12)]
                    #[doc = "Add a referral ticket associated with a person."]
                    #[doc = ""]
                    #[doc = "Only one referral ticket may be active at any given time for one person. Calling this"]
                    #[doc = "extrinsic while a valid ticket is set will overwrite the existing ticket."]
                    #[doc = ""]
                    #[doc = "If any pending referral vouchers are present, they need to be registered first."]
                    set_referral_ticket {
                        ticket: ::subxt::ext::subxt_core::utils::AccountId32,
                    },
                    #[codec(index = 13)]
                    #[doc = "Cancel a referral ticket associated with a person."]
                    cancel_referral_ticket {
                        ticket: ::subxt::ext::subxt_core::utils::AccountId32,
                    },
                    #[codec(index = 14)]
                    register_successful_referral_voucher {
                        voucher_key: runtime_types::verifiable::ring_vrf_impl::EncodedPublicKey,
                    },
                    #[codec(index = 15)]
                    #[doc = "Grants invites to an account so they can distribute them."]
                    #[doc = ""]
                    #[doc = "The origin must be `InvitationsOrigin."]
                    #[doc = ""]
                    #[doc = "- `account`: The account to give invites to."]
                    #[doc = "- `count`: The number of invites to give."]
                    grant_invites {
                        account: ::subxt::ext::subxt_core::utils::AccountId32,
                        count: ::core::primitive::u32,
                    },
                    #[codec(index = 16)]
                    #[doc = "Remove all invites given to an account."]
                    #[doc = ""]
                    #[doc = "The origin must be `InvitationsOrigin`."]
                    #[doc = ""]
                    #[doc = "- `account`: The account to remove all invites from."]
                    #[doc = "- `limit`: The maximum number of pending invites to remove."]
                    remove_available_and_pending_invites {
                        account: ::subxt::ext::subxt_core::utils::AccountId32,
                        limit: ::core::primitive::u32,
                    },
                    #[codec(index = 17)]
                    #[doc = "Invite an account."]
                    #[doc = ""]
                    #[doc = "The origin must be signed by an account and have some invites left."]
                    #[doc = ""]
                    #[doc = "- `ticket`: The invite ticket to set."]
                    set_invite_ticket {
                        ticket: ::subxt::ext::subxt_core::utils::AccountId32,
                    },
                    #[codec(index = 18)]
                    #[doc = "Cancel an invitation."]
                    #[doc = ""]
                    #[doc = "The origin must be signed by the account that owns the ticket to cancel."]
                    #[doc = ""]
                    #[doc = "- `ticket`: The invite ticket to cancel."]
                    cancel_invite_ticket {
                        ticket: ::subxt::ext::subxt_core::utils::AccountId32,
                    },
                    #[codec(index = 50)]
                    #[doc = "Set the configuration record. Must have privileged access to do this."]
                    set_configuration {
                        config: runtime_types::pallet_proof_of_ink::pallet::ConfigRecord<
                            ::core::primitive::u32,
                        >,
                    },
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct ConfigRecord<_0> {
                    pub reroll_timeout: _0,
                    pub fasttrack_count: ::core::primitive::u32,
                    pub maximum: ::core::primitive::u32,
                    pub full_alloc_len: ::core::primitive::u64,
                    pub full_alloc_count: ::core::primitive::u32,
                    pub init_alloc_len: ::core::primitive::u64,
                    pub init_alloc_count: ::core::primitive::u32,
                    pub timeout: _0,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "The `Error` enum of this pallet."]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "Account is already applying to make a proof-of-ink."]
                    InProgress,
                    #[codec(index = 1)]
                    #[doc = "Account has not been referred."]
                    NoReferral,
                    #[codec(index = 2)]
                    #[doc = "The callback context is invalid; this should never happen."]
                    BadContext,
                    #[codec(index = 3)]
                    #[doc = "The incoming judgement ID is not what we were expecting for the account."]
                    UnexpectedJudgement,
                    #[codec(index = 4)]
                    #[doc = "No arguments were supplied with the judgement."]
                    NoArgs,
                    #[codec(index = 5)]
                    #[doc = "The account has not applied to make a proof-of-ink."]
                    NotApplied,
                    #[codec(index = 6)]
                    #[doc = "The candidate has not committed to a design."]
                    NotSelected,
                    #[codec(index = 7)]
                    #[doc = "The candidate did not prove themselves yet."]
                    NotProven,
                    #[codec(index = 8)]
                    #[doc = "The candidate has already started their judgement."]
                    AlreadyStarted,
                    #[codec(index = 9)]
                    #[doc = "The personal ID is not in range."]
                    OutOfRange,
                    #[codec(index = 10)]
                    #[doc = "The personal ID has already been taken."]
                    AlreadyTaken,
                    #[codec(index = 11)]
                    #[doc = "The person has no more referrals left to give."]
                    NoMoreReferrals,
                    #[codec(index = 12)]
                    #[doc = "The reroll is too early."]
                    TooEarly,
                    #[codec(index = 13)]
                    #[doc = "The referrer's design is not procedural."]
                    DesignInvalid,
                    #[codec(index = 14)]
                    #[doc = "The referrer's design is already taken."]
                    DesignTaken,
                    #[codec(index = 15)]
                    #[doc = "The referrer doesn't appear to be a person. This should never happen."]
                    BadParent,
                    #[codec(index = 16)]
                    #[doc = "The design family doesn't exist."]
                    BadFamily,
                    #[codec(index = 17)]
                    #[doc = "The design family is invalid for this choice."]
                    WrongFamily,
                    #[codec(index = 18)]
                    #[doc = "The index of the design or variant is beyond the allowed maximum for the family."]
                    IndexTooBig,
                    #[codec(index = 19)]
                    #[doc = "The system is busy with too many commitments; try again later."]
                    Busy,
                    #[codec(index = 20)]
                    #[doc = "The person has been banned from referring."]
                    Banned,
                    #[codec(index = 21)]
                    #[doc = "The candidate has not demonstrated probably through the initial evidence."]
                    Improbable,
                    #[codec(index = 22)]
                    #[doc = "The personal identity has already been reserved."]
                    IdReserved,
                    #[codec(index = 23)]
                    #[doc = "The personal identity is already taken by a proven person."]
                    IdUsed,
                    #[codec(index = 24)]
                    #[doc = "The ticket provided is invalid."]
                    InvalidTicket,
                    #[codec(index = 25)]
                    #[doc = "The caller has not provided a referral or invitation ticket."]
                    NoTicket,
                    #[codec(index = 26)]
                    #[doc = "The account is not authorized to do this."]
                    NotAuthorized,
                    #[codec(index = 27)]
                    #[doc = "The personal identity doesn't exist."]
                    NotPerson,
                    #[codec(index = 28)]
                    #[doc = "The candidate is referred. But the operation requires the candidate to not be referred."]
                    ReferredCandidate,
                    #[codec(index = 29)]
                    #[doc = "The candidate is not referred. But the operation requires the candidate to be referred."]
                    NotReferredCandidate,
                    #[codec(index = 30)]
                    #[doc = "There is no voucher to register."]
                    NoVoucherToRegister,
                    #[codec(index = 31)]
                    #[doc = "There is voucher that needs to be registered first."]
                    VoucherToRegister,
                    #[codec(index = 32)]
                    #[doc = "No inviter found."]
                    NoInviter,
                    #[codec(index = 33)]
                    #[doc = "Invalid signature."]
                    InvalidSignature,
                    #[codec(index = 34)]
                    #[doc = "No invite available."]
                    NoInvites,
                    #[codec(index = 35)]
                    #[doc = "Invite is already set."]
                    AlreadyInvited,
                    #[codec(index = 36)]
                    #[doc = "The referrer is not a person."]
                    NoReferrer,
                    #[codec(index = 37)]
                    #[doc = "The individual is not a person recognized by proof of ink."]
                    NotPoiPerson,
                    #[codec(index = 38)]
                    #[doc = "The proof of ownership is invalid."]
                    InvalidProofOfOwnership,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "The `Event` enum of this pallet"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "Candidate applied for verification."]
                    CandidateApplied {
                        account_id: ::subxt::ext::subxt_core::utils::AccountId32,
                    },
                    #[codec(index = 1)]
                    #[doc = "Candidate applied for verification through a referral."]
                    ReferralApplied {
                        account_id: ::subxt::ext::subxt_core::utils::AccountId32,
                        referrer: ::core::primitive::u64,
                    },
                    #[codec(index = 2)]
                    #[doc = "Candidate opened a judicial case for their verification evidence."]
                    JudgementRequested {
                        account_id: ::subxt::ext::subxt_core::utils::AccountId32,
                    },
                    #[codec(index = 3)]
                    #[doc = "Oracle has provided the judgement for a judicial case."]
                    JudgementProvided {
                        account_id: ::subxt::ext::subxt_core::utils::AccountId32,
                        judgement: runtime_types::individuality_support::traits::reality::Judgement,
                    },
                    #[codec(index = 4)]
                    #[doc = "A candidate has been granted a free retry attempt after a failure to verify their"]
                    #[doc = "evidence."]
                    RetryGranted {
                        account_id: ::subxt::ext::subxt_core::utils::AccountId32,
                        failures: ::core::primitive::u32,
                    },
                    #[codec(index = 5)]
                    #[doc = "Register an account as a person."]
                    PersonRegistered {
                        account_id: ::subxt::ext::subxt_core::utils::AccountId32,
                        personal_id: ::core::primitive::u64,
                    },
                    #[codec(index = 6)]
                    #[doc = "Person referred an account."]
                    CandidateReferred {
                        referrer: ::core::primitive::u64,
                        referred: ::subxt::ext::subxt_core::utils::AccountId32,
                    },
                    #[codec(index = 7)]
                    #[doc = "Entropy for a candidate changed after expiry."]
                    Rerolled {
                        account_id: ::subxt::ext::subxt_core::utils::AccountId32,
                    },
                    #[codec(index = 8)]
                    #[doc = "Candidate committed to an ink design."]
                    DesignCommitted {
                        account_id: ::subxt::ext::subxt_core::utils::AccountId32,
                        reserved_id: ::core::primitive::u64,
                    },
                    #[codec(index = 9)]
                    #[doc = "Storage fully allocated for a committed design."]
                    FullyAllocated {
                        account_id: ::subxt::ext::subxt_core::utils::AccountId32,
                    },
                    #[codec(index = 10)]
                    #[doc = "Candidate removed after timeout."]
                    TimedOut {
                        account_id: ::subxt::ext::subxt_core::utils::AccountId32,
                    },
                    #[codec(index = 11)]
                    #[doc = "Uncommitted candidate removed."]
                    FlakedOut {
                        account_id: ::subxt::ext::subxt_core::utils::AccountId32,
                    },
                    #[codec(index = 12)]
                    #[doc = "Referral ticket created by storing the public key on chain."]
                    TicketReferred { referrer: ::core::primitive::u64 },
                    #[codec(index = 13)]
                    #[doc = "Referral ticket removed by removing the public key from chain storage."]
                    TicketCancelled { referrer: ::core::primitive::u64 },
                    #[codec(index = 14)]
                    #[doc = "Candidate applied using a referral ticket."]
                    TicketApplied {
                        account_id: ::subxt::ext::subxt_core::utils::AccountId32,
                        referrer: ::core::primitive::u64,
                    },
                    #[codec(index = 15)]
                    #[doc = "Design family added."]
                    FamilyAdded {
                        index: ::core::primitive::u16,
                        kind: runtime_types::pallet_proof_of_ink::pallet::FamilyKind,
                        id: [::core::primitive::u8; 32usize],
                    },
                    #[codec(index = 16)]
                    #[doc = "All invites have been removed for the inviter."]
                    AllInvitesRemoved {
                        inviter: ::subxt::ext::subxt_core::utils::AccountId32,
                    },
                    #[codec(index = 17)]
                    #[doc = "Some invites have been removed for the inviter, some are remaining."]
                    SomeInvitesRemoved {
                        inviter: ::subxt::ext::subxt_core::utils::AccountId32,
                    },
                    #[codec(index = 18)]
                    #[doc = "Candidate applied using an invitation."]
                    InvitedCandidateApplied {
                        who: ::subxt::ext::subxt_core::utils::AccountId32,
                        inviter: ::subxt::ext::subxt_core::utils::AccountId32,
                    },
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub enum FamilyKind {
                    #[codec(index = 0)]
                    Designed { count: ::core::primitive::u16 },
                    #[codec(index = 1)]
                    ProceduralAccount,
                    #[codec(index = 2)]
                    ProceduralPersonal,
                    #[codec(index = 3)]
                    Procedural { range: ::core::primitive::u8 },
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub enum HoldReason {
                    #[codec(index = 0)]
                    ProofOfInk,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub enum InkChoice {
                    #[codec(index = 0)]
                    DesignedElective(::core::primitive::u16, ::core::primitive::u16),
                    #[codec(index = 1)]
                    ProceduralAccount(::core::primitive::u16),
                    #[codec(index = 2)]
                    ProceduralPersonal(::core::primitive::u16),
                    #[codec(index = 3)]
                    Procedural(::core::primitive::u16, ::core::primitive::u8),
                    #[codec(index = 4)]
                    ProceduralDerivative(
                        ::core::primitive::u64,
                        ::core::option::Option<::core::primitive::u64>,
                    ),
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub enum Origin {
                    #[codec(index = 0)]
                    AuthorizedApplyWithSig(::subxt::ext::subxt_core::utils::AccountId32),
                    #[codec(index = 1)]
                    ReferredCandidate(::subxt::ext::subxt_core::utils::AccountId32),
                    #[codec(index = 2)]
                    InvitedCandidate(::subxt::ext::subxt_core::utils::AccountId32),
                }
            }
        }
        pub mod pallet_score {
            use super::runtime_types;
            pub mod extension {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct ScoreAsParticipant(
                    pub  ::core::option::Option<
                        runtime_types::pallet_score::extension::ScoreAsParticipantData<
                            ::core::primitive::u32,
                        >,
                    >,
                );
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct ScoreAsParticipantData<_0> {
                    pub nonce: _0,
                }
            }
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
                pub enum Call {
                    #[codec(index = 1)]
                    #[doc = "Schedule payout rounds."]
                    #[doc = ""]
                    #[doc = "Called from `ScheduleOrigin` or root."]
                    schedule_payout_rounds {
                        amount: ::core::primitive::u128,
                        count: ::core::primitive::u32,
                        duration: ::core::primitive::u32,
                    },
                    #[codec(index = 2)]
                    #[doc = "Remove a scheduled payout round."]
                    #[doc = ""]
                    #[doc = "Called from `ScheduleOrigin` or root."]
                    remove_payout_schedule { index: ::core::primitive::u32 },
                    #[codec(index = 3)]
                    #[doc = "Start a new round."]
                    #[doc = ""]
                    #[doc = "This is valid if the current round is finished, or if the current round doesn't have"]
                    #[doc = "planning and a schedule exists to plan it."]
                    #[doc = ""]
                    #[doc = "This is a task, can be called from anybody."]
                    transition_round,
                    #[codec(index = 4)]
                    #[doc = "Operate some round paying out."]
                    #[doc = ""]
                    #[doc = "This is a task, can be called from anybody."]
                    #[doc = ""]
                    #[doc = "* round_index: The index of the round to operate."]
                    #[doc = "* limit: The maximum number of participants to operate in this call."]
                    operate_payout_round {
                        round_index: ::core::primitive::u32,
                        limit: ::core::primitive::u32,
                    },
                    #[codec(index = 5)]
                    #[doc = "Cash out. Reduce score, get some points. Caller must be a never recognized participant."]
                    #[doc = ""]
                    #[doc = "Origin must be signed or participant (signed extrinsic using ScoreAsParticipant"]
                    #[doc = "transaction extension)"]
                    #[doc = ""]
                    #[doc = "Alias origin is not allowed as they can't cash out."]
                    cash_out,
                    #[codec(index = 6)]
                    #[doc = "Redeem credit. Register a voucher."]
                    #[doc = ""]
                    #[doc = "Origin must be an alias or a signed account or a participant (signed extrinsic"]
                    #[doc = "using ScoreAsParticipant transaction extension)."]
                    redeem_credit {
                        voucher: runtime_types::verifiable::ring_vrf_impl::EncodedPublicKey,
                    },
                    #[codec(index = 7)]
                    #[doc = "Register as a person, or resume personhood."]
                    #[doc = ""]
                    #[doc = "Origin must be signed or a participant (signed extrinsic using ScoreAsParticipant"]
                    #[doc = "transaction extension)."]
                    #[doc = ""]
                    #[doc = "If the person was recognized and suspended, they must not provide a key."]
                    #[doc = "If the person was never recognized, they must provide a key and proof of ownership."]
                    #[doc = "* `key` - The key and proof of ownership signature of `\"pop register using\" ++ sender"]
                    #[doc = "  account id`."]
                    register {
                        key: ::core::option::Option<(
                            runtime_types::verifiable::ring_vrf_impl::EncodedPublicKey,
                            [::core::primitive::u8; 96usize],
                        )>,
                    },
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "The `Error` enum of this pallet."]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "The calling origin is not a person."]
                    NotPerson,
                    #[codec(index = 1)]
                    #[doc = "The person does not have a high enough score."]
                    LowScore,
                    #[codec(index = 2)]
                    #[doc = "No reward available."]
                    NoReward,
                    #[codec(index = 3)]
                    #[doc = "The person has no associated score."]
                    NoScore,
                    #[codec(index = 4)]
                    #[doc = "No payout schedule available."]
                    NoSchedule,
                    #[codec(index = 5)]
                    #[doc = "Too many payout schedules already registered."]
                    TooManySchedules,
                    #[codec(index = 6)]
                    #[doc = "The participant is recognized or has been recognized as a person."]
                    Recognized,
                    #[codec(index = 7)]
                    #[doc = "The participant has already cashed out in this era."]
                    CashOutCooldown,
                    #[codec(index = 8)]
                    #[doc = "The round is on going or no schedule."]
                    RoundOnGoingOrNoSchedule,
                    #[codec(index = 9)]
                    #[doc = "The payout round has not started."]
                    NoRound,
                    #[codec(index = 10)]
                    #[doc = "The origin is neither a person nor a signed account."]
                    BadOriginNotPersonNotSigned,
                    #[codec(index = 11)]
                    #[doc = "The origin is neither a person nor a signed account nor an account participant."]
                    BadOriginNotPersonNotSignedNotAccountParticipant,
                    #[codec(index = 12)]
                    #[doc = "The origin is neither a signed account nor an account participant."]
                    BadOriginNotSignedNotAccountParticipant,
                    #[codec(index = 13)]
                    #[doc = "The participant is already participating."]
                    AlreadyParticipating,
                    #[codec(index = 14)]
                    #[doc = "The key must be provided."]
                    KeyMustBeProvided,
                    #[codec(index = 15)]
                    #[doc = "The key must not be provided."]
                    KeyMustNotBeProvided,
                    #[codec(index = 16)]
                    #[doc = "Has reached personhood in the past."]
                    HasReachedPersonhood,
                    #[codec(index = 17)]
                    #[doc = "The proof of ownership is invalid."]
                    InvalidProofOfOwnership,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "The `Event` enum of this pallet"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "A person has claimed credit."]
                    Credit {
                        who: runtime_types::pallet_score::types::AccountOrPerson<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                        >,
                        voucher: runtime_types::verifiable::ring_vrf_impl::EncodedPublicKey,
                    },
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub enum HoldReason {
                    #[codec(index = 0)]
                    Payout,
                    #[codec(index = 1)]
                    Credit,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub enum Origin {
                    #[codec(index = 0)]
                    AccountParticipant(::subxt::ext::subxt_core::utils::AccountId32),
                }
            }
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub enum AccountOrPerson<_0> {
                    #[codec(index = 0)]
                    Account(_0),
                    #[codec(index = 1)]
                    Person([::core::primitive::u8; 32usize]),
                }
            }
        }
        pub mod pallet_session {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Sets the session key(s) of the function caller to `keys`."]
                    #[doc = "Allows an account to set its session key prior to becoming a validator."]
                    #[doc = "This doesn't take effect until the next session."]
                    #[doc = ""]
                    #[doc = "The dispatch origin of this function must be signed."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- `O(1)`. Actual cost depends on the number of length of `T::Keys::key_ids()` which is"]
                    #[doc = "  fixed."]
                    set_keys {
                        keys: runtime_types::people_rococo_runtime::SessionKeys,
                        proof: ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 1)]
                    #[doc = "Removes any session key(s) of the function caller."]
                    #[doc = ""]
                    #[doc = "This doesn't take effect until the next session."]
                    #[doc = ""]
                    #[doc = "The dispatch origin of this function must be Signed and the account must be either be"]
                    #[doc = "convertible to a validator ID using the chain's typical addressing system (this usually"]
                    #[doc = "means being a controller account) or directly convertible into a validator ID (which"]
                    #[doc = "usually means being a stash account)."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- `O(1)` in number of key types. Actual cost depends on the number of length of"]
                    #[doc = "  `T::Keys::key_ids()` which is fixed."]
                    purge_keys,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Error for the session pallet."]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "Invalid ownership proof."]
                    InvalidProof,
                    #[codec(index = 1)]
                    #[doc = "No associated validator ID for account."]
                    NoAssociatedValidatorId,
                    #[codec(index = 2)]
                    #[doc = "Registered duplicate key."]
                    DuplicatedKey,
                    #[codec(index = 3)]
                    #[doc = "No keys are associated with this account."]
                    NoKeys,
                    #[codec(index = 4)]
                    #[doc = "Key setting account is not live, so it's impossible to associate keys."]
                    NoAccount,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "The `Event` enum of this pallet"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "New session has happened. Note that the argument is the session index, not the"]
                    #[doc = "block number as the type might suggest."]
                    NewSession {
                        session_index: ::core::primitive::u32,
                    },
                    #[codec(index = 1)]
                    #[doc = "The `NewSession` event in the current block also implies a new validator set to be"]
                    #[doc = "queued."]
                    NewQueued,
                    #[codec(index = 2)]
                    #[doc = "Validator has been disabled."]
                    ValidatorDisabled {
                        validator: ::subxt::ext::subxt_core::utils::AccountId32,
                    },
                    #[codec(index = 3)]
                    #[doc = "Validator has been re-enabled."]
                    ValidatorReenabled {
                        validator: ::subxt::ext::subxt_core::utils::AccountId32,
                    },
                }
            }
        }
        pub mod pallet_skip_feeless_payment {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "The `Event` enum of this pallet"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "A transaction fee was skipped."]
                    FeeSkipped {
                        origin: runtime_types::people_rococo_runtime::OriginCaller,
                    },
                }
            }
        }
        pub mod pallet_sudo {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Authenticates the sudo key and dispatches a function call with `Root` origin."]
                    sudo {
                        call: ::subxt::ext::subxt_core::alloc::boxed::Box<
                            runtime_types::people_rococo_runtime::RuntimeCall,
                        >,
                    },
                    #[codec(index = 1)]
                    #[doc = "Authenticates the sudo key and dispatches a function call with `Root` origin."]
                    #[doc = "This function does not check the weight of the call, and instead allows the"]
                    #[doc = "Sudo user to specify the weight of the call."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_."]
                    sudo_unchecked_weight {
                        call: ::subxt::ext::subxt_core::alloc::boxed::Box<
                            runtime_types::people_rococo_runtime::RuntimeCall,
                        >,
                        weight: runtime_types::sp_weights::weight_v2::Weight,
                    },
                    #[codec(index = 2)]
                    #[doc = "Authenticates the current sudo key and sets the given AccountId (`new`) as the new sudo"]
                    #[doc = "key."]
                    set_key {
                        new: ::subxt::ext::subxt_core::utils::MultiAddress<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            (),
                        >,
                    },
                    #[codec(index = 3)]
                    #[doc = "Authenticates the sudo key and dispatches a function call with `Signed` origin from"]
                    #[doc = "a given account."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_."]
                    sudo_as {
                        who: ::subxt::ext::subxt_core::utils::MultiAddress<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            (),
                        >,
                        call: ::subxt::ext::subxt_core::alloc::boxed::Box<
                            runtime_types::people_rococo_runtime::RuntimeCall,
                        >,
                    },
                    #[codec(index = 4)]
                    #[doc = "Permanently removes the sudo key."]
                    #[doc = ""]
                    #[doc = "**This cannot be un-done.**"]
                    remove_key,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Error for the Sudo pallet."]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "Sender must be the Sudo account."]
                    RequireSudo,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "The `Event` enum of this pallet"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "A sudo call just took place."]
                    Sudid {
                        sudo_result:
                            ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
                    },
                    #[codec(index = 1)]
                    #[doc = "The sudo key has been updated."]
                    KeyChanged {
                        old: ::core::option::Option<::subxt::ext::subxt_core::utils::AccountId32>,
                        new: ::subxt::ext::subxt_core::utils::AccountId32,
                    },
                    #[codec(index = 2)]
                    #[doc = "The key was permanently removed."]
                    KeyRemoved,
                    #[codec(index = 3)]
                    #[doc = "A [sudo_as](Pallet::sudo_as) call just took place."]
                    SudoAsDone {
                        sudo_result:
                            ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
                    },
                }
            }
        }
        pub mod pallet_timestamp {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Set the current time."]
                    #[doc = ""]
                    #[doc = "This call should be invoked exactly once per block. It will panic at the finalization"]
                    #[doc = "phase, if this call hasn't been invoked by that time."]
                    #[doc = ""]
                    #[doc = "The timestamp should be greater than the previous one by the amount specified by"]
                    #[doc = "[`Config::MinimumPeriod`]."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _None_."]
                    #[doc = ""]
                    #[doc = "This dispatch class is _Mandatory_ to ensure it gets executed in the block. Be aware"]
                    #[doc = "that changing the complexity of this call could result exhausting the resources in a"]
                    #[doc = "block to execute any other calls."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- `O(1)` (Note that implementations of `OnTimestampSet` must also be `O(1)`)"]
                    #[doc = "- 1 storage read and 1 storage mutation (codec `O(1)` because of `DidUpdate::take` in"]
                    #[doc = "  `on_finalize`)"]
                    #[doc = "- 1 event handler `on_timestamp_set`. Must be `O(1)`."]
                    set {
                        #[codec(compact)]
                        now: ::core::primitive::u64,
                    },
                }
            }
        }
        pub mod pallet_transaction_payment {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "The `Event` enum of this pallet"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "A transaction fee `actual_fee`, of which `tip` was added to the minimum inclusion fee,"]
                    #[doc = "has been paid by `who`."]
                    TransactionFeePaid {
                        who: ::subxt::ext::subxt_core::utils::AccountId32,
                        actual_fee: ::core::primitive::u128,
                        tip: ::core::primitive::u128,
                    },
                }
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[codec(dumb_trait_bound)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct ChargeTransactionPayment(#[codec(compact)] pub ::core::primitive::u128);
        }
        pub mod pallet_utility {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Send a batch of dispatch calls."]
                    #[doc = ""]
                    #[doc = "May be called from any origin except `None`."]
                    #[doc = ""]
                    #[doc = "- `calls`: The calls to be dispatched from the same origin. The number of call must not"]
                    #[doc = "  exceed the constant: `batched_calls_limit` (available in constant metadata)."]
                    #[doc = ""]
                    #[doc = "If origin is root then the calls are dispatched without checking origin filter. (This"]
                    #[doc = "includes bypassing `frame_system::Config::BaseCallFilter`)."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- O(C) where C is the number of calls to be batched."]
                    #[doc = ""]
                    #[doc = "This will return `Ok` in all circumstances. To determine the success of the batch, an"]
                    #[doc = "event is deposited. If a call failed and the batch was interrupted, then the"]
                    #[doc = "`BatchInterrupted` event is deposited, along with the number of successful calls made"]
                    #[doc = "and the error of the failed call. If all were successful, then the `BatchCompleted`"]
                    #[doc = "event is deposited."]
                    batch {
                        calls: ::subxt::ext::subxt_core::alloc::vec::Vec<
                            runtime_types::people_rococo_runtime::RuntimeCall,
                        >,
                    },
                    #[codec(index = 1)]
                    #[doc = "Send a call through an indexed pseudonym of the sender."]
                    #[doc = ""]
                    #[doc = "Filter from origin are passed along. The call will be dispatched with an origin which"]
                    #[doc = "use the same filter as the origin of this call."]
                    #[doc = ""]
                    #[doc = "NOTE: If you need to ensure that any account-based filtering is not honored (i.e."]
                    #[doc = "because you expect `proxy` to have been used prior in the call stack and you do not want"]
                    #[doc = "the call restrictions to apply to any sub-accounts), then use `as_multi_threshold_1`"]
                    #[doc = "in the Multisig pallet instead."]
                    #[doc = ""]
                    #[doc = "NOTE: Prior to version *12, this was called `as_limited_sub`."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_."]
                    as_derivative {
                        index: ::core::primitive::u16,
                        call: ::subxt::ext::subxt_core::alloc::boxed::Box<
                            runtime_types::people_rococo_runtime::RuntimeCall,
                        >,
                    },
                    #[codec(index = 2)]
                    #[doc = "Send a batch of dispatch calls and atomically execute them."]
                    #[doc = "The whole transaction will rollback and fail if any of the calls failed."]
                    #[doc = ""]
                    #[doc = "May be called from any origin except `None`."]
                    #[doc = ""]
                    #[doc = "- `calls`: The calls to be dispatched from the same origin. The number of call must not"]
                    #[doc = "  exceed the constant: `batched_calls_limit` (available in constant metadata)."]
                    #[doc = ""]
                    #[doc = "If origin is root then the calls are dispatched without checking origin filter. (This"]
                    #[doc = "includes bypassing `frame_system::Config::BaseCallFilter`)."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- O(C) where C is the number of calls to be batched."]
                    batch_all {
                        calls: ::subxt::ext::subxt_core::alloc::vec::Vec<
                            runtime_types::people_rococo_runtime::RuntimeCall,
                        >,
                    },
                    #[codec(index = 3)]
                    #[doc = "Dispatches a function call with a provided origin."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Root_."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- O(1)."]
                    dispatch_as {
                        as_origin: ::subxt::ext::subxt_core::alloc::boxed::Box<
                            runtime_types::people_rococo_runtime::OriginCaller,
                        >,
                        call: ::subxt::ext::subxt_core::alloc::boxed::Box<
                            runtime_types::people_rococo_runtime::RuntimeCall,
                        >,
                    },
                    #[codec(index = 4)]
                    #[doc = "Send a batch of dispatch calls."]
                    #[doc = "Unlike `batch`, it allows errors and won't interrupt."]
                    #[doc = ""]
                    #[doc = "May be called from any origin except `None`."]
                    #[doc = ""]
                    #[doc = "- `calls`: The calls to be dispatched from the same origin. The number of call must not"]
                    #[doc = "  exceed the constant: `batched_calls_limit` (available in constant metadata)."]
                    #[doc = ""]
                    #[doc = "If origin is root then the calls are dispatch without checking origin filter. (This"]
                    #[doc = "includes bypassing `frame_system::Config::BaseCallFilter`)."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- O(C) where C is the number of calls to be batched."]
                    force_batch {
                        calls: ::subxt::ext::subxt_core::alloc::vec::Vec<
                            runtime_types::people_rococo_runtime::RuntimeCall,
                        >,
                    },
                    #[codec(index = 5)]
                    #[doc = "Dispatch a function call with a specified weight."]
                    #[doc = ""]
                    #[doc = "This function does not check the weight of the call, and instead allows the"]
                    #[doc = "Root origin to specify the weight of the call."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Root_."]
                    with_weight {
                        call: ::subxt::ext::subxt_core::alloc::boxed::Box<
                            runtime_types::people_rococo_runtime::RuntimeCall,
                        >,
                        weight: runtime_types::sp_weights::weight_v2::Weight,
                    },
                    #[codec(index = 6)]
                    #[doc = "Dispatch a fallback call in the event the main call fails to execute."]
                    #[doc = "May be called from any origin except `None`."]
                    #[doc = ""]
                    #[doc = "This function first attempts to dispatch the `main` call."]
                    #[doc = "If the `main` call fails, the `fallback` is attemted."]
                    #[doc = "if the fallback is successfully dispatched, the weights of both calls"]
                    #[doc = "are accumulated and an event containing the main call error is deposited."]
                    #[doc = ""]
                    #[doc = "In the event of a fallback failure the whole call fails"]
                    #[doc = "with the weights returned."]
                    #[doc = ""]
                    #[doc = "- `main`: The main call to be dispatched. This is the primary action to execute."]
                    #[doc = "- `fallback`: The fallback call to be dispatched in case the `main` call fails."]
                    #[doc = ""]
                    #[doc = "## Dispatch Logic"]
                    #[doc = "- If the origin is `root`, both the main and fallback calls are executed without"]
                    #[doc = "  applying any origin filters."]
                    #[doc = "- If the origin is not `root`, the origin filter is applied to both the `main` and"]
                    #[doc = "  `fallback` calls."]
                    #[doc = ""]
                    #[doc = "## Use Case"]
                    #[doc = "- Some use cases might involve submitting a `batch` type call in either main, fallback"]
                    #[doc = "  or both."]
                    if_else {
                        main: ::subxt::ext::subxt_core::alloc::boxed::Box<
                            runtime_types::people_rococo_runtime::RuntimeCall,
                        >,
                        fallback: ::subxt::ext::subxt_core::alloc::boxed::Box<
                            runtime_types::people_rococo_runtime::RuntimeCall,
                        >,
                    },
                    #[codec(index = 7)]
                    #[doc = "Dispatches a function call with a provided origin."]
                    #[doc = ""]
                    #[doc = "Almost the same as [`Pallet::dispatch_as`] but forwards any error of the inner call."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Root_."]
                    dispatch_as_fallible {
                        as_origin: ::subxt::ext::subxt_core::alloc::boxed::Box<
                            runtime_types::people_rococo_runtime::OriginCaller,
                        >,
                        call: ::subxt::ext::subxt_core::alloc::boxed::Box<
                            runtime_types::people_rococo_runtime::RuntimeCall,
                        >,
                    },
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "The `Error` enum of this pallet."]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "Too many calls batched."]
                    TooManyCalls,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "The `Event` enum of this pallet"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "Batch of dispatches did not complete fully. Index of first failing dispatch given, as"]
                    #[doc = "well as the error."]
                    BatchInterrupted {
                        index: ::core::primitive::u32,
                        error: runtime_types::sp_runtime::DispatchError,
                    },
                    #[codec(index = 1)]
                    #[doc = "Batch of dispatches completed fully with no error."]
                    BatchCompleted,
                    #[codec(index = 2)]
                    #[doc = "Batch of dispatches completed but has errors."]
                    BatchCompletedWithErrors,
                    #[codec(index = 3)]
                    #[doc = "A single item within a Batch of dispatches has completed with no error."]
                    ItemCompleted,
                    #[codec(index = 4)]
                    #[doc = "A single item within a Batch of dispatches has completed with error."]
                    ItemFailed {
                        error: runtime_types::sp_runtime::DispatchError,
                    },
                    #[codec(index = 5)]
                    #[doc = "A call was dispatched."]
                    DispatchedAs {
                        result:
                            ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
                    },
                    #[codec(index = 6)]
                    #[doc = "Main call was dispatched."]
                    IfElseMainSuccess,
                    #[codec(index = 7)]
                    #[doc = "The fallback call was dispatched."]
                    IfElseFallbackCalled {
                        main_error: runtime_types::sp_runtime::DispatchError,
                    },
                }
            }
        }
        pub mod pallet_verify_signature {
            use super::runtime_types;
            pub mod extension {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub enum VerifySignature {
                    #[codec(index = 0)]
                    Signed {
                        signature: runtime_types::sp_runtime::MultiSignature,
                        account: ::subxt::ext::subxt_core::utils::AccountId32,
                    },
                    #[codec(index = 1)]
                    Disabled,
                }
            }
        }
        pub mod pallet_xcm {
            use super::runtime_types;
            pub mod errors {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub enum ExecutionError {
                    #[codec(index = 0)]
                    Overflow,
                    #[codec(index = 1)]
                    Unimplemented,
                    #[codec(index = 2)]
                    UntrustedReserveLocation,
                    #[codec(index = 3)]
                    UntrustedTeleportLocation,
                    #[codec(index = 4)]
                    LocationFull,
                    #[codec(index = 5)]
                    LocationNotInvertible,
                    #[codec(index = 6)]
                    BadOrigin,
                    #[codec(index = 7)]
                    InvalidLocation,
                    #[codec(index = 8)]
                    AssetNotFound,
                    #[codec(index = 9)]
                    FailedToTransactAsset,
                    #[codec(index = 10)]
                    NotWithdrawable,
                    #[codec(index = 11)]
                    LocationCannotHold,
                    #[codec(index = 12)]
                    ExceedsMaxMessageSize,
                    #[codec(index = 13)]
                    DestinationUnsupported,
                    #[codec(index = 14)]
                    Transport,
                    #[codec(index = 15)]
                    Unroutable,
                    #[codec(index = 16)]
                    UnknownClaim,
                    #[codec(index = 17)]
                    FailedToDecode,
                    #[codec(index = 18)]
                    MaxWeightInvalid,
                    #[codec(index = 19)]
                    NotHoldingFees,
                    #[codec(index = 20)]
                    TooExpensive,
                    #[codec(index = 21)]
                    Trap,
                    #[codec(index = 22)]
                    ExpectationFalse,
                    #[codec(index = 23)]
                    PalletNotFound,
                    #[codec(index = 24)]
                    NameMismatch,
                    #[codec(index = 25)]
                    VersionIncompatible,
                    #[codec(index = 26)]
                    HoldingWouldOverflow,
                    #[codec(index = 27)]
                    ExportError,
                    #[codec(index = 28)]
                    ReanchorFailed,
                    #[codec(index = 29)]
                    NoDeal,
                    #[codec(index = 30)]
                    FeesNotMet,
                    #[codec(index = 31)]
                    LockError,
                    #[codec(index = 32)]
                    NoPermission,
                    #[codec(index = 33)]
                    Unanchored,
                    #[codec(index = 34)]
                    NotDepositable,
                    #[codec(index = 35)]
                    TooManyAssets,
                    #[codec(index = 36)]
                    UnhandledXcmVersion,
                    #[codec(index = 37)]
                    WeightLimitReached,
                    #[codec(index = 38)]
                    Barrier,
                    #[codec(index = 39)]
                    WeightNotComputable,
                    #[codec(index = 40)]
                    ExceedsStackLimit,
                }
            }
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
                pub enum Call {
                    # [codec (index = 0)] send { dest : :: subxt :: ext :: subxt_core :: alloc :: boxed :: Box < runtime_types :: xcm :: VersionedLocation > , message : :: subxt :: ext :: subxt_core :: alloc :: boxed :: Box < runtime_types :: xcm :: VersionedXcm > , } , # [codec (index = 1)] # [doc = "Teleport some assets from the local chain to some destination chain."] # [doc = ""] # [doc = "**This function is deprecated: Use `limited_teleport_assets` instead.**"] # [doc = ""] # [doc = "Fee payment on the destination side is made from the asset in the `assets` vector of"] # [doc = "index `fee_asset_item`. The weight limit for fees is not provided and thus is unlimited,"] # [doc = "with all fees taken as needed from the asset."] # [doc = ""] # [doc = "- `origin`: Must be capable of withdrawing the `assets` and executing XCM."] # [doc = "- `dest`: Destination context for the assets. Will typically be `[Parent,"] # [doc = "  Parachain(..)]` to send from parachain to parachain, or `[Parachain(..)]` to send from"] # [doc = "  relay to parachain."] # [doc = "- `beneficiary`: A beneficiary location for the assets in the context of `dest`. Will"] # [doc = "  generally be an `AccountId32` value."] # [doc = "- `assets`: The assets to be withdrawn. This should include the assets used to pay the"] # [doc = "  fee on the `dest` chain."] # [doc = "- `fee_asset_item`: The index into `assets` of the item which should be used to pay"] # [doc = "  fees."] teleport_assets { dest : :: subxt :: ext :: subxt_core :: alloc :: boxed :: Box < runtime_types :: xcm :: VersionedLocation > , beneficiary : :: subxt :: ext :: subxt_core :: alloc :: boxed :: Box < runtime_types :: xcm :: VersionedLocation > , assets : :: subxt :: ext :: subxt_core :: alloc :: boxed :: Box < runtime_types :: xcm :: VersionedAssets > , fee_asset_item : :: core :: primitive :: u32 , } , # [codec (index = 2)] # [doc = "Transfer some assets from the local chain to the destination chain through their local,"] # [doc = "destination or remote reserve."] # [doc = ""] # [doc = "`assets` must have same reserve location and may not be teleportable to `dest`."] # [doc = " - `assets` have local reserve: transfer assets to sovereign account of destination"] # [doc = "   chain and forward a notification XCM to `dest` to mint and deposit reserve-based"] # [doc = "   assets to `beneficiary`."] # [doc = " - `assets` have destination reserve: burn local assets and forward a notification to"] # [doc = "   `dest` chain to withdraw the reserve assets from this chain's sovereign account and"] # [doc = "   deposit them to `beneficiary`."] # [doc = " - `assets` have remote reserve: burn local assets, forward XCM to reserve chain to move"] # [doc = "   reserves from this chain's SA to `dest` chain's SA, and forward another XCM to `dest`"] # [doc = "   to mint and deposit reserve-based assets to `beneficiary`."] # [doc = ""] # [doc = "**This function is deprecated: Use `limited_reserve_transfer_assets` instead.**"] # [doc = ""] # [doc = "Fee payment on the destination side is made from the asset in the `assets` vector of"] # [doc = "index `fee_asset_item`. The weight limit for fees is not provided and thus is unlimited,"] # [doc = "with all fees taken as needed from the asset."] # [doc = ""] # [doc = "- `origin`: Must be capable of withdrawing the `assets` and executing XCM."] # [doc = "- `dest`: Destination context for the assets. Will typically be `[Parent,"] # [doc = "  Parachain(..)]` to send from parachain to parachain, or `[Parachain(..)]` to send from"] # [doc = "  relay to parachain."] # [doc = "- `beneficiary`: A beneficiary location for the assets in the context of `dest`. Will"] # [doc = "  generally be an `AccountId32` value."] # [doc = "- `assets`: The assets to be withdrawn. This should include the assets used to pay the"] # [doc = "  fee on the `dest` (and possibly reserve) chains."] # [doc = "- `fee_asset_item`: The index into `assets` of the item which should be used to pay"] # [doc = "  fees."] reserve_transfer_assets { dest : :: subxt :: ext :: subxt_core :: alloc :: boxed :: Box < runtime_types :: xcm :: VersionedLocation > , beneficiary : :: subxt :: ext :: subxt_core :: alloc :: boxed :: Box < runtime_types :: xcm :: VersionedLocation > , assets : :: subxt :: ext :: subxt_core :: alloc :: boxed :: Box < runtime_types :: xcm :: VersionedAssets > , fee_asset_item : :: core :: primitive :: u32 , } , # [codec (index = 3)] # [doc = "Execute an XCM message from a local, signed, origin."] # [doc = ""] # [doc = "An event is deposited indicating whether `msg` could be executed completely or only"] # [doc = "partially."] # [doc = ""] # [doc = "No more than `max_weight` will be used in its attempted execution. If this is less than"] # [doc = "the maximum amount of weight that the message could take to be executed, then no"] # [doc = "execution attempt will be made."] execute { message : :: subxt :: ext :: subxt_core :: alloc :: boxed :: Box < runtime_types :: xcm :: VersionedXcm > , max_weight : runtime_types :: sp_weights :: weight_v2 :: Weight , } , # [codec (index = 4)] # [doc = "Extoll that a particular destination can be communicated with through a particular"] # [doc = "version of XCM."] # [doc = ""] # [doc = "- `origin`: Must be an origin specified by AdminOrigin."] # [doc = "- `location`: The destination that is being described."] # [doc = "- `xcm_version`: The latest version of XCM that `location` supports."] force_xcm_version { location : :: subxt :: ext :: subxt_core :: alloc :: boxed :: Box < runtime_types :: staging_xcm :: v5 :: location :: Location > , version : :: core :: primitive :: u32 , } , # [codec (index = 5)] # [doc = "Set a safe XCM version (the version that XCM should be encoded with if the most recent"] # [doc = "version a destination can accept is unknown)."] # [doc = ""] # [doc = "- `origin`: Must be an origin specified by AdminOrigin."] # [doc = "- `maybe_xcm_version`: The default XCM encoding version, or `None` to disable."] force_default_xcm_version { maybe_xcm_version : :: core :: option :: Option < :: core :: primitive :: u32 > , } , # [codec (index = 6)] # [doc = "Ask a location to notify us regarding their XCM version and any changes to it."] # [doc = ""] # [doc = "- `origin`: Must be an origin specified by AdminOrigin."] # [doc = "- `location`: The location to which we should subscribe for XCM version notifications."] force_subscribe_version_notify { location : :: subxt :: ext :: subxt_core :: alloc :: boxed :: Box < runtime_types :: xcm :: VersionedLocation > , } , # [codec (index = 7)] # [doc = "Require that a particular destination should no longer notify us regarding any XCM"] # [doc = "version changes."] # [doc = ""] # [doc = "- `origin`: Must be an origin specified by AdminOrigin."] # [doc = "- `location`: The location to which we are currently subscribed for XCM version"] # [doc = "  notifications which we no longer desire."] force_unsubscribe_version_notify { location : :: subxt :: ext :: subxt_core :: alloc :: boxed :: Box < runtime_types :: xcm :: VersionedLocation > , } , # [codec (index = 8)] # [doc = "Transfer some assets from the local chain to the destination chain through their local,"] # [doc = "destination or remote reserve."] # [doc = ""] # [doc = "`assets` must have same reserve location and may not be teleportable to `dest`."] # [doc = " - `assets` have local reserve: transfer assets to sovereign account of destination"] # [doc = "   chain and forward a notification XCM to `dest` to mint and deposit reserve-based"] # [doc = "   assets to `beneficiary`."] # [doc = " - `assets` have destination reserve: burn local assets and forward a notification to"] # [doc = "   `dest` chain to withdraw the reserve assets from this chain's sovereign account and"] # [doc = "   deposit them to `beneficiary`."] # [doc = " - `assets` have remote reserve: burn local assets, forward XCM to reserve chain to move"] # [doc = "   reserves from this chain's SA to `dest` chain's SA, and forward another XCM to `dest`"] # [doc = "   to mint and deposit reserve-based assets to `beneficiary`."] # [doc = ""] # [doc = "Fee payment on the destination side is made from the asset in the `assets` vector of"] # [doc = "index `fee_asset_item`, up to enough to pay for `weight_limit` of weight. If more weight"] # [doc = "is needed than `weight_limit`, then the operation will fail and the sent assets may be"] # [doc = "at risk."] # [doc = ""] # [doc = "- `origin`: Must be capable of withdrawing the `assets` and executing XCM."] # [doc = "- `dest`: Destination context for the assets. Will typically be `[Parent,"] # [doc = "  Parachain(..)]` to send from parachain to parachain, or `[Parachain(..)]` to send from"] # [doc = "  relay to parachain."] # [doc = "- `beneficiary`: A beneficiary location for the assets in the context of `dest`. Will"] # [doc = "  generally be an `AccountId32` value."] # [doc = "- `assets`: The assets to be withdrawn. This should include the assets used to pay the"] # [doc = "  fee on the `dest` (and possibly reserve) chains."] # [doc = "- `fee_asset_item`: The index into `assets` of the item which should be used to pay"] # [doc = "  fees."] # [doc = "- `weight_limit`: The remote-side weight limit, if any, for the XCM fee purchase."] limited_reserve_transfer_assets { dest : :: subxt :: ext :: subxt_core :: alloc :: boxed :: Box < runtime_types :: xcm :: VersionedLocation > , beneficiary : :: subxt :: ext :: subxt_core :: alloc :: boxed :: Box < runtime_types :: xcm :: VersionedLocation > , assets : :: subxt :: ext :: subxt_core :: alloc :: boxed :: Box < runtime_types :: xcm :: VersionedAssets > , fee_asset_item : :: core :: primitive :: u32 , weight_limit : runtime_types :: xcm :: v3 :: WeightLimit , } , # [codec (index = 9)] # [doc = "Teleport some assets from the local chain to some destination chain."] # [doc = ""] # [doc = "Fee payment on the destination side is made from the asset in the `assets` vector of"] # [doc = "index `fee_asset_item`, up to enough to pay for `weight_limit` of weight. If more weight"] # [doc = "is needed than `weight_limit`, then the operation will fail and the sent assets may be"] # [doc = "at risk."] # [doc = ""] # [doc = "- `origin`: Must be capable of withdrawing the `assets` and executing XCM."] # [doc = "- `dest`: Destination context for the assets. Will typically be `[Parent,"] # [doc = "  Parachain(..)]` to send from parachain to parachain, or `[Parachain(..)]` to send from"] # [doc = "  relay to parachain."] # [doc = "- `beneficiary`: A beneficiary location for the assets in the context of `dest`. Will"] # [doc = "  generally be an `AccountId32` value."] # [doc = "- `assets`: The assets to be withdrawn. This should include the assets used to pay the"] # [doc = "  fee on the `dest` chain."] # [doc = "- `fee_asset_item`: The index into `assets` of the item which should be used to pay"] # [doc = "  fees."] # [doc = "- `weight_limit`: The remote-side weight limit, if any, for the XCM fee purchase."] limited_teleport_assets { dest : :: subxt :: ext :: subxt_core :: alloc :: boxed :: Box < runtime_types :: xcm :: VersionedLocation > , beneficiary : :: subxt :: ext :: subxt_core :: alloc :: boxed :: Box < runtime_types :: xcm :: VersionedLocation > , assets : :: subxt :: ext :: subxt_core :: alloc :: boxed :: Box < runtime_types :: xcm :: VersionedAssets > , fee_asset_item : :: core :: primitive :: u32 , weight_limit : runtime_types :: xcm :: v3 :: WeightLimit , } , # [codec (index = 10)] # [doc = "Set or unset the global suspension state of the XCM executor."] # [doc = ""] # [doc = "- `origin`: Must be an origin specified by AdminOrigin."] # [doc = "- `suspended`: `true` to suspend, `false` to resume."] force_suspension { suspended : :: core :: primitive :: bool , } , # [codec (index = 11)] # [doc = "Transfer some assets from the local chain to the destination chain through their local,"] # [doc = "destination or remote reserve, or through teleports."] # [doc = ""] # [doc = "Fee payment on the destination side is made from the asset in the `assets` vector of"] # [doc = "index `fee_asset_item` (hence referred to as `fees`), up to enough to pay for"] # [doc = "`weight_limit` of weight. If more weight is needed than `weight_limit`, then the"] # [doc = "operation will fail and the sent assets may be at risk."] # [doc = ""] # [doc = "`assets` (excluding `fees`) must have same reserve location or otherwise be teleportable"] # [doc = "to `dest`, no limitations imposed on `fees`."] # [doc = " - for local reserve: transfer assets to sovereign account of destination chain and"] # [doc = "   forward a notification XCM to `dest` to mint and deposit reserve-based assets to"] # [doc = "   `beneficiary`."] # [doc = " - for destination reserve: burn local assets and forward a notification to `dest` chain"] # [doc = "   to withdraw the reserve assets from this chain's sovereign account and deposit them"] # [doc = "   to `beneficiary`."] # [doc = " - for remote reserve: burn local assets, forward XCM to reserve chain to move reserves"] # [doc = "   from this chain's SA to `dest` chain's SA, and forward another XCM to `dest` to mint"] # [doc = "   and deposit reserve-based assets to `beneficiary`."] # [doc = " - for teleports: burn local assets and forward XCM to `dest` chain to mint/teleport"] # [doc = "   assets and deposit them to `beneficiary`."] # [doc = ""] # [doc = "- `origin`: Must be capable of withdrawing the `assets` and executing XCM."] # [doc = "- `dest`: Destination context for the assets. Will typically be `X2(Parent,"] # [doc = "  Parachain(..))` to send from parachain to parachain, or `X1(Parachain(..))` to send"] # [doc = "  from relay to parachain."] # [doc = "- `beneficiary`: A beneficiary location for the assets in the context of `dest`. Will"] # [doc = "  generally be an `AccountId32` value."] # [doc = "- `assets`: The assets to be withdrawn. This should include the assets used to pay the"] # [doc = "  fee on the `dest` (and possibly reserve) chains."] # [doc = "- `fee_asset_item`: The index into `assets` of the item which should be used to pay"] # [doc = "  fees."] # [doc = "- `weight_limit`: The remote-side weight limit, if any, for the XCM fee purchase."] transfer_assets { dest : :: subxt :: ext :: subxt_core :: alloc :: boxed :: Box < runtime_types :: xcm :: VersionedLocation > , beneficiary : :: subxt :: ext :: subxt_core :: alloc :: boxed :: Box < runtime_types :: xcm :: VersionedLocation > , assets : :: subxt :: ext :: subxt_core :: alloc :: boxed :: Box < runtime_types :: xcm :: VersionedAssets > , fee_asset_item : :: core :: primitive :: u32 , weight_limit : runtime_types :: xcm :: v3 :: WeightLimit , } , # [codec (index = 12)] # [doc = "Claims assets trapped on this pallet because of leftover assets during XCM execution."] # [doc = ""] # [doc = "- `origin`: Anyone can call this extrinsic."] # [doc = "- `assets`: The exact assets that were trapped. Use the version to specify what version"] # [doc = "was the latest when they were trapped."] # [doc = "- `beneficiary`: The location/account where the claimed assets will be deposited."] claim_assets { assets : :: subxt :: ext :: subxt_core :: alloc :: boxed :: Box < runtime_types :: xcm :: VersionedAssets > , beneficiary : :: subxt :: ext :: subxt_core :: alloc :: boxed :: Box < runtime_types :: xcm :: VersionedLocation > , } , # [codec (index = 13)] # [doc = "Transfer assets from the local chain to the destination chain using explicit transfer"] # [doc = "types for assets and fees."] # [doc = ""] # [doc = "`assets` must have same reserve location or may be teleportable to `dest`. Caller must"] # [doc = "provide the `assets_transfer_type` to be used for `assets`:"] # [doc = " - `TransferType::LocalReserve`: transfer assets to sovereign account of destination"] # [doc = "   chain and forward a notification XCM to `dest` to mint and deposit reserve-based"] # [doc = "   assets to `beneficiary`."] # [doc = " - `TransferType::DestinationReserve`: burn local assets and forward a notification to"] # [doc = "   `dest` chain to withdraw the reserve assets from this chain's sovereign account and"] # [doc = "   deposit them to `beneficiary`."] # [doc = " - `TransferType::RemoteReserve(reserve)`: burn local assets, forward XCM to `reserve`"] # [doc = "   chain to move reserves from this chain's SA to `dest` chain's SA, and forward another"] # [doc = "   XCM to `dest` to mint and deposit reserve-based assets to `beneficiary`. Typically"] # [doc = "   the remote `reserve` is Asset Hub."] # [doc = " - `TransferType::Teleport`: burn local assets and forward XCM to `dest` chain to"] # [doc = "   mint/teleport assets and deposit them to `beneficiary`."] # [doc = ""] # [doc = "On the destination chain, as well as any intermediary hops, `BuyExecution` is used to"] # [doc = "buy execution using transferred `assets` identified by `remote_fees_id`."] # [doc = "Make sure enough of the specified `remote_fees_id` asset is included in the given list"] # [doc = "of `assets`. `remote_fees_id` should be enough to pay for `weight_limit`. If more weight"] # [doc = "is needed than `weight_limit`, then the operation will fail and the sent assets may be"] # [doc = "at risk."] # [doc = ""] # [doc = "`remote_fees_id` may use different transfer type than rest of `assets` and can be"] # [doc = "specified through `fees_transfer_type`."] # [doc = ""] # [doc = "The caller needs to specify what should happen to the transferred assets once they reach"] # [doc = "the `dest` chain. This is done through the `custom_xcm_on_dest` parameter, which"] # [doc = "contains the instructions to execute on `dest` as a final step."] # [doc = "  This is usually as simple as:"] # [doc = "  `Xcm(vec![DepositAsset { assets: Wild(AllCounted(assets.len())), beneficiary }])`,"] # [doc = "  but could be something more exotic like sending the `assets` even further."] # [doc = ""] # [doc = "- `origin`: Must be capable of withdrawing the `assets` and executing XCM."] # [doc = "- `dest`: Destination context for the assets. Will typically be `[Parent,"] # [doc = "  Parachain(..)]` to send from parachain to parachain, or `[Parachain(..)]` to send from"] # [doc = "  relay to parachain, or `(parents: 2, (GlobalConsensus(..), ..))` to send from"] # [doc = "  parachain across a bridge to another ecosystem destination."] # [doc = "- `assets`: The assets to be withdrawn. This should include the assets used to pay the"] # [doc = "  fee on the `dest` (and possibly reserve) chains."] # [doc = "- `assets_transfer_type`: The XCM `TransferType` used to transfer the `assets`."] # [doc = "- `remote_fees_id`: One of the included `assets` to be used to pay fees."] # [doc = "- `fees_transfer_type`: The XCM `TransferType` used to transfer the `fees` assets."] # [doc = "- `custom_xcm_on_dest`: The XCM to be executed on `dest` chain as the last step of the"] # [doc = "  transfer, which also determines what happens to the assets on the destination chain."] # [doc = "- `weight_limit`: The remote-side weight limit, if any, for the XCM fee purchase."] transfer_assets_using_type_and_then { dest : :: subxt :: ext :: subxt_core :: alloc :: boxed :: Box < runtime_types :: xcm :: VersionedLocation > , assets : :: subxt :: ext :: subxt_core :: alloc :: boxed :: Box < runtime_types :: xcm :: VersionedAssets > , assets_transfer_type : :: subxt :: ext :: subxt_core :: alloc :: boxed :: Box < runtime_types :: staging_xcm_executor :: traits :: asset_transfer :: TransferType > , remote_fees_id : :: subxt :: ext :: subxt_core :: alloc :: boxed :: Box < runtime_types :: xcm :: VersionedAssetId > , fees_transfer_type : :: subxt :: ext :: subxt_core :: alloc :: boxed :: Box < runtime_types :: staging_xcm_executor :: traits :: asset_transfer :: TransferType > , custom_xcm_on_dest : :: subxt :: ext :: subxt_core :: alloc :: boxed :: Box < runtime_types :: xcm :: VersionedXcm > , weight_limit : runtime_types :: xcm :: v3 :: WeightLimit , } , # [codec (index = 14)] # [doc = "Authorize another `aliaser` location to alias into the local `origin` making this call."] # [doc = "The `aliaser` is only authorized until the provided `expiry` block number."] # [doc = "The call can also be used for a previously authorized alias in order to update its"] # [doc = "`expiry` block number."] # [doc = ""] # [doc = "Usually useful to allow your local account to be aliased into from a remote location"] # [doc = "also under your control (like your account on another chain)."] # [doc = ""] # [doc = "WARNING: make sure the caller `origin` (you) trusts the `aliaser` location to act in"] # [doc = "their/your name. Once authorized using this call, the `aliaser` can freely impersonate"] # [doc = "`origin` in XCM programs executed on the local chain."] add_authorized_alias { aliaser : :: subxt :: ext :: subxt_core :: alloc :: boxed :: Box < runtime_types :: xcm :: VersionedLocation > , expires : :: core :: option :: Option < :: core :: primitive :: u64 > , } , # [codec (index = 15)] # [doc = "Remove a previously authorized `aliaser` from the list of locations that can alias into"] # [doc = "the local `origin` making this call."] remove_authorized_alias { aliaser : :: subxt :: ext :: subxt_core :: alloc :: boxed :: Box < runtime_types :: xcm :: VersionedLocation > , } , # [codec (index = 16)] # [doc = "Remove all previously authorized `aliaser`s that can alias into the local `origin`"] # [doc = "making this call."] remove_all_authorized_aliases , }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "The `Error` enum of this pallet."]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "The desired destination was unreachable, generally because there is a no way of routing"]
                    #[doc = "to it."]
                    Unreachable,
                    #[codec(index = 1)]
                    #[doc = "There was some other issue (i.e. not to do with routing) in sending the message."]
                    #[doc = "Perhaps a lack of space for buffering the message."]
                    SendFailure,
                    #[codec(index = 2)]
                    #[doc = "The message execution fails the filter."]
                    Filtered,
                    #[codec(index = 3)]
                    #[doc = "The message's weight could not be determined."]
                    UnweighableMessage,
                    #[codec(index = 4)]
                    #[doc = "The destination `Location` provided cannot be inverted."]
                    DestinationNotInvertible,
                    #[codec(index = 5)]
                    #[doc = "The assets to be sent are empty."]
                    Empty,
                    #[codec(index = 6)]
                    #[doc = "Could not re-anchor the assets to declare the fees for the destination chain."]
                    CannotReanchor,
                    #[codec(index = 7)]
                    #[doc = "Too many assets have been attempted for transfer."]
                    TooManyAssets,
                    #[codec(index = 8)]
                    #[doc = "Origin is invalid for sending."]
                    InvalidOrigin,
                    #[codec(index = 9)]
                    #[doc = "The version of the `Versioned` value used is not able to be interpreted."]
                    BadVersion,
                    #[codec(index = 10)]
                    #[doc = "The given location could not be used (e.g. because it cannot be expressed in the"]
                    #[doc = "desired version of XCM)."]
                    BadLocation,
                    #[codec(index = 11)]
                    #[doc = "The referenced subscription could not be found."]
                    NoSubscription,
                    #[codec(index = 12)]
                    #[doc = "The location is invalid since it already has a subscription from us."]
                    AlreadySubscribed,
                    #[codec(index = 13)]
                    #[doc = "Could not check-out the assets for teleportation to the destination chain."]
                    CannotCheckOutTeleport,
                    #[codec(index = 14)]
                    #[doc = "The owner does not own (all) of the asset that they wish to do the operation on."]
                    LowBalance,
                    #[codec(index = 15)]
                    #[doc = "The asset owner has too many locks on the asset."]
                    TooManyLocks,
                    #[codec(index = 16)]
                    #[doc = "The given account is not an identifiable sovereign account for any location."]
                    AccountNotSovereign,
                    #[codec(index = 17)]
                    #[doc = "The operation required fees to be paid which the initiator could not meet."]
                    FeesNotMet,
                    #[codec(index = 18)]
                    #[doc = "A remote lock with the corresponding data could not be found."]
                    LockNotFound,
                    #[codec(index = 19)]
                    #[doc = "The unlock operation cannot succeed because there are still consumers of the lock."]
                    InUse,
                    #[codec(index = 21)]
                    #[doc = "Invalid asset, reserve chain could not be determined for it."]
                    InvalidAssetUnknownReserve,
                    #[codec(index = 22)]
                    #[doc = "Invalid asset, do not support remote asset reserves with different fees reserves."]
                    InvalidAssetUnsupportedReserve,
                    #[codec(index = 23)]
                    #[doc = "Too many assets with different reserve locations have been attempted for transfer."]
                    TooManyReserves,
                    #[codec(index = 24)]
                    #[doc = "Local XCM execution incomplete."]
                    LocalExecutionIncomplete,
                    #[codec(index = 25)]
                    #[doc = "Too many locations authorized to alias origin."]
                    TooManyAuthorizedAliases,
                    #[codec(index = 26)]
                    #[doc = "Expiry block number is in the past."]
                    ExpiresInPast,
                    #[codec(index = 27)]
                    #[doc = "The alias to remove authorization for was not found."]
                    AliasNotFound,
                    #[codec(index = 28)]
                    #[doc = "Local XCM execution incomplete with the actual XCM error and the index of the"]
                    #[doc = "instruction that caused the error."]
                    LocalExecutionIncompleteWithError {
                        index: ::core::primitive::u8,
                        error: runtime_types::pallet_xcm::errors::ExecutionError,
                    },
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "The `Event` enum of this pallet"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "Execution of an XCM message was attempted."]
                    Attempted {
                        outcome: runtime_types::staging_xcm::v5::traits::Outcome,
                    },
                    #[codec(index = 1)]
                    #[doc = "An XCM message was sent."]
                    Sent {
                        origin: runtime_types::staging_xcm::v5::location::Location,
                        destination: runtime_types::staging_xcm::v5::location::Location,
                        message: runtime_types::staging_xcm::v5::Xcm,
                        message_id: [::core::primitive::u8; 32usize],
                    },
                    #[codec(index = 2)]
                    #[doc = "An XCM message failed to send."]
                    SendFailed {
                        origin: runtime_types::staging_xcm::v5::location::Location,
                        destination: runtime_types::staging_xcm::v5::location::Location,
                        error: runtime_types::xcm::v3::traits::SendError,
                        message_id: [::core::primitive::u8; 32usize],
                    },
                    #[codec(index = 3)]
                    #[doc = "An XCM message failed to process."]
                    ProcessXcmError {
                        origin: runtime_types::staging_xcm::v5::location::Location,
                        error: runtime_types::xcm::v5::traits::Error,
                        message_id: [::core::primitive::u8; 32usize],
                    },
                    #[codec(index = 4)]
                    #[doc = "Query response received which does not match a registered query. This may be because a"]
                    #[doc = "matching query was never registered, it may be because it is a duplicate response, or"]
                    #[doc = "because the query timed out."]
                    UnexpectedResponse {
                        origin: runtime_types::staging_xcm::v5::location::Location,
                        query_id: ::core::primitive::u64,
                    },
                    #[codec(index = 5)]
                    #[doc = "Query response has been received and is ready for taking with `take_response`. There is"]
                    #[doc = "no registered notification call."]
                    ResponseReady {
                        query_id: ::core::primitive::u64,
                        response: runtime_types::staging_xcm::v5::Response,
                    },
                    #[codec(index = 6)]
                    #[doc = "Query response has been received and query is removed. The registered notification has"]
                    #[doc = "been dispatched and executed successfully."]
                    Notified {
                        query_id: ::core::primitive::u64,
                        pallet_index: ::core::primitive::u8,
                        call_index: ::core::primitive::u8,
                    },
                    #[codec(index = 7)]
                    #[doc = "Query response has been received and query is removed. The registered notification"]
                    #[doc = "could not be dispatched because the dispatch weight is greater than the maximum weight"]
                    #[doc = "originally budgeted by this runtime for the query result."]
                    NotifyOverweight {
                        query_id: ::core::primitive::u64,
                        pallet_index: ::core::primitive::u8,
                        call_index: ::core::primitive::u8,
                        actual_weight: runtime_types::sp_weights::weight_v2::Weight,
                        max_budgeted_weight: runtime_types::sp_weights::weight_v2::Weight,
                    },
                    #[codec(index = 8)]
                    #[doc = "Query response has been received and query is removed. There was a general error with"]
                    #[doc = "dispatching the notification call."]
                    NotifyDispatchError {
                        query_id: ::core::primitive::u64,
                        pallet_index: ::core::primitive::u8,
                        call_index: ::core::primitive::u8,
                    },
                    #[codec(index = 9)]
                    #[doc = "Query response has been received and query is removed. The dispatch was unable to be"]
                    #[doc = "decoded into a `Call`; this might be due to dispatch function having a signature which"]
                    #[doc = "is not `(origin, QueryId, Response)`."]
                    NotifyDecodeFailed {
                        query_id: ::core::primitive::u64,
                        pallet_index: ::core::primitive::u8,
                        call_index: ::core::primitive::u8,
                    },
                    #[codec(index = 10)]
                    #[doc = "Expected query response has been received but the origin location of the response does"]
                    #[doc = "not match that expected. The query remains registered for a later, valid, response to"]
                    #[doc = "be received and acted upon."]
                    InvalidResponder {
                        origin: runtime_types::staging_xcm::v5::location::Location,
                        query_id: ::core::primitive::u64,
                        expected_location: ::core::option::Option<
                            runtime_types::staging_xcm::v5::location::Location,
                        >,
                    },
                    #[codec(index = 11)]
                    #[doc = "Expected query response has been received but the expected origin location placed in"]
                    #[doc = "storage by this runtime previously cannot be decoded. The query remains registered."]
                    #[doc = ""]
                    #[doc = "This is unexpected (since a location placed in storage in a previously executing"]
                    #[doc = "runtime should be readable prior to query timeout) and dangerous since the possibly"]
                    #[doc = "valid response will be dropped. Manual governance intervention is probably going to be"]
                    #[doc = "needed."]
                    InvalidResponderVersion {
                        origin: runtime_types::staging_xcm::v5::location::Location,
                        query_id: ::core::primitive::u64,
                    },
                    #[codec(index = 12)]
                    #[doc = "Received query response has been read and removed."]
                    ResponseTaken { query_id: ::core::primitive::u64 },
                    #[codec(index = 13)]
                    #[doc = "Some assets have been placed in an asset trap."]
                    AssetsTrapped {
                        hash: ::subxt::ext::subxt_core::utils::H256,
                        origin: runtime_types::staging_xcm::v5::location::Location,
                        assets: runtime_types::xcm::VersionedAssets,
                    },
                    #[codec(index = 14)]
                    #[doc = "An XCM version change notification message has been attempted to be sent."]
                    #[doc = ""]
                    #[doc = "The cost of sending it (borne by the chain) is included."]
                    VersionChangeNotified {
                        destination: runtime_types::staging_xcm::v5::location::Location,
                        result: ::core::primitive::u32,
                        cost: runtime_types::staging_xcm::v5::asset::Assets,
                        message_id: [::core::primitive::u8; 32usize],
                    },
                    #[codec(index = 15)]
                    #[doc = "The supported version of a location has been changed. This might be through an"]
                    #[doc = "automatic notification or a manual intervention."]
                    SupportedVersionChanged {
                        location: runtime_types::staging_xcm::v5::location::Location,
                        version: ::core::primitive::u32,
                    },
                    #[codec(index = 16)]
                    #[doc = "A given location which had a version change subscription was dropped owing to an error"]
                    #[doc = "sending the notification to it."]
                    NotifyTargetSendFail {
                        location: runtime_types::staging_xcm::v5::location::Location,
                        query_id: ::core::primitive::u64,
                        error: runtime_types::xcm::v5::traits::Error,
                    },
                    #[codec(index = 17)]
                    #[doc = "A given location which had a version change subscription was dropped owing to an error"]
                    #[doc = "migrating the location to our new XCM format."]
                    NotifyTargetMigrationFail {
                        location: runtime_types::xcm::VersionedLocation,
                        query_id: ::core::primitive::u64,
                    },
                    #[codec(index = 18)]
                    #[doc = "Expected query response has been received but the expected querier location placed in"]
                    #[doc = "storage by this runtime previously cannot be decoded. The query remains registered."]
                    #[doc = ""]
                    #[doc = "This is unexpected (since a location placed in storage in a previously executing"]
                    #[doc = "runtime should be readable prior to query timeout) and dangerous since the possibly"]
                    #[doc = "valid response will be dropped. Manual governance intervention is probably going to be"]
                    #[doc = "needed."]
                    InvalidQuerierVersion {
                        origin: runtime_types::staging_xcm::v5::location::Location,
                        query_id: ::core::primitive::u64,
                    },
                    #[codec(index = 19)]
                    #[doc = "Expected query response has been received but the querier location of the response does"]
                    #[doc = "not match the expected. The query remains registered for a later, valid, response to"]
                    #[doc = "be received and acted upon."]
                    InvalidQuerier {
                        origin: runtime_types::staging_xcm::v5::location::Location,
                        query_id: ::core::primitive::u64,
                        expected_querier: runtime_types::staging_xcm::v5::location::Location,
                        maybe_actual_querier: ::core::option::Option<
                            runtime_types::staging_xcm::v5::location::Location,
                        >,
                    },
                    #[codec(index = 20)]
                    #[doc = "A remote has requested XCM version change notification from us and we have honored it."]
                    #[doc = "A version information message is sent to them and its cost is included."]
                    VersionNotifyStarted {
                        destination: runtime_types::staging_xcm::v5::location::Location,
                        cost: runtime_types::staging_xcm::v5::asset::Assets,
                        message_id: [::core::primitive::u8; 32usize],
                    },
                    #[codec(index = 21)]
                    #[doc = "We have requested that a remote chain send us XCM version change notifications."]
                    VersionNotifyRequested {
                        destination: runtime_types::staging_xcm::v5::location::Location,
                        cost: runtime_types::staging_xcm::v5::asset::Assets,
                        message_id: [::core::primitive::u8; 32usize],
                    },
                    #[codec(index = 22)]
                    #[doc = "We have requested that a remote chain stops sending us XCM version change"]
                    #[doc = "notifications."]
                    VersionNotifyUnrequested {
                        destination: runtime_types::staging_xcm::v5::location::Location,
                        cost: runtime_types::staging_xcm::v5::asset::Assets,
                        message_id: [::core::primitive::u8; 32usize],
                    },
                    #[codec(index = 23)]
                    #[doc = "Fees were paid from a location for an operation (often for using `SendXcm`)."]
                    FeesPaid {
                        paying: runtime_types::staging_xcm::v5::location::Location,
                        fees: runtime_types::staging_xcm::v5::asset::Assets,
                    },
                    #[codec(index = 24)]
                    #[doc = "Some assets have been claimed from an asset trap"]
                    AssetsClaimed {
                        hash: ::subxt::ext::subxt_core::utils::H256,
                        origin: runtime_types::staging_xcm::v5::location::Location,
                        assets: runtime_types::xcm::VersionedAssets,
                    },
                    #[codec(index = 25)]
                    #[doc = "A XCM version migration finished."]
                    VersionMigrationFinished { version: ::core::primitive::u32 },
                    #[codec(index = 26)]
                    #[doc = "An `aliaser` location was authorized by `target` to alias it, authorization valid until"]
                    #[doc = "`expiry` block number."]
                    AliasAuthorized {
                        aliaser: runtime_types::staging_xcm::v5::location::Location,
                        target: runtime_types::staging_xcm::v5::location::Location,
                        expiry: ::core::option::Option<::core::primitive::u64>,
                    },
                    #[codec(index = 27)]
                    #[doc = "`target` removed alias authorization for `aliaser`."]
                    AliasAuthorizationRemoved {
                        aliaser: runtime_types::staging_xcm::v5::location::Location,
                        target: runtime_types::staging_xcm::v5::location::Location,
                    },
                    #[codec(index = 28)]
                    #[doc = "`target` removed all alias authorizations."]
                    AliasesAuthorizationsRemoved {
                        target: runtime_types::staging_xcm::v5::location::Location,
                    },
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub enum HoldReason {
                    #[codec(index = 0)]
                    AuthorizeAlias,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub enum Origin {
                    #[codec(index = 0)]
                    Xcm(runtime_types::staging_xcm::v5::location::Location),
                    #[codec(index = 1)]
                    Response(runtime_types::staging_xcm::v5::location::Location),
                }
            }
        }
        pub mod people_rococo_runtime {
            use super::runtime_types;
            pub mod people {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct IdentityInfo {
                    pub display: runtime_types::pallet_identity::types::Data,
                    pub legal: runtime_types::pallet_identity::types::Data,
                    pub web: runtime_types::pallet_identity::types::Data,
                    pub matrix: runtime_types::pallet_identity::types::Data,
                    pub email: runtime_types::pallet_identity::types::Data,
                    pub pgp_fingerprint: ::core::option::Option<[::core::primitive::u8; 20usize]>,
                    pub image: runtime_types::pallet_identity::types::Data,
                    pub twitter: runtime_types::pallet_identity::types::Data,
                    pub github: runtime_types::pallet_identity::types::Data,
                    pub discord: runtime_types::pallet_identity::types::Data,
                }
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[codec(dumb_trait_bound)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub enum OriginCaller {
                #[codec(index = 0)]
                system(
                    runtime_types::frame_support::dispatch::RawOrigin<
                        ::subxt::ext::subxt_core::utils::AccountId32,
                    >,
                ),
                #[codec(index = 31)]
                PolkadotXcm(runtime_types::pallet_xcm::pallet::Origin),
                #[codec(index = 32)]
                CumulusXcm(runtime_types::cumulus_pallet_xcm::pallet::Origin),
                #[codec(index = 51)]
                People(runtime_types::pallet_people_multi::pallet::Origin),
                #[codec(index = 53)]
                ProofOfInk(runtime_types::pallet_proof_of_ink::pallet::Origin),
                #[codec(index = 54)]
                PrivacyVoucher(runtime_types::pallet_privacy_voucher::pallet::Origin),
                #[codec(index = 55)]
                Game(runtime_types::pallet_game::pallet::Origin),
                #[codec(index = 56)]
                Score(runtime_types::pallet_score::pallet::Origin),
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[codec(dumb_trait_bound)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub enum RestrictedEntity {
                #[codec(index = 0)]
                PersonalAlias([::core::primitive::u8; 32usize]),
                #[codec(index = 1)]
                PersonalIdentity(::core::primitive::u64),
                #[codec(index = 2)]
                ReferredCandidate(::subxt::ext::subxt_core::utils::AccountId32),
                #[codec(index = 3)]
                AccountParticipant(::subxt::ext::subxt_core::utils::AccountId32),
                #[codec(index = 4)]
                InvitedCandidate(::subxt::ext::subxt_core::utils::AccountId32),
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[codec(dumb_trait_bound)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub enum RuntimeCall {
                #[codec(index = 0)]
                System(runtime_types::frame_system::pallet::Call),
                #[codec(index = 1)]
                ParachainSystem(runtime_types::cumulus_pallet_parachain_system::pallet::Call),
                #[codec(index = 2)]
                Timestamp(runtime_types::pallet_timestamp::pallet::Call),
                #[codec(index = 3)]
                ParachainInfo(runtime_types::staging_parachain_info::pallet::Call),
                #[codec(index = 10)]
                Balances(runtime_types::pallet_balances::pallet::Call),
                #[codec(index = 13)]
                OriginRestriction(runtime_types::pallet_origin_restriction::pallet::Call),
                #[codec(index = 14)]
                Assets(runtime_types::pallet_assets::pallet::Call),
                #[codec(index = 21)]
                CollatorSelection(runtime_types::pallet_collator_selection::pallet::Call),
                #[codec(index = 22)]
                Session(runtime_types::pallet_session::pallet::Call),
                #[codec(index = 30)]
                XcmpQueue(runtime_types::cumulus_pallet_xcmp_queue::pallet::Call),
                #[codec(index = 31)]
                PolkadotXcm(runtime_types::pallet_xcm::pallet::Call),
                #[codec(index = 32)]
                CumulusXcm(runtime_types::cumulus_pallet_xcm::pallet::Call),
                #[codec(index = 34)]
                MessageQueue(runtime_types::pallet_message_queue::pallet::Call),
                #[codec(index = 40)]
                Utility(runtime_types::pallet_utility::pallet::Call),
                #[codec(index = 41)]
                Multisig(runtime_types::pallet_multisig::pallet::Call),
                #[codec(index = 42)]
                Sudo(runtime_types::pallet_sudo::pallet::Call),
                #[codec(index = 50)]
                Identity(runtime_types::pallet_identity::pallet::Call),
                #[codec(index = 51)]
                People(runtime_types::pallet_people_multi::pallet::Call),
                #[codec(index = 52)]
                MobRule(runtime_types::pallet_mob_rule::pallet::Call),
                #[codec(index = 53)]
                ProofOfInk(runtime_types::pallet_proof_of_ink::pallet::Call),
                #[codec(index = 54)]
                PrivacyVoucher(runtime_types::pallet_privacy_voucher::pallet::Call),
                #[codec(index = 55)]
                Game(runtime_types::pallet_game::pallet::Call),
                #[codec(index = 56)]
                Score(runtime_types::pallet_score::pallet::Call),
                #[codec(index = 59)]
                DummyDim(runtime_types::pallet_dummy_dim::pallet::Call),
                #[codec(index = 61)]
                Lottery(runtime_types::pallet_lottery::pallet::Call),
                #[codec(index = 98)]
                MultiBlockMigrations(runtime_types::pallet_migrations::pallet::Call),
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[codec(dumb_trait_bound)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub enum RuntimeError {
                #[codec(index = 0)]
                System(runtime_types::frame_system::pallet::Error),
                #[codec(index = 1)]
                ParachainSystem(runtime_types::cumulus_pallet_parachain_system::pallet::Error),
                #[codec(index = 10)]
                Balances(runtime_types::pallet_balances::pallet::Error),
                #[codec(index = 13)]
                OriginRestriction(runtime_types::pallet_origin_restriction::pallet::Error),
                #[codec(index = 14)]
                Assets(runtime_types::pallet_assets::pallet::Error),
                #[codec(index = 15)]
                AssetsHolder(runtime_types::pallet_assets_holder::pallet::Error),
                #[codec(index = 21)]
                CollatorSelection(runtime_types::pallet_collator_selection::pallet::Error),
                #[codec(index = 22)]
                Session(runtime_types::pallet_session::pallet::Error),
                #[codec(index = 30)]
                XcmpQueue(runtime_types::cumulus_pallet_xcmp_queue::pallet::Error),
                #[codec(index = 31)]
                PolkadotXcm(runtime_types::pallet_xcm::pallet::Error),
                #[codec(index = 34)]
                MessageQueue(runtime_types::pallet_message_queue::pallet::Error),
                #[codec(index = 40)]
                Utility(runtime_types::pallet_utility::pallet::Error),
                #[codec(index = 41)]
                Multisig(runtime_types::pallet_multisig::pallet::Error),
                #[codec(index = 42)]
                Sudo(runtime_types::pallet_sudo::pallet::Error),
                #[codec(index = 50)]
                Identity(runtime_types::pallet_identity::pallet::Error),
                #[codec(index = 51)]
                People(runtime_types::pallet_people_multi::pallet::Error),
                #[codec(index = 52)]
                MobRule(runtime_types::pallet_mob_rule::pallet::Error),
                #[codec(index = 53)]
                ProofOfInk(runtime_types::pallet_proof_of_ink::pallet::Error),
                #[codec(index = 54)]
                PrivacyVoucher(runtime_types::pallet_privacy_voucher::pallet::Error),
                #[codec(index = 55)]
                Game(runtime_types::pallet_game::pallet::Error),
                #[codec(index = 56)]
                Score(runtime_types::pallet_score::pallet::Error),
                #[codec(index = 59)]
                DummyDim(runtime_types::pallet_dummy_dim::pallet::Error),
                #[codec(index = 61)]
                Lottery(runtime_types::pallet_lottery::pallet::Error),
                #[codec(index = 98)]
                MultiBlockMigrations(runtime_types::pallet_migrations::pallet::Error),
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[codec(dumb_trait_bound)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub enum RuntimeEvent {
                #[codec(index = 0)]
                System(runtime_types::frame_system::pallet::Event),
                #[codec(index = 1)]
                ParachainSystem(runtime_types::cumulus_pallet_parachain_system::pallet::Event),
                #[codec(index = 10)]
                Balances(runtime_types::pallet_balances::pallet::Event),
                #[codec(index = 11)]
                TransactionPayment(runtime_types::pallet_transaction_payment::pallet::Event),
                #[codec(index = 12)]
                SkipFeelessPayment(runtime_types::pallet_skip_feeless_payment::pallet::Event),
                #[codec(index = 13)]
                OriginRestriction(runtime_types::pallet_origin_restriction::pallet::Event),
                #[codec(index = 14)]
                Assets(runtime_types::pallet_assets::pallet::Event),
                #[codec(index = 15)]
                AssetsHolder(runtime_types::pallet_assets_holder::pallet::Event),
                #[codec(index = 21)]
                CollatorSelection(runtime_types::pallet_collator_selection::pallet::Event),
                #[codec(index = 22)]
                Session(runtime_types::pallet_session::pallet::Event),
                #[codec(index = 30)]
                XcmpQueue(runtime_types::cumulus_pallet_xcmp_queue::pallet::Event),
                #[codec(index = 31)]
                PolkadotXcm(runtime_types::pallet_xcm::pallet::Event),
                #[codec(index = 32)]
                CumulusXcm(runtime_types::cumulus_pallet_xcm::pallet::Event),
                #[codec(index = 34)]
                MessageQueue(runtime_types::pallet_message_queue::pallet::Event),
                #[codec(index = 40)]
                Utility(runtime_types::pallet_utility::pallet::Event),
                #[codec(index = 41)]
                Multisig(runtime_types::pallet_multisig::pallet::Event),
                #[codec(index = 42)]
                Sudo(runtime_types::pallet_sudo::pallet::Event),
                #[codec(index = 50)]
                Identity(runtime_types::pallet_identity::pallet::Event),
                #[codec(index = 51)]
                People(runtime_types::pallet_people_multi::pallet::Event),
                #[codec(index = 52)]
                MobRule(runtime_types::pallet_mob_rule::pallet::Event),
                #[codec(index = 53)]
                ProofOfInk(runtime_types::pallet_proof_of_ink::pallet::Event),
                #[codec(index = 54)]
                PrivacyVoucher(runtime_types::pallet_privacy_voucher::pallet::Event),
                #[codec(index = 55)]
                Game(runtime_types::pallet_game::pallet::Event),
                #[codec(index = 56)]
                Score(runtime_types::pallet_score::pallet::Event),
                #[codec(index = 59)]
                DummyDim(runtime_types::pallet_dummy_dim::pallet::Event),
                #[codec(index = 61)]
                Lottery(runtime_types::pallet_lottery::pallet::Event),
                #[codec(index = 98)]
                MultiBlockMigrations(runtime_types::pallet_migrations::pallet::Event),
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[codec(dumb_trait_bound)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub enum RuntimeHoldReason {
                #[codec(index = 31)]
                PolkadotXcm(runtime_types::pallet_xcm::pallet::HoldReason),
                #[codec(index = 52)]
                MobRule(runtime_types::pallet_mob_rule::pallet::HoldReason),
                #[codec(index = 53)]
                ProofOfInk(runtime_types::pallet_proof_of_ink::pallet::HoldReason),
                #[codec(index = 55)]
                Game(runtime_types::pallet_game::pallet::HoldReason),
                #[codec(index = 56)]
                Score(runtime_types::pallet_score::pallet::HoldReason),
                #[codec(index = 61)]
                Lottery(runtime_types::pallet_lottery::pallet::HoldReason),
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[codec(dumb_trait_bound)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct SessionKeys {
                pub aura: runtime_types::sp_consensus_aura::sr25519::app_sr25519::Public,
            }
        }
        pub mod polkadot_core_primitives {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[codec(dumb_trait_bound)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct InboundDownwardMessage<_0> {
                pub sent_at: _0,
                pub msg: ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[codec(dumb_trait_bound)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct InboundHrmpMessage<_0> {
                pub sent_at: _0,
                pub data: ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
            }
        }
        pub mod polkadot_parachain_primitives {
            use super::runtime_types;
            pub mod primitives {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct HeadData(
                    pub ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                );
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: CompactAs,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct Id(pub ::core::primitive::u32);
            }
        }
        pub mod polkadot_primitives {
            use super::runtime_types;
            pub mod v8 {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct PersistedValidationData<_0, _1> {
                    pub parent_head:
                        runtime_types::polkadot_parachain_primitives::primitives::HeadData,
                    pub relay_parent_number: _1,
                    pub relay_parent_storage_root: _0,
                    pub max_pov_size: ::core::primitive::u32,
                }
            }
        }
        pub mod sp_arithmetic {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[codec(dumb_trait_bound)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub enum ArithmeticError {
                #[codec(index = 0)]
                Underflow,
                #[codec(index = 1)]
                Overflow,
                #[codec(index = 2)]
                DivisionByZero,
            }
        }
        pub mod sp_consensus_aura {
            use super::runtime_types;
            pub mod sr25519 {
                use super::runtime_types;
                pub mod app_sr25519 {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                        :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                    #[codec(dumb_trait_bound)]
                    #[decode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                    )]
                    #[encode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                    )]
                    pub struct Public(pub [::core::primitive::u8; 32usize]);
                }
            }
        }
        pub mod sp_runtime {
            use super::runtime_types;
            pub mod generic {
                use super::runtime_types;
                pub mod block {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                        :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                    #[codec(dumb_trait_bound)]
                    #[decode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                    )]
                    #[encode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                    )]
                    pub struct Block<_0, _1> {
                        pub header: _0,
                        pub extrinsics: ::subxt::ext::subxt_core::alloc::vec::Vec<_1>,
                    }
                }
                pub mod digest {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                        :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                    #[codec(dumb_trait_bound)]
                    #[decode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                    )]
                    #[encode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                    )]
                    pub struct Digest {
                        pub logs: ::subxt::ext::subxt_core::alloc::vec::Vec<
                            runtime_types::sp_runtime::generic::digest::DigestItem,
                        >,
                    }
                    #[derive(
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                        :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                    #[codec(dumb_trait_bound)]
                    #[decode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                    )]
                    #[encode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                    )]
                    pub enum DigestItem {
                        #[codec(index = 6)]
                        PreRuntime(
                            [::core::primitive::u8; 4usize],
                            ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                        ),
                        #[codec(index = 4)]
                        Consensus(
                            [::core::primitive::u8; 4usize],
                            ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                        ),
                        #[codec(index = 5)]
                        Seal(
                            [::core::primitive::u8; 4usize],
                            ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                        ),
                        #[codec(index = 0)]
                        Other(::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>),
                        #[codec(index = 8)]
                        RuntimeEnvironmentUpdated,
                    }
                }
                pub mod era {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                        :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                    #[codec(dumb_trait_bound)]
                    #[decode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                    )]
                    #[encode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                    )]
                    pub enum Era {
                        #[codec(index = 0)]
                        Immortal,
                        #[codec(index = 1)]
                        Mortal1(::core::primitive::u8),
                        #[codec(index = 2)]
                        Mortal2(::core::primitive::u8),
                        #[codec(index = 3)]
                        Mortal3(::core::primitive::u8),
                        #[codec(index = 4)]
                        Mortal4(::core::primitive::u8),
                        #[codec(index = 5)]
                        Mortal5(::core::primitive::u8),
                        #[codec(index = 6)]
                        Mortal6(::core::primitive::u8),
                        #[codec(index = 7)]
                        Mortal7(::core::primitive::u8),
                        #[codec(index = 8)]
                        Mortal8(::core::primitive::u8),
                        #[codec(index = 9)]
                        Mortal9(::core::primitive::u8),
                        #[codec(index = 10)]
                        Mortal10(::core::primitive::u8),
                        #[codec(index = 11)]
                        Mortal11(::core::primitive::u8),
                        #[codec(index = 12)]
                        Mortal12(::core::primitive::u8),
                        #[codec(index = 13)]
                        Mortal13(::core::primitive::u8),
                        #[codec(index = 14)]
                        Mortal14(::core::primitive::u8),
                        #[codec(index = 15)]
                        Mortal15(::core::primitive::u8),
                        #[codec(index = 16)]
                        Mortal16(::core::primitive::u8),
                        #[codec(index = 17)]
                        Mortal17(::core::primitive::u8),
                        #[codec(index = 18)]
                        Mortal18(::core::primitive::u8),
                        #[codec(index = 19)]
                        Mortal19(::core::primitive::u8),
                        #[codec(index = 20)]
                        Mortal20(::core::primitive::u8),
                        #[codec(index = 21)]
                        Mortal21(::core::primitive::u8),
                        #[codec(index = 22)]
                        Mortal22(::core::primitive::u8),
                        #[codec(index = 23)]
                        Mortal23(::core::primitive::u8),
                        #[codec(index = 24)]
                        Mortal24(::core::primitive::u8),
                        #[codec(index = 25)]
                        Mortal25(::core::primitive::u8),
                        #[codec(index = 26)]
                        Mortal26(::core::primitive::u8),
                        #[codec(index = 27)]
                        Mortal27(::core::primitive::u8),
                        #[codec(index = 28)]
                        Mortal28(::core::primitive::u8),
                        #[codec(index = 29)]
                        Mortal29(::core::primitive::u8),
                        #[codec(index = 30)]
                        Mortal30(::core::primitive::u8),
                        #[codec(index = 31)]
                        Mortal31(::core::primitive::u8),
                        #[codec(index = 32)]
                        Mortal32(::core::primitive::u8),
                        #[codec(index = 33)]
                        Mortal33(::core::primitive::u8),
                        #[codec(index = 34)]
                        Mortal34(::core::primitive::u8),
                        #[codec(index = 35)]
                        Mortal35(::core::primitive::u8),
                        #[codec(index = 36)]
                        Mortal36(::core::primitive::u8),
                        #[codec(index = 37)]
                        Mortal37(::core::primitive::u8),
                        #[codec(index = 38)]
                        Mortal38(::core::primitive::u8),
                        #[codec(index = 39)]
                        Mortal39(::core::primitive::u8),
                        #[codec(index = 40)]
                        Mortal40(::core::primitive::u8),
                        #[codec(index = 41)]
                        Mortal41(::core::primitive::u8),
                        #[codec(index = 42)]
                        Mortal42(::core::primitive::u8),
                        #[codec(index = 43)]
                        Mortal43(::core::primitive::u8),
                        #[codec(index = 44)]
                        Mortal44(::core::primitive::u8),
                        #[codec(index = 45)]
                        Mortal45(::core::primitive::u8),
                        #[codec(index = 46)]
                        Mortal46(::core::primitive::u8),
                        #[codec(index = 47)]
                        Mortal47(::core::primitive::u8),
                        #[codec(index = 48)]
                        Mortal48(::core::primitive::u8),
                        #[codec(index = 49)]
                        Mortal49(::core::primitive::u8),
                        #[codec(index = 50)]
                        Mortal50(::core::primitive::u8),
                        #[codec(index = 51)]
                        Mortal51(::core::primitive::u8),
                        #[codec(index = 52)]
                        Mortal52(::core::primitive::u8),
                        #[codec(index = 53)]
                        Mortal53(::core::primitive::u8),
                        #[codec(index = 54)]
                        Mortal54(::core::primitive::u8),
                        #[codec(index = 55)]
                        Mortal55(::core::primitive::u8),
                        #[codec(index = 56)]
                        Mortal56(::core::primitive::u8),
                        #[codec(index = 57)]
                        Mortal57(::core::primitive::u8),
                        #[codec(index = 58)]
                        Mortal58(::core::primitive::u8),
                        #[codec(index = 59)]
                        Mortal59(::core::primitive::u8),
                        #[codec(index = 60)]
                        Mortal60(::core::primitive::u8),
                        #[codec(index = 61)]
                        Mortal61(::core::primitive::u8),
                        #[codec(index = 62)]
                        Mortal62(::core::primitive::u8),
                        #[codec(index = 63)]
                        Mortal63(::core::primitive::u8),
                        #[codec(index = 64)]
                        Mortal64(::core::primitive::u8),
                        #[codec(index = 65)]
                        Mortal65(::core::primitive::u8),
                        #[codec(index = 66)]
                        Mortal66(::core::primitive::u8),
                        #[codec(index = 67)]
                        Mortal67(::core::primitive::u8),
                        #[codec(index = 68)]
                        Mortal68(::core::primitive::u8),
                        #[codec(index = 69)]
                        Mortal69(::core::primitive::u8),
                        #[codec(index = 70)]
                        Mortal70(::core::primitive::u8),
                        #[codec(index = 71)]
                        Mortal71(::core::primitive::u8),
                        #[codec(index = 72)]
                        Mortal72(::core::primitive::u8),
                        #[codec(index = 73)]
                        Mortal73(::core::primitive::u8),
                        #[codec(index = 74)]
                        Mortal74(::core::primitive::u8),
                        #[codec(index = 75)]
                        Mortal75(::core::primitive::u8),
                        #[codec(index = 76)]
                        Mortal76(::core::primitive::u8),
                        #[codec(index = 77)]
                        Mortal77(::core::primitive::u8),
                        #[codec(index = 78)]
                        Mortal78(::core::primitive::u8),
                        #[codec(index = 79)]
                        Mortal79(::core::primitive::u8),
                        #[codec(index = 80)]
                        Mortal80(::core::primitive::u8),
                        #[codec(index = 81)]
                        Mortal81(::core::primitive::u8),
                        #[codec(index = 82)]
                        Mortal82(::core::primitive::u8),
                        #[codec(index = 83)]
                        Mortal83(::core::primitive::u8),
                        #[codec(index = 84)]
                        Mortal84(::core::primitive::u8),
                        #[codec(index = 85)]
                        Mortal85(::core::primitive::u8),
                        #[codec(index = 86)]
                        Mortal86(::core::primitive::u8),
                        #[codec(index = 87)]
                        Mortal87(::core::primitive::u8),
                        #[codec(index = 88)]
                        Mortal88(::core::primitive::u8),
                        #[codec(index = 89)]
                        Mortal89(::core::primitive::u8),
                        #[codec(index = 90)]
                        Mortal90(::core::primitive::u8),
                        #[codec(index = 91)]
                        Mortal91(::core::primitive::u8),
                        #[codec(index = 92)]
                        Mortal92(::core::primitive::u8),
                        #[codec(index = 93)]
                        Mortal93(::core::primitive::u8),
                        #[codec(index = 94)]
                        Mortal94(::core::primitive::u8),
                        #[codec(index = 95)]
                        Mortal95(::core::primitive::u8),
                        #[codec(index = 96)]
                        Mortal96(::core::primitive::u8),
                        #[codec(index = 97)]
                        Mortal97(::core::primitive::u8),
                        #[codec(index = 98)]
                        Mortal98(::core::primitive::u8),
                        #[codec(index = 99)]
                        Mortal99(::core::primitive::u8),
                        #[codec(index = 100)]
                        Mortal100(::core::primitive::u8),
                        #[codec(index = 101)]
                        Mortal101(::core::primitive::u8),
                        #[codec(index = 102)]
                        Mortal102(::core::primitive::u8),
                        #[codec(index = 103)]
                        Mortal103(::core::primitive::u8),
                        #[codec(index = 104)]
                        Mortal104(::core::primitive::u8),
                        #[codec(index = 105)]
                        Mortal105(::core::primitive::u8),
                        #[codec(index = 106)]
                        Mortal106(::core::primitive::u8),
                        #[codec(index = 107)]
                        Mortal107(::core::primitive::u8),
                        #[codec(index = 108)]
                        Mortal108(::core::primitive::u8),
                        #[codec(index = 109)]
                        Mortal109(::core::primitive::u8),
                        #[codec(index = 110)]
                        Mortal110(::core::primitive::u8),
                        #[codec(index = 111)]
                        Mortal111(::core::primitive::u8),
                        #[codec(index = 112)]
                        Mortal112(::core::primitive::u8),
                        #[codec(index = 113)]
                        Mortal113(::core::primitive::u8),
                        #[codec(index = 114)]
                        Mortal114(::core::primitive::u8),
                        #[codec(index = 115)]
                        Mortal115(::core::primitive::u8),
                        #[codec(index = 116)]
                        Mortal116(::core::primitive::u8),
                        #[codec(index = 117)]
                        Mortal117(::core::primitive::u8),
                        #[codec(index = 118)]
                        Mortal118(::core::primitive::u8),
                        #[codec(index = 119)]
                        Mortal119(::core::primitive::u8),
                        #[codec(index = 120)]
                        Mortal120(::core::primitive::u8),
                        #[codec(index = 121)]
                        Mortal121(::core::primitive::u8),
                        #[codec(index = 122)]
                        Mortal122(::core::primitive::u8),
                        #[codec(index = 123)]
                        Mortal123(::core::primitive::u8),
                        #[codec(index = 124)]
                        Mortal124(::core::primitive::u8),
                        #[codec(index = 125)]
                        Mortal125(::core::primitive::u8),
                        #[codec(index = 126)]
                        Mortal126(::core::primitive::u8),
                        #[codec(index = 127)]
                        Mortal127(::core::primitive::u8),
                        #[codec(index = 128)]
                        Mortal128(::core::primitive::u8),
                        #[codec(index = 129)]
                        Mortal129(::core::primitive::u8),
                        #[codec(index = 130)]
                        Mortal130(::core::primitive::u8),
                        #[codec(index = 131)]
                        Mortal131(::core::primitive::u8),
                        #[codec(index = 132)]
                        Mortal132(::core::primitive::u8),
                        #[codec(index = 133)]
                        Mortal133(::core::primitive::u8),
                        #[codec(index = 134)]
                        Mortal134(::core::primitive::u8),
                        #[codec(index = 135)]
                        Mortal135(::core::primitive::u8),
                        #[codec(index = 136)]
                        Mortal136(::core::primitive::u8),
                        #[codec(index = 137)]
                        Mortal137(::core::primitive::u8),
                        #[codec(index = 138)]
                        Mortal138(::core::primitive::u8),
                        #[codec(index = 139)]
                        Mortal139(::core::primitive::u8),
                        #[codec(index = 140)]
                        Mortal140(::core::primitive::u8),
                        #[codec(index = 141)]
                        Mortal141(::core::primitive::u8),
                        #[codec(index = 142)]
                        Mortal142(::core::primitive::u8),
                        #[codec(index = 143)]
                        Mortal143(::core::primitive::u8),
                        #[codec(index = 144)]
                        Mortal144(::core::primitive::u8),
                        #[codec(index = 145)]
                        Mortal145(::core::primitive::u8),
                        #[codec(index = 146)]
                        Mortal146(::core::primitive::u8),
                        #[codec(index = 147)]
                        Mortal147(::core::primitive::u8),
                        #[codec(index = 148)]
                        Mortal148(::core::primitive::u8),
                        #[codec(index = 149)]
                        Mortal149(::core::primitive::u8),
                        #[codec(index = 150)]
                        Mortal150(::core::primitive::u8),
                        #[codec(index = 151)]
                        Mortal151(::core::primitive::u8),
                        #[codec(index = 152)]
                        Mortal152(::core::primitive::u8),
                        #[codec(index = 153)]
                        Mortal153(::core::primitive::u8),
                        #[codec(index = 154)]
                        Mortal154(::core::primitive::u8),
                        #[codec(index = 155)]
                        Mortal155(::core::primitive::u8),
                        #[codec(index = 156)]
                        Mortal156(::core::primitive::u8),
                        #[codec(index = 157)]
                        Mortal157(::core::primitive::u8),
                        #[codec(index = 158)]
                        Mortal158(::core::primitive::u8),
                        #[codec(index = 159)]
                        Mortal159(::core::primitive::u8),
                        #[codec(index = 160)]
                        Mortal160(::core::primitive::u8),
                        #[codec(index = 161)]
                        Mortal161(::core::primitive::u8),
                        #[codec(index = 162)]
                        Mortal162(::core::primitive::u8),
                        #[codec(index = 163)]
                        Mortal163(::core::primitive::u8),
                        #[codec(index = 164)]
                        Mortal164(::core::primitive::u8),
                        #[codec(index = 165)]
                        Mortal165(::core::primitive::u8),
                        #[codec(index = 166)]
                        Mortal166(::core::primitive::u8),
                        #[codec(index = 167)]
                        Mortal167(::core::primitive::u8),
                        #[codec(index = 168)]
                        Mortal168(::core::primitive::u8),
                        #[codec(index = 169)]
                        Mortal169(::core::primitive::u8),
                        #[codec(index = 170)]
                        Mortal170(::core::primitive::u8),
                        #[codec(index = 171)]
                        Mortal171(::core::primitive::u8),
                        #[codec(index = 172)]
                        Mortal172(::core::primitive::u8),
                        #[codec(index = 173)]
                        Mortal173(::core::primitive::u8),
                        #[codec(index = 174)]
                        Mortal174(::core::primitive::u8),
                        #[codec(index = 175)]
                        Mortal175(::core::primitive::u8),
                        #[codec(index = 176)]
                        Mortal176(::core::primitive::u8),
                        #[codec(index = 177)]
                        Mortal177(::core::primitive::u8),
                        #[codec(index = 178)]
                        Mortal178(::core::primitive::u8),
                        #[codec(index = 179)]
                        Mortal179(::core::primitive::u8),
                        #[codec(index = 180)]
                        Mortal180(::core::primitive::u8),
                        #[codec(index = 181)]
                        Mortal181(::core::primitive::u8),
                        #[codec(index = 182)]
                        Mortal182(::core::primitive::u8),
                        #[codec(index = 183)]
                        Mortal183(::core::primitive::u8),
                        #[codec(index = 184)]
                        Mortal184(::core::primitive::u8),
                        #[codec(index = 185)]
                        Mortal185(::core::primitive::u8),
                        #[codec(index = 186)]
                        Mortal186(::core::primitive::u8),
                        #[codec(index = 187)]
                        Mortal187(::core::primitive::u8),
                        #[codec(index = 188)]
                        Mortal188(::core::primitive::u8),
                        #[codec(index = 189)]
                        Mortal189(::core::primitive::u8),
                        #[codec(index = 190)]
                        Mortal190(::core::primitive::u8),
                        #[codec(index = 191)]
                        Mortal191(::core::primitive::u8),
                        #[codec(index = 192)]
                        Mortal192(::core::primitive::u8),
                        #[codec(index = 193)]
                        Mortal193(::core::primitive::u8),
                        #[codec(index = 194)]
                        Mortal194(::core::primitive::u8),
                        #[codec(index = 195)]
                        Mortal195(::core::primitive::u8),
                        #[codec(index = 196)]
                        Mortal196(::core::primitive::u8),
                        #[codec(index = 197)]
                        Mortal197(::core::primitive::u8),
                        #[codec(index = 198)]
                        Mortal198(::core::primitive::u8),
                        #[codec(index = 199)]
                        Mortal199(::core::primitive::u8),
                        #[codec(index = 200)]
                        Mortal200(::core::primitive::u8),
                        #[codec(index = 201)]
                        Mortal201(::core::primitive::u8),
                        #[codec(index = 202)]
                        Mortal202(::core::primitive::u8),
                        #[codec(index = 203)]
                        Mortal203(::core::primitive::u8),
                        #[codec(index = 204)]
                        Mortal204(::core::primitive::u8),
                        #[codec(index = 205)]
                        Mortal205(::core::primitive::u8),
                        #[codec(index = 206)]
                        Mortal206(::core::primitive::u8),
                        #[codec(index = 207)]
                        Mortal207(::core::primitive::u8),
                        #[codec(index = 208)]
                        Mortal208(::core::primitive::u8),
                        #[codec(index = 209)]
                        Mortal209(::core::primitive::u8),
                        #[codec(index = 210)]
                        Mortal210(::core::primitive::u8),
                        #[codec(index = 211)]
                        Mortal211(::core::primitive::u8),
                        #[codec(index = 212)]
                        Mortal212(::core::primitive::u8),
                        #[codec(index = 213)]
                        Mortal213(::core::primitive::u8),
                        #[codec(index = 214)]
                        Mortal214(::core::primitive::u8),
                        #[codec(index = 215)]
                        Mortal215(::core::primitive::u8),
                        #[codec(index = 216)]
                        Mortal216(::core::primitive::u8),
                        #[codec(index = 217)]
                        Mortal217(::core::primitive::u8),
                        #[codec(index = 218)]
                        Mortal218(::core::primitive::u8),
                        #[codec(index = 219)]
                        Mortal219(::core::primitive::u8),
                        #[codec(index = 220)]
                        Mortal220(::core::primitive::u8),
                        #[codec(index = 221)]
                        Mortal221(::core::primitive::u8),
                        #[codec(index = 222)]
                        Mortal222(::core::primitive::u8),
                        #[codec(index = 223)]
                        Mortal223(::core::primitive::u8),
                        #[codec(index = 224)]
                        Mortal224(::core::primitive::u8),
                        #[codec(index = 225)]
                        Mortal225(::core::primitive::u8),
                        #[codec(index = 226)]
                        Mortal226(::core::primitive::u8),
                        #[codec(index = 227)]
                        Mortal227(::core::primitive::u8),
                        #[codec(index = 228)]
                        Mortal228(::core::primitive::u8),
                        #[codec(index = 229)]
                        Mortal229(::core::primitive::u8),
                        #[codec(index = 230)]
                        Mortal230(::core::primitive::u8),
                        #[codec(index = 231)]
                        Mortal231(::core::primitive::u8),
                        #[codec(index = 232)]
                        Mortal232(::core::primitive::u8),
                        #[codec(index = 233)]
                        Mortal233(::core::primitive::u8),
                        #[codec(index = 234)]
                        Mortal234(::core::primitive::u8),
                        #[codec(index = 235)]
                        Mortal235(::core::primitive::u8),
                        #[codec(index = 236)]
                        Mortal236(::core::primitive::u8),
                        #[codec(index = 237)]
                        Mortal237(::core::primitive::u8),
                        #[codec(index = 238)]
                        Mortal238(::core::primitive::u8),
                        #[codec(index = 239)]
                        Mortal239(::core::primitive::u8),
                        #[codec(index = 240)]
                        Mortal240(::core::primitive::u8),
                        #[codec(index = 241)]
                        Mortal241(::core::primitive::u8),
                        #[codec(index = 242)]
                        Mortal242(::core::primitive::u8),
                        #[codec(index = 243)]
                        Mortal243(::core::primitive::u8),
                        #[codec(index = 244)]
                        Mortal244(::core::primitive::u8),
                        #[codec(index = 245)]
                        Mortal245(::core::primitive::u8),
                        #[codec(index = 246)]
                        Mortal246(::core::primitive::u8),
                        #[codec(index = 247)]
                        Mortal247(::core::primitive::u8),
                        #[codec(index = 248)]
                        Mortal248(::core::primitive::u8),
                        #[codec(index = 249)]
                        Mortal249(::core::primitive::u8),
                        #[codec(index = 250)]
                        Mortal250(::core::primitive::u8),
                        #[codec(index = 251)]
                        Mortal251(::core::primitive::u8),
                        #[codec(index = 252)]
                        Mortal252(::core::primitive::u8),
                        #[codec(index = 253)]
                        Mortal253(::core::primitive::u8),
                        #[codec(index = 254)]
                        Mortal254(::core::primitive::u8),
                        #[codec(index = 255)]
                        Mortal255(::core::primitive::u8),
                    }
                }
                pub mod header {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                        :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                    #[codec(dumb_trait_bound)]
                    #[decode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                    )]
                    #[encode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                    )]
                    pub struct Header<_0> {
                        pub parent_hash: ::subxt::ext::subxt_core::utils::H256,
                        #[codec(compact)]
                        pub number: _0,
                        pub state_root: ::subxt::ext::subxt_core::utils::H256,
                        pub extrinsics_root: ::subxt::ext::subxt_core::utils::H256,
                        pub digest: runtime_types::sp_runtime::generic::digest::Digest,
                    }
                }
            }
            pub mod proving_trie {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub enum TrieError {
                    #[codec(index = 0)]
                    InvalidStateRoot,
                    #[codec(index = 1)]
                    IncompleteDatabase,
                    #[codec(index = 2)]
                    ValueAtIncompleteKey,
                    #[codec(index = 3)]
                    DecoderError,
                    #[codec(index = 4)]
                    InvalidHash,
                    #[codec(index = 5)]
                    DuplicateKey,
                    #[codec(index = 6)]
                    ExtraneousNode,
                    #[codec(index = 7)]
                    ExtraneousValue,
                    #[codec(index = 8)]
                    ExtraneousHashReference,
                    #[codec(index = 9)]
                    InvalidChildReference,
                    #[codec(index = 10)]
                    ValueMismatch,
                    #[codec(index = 11)]
                    IncompleteProof,
                    #[codec(index = 12)]
                    RootMismatch,
                    #[codec(index = 13)]
                    DecodeError,
                }
            }
            pub mod traits {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct BlakeTwo256;
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[codec(dumb_trait_bound)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub enum DispatchError {
                #[codec(index = 0)]
                Other,
                #[codec(index = 1)]
                CannotLookup,
                #[codec(index = 2)]
                BadOrigin,
                #[codec(index = 3)]
                Module(runtime_types::sp_runtime::ModuleError),
                #[codec(index = 4)]
                ConsumerRemaining,
                #[codec(index = 5)]
                NoProviders,
                #[codec(index = 6)]
                TooManyConsumers,
                #[codec(index = 7)]
                Token(runtime_types::sp_runtime::TokenError),
                #[codec(index = 8)]
                Arithmetic(runtime_types::sp_arithmetic::ArithmeticError),
                #[codec(index = 9)]
                Transactional(runtime_types::sp_runtime::TransactionalError),
                #[codec(index = 10)]
                Exhausted,
                #[codec(index = 11)]
                Corruption,
                #[codec(index = 12)]
                Unavailable,
                #[codec(index = 13)]
                RootNotAllowed,
                #[codec(index = 14)]
                Trie(runtime_types::sp_runtime::proving_trie::TrieError),
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[codec(dumb_trait_bound)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct ModuleError {
                pub index: ::core::primitive::u8,
                pub error: [::core::primitive::u8; 4usize],
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[codec(dumb_trait_bound)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub enum MultiSignature {
                #[codec(index = 0)]
                Ed25519([::core::primitive::u8; 64usize]),
                #[codec(index = 1)]
                Sr25519([::core::primitive::u8; 64usize]),
                #[codec(index = 2)]
                Ecdsa([::core::primitive::u8; 65usize]),
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[codec(dumb_trait_bound)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub enum TokenError {
                #[codec(index = 0)]
                FundsUnavailable,
                #[codec(index = 1)]
                OnlyProvider,
                #[codec(index = 2)]
                BelowMinimum,
                #[codec(index = 3)]
                CannotCreate,
                #[codec(index = 4)]
                UnknownAsset,
                #[codec(index = 5)]
                Frozen,
                #[codec(index = 6)]
                Unsupported,
                #[codec(index = 7)]
                CannotCreateHold,
                #[codec(index = 8)]
                NotExpendable,
                #[codec(index = 9)]
                Blocked,
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[codec(dumb_trait_bound)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub enum TransactionalError {
                #[codec(index = 0)]
                LimitReached,
                #[codec(index = 1)]
                NoLayer,
            }
        }
        pub mod sp_trie {
            use super::runtime_types;
            pub mod storage_proof {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct StorageProof {
                    pub trie_nodes: ::subxt::ext::subxt_core::alloc::vec::Vec<
                        ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                    >,
                }
            }
        }
        pub mod sp_weights {
            use super::runtime_types;
            pub mod weight_v2 {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct Weight {
                    #[codec(compact)]
                    pub ref_time: ::core::primitive::u64,
                    #[codec(compact)]
                    pub proof_size: ::core::primitive::u64,
                }
            }
        }
        pub mod staging_parachain_info {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
                pub enum Call {}
            }
        }
        pub mod staging_xcm {
            use super::runtime_types;
            pub mod v3 {
                use super::runtime_types;
                pub mod multilocation {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                        :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                    #[codec(dumb_trait_bound)]
                    #[decode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                    )]
                    #[encode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                    )]
                    pub struct MultiLocation {
                        pub parents: ::core::primitive::u8,
                        pub interior: runtime_types::xcm::v3::junctions::Junctions,
                    }
                }
            }
            pub mod v4 {
                use super::runtime_types;
                pub mod asset {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                        :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                    #[codec(dumb_trait_bound)]
                    #[decode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                    )]
                    #[encode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                    )]
                    pub struct Asset {
                        pub id: runtime_types::staging_xcm::v4::asset::AssetId,
                        pub fun: runtime_types::staging_xcm::v4::asset::Fungibility,
                    }
                    #[derive(
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                        :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                    #[codec(dumb_trait_bound)]
                    #[decode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                    )]
                    #[encode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                    )]
                    pub enum AssetFilter {
                        #[codec(index = 0)]
                        Definite(runtime_types::staging_xcm::v4::asset::Assets),
                        #[codec(index = 1)]
                        Wild(runtime_types::staging_xcm::v4::asset::WildAsset),
                    }
                    #[derive(
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                        :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                    #[codec(dumb_trait_bound)]
                    #[decode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                    )]
                    #[encode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                    )]
                    pub struct AssetId(pub runtime_types::staging_xcm::v4::location::Location);
                    #[derive(
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                        :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                    #[codec(dumb_trait_bound)]
                    #[decode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                    )]
                    #[encode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                    )]
                    pub enum AssetInstance {
                        #[codec(index = 0)]
                        Undefined,
                        #[codec(index = 1)]
                        Index(#[codec(compact)] ::core::primitive::u128),
                        #[codec(index = 2)]
                        Array4([::core::primitive::u8; 4usize]),
                        #[codec(index = 3)]
                        Array8([::core::primitive::u8; 8usize]),
                        #[codec(index = 4)]
                        Array16([::core::primitive::u8; 16usize]),
                        #[codec(index = 5)]
                        Array32([::core::primitive::u8; 32usize]),
                    }
                    #[derive(
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                        :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                    #[codec(dumb_trait_bound)]
                    #[decode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                    )]
                    #[encode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                    )]
                    pub struct Assets(
                        pub  ::subxt::ext::subxt_core::alloc::vec::Vec<
                            runtime_types::staging_xcm::v4::asset::Asset,
                        >,
                    );
                    #[derive(
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                        :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                    #[codec(dumb_trait_bound)]
                    #[decode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                    )]
                    #[encode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                    )]
                    pub enum Fungibility {
                        #[codec(index = 0)]
                        Fungible(#[codec(compact)] ::core::primitive::u128),
                        #[codec(index = 1)]
                        NonFungible(runtime_types::staging_xcm::v4::asset::AssetInstance),
                    }
                    #[derive(
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                        :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                    #[codec(dumb_trait_bound)]
                    #[decode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                    )]
                    #[encode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                    )]
                    pub enum WildAsset {
                        #[codec(index = 0)]
                        All,
                        #[codec(index = 1)]
                        AllOf {
                            id: runtime_types::staging_xcm::v4::asset::AssetId,
                            fun: runtime_types::staging_xcm::v4::asset::WildFungibility,
                        },
                        #[codec(index = 2)]
                        AllCounted(#[codec(compact)] ::core::primitive::u32),
                        #[codec(index = 3)]
                        AllOfCounted {
                            id: runtime_types::staging_xcm::v4::asset::AssetId,
                            fun: runtime_types::staging_xcm::v4::asset::WildFungibility,
                            #[codec(compact)]
                            count: ::core::primitive::u32,
                        },
                    }
                    #[derive(
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                        :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                    #[codec(dumb_trait_bound)]
                    #[decode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                    )]
                    #[encode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                    )]
                    pub enum WildFungibility {
                        #[codec(index = 0)]
                        Fungible,
                        #[codec(index = 1)]
                        NonFungible,
                    }
                }
                pub mod junction {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                        :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                    #[codec(dumb_trait_bound)]
                    #[decode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                    )]
                    #[encode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                    )]
                    pub enum Junction {
                        #[codec(index = 0)]
                        Parachain(#[codec(compact)] ::core::primitive::u32),
                        #[codec(index = 1)]
                        AccountId32 {
                            network: ::core::option::Option<
                                runtime_types::staging_xcm::v4::junction::NetworkId,
                            >,
                            id: [::core::primitive::u8; 32usize],
                        },
                        #[codec(index = 2)]
                        AccountIndex64 {
                            network: ::core::option::Option<
                                runtime_types::staging_xcm::v4::junction::NetworkId,
                            >,
                            #[codec(compact)]
                            index: ::core::primitive::u64,
                        },
                        #[codec(index = 3)]
                        AccountKey20 {
                            network: ::core::option::Option<
                                runtime_types::staging_xcm::v4::junction::NetworkId,
                            >,
                            key: [::core::primitive::u8; 20usize],
                        },
                        #[codec(index = 4)]
                        PalletInstance(::core::primitive::u8),
                        #[codec(index = 5)]
                        GeneralIndex(#[codec(compact)] ::core::primitive::u128),
                        #[codec(index = 6)]
                        GeneralKey {
                            length: ::core::primitive::u8,
                            data: [::core::primitive::u8; 32usize],
                        },
                        #[codec(index = 7)]
                        OnlyChild,
                        #[codec(index = 8)]
                        Plurality {
                            id: runtime_types::xcm::v3::junction::BodyId,
                            part: runtime_types::xcm::v3::junction::BodyPart,
                        },
                        #[codec(index = 9)]
                        GlobalConsensus(runtime_types::staging_xcm::v4::junction::NetworkId),
                    }
                    #[derive(
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                        :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                    #[codec(dumb_trait_bound)]
                    #[decode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                    )]
                    #[encode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                    )]
                    pub enum NetworkId {
                        #[codec(index = 0)]
                        ByGenesis([::core::primitive::u8; 32usize]),
                        #[codec(index = 1)]
                        ByFork {
                            block_number: ::core::primitive::u64,
                            block_hash: [::core::primitive::u8; 32usize],
                        },
                        #[codec(index = 2)]
                        Polkadot,
                        #[codec(index = 3)]
                        Kusama,
                        #[codec(index = 4)]
                        Westend,
                        #[codec(index = 5)]
                        Rococo,
                        #[codec(index = 6)]
                        Wococo,
                        #[codec(index = 7)]
                        Ethereum {
                            #[codec(compact)]
                            chain_id: ::core::primitive::u64,
                        },
                        #[codec(index = 8)]
                        BitcoinCore,
                        #[codec(index = 9)]
                        BitcoinCash,
                        #[codec(index = 10)]
                        PolkadotBulletin,
                    }
                }
                pub mod junctions {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                        :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                    #[codec(dumb_trait_bound)]
                    #[decode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                    )]
                    #[encode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                    )]
                    pub enum Junctions {
                        #[codec(index = 0)]
                        Here,
                        #[codec(index = 1)]
                        X1([runtime_types::staging_xcm::v4::junction::Junction; 1usize]),
                        #[codec(index = 2)]
                        X2([runtime_types::staging_xcm::v4::junction::Junction; 2usize]),
                        #[codec(index = 3)]
                        X3([runtime_types::staging_xcm::v4::junction::Junction; 3usize]),
                        #[codec(index = 4)]
                        X4([runtime_types::staging_xcm::v4::junction::Junction; 4usize]),
                        #[codec(index = 5)]
                        X5([runtime_types::staging_xcm::v4::junction::Junction; 5usize]),
                        #[codec(index = 6)]
                        X6([runtime_types::staging_xcm::v4::junction::Junction; 6usize]),
                        #[codec(index = 7)]
                        X7([runtime_types::staging_xcm::v4::junction::Junction; 7usize]),
                        #[codec(index = 8)]
                        X8([runtime_types::staging_xcm::v4::junction::Junction; 8usize]),
                    }
                }
                pub mod location {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                        :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                    #[codec(dumb_trait_bound)]
                    #[decode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                    )]
                    #[encode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                    )]
                    pub struct Location {
                        pub parents: ::core::primitive::u8,
                        pub interior: runtime_types::staging_xcm::v4::junctions::Junctions,
                    }
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub enum Instruction {
                    #[codec(index = 0)]
                    WithdrawAsset(runtime_types::staging_xcm::v4::asset::Assets),
                    #[codec(index = 1)]
                    ReserveAssetDeposited(runtime_types::staging_xcm::v4::asset::Assets),
                    #[codec(index = 2)]
                    ReceiveTeleportedAsset(runtime_types::staging_xcm::v4::asset::Assets),
                    #[codec(index = 3)]
                    QueryResponse {
                        #[codec(compact)]
                        query_id: ::core::primitive::u64,
                        response: runtime_types::staging_xcm::v4::Response,
                        max_weight: runtime_types::sp_weights::weight_v2::Weight,
                        querier: ::core::option::Option<
                            runtime_types::staging_xcm::v4::location::Location,
                        >,
                    },
                    #[codec(index = 4)]
                    TransferAsset {
                        assets: runtime_types::staging_xcm::v4::asset::Assets,
                        beneficiary: runtime_types::staging_xcm::v4::location::Location,
                    },
                    #[codec(index = 5)]
                    TransferReserveAsset {
                        assets: runtime_types::staging_xcm::v4::asset::Assets,
                        dest: runtime_types::staging_xcm::v4::location::Location,
                        xcm: runtime_types::staging_xcm::v4::Xcm,
                    },
                    #[codec(index = 6)]
                    Transact {
                        origin_kind: runtime_types::xcm::v3::OriginKind,
                        require_weight_at_most: runtime_types::sp_weights::weight_v2::Weight,
                        call: runtime_types::xcm::double_encoded::DoubleEncoded,
                    },
                    #[codec(index = 7)]
                    HrmpNewChannelOpenRequest {
                        #[codec(compact)]
                        sender: ::core::primitive::u32,
                        #[codec(compact)]
                        max_message_size: ::core::primitive::u32,
                        #[codec(compact)]
                        max_capacity: ::core::primitive::u32,
                    },
                    #[codec(index = 8)]
                    HrmpChannelAccepted {
                        #[codec(compact)]
                        recipient: ::core::primitive::u32,
                    },
                    #[codec(index = 9)]
                    HrmpChannelClosing {
                        #[codec(compact)]
                        initiator: ::core::primitive::u32,
                        #[codec(compact)]
                        sender: ::core::primitive::u32,
                        #[codec(compact)]
                        recipient: ::core::primitive::u32,
                    },
                    #[codec(index = 10)]
                    ClearOrigin,
                    #[codec(index = 11)]
                    DescendOrigin(runtime_types::staging_xcm::v4::junctions::Junctions),
                    #[codec(index = 12)]
                    ReportError(runtime_types::staging_xcm::v4::QueryResponseInfo),
                    #[codec(index = 13)]
                    DepositAsset {
                        assets: runtime_types::staging_xcm::v4::asset::AssetFilter,
                        beneficiary: runtime_types::staging_xcm::v4::location::Location,
                    },
                    #[codec(index = 14)]
                    DepositReserveAsset {
                        assets: runtime_types::staging_xcm::v4::asset::AssetFilter,
                        dest: runtime_types::staging_xcm::v4::location::Location,
                        xcm: runtime_types::staging_xcm::v4::Xcm,
                    },
                    #[codec(index = 15)]
                    ExchangeAsset {
                        give: runtime_types::staging_xcm::v4::asset::AssetFilter,
                        want: runtime_types::staging_xcm::v4::asset::Assets,
                        maximal: ::core::primitive::bool,
                    },
                    #[codec(index = 16)]
                    InitiateReserveWithdraw {
                        assets: runtime_types::staging_xcm::v4::asset::AssetFilter,
                        reserve: runtime_types::staging_xcm::v4::location::Location,
                        xcm: runtime_types::staging_xcm::v4::Xcm,
                    },
                    #[codec(index = 17)]
                    InitiateTeleport {
                        assets: runtime_types::staging_xcm::v4::asset::AssetFilter,
                        dest: runtime_types::staging_xcm::v4::location::Location,
                        xcm: runtime_types::staging_xcm::v4::Xcm,
                    },
                    #[codec(index = 18)]
                    ReportHolding {
                        response_info: runtime_types::staging_xcm::v4::QueryResponseInfo,
                        assets: runtime_types::staging_xcm::v4::asset::AssetFilter,
                    },
                    #[codec(index = 19)]
                    BuyExecution {
                        fees: runtime_types::staging_xcm::v4::asset::Asset,
                        weight_limit: runtime_types::xcm::v3::WeightLimit,
                    },
                    #[codec(index = 20)]
                    RefundSurplus,
                    #[codec(index = 21)]
                    SetErrorHandler(runtime_types::staging_xcm::v4::Xcm),
                    #[codec(index = 22)]
                    SetAppendix(runtime_types::staging_xcm::v4::Xcm),
                    #[codec(index = 23)]
                    ClearError,
                    #[codec(index = 24)]
                    ClaimAsset {
                        assets: runtime_types::staging_xcm::v4::asset::Assets,
                        ticket: runtime_types::staging_xcm::v4::location::Location,
                    },
                    #[codec(index = 25)]
                    Trap(#[codec(compact)] ::core::primitive::u64),
                    #[codec(index = 26)]
                    SubscribeVersion {
                        #[codec(compact)]
                        query_id: ::core::primitive::u64,
                        max_response_weight: runtime_types::sp_weights::weight_v2::Weight,
                    },
                    #[codec(index = 27)]
                    UnsubscribeVersion,
                    #[codec(index = 28)]
                    BurnAsset(runtime_types::staging_xcm::v4::asset::Assets),
                    #[codec(index = 29)]
                    ExpectAsset(runtime_types::staging_xcm::v4::asset::Assets),
                    #[codec(index = 30)]
                    ExpectOrigin(
                        ::core::option::Option<runtime_types::staging_xcm::v4::location::Location>,
                    ),
                    #[codec(index = 31)]
                    ExpectError(
                        ::core::option::Option<(
                            ::core::primitive::u32,
                            runtime_types::xcm::v3::traits::Error,
                        )>,
                    ),
                    #[codec(index = 32)]
                    ExpectTransactStatus(runtime_types::xcm::v3::MaybeErrorCode),
                    #[codec(index = 33)]
                    QueryPallet {
                        module_name:
                            ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                        response_info: runtime_types::staging_xcm::v4::QueryResponseInfo,
                    },
                    #[codec(index = 34)]
                    ExpectPallet {
                        #[codec(compact)]
                        index: ::core::primitive::u32,
                        name: ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                        module_name:
                            ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                        #[codec(compact)]
                        crate_major: ::core::primitive::u32,
                        #[codec(compact)]
                        min_crate_minor: ::core::primitive::u32,
                    },
                    #[codec(index = 35)]
                    ReportTransactStatus(runtime_types::staging_xcm::v4::QueryResponseInfo),
                    #[codec(index = 36)]
                    ClearTransactStatus,
                    #[codec(index = 37)]
                    UniversalOrigin(runtime_types::staging_xcm::v4::junction::Junction),
                    #[codec(index = 38)]
                    ExportMessage {
                        network: runtime_types::staging_xcm::v4::junction::NetworkId,
                        destination: runtime_types::staging_xcm::v4::junctions::Junctions,
                        xcm: runtime_types::staging_xcm::v4::Xcm,
                    },
                    #[codec(index = 39)]
                    LockAsset {
                        asset: runtime_types::staging_xcm::v4::asset::Asset,
                        unlocker: runtime_types::staging_xcm::v4::location::Location,
                    },
                    #[codec(index = 40)]
                    UnlockAsset {
                        asset: runtime_types::staging_xcm::v4::asset::Asset,
                        target: runtime_types::staging_xcm::v4::location::Location,
                    },
                    #[codec(index = 41)]
                    NoteUnlockable {
                        asset: runtime_types::staging_xcm::v4::asset::Asset,
                        owner: runtime_types::staging_xcm::v4::location::Location,
                    },
                    #[codec(index = 42)]
                    RequestUnlock {
                        asset: runtime_types::staging_xcm::v4::asset::Asset,
                        locker: runtime_types::staging_xcm::v4::location::Location,
                    },
                    #[codec(index = 43)]
                    SetFeesMode {
                        jit_withdraw: ::core::primitive::bool,
                    },
                    #[codec(index = 44)]
                    SetTopic([::core::primitive::u8; 32usize]),
                    #[codec(index = 45)]
                    ClearTopic,
                    #[codec(index = 46)]
                    AliasOrigin(runtime_types::staging_xcm::v4::location::Location),
                    #[codec(index = 47)]
                    UnpaidExecution {
                        weight_limit: runtime_types::xcm::v3::WeightLimit,
                        check_origin: ::core::option::Option<
                            runtime_types::staging_xcm::v4::location::Location,
                        >,
                    },
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct PalletInfo {
                    #[codec(compact)]
                    pub index: ::core::primitive::u32,
                    pub name: runtime_types::bounded_collections::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >,
                    pub module_name: runtime_types::bounded_collections::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >,
                    #[codec(compact)]
                    pub major: ::core::primitive::u32,
                    #[codec(compact)]
                    pub minor: ::core::primitive::u32,
                    #[codec(compact)]
                    pub patch: ::core::primitive::u32,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct QueryResponseInfo {
                    pub destination: runtime_types::staging_xcm::v4::location::Location,
                    #[codec(compact)]
                    pub query_id: ::core::primitive::u64,
                    pub max_weight: runtime_types::sp_weights::weight_v2::Weight,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub enum Response {
                    #[codec(index = 0)]
                    Null,
                    #[codec(index = 1)]
                    Assets(runtime_types::staging_xcm::v4::asset::Assets),
                    #[codec(index = 2)]
                    ExecutionResult(
                        ::core::option::Option<(
                            ::core::primitive::u32,
                            runtime_types::xcm::v3::traits::Error,
                        )>,
                    ),
                    #[codec(index = 3)]
                    Version(::core::primitive::u32),
                    #[codec(index = 4)]
                    PalletsInfo(
                        runtime_types::bounded_collections::bounded_vec::BoundedVec<
                            runtime_types::staging_xcm::v4::PalletInfo,
                        >,
                    ),
                    #[codec(index = 5)]
                    DispatchResult(runtime_types::xcm::v3::MaybeErrorCode),
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct Xcm(
                    pub  ::subxt::ext::subxt_core::alloc::vec::Vec<
                        runtime_types::staging_xcm::v4::Instruction,
                    >,
                );
            }
            pub mod v5 {
                use super::runtime_types;
                pub mod asset {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                        :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                    #[codec(dumb_trait_bound)]
                    #[decode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                    )]
                    #[encode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                    )]
                    pub struct Asset {
                        pub id: runtime_types::staging_xcm::v5::asset::AssetId,
                        pub fun: runtime_types::staging_xcm::v5::asset::Fungibility,
                    }
                    #[derive(
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                        :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                    #[codec(dumb_trait_bound)]
                    #[decode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                    )]
                    #[encode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                    )]
                    pub enum AssetFilter {
                        #[codec(index = 0)]
                        Definite(runtime_types::staging_xcm::v5::asset::Assets),
                        #[codec(index = 1)]
                        Wild(runtime_types::staging_xcm::v5::asset::WildAsset),
                    }
                    #[derive(
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                        :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                    #[codec(dumb_trait_bound)]
                    #[decode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                    )]
                    #[encode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                    )]
                    pub struct AssetId(pub runtime_types::staging_xcm::v5::location::Location);
                    #[derive(
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                        :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                    #[codec(dumb_trait_bound)]
                    #[decode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                    )]
                    #[encode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                    )]
                    pub enum AssetInstance {
                        #[codec(index = 0)]
                        Undefined,
                        #[codec(index = 1)]
                        Index(#[codec(compact)] ::core::primitive::u128),
                        #[codec(index = 2)]
                        Array4([::core::primitive::u8; 4usize]),
                        #[codec(index = 3)]
                        Array8([::core::primitive::u8; 8usize]),
                        #[codec(index = 4)]
                        Array16([::core::primitive::u8; 16usize]),
                        #[codec(index = 5)]
                        Array32([::core::primitive::u8; 32usize]),
                    }
                    #[derive(
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                        :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                    #[codec(dumb_trait_bound)]
                    #[decode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                    )]
                    #[encode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                    )]
                    pub enum AssetTransferFilter {
                        #[codec(index = 0)]
                        Teleport(runtime_types::staging_xcm::v5::asset::AssetFilter),
                        #[codec(index = 1)]
                        ReserveDeposit(runtime_types::staging_xcm::v5::asset::AssetFilter),
                        #[codec(index = 2)]
                        ReserveWithdraw(runtime_types::staging_xcm::v5::asset::AssetFilter),
                    }
                    #[derive(
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                        :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                    #[codec(dumb_trait_bound)]
                    #[decode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                    )]
                    #[encode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                    )]
                    pub struct Assets(
                        pub  ::subxt::ext::subxt_core::alloc::vec::Vec<
                            runtime_types::staging_xcm::v5::asset::Asset,
                        >,
                    );
                    #[derive(
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                        :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                    #[codec(dumb_trait_bound)]
                    #[decode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                    )]
                    #[encode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                    )]
                    pub enum Fungibility {
                        #[codec(index = 0)]
                        Fungible(#[codec(compact)] ::core::primitive::u128),
                        #[codec(index = 1)]
                        NonFungible(runtime_types::staging_xcm::v5::asset::AssetInstance),
                    }
                    #[derive(
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                        :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                    #[codec(dumb_trait_bound)]
                    #[decode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                    )]
                    #[encode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                    )]
                    pub enum WildAsset {
                        #[codec(index = 0)]
                        All,
                        #[codec(index = 1)]
                        AllOf {
                            id: runtime_types::staging_xcm::v5::asset::AssetId,
                            fun: runtime_types::staging_xcm::v5::asset::WildFungibility,
                        },
                        #[codec(index = 2)]
                        AllCounted(#[codec(compact)] ::core::primitive::u32),
                        #[codec(index = 3)]
                        AllOfCounted {
                            id: runtime_types::staging_xcm::v5::asset::AssetId,
                            fun: runtime_types::staging_xcm::v5::asset::WildFungibility,
                            #[codec(compact)]
                            count: ::core::primitive::u32,
                        },
                    }
                    #[derive(
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                        :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                    #[codec(dumb_trait_bound)]
                    #[decode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                    )]
                    #[encode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                    )]
                    pub enum WildFungibility {
                        #[codec(index = 0)]
                        Fungible,
                        #[codec(index = 1)]
                        NonFungible,
                    }
                }
                pub mod junction {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                        :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                    #[codec(dumb_trait_bound)]
                    #[decode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                    )]
                    #[encode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                    )]
                    pub enum Junction {
                        #[codec(index = 0)]
                        Parachain(#[codec(compact)] ::core::primitive::u32),
                        #[codec(index = 1)]
                        AccountId32 {
                            network: ::core::option::Option<
                                runtime_types::staging_xcm::v5::junction::NetworkId,
                            >,
                            id: [::core::primitive::u8; 32usize],
                        },
                        #[codec(index = 2)]
                        AccountIndex64 {
                            network: ::core::option::Option<
                                runtime_types::staging_xcm::v5::junction::NetworkId,
                            >,
                            #[codec(compact)]
                            index: ::core::primitive::u64,
                        },
                        #[codec(index = 3)]
                        AccountKey20 {
                            network: ::core::option::Option<
                                runtime_types::staging_xcm::v5::junction::NetworkId,
                            >,
                            key: [::core::primitive::u8; 20usize],
                        },
                        #[codec(index = 4)]
                        PalletInstance(::core::primitive::u8),
                        #[codec(index = 5)]
                        GeneralIndex(#[codec(compact)] ::core::primitive::u128),
                        #[codec(index = 6)]
                        GeneralKey {
                            length: ::core::primitive::u8,
                            data: [::core::primitive::u8; 32usize],
                        },
                        #[codec(index = 7)]
                        OnlyChild,
                        #[codec(index = 8)]
                        Plurality {
                            id: runtime_types::xcm::v3::junction::BodyId,
                            part: runtime_types::xcm::v3::junction::BodyPart,
                        },
                        #[codec(index = 9)]
                        GlobalConsensus(runtime_types::staging_xcm::v5::junction::NetworkId),
                    }
                    #[derive(
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                        :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                    #[codec(dumb_trait_bound)]
                    #[decode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                    )]
                    #[encode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                    )]
                    pub enum NetworkId {
                        #[codec(index = 0)]
                        ByGenesis([::core::primitive::u8; 32usize]),
                        #[codec(index = 1)]
                        ByFork {
                            block_number: ::core::primitive::u64,
                            block_hash: [::core::primitive::u8; 32usize],
                        },
                        #[codec(index = 2)]
                        Polkadot,
                        #[codec(index = 3)]
                        Kusama,
                        #[codec(index = 7)]
                        Ethereum {
                            #[codec(compact)]
                            chain_id: ::core::primitive::u64,
                        },
                        #[codec(index = 8)]
                        BitcoinCore,
                        #[codec(index = 9)]
                        BitcoinCash,
                        #[codec(index = 10)]
                        PolkadotBulletin,
                    }
                }
                pub mod junctions {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                        :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                    #[codec(dumb_trait_bound)]
                    #[decode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                    )]
                    #[encode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                    )]
                    pub enum Junctions {
                        #[codec(index = 0)]
                        Here,
                        #[codec(index = 1)]
                        X1([runtime_types::staging_xcm::v5::junction::Junction; 1usize]),
                        #[codec(index = 2)]
                        X2([runtime_types::staging_xcm::v5::junction::Junction; 2usize]),
                        #[codec(index = 3)]
                        X3([runtime_types::staging_xcm::v5::junction::Junction; 3usize]),
                        #[codec(index = 4)]
                        X4([runtime_types::staging_xcm::v5::junction::Junction; 4usize]),
                        #[codec(index = 5)]
                        X5([runtime_types::staging_xcm::v5::junction::Junction; 5usize]),
                        #[codec(index = 6)]
                        X6([runtime_types::staging_xcm::v5::junction::Junction; 6usize]),
                        #[codec(index = 7)]
                        X7([runtime_types::staging_xcm::v5::junction::Junction; 7usize]),
                        #[codec(index = 8)]
                        X8([runtime_types::staging_xcm::v5::junction::Junction; 8usize]),
                    }
                }
                pub mod location {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                        :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                    #[codec(dumb_trait_bound)]
                    #[decode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                    )]
                    #[encode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                    )]
                    pub struct Location {
                        pub parents: ::core::primitive::u8,
                        pub interior: runtime_types::staging_xcm::v5::junctions::Junctions,
                    }
                }
                pub mod traits {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                        :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                    #[codec(dumb_trait_bound)]
                    #[decode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                    )]
                    #[encode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                    )]
                    pub struct InstructionError {
                        pub index: ::core::primitive::u8,
                        pub error: runtime_types::xcm::v5::traits::Error,
                    }
                    #[derive(
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                        :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                    #[codec(dumb_trait_bound)]
                    #[decode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                    )]
                    #[encode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                    )]
                    pub enum Outcome {
                        #[codec(index = 0)]
                        Complete {
                            used: runtime_types::sp_weights::weight_v2::Weight,
                        },
                        #[codec(index = 1)]
                        Incomplete {
                            used: runtime_types::sp_weights::weight_v2::Weight,
                            error: runtime_types::staging_xcm::v5::traits::InstructionError,
                        },
                        #[codec(index = 2)]
                        Error(runtime_types::staging_xcm::v5::traits::InstructionError),
                    }
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub enum Hint {
                    #[codec(index = 0)]
                    AssetClaimer {
                        location: runtime_types::staging_xcm::v5::location::Location,
                    },
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub enum Instruction {
                    #[codec(index = 0)]
                    WithdrawAsset(runtime_types::staging_xcm::v5::asset::Assets),
                    #[codec(index = 1)]
                    ReserveAssetDeposited(runtime_types::staging_xcm::v5::asset::Assets),
                    #[codec(index = 2)]
                    ReceiveTeleportedAsset(runtime_types::staging_xcm::v5::asset::Assets),
                    #[codec(index = 3)]
                    QueryResponse {
                        #[codec(compact)]
                        query_id: ::core::primitive::u64,
                        response: runtime_types::staging_xcm::v5::Response,
                        max_weight: runtime_types::sp_weights::weight_v2::Weight,
                        querier: ::core::option::Option<
                            runtime_types::staging_xcm::v5::location::Location,
                        >,
                    },
                    #[codec(index = 4)]
                    TransferAsset {
                        assets: runtime_types::staging_xcm::v5::asset::Assets,
                        beneficiary: runtime_types::staging_xcm::v5::location::Location,
                    },
                    #[codec(index = 5)]
                    TransferReserveAsset {
                        assets: runtime_types::staging_xcm::v5::asset::Assets,
                        dest: runtime_types::staging_xcm::v5::location::Location,
                        xcm: runtime_types::staging_xcm::v5::Xcm,
                    },
                    #[codec(index = 6)]
                    Transact {
                        origin_kind: runtime_types::xcm::v3::OriginKind,
                        fallback_max_weight:
                            ::core::option::Option<runtime_types::sp_weights::weight_v2::Weight>,
                        call: runtime_types::xcm::double_encoded::DoubleEncoded,
                    },
                    #[codec(index = 7)]
                    HrmpNewChannelOpenRequest {
                        #[codec(compact)]
                        sender: ::core::primitive::u32,
                        #[codec(compact)]
                        max_message_size: ::core::primitive::u32,
                        #[codec(compact)]
                        max_capacity: ::core::primitive::u32,
                    },
                    #[codec(index = 8)]
                    HrmpChannelAccepted {
                        #[codec(compact)]
                        recipient: ::core::primitive::u32,
                    },
                    #[codec(index = 9)]
                    HrmpChannelClosing {
                        #[codec(compact)]
                        initiator: ::core::primitive::u32,
                        #[codec(compact)]
                        sender: ::core::primitive::u32,
                        #[codec(compact)]
                        recipient: ::core::primitive::u32,
                    },
                    #[codec(index = 10)]
                    ClearOrigin,
                    #[codec(index = 11)]
                    DescendOrigin(runtime_types::staging_xcm::v5::junctions::Junctions),
                    #[codec(index = 12)]
                    ReportError(runtime_types::staging_xcm::v5::QueryResponseInfo),
                    #[codec(index = 13)]
                    DepositAsset {
                        assets: runtime_types::staging_xcm::v5::asset::AssetFilter,
                        beneficiary: runtime_types::staging_xcm::v5::location::Location,
                    },
                    #[codec(index = 14)]
                    DepositReserveAsset {
                        assets: runtime_types::staging_xcm::v5::asset::AssetFilter,
                        dest: runtime_types::staging_xcm::v5::location::Location,
                        xcm: runtime_types::staging_xcm::v5::Xcm,
                    },
                    #[codec(index = 15)]
                    ExchangeAsset {
                        give: runtime_types::staging_xcm::v5::asset::AssetFilter,
                        want: runtime_types::staging_xcm::v5::asset::Assets,
                        maximal: ::core::primitive::bool,
                    },
                    #[codec(index = 16)]
                    InitiateReserveWithdraw {
                        assets: runtime_types::staging_xcm::v5::asset::AssetFilter,
                        reserve: runtime_types::staging_xcm::v5::location::Location,
                        xcm: runtime_types::staging_xcm::v5::Xcm,
                    },
                    #[codec(index = 17)]
                    InitiateTeleport {
                        assets: runtime_types::staging_xcm::v5::asset::AssetFilter,
                        dest: runtime_types::staging_xcm::v5::location::Location,
                        xcm: runtime_types::staging_xcm::v5::Xcm,
                    },
                    #[codec(index = 18)]
                    ReportHolding {
                        response_info: runtime_types::staging_xcm::v5::QueryResponseInfo,
                        assets: runtime_types::staging_xcm::v5::asset::AssetFilter,
                    },
                    #[codec(index = 19)]
                    BuyExecution {
                        fees: runtime_types::staging_xcm::v5::asset::Asset,
                        weight_limit: runtime_types::xcm::v3::WeightLimit,
                    },
                    #[codec(index = 20)]
                    RefundSurplus,
                    #[codec(index = 21)]
                    SetErrorHandler(runtime_types::staging_xcm::v5::Xcm),
                    #[codec(index = 22)]
                    SetAppendix(runtime_types::staging_xcm::v5::Xcm),
                    #[codec(index = 23)]
                    ClearError,
                    #[codec(index = 24)]
                    ClaimAsset {
                        assets: runtime_types::staging_xcm::v5::asset::Assets,
                        ticket: runtime_types::staging_xcm::v5::location::Location,
                    },
                    #[codec(index = 25)]
                    Trap(#[codec(compact)] ::core::primitive::u64),
                    #[codec(index = 26)]
                    SubscribeVersion {
                        #[codec(compact)]
                        query_id: ::core::primitive::u64,
                        max_response_weight: runtime_types::sp_weights::weight_v2::Weight,
                    },
                    #[codec(index = 27)]
                    UnsubscribeVersion,
                    #[codec(index = 28)]
                    BurnAsset(runtime_types::staging_xcm::v5::asset::Assets),
                    #[codec(index = 29)]
                    ExpectAsset(runtime_types::staging_xcm::v5::asset::Assets),
                    #[codec(index = 30)]
                    ExpectOrigin(
                        ::core::option::Option<runtime_types::staging_xcm::v5::location::Location>,
                    ),
                    #[codec(index = 31)]
                    ExpectError(
                        ::core::option::Option<(
                            ::core::primitive::u32,
                            runtime_types::xcm::v5::traits::Error,
                        )>,
                    ),
                    #[codec(index = 32)]
                    ExpectTransactStatus(runtime_types::xcm::v3::MaybeErrorCode),
                    #[codec(index = 33)]
                    QueryPallet {
                        module_name:
                            ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                        response_info: runtime_types::staging_xcm::v5::QueryResponseInfo,
                    },
                    #[codec(index = 34)]
                    ExpectPallet {
                        #[codec(compact)]
                        index: ::core::primitive::u32,
                        name: ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                        module_name:
                            ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                        #[codec(compact)]
                        crate_major: ::core::primitive::u32,
                        #[codec(compact)]
                        min_crate_minor: ::core::primitive::u32,
                    },
                    #[codec(index = 35)]
                    ReportTransactStatus(runtime_types::staging_xcm::v5::QueryResponseInfo),
                    #[codec(index = 36)]
                    ClearTransactStatus,
                    #[codec(index = 37)]
                    UniversalOrigin(runtime_types::staging_xcm::v5::junction::Junction),
                    #[codec(index = 38)]
                    ExportMessage {
                        network: runtime_types::staging_xcm::v5::junction::NetworkId,
                        destination: runtime_types::staging_xcm::v5::junctions::Junctions,
                        xcm: runtime_types::staging_xcm::v5::Xcm,
                    },
                    #[codec(index = 39)]
                    LockAsset {
                        asset: runtime_types::staging_xcm::v5::asset::Asset,
                        unlocker: runtime_types::staging_xcm::v5::location::Location,
                    },
                    #[codec(index = 40)]
                    UnlockAsset {
                        asset: runtime_types::staging_xcm::v5::asset::Asset,
                        target: runtime_types::staging_xcm::v5::location::Location,
                    },
                    #[codec(index = 41)]
                    NoteUnlockable {
                        asset: runtime_types::staging_xcm::v5::asset::Asset,
                        owner: runtime_types::staging_xcm::v5::location::Location,
                    },
                    #[codec(index = 42)]
                    RequestUnlock {
                        asset: runtime_types::staging_xcm::v5::asset::Asset,
                        locker: runtime_types::staging_xcm::v5::location::Location,
                    },
                    #[codec(index = 43)]
                    SetFeesMode {
                        jit_withdraw: ::core::primitive::bool,
                    },
                    #[codec(index = 44)]
                    SetTopic([::core::primitive::u8; 32usize]),
                    #[codec(index = 45)]
                    ClearTopic,
                    #[codec(index = 46)]
                    AliasOrigin(runtime_types::staging_xcm::v5::location::Location),
                    #[codec(index = 47)]
                    UnpaidExecution {
                        weight_limit: runtime_types::xcm::v3::WeightLimit,
                        check_origin: ::core::option::Option<
                            runtime_types::staging_xcm::v5::location::Location,
                        >,
                    },
                    #[codec(index = 48)]
                    PayFees {
                        asset: runtime_types::staging_xcm::v5::asset::Asset,
                    },
                    #[codec(index = 49)]
                    InitiateTransfer {
                        destination: runtime_types::staging_xcm::v5::location::Location,
                        remote_fees: ::core::option::Option<
                            runtime_types::staging_xcm::v5::asset::AssetTransferFilter,
                        >,
                        preserve_origin: ::core::primitive::bool,
                        assets: runtime_types::bounded_collections::bounded_vec::BoundedVec<
                            runtime_types::staging_xcm::v5::asset::AssetTransferFilter,
                        >,
                        remote_xcm: runtime_types::staging_xcm::v5::Xcm,
                    },
                    #[codec(index = 50)]
                    ExecuteWithOrigin {
                        descendant_origin: ::core::option::Option<
                            runtime_types::staging_xcm::v5::junctions::Junctions,
                        >,
                        xcm: runtime_types::staging_xcm::v5::Xcm,
                    },
                    #[codec(index = 51)]
                    SetHints {
                        hints: runtime_types::bounded_collections::bounded_vec::BoundedVec<
                            runtime_types::staging_xcm::v5::Hint,
                        >,
                    },
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct PalletInfo {
                    #[codec(compact)]
                    pub index: ::core::primitive::u32,
                    pub name: runtime_types::bounded_collections::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >,
                    pub module_name: runtime_types::bounded_collections::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >,
                    #[codec(compact)]
                    pub major: ::core::primitive::u32,
                    #[codec(compact)]
                    pub minor: ::core::primitive::u32,
                    #[codec(compact)]
                    pub patch: ::core::primitive::u32,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct QueryResponseInfo {
                    pub destination: runtime_types::staging_xcm::v5::location::Location,
                    #[codec(compact)]
                    pub query_id: ::core::primitive::u64,
                    pub max_weight: runtime_types::sp_weights::weight_v2::Weight,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub enum Response {
                    #[codec(index = 0)]
                    Null,
                    #[codec(index = 1)]
                    Assets(runtime_types::staging_xcm::v5::asset::Assets),
                    #[codec(index = 2)]
                    ExecutionResult(
                        ::core::option::Option<(
                            ::core::primitive::u32,
                            runtime_types::xcm::v5::traits::Error,
                        )>,
                    ),
                    #[codec(index = 3)]
                    Version(::core::primitive::u32),
                    #[codec(index = 4)]
                    PalletsInfo(
                        runtime_types::bounded_collections::bounded_vec::BoundedVec<
                            runtime_types::staging_xcm::v5::PalletInfo,
                        >,
                    ),
                    #[codec(index = 5)]
                    DispatchResult(runtime_types::xcm::v3::MaybeErrorCode),
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct Xcm(
                    pub  ::subxt::ext::subxt_core::alloc::vec::Vec<
                        runtime_types::staging_xcm::v5::Instruction,
                    >,
                );
            }
        }
        pub mod staging_xcm_executor {
            use super::runtime_types;
            pub mod traits {
                use super::runtime_types;
                pub mod asset_transfer {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                        :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                    #[codec(dumb_trait_bound)]
                    #[decode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                    )]
                    #[encode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                    )]
                    pub enum TransferType {
                        #[codec(index = 0)]
                        Teleport,
                        #[codec(index = 1)]
                        LocalReserve,
                        #[codec(index = 2)]
                        DestinationReserve,
                        #[codec(index = 3)]
                        RemoteReserve(runtime_types::xcm::VersionedLocation),
                    }
                }
            }
        }
        pub mod verifiable {
            use super::runtime_types;
            pub mod ring_vrf_impl {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct EncodedPublicKey(pub [::core::primitive::u8; 32usize]);
            }
        }
        pub mod xcm {
            use super::runtime_types;
            pub mod double_encoded {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct DoubleEncoded {
                    pub encoded: ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                }
            }
            pub mod v3 {
                use super::runtime_types;
                pub mod junction {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                        :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                    #[codec(dumb_trait_bound)]
                    #[decode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                    )]
                    #[encode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                    )]
                    pub enum BodyId {
                        #[codec(index = 0)]
                        Unit,
                        #[codec(index = 1)]
                        Moniker([::core::primitive::u8; 4usize]),
                        #[codec(index = 2)]
                        Index(#[codec(compact)] ::core::primitive::u32),
                        #[codec(index = 3)]
                        Executive,
                        #[codec(index = 4)]
                        Technical,
                        #[codec(index = 5)]
                        Legislative,
                        #[codec(index = 6)]
                        Judicial,
                        #[codec(index = 7)]
                        Defense,
                        #[codec(index = 8)]
                        Administration,
                        #[codec(index = 9)]
                        Treasury,
                    }
                    #[derive(
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                        :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                    #[codec(dumb_trait_bound)]
                    #[decode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                    )]
                    #[encode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                    )]
                    pub enum BodyPart {
                        #[codec(index = 0)]
                        Voice,
                        #[codec(index = 1)]
                        Members {
                            #[codec(compact)]
                            count: ::core::primitive::u32,
                        },
                        #[codec(index = 2)]
                        Fraction {
                            #[codec(compact)]
                            nom: ::core::primitive::u32,
                            #[codec(compact)]
                            denom: ::core::primitive::u32,
                        },
                        #[codec(index = 3)]
                        AtLeastProportion {
                            #[codec(compact)]
                            nom: ::core::primitive::u32,
                            #[codec(compact)]
                            denom: ::core::primitive::u32,
                        },
                        #[codec(index = 4)]
                        MoreThanProportion {
                            #[codec(compact)]
                            nom: ::core::primitive::u32,
                            #[codec(compact)]
                            denom: ::core::primitive::u32,
                        },
                    }
                    #[derive(
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                        :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                    #[codec(dumb_trait_bound)]
                    #[decode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                    )]
                    #[encode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                    )]
                    pub enum Junction {
                        #[codec(index = 0)]
                        Parachain(#[codec(compact)] ::core::primitive::u32),
                        #[codec(index = 1)]
                        AccountId32 {
                            network:
                                ::core::option::Option<runtime_types::xcm::v3::junction::NetworkId>,
                            id: [::core::primitive::u8; 32usize],
                        },
                        #[codec(index = 2)]
                        AccountIndex64 {
                            network:
                                ::core::option::Option<runtime_types::xcm::v3::junction::NetworkId>,
                            #[codec(compact)]
                            index: ::core::primitive::u64,
                        },
                        #[codec(index = 3)]
                        AccountKey20 {
                            network:
                                ::core::option::Option<runtime_types::xcm::v3::junction::NetworkId>,
                            key: [::core::primitive::u8; 20usize],
                        },
                        #[codec(index = 4)]
                        PalletInstance(::core::primitive::u8),
                        #[codec(index = 5)]
                        GeneralIndex(#[codec(compact)] ::core::primitive::u128),
                        #[codec(index = 6)]
                        GeneralKey {
                            length: ::core::primitive::u8,
                            data: [::core::primitive::u8; 32usize],
                        },
                        #[codec(index = 7)]
                        OnlyChild,
                        #[codec(index = 8)]
                        Plurality {
                            id: runtime_types::xcm::v3::junction::BodyId,
                            part: runtime_types::xcm::v3::junction::BodyPart,
                        },
                        #[codec(index = 9)]
                        GlobalConsensus(runtime_types::xcm::v3::junction::NetworkId),
                    }
                    #[derive(
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                        :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                    #[codec(dumb_trait_bound)]
                    #[decode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                    )]
                    #[encode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                    )]
                    pub enum NetworkId {
                        #[codec(index = 0)]
                        ByGenesis([::core::primitive::u8; 32usize]),
                        #[codec(index = 1)]
                        ByFork {
                            block_number: ::core::primitive::u64,
                            block_hash: [::core::primitive::u8; 32usize],
                        },
                        #[codec(index = 2)]
                        Polkadot,
                        #[codec(index = 3)]
                        Kusama,
                        #[codec(index = 4)]
                        Westend,
                        #[codec(index = 5)]
                        Rococo,
                        #[codec(index = 6)]
                        Wococo,
                        #[codec(index = 7)]
                        Ethereum {
                            #[codec(compact)]
                            chain_id: ::core::primitive::u64,
                        },
                        #[codec(index = 8)]
                        BitcoinCore,
                        #[codec(index = 9)]
                        BitcoinCash,
                        #[codec(index = 10)]
                        PolkadotBulletin,
                    }
                }
                pub mod junctions {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                        :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                    #[codec(dumb_trait_bound)]
                    #[decode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                    )]
                    #[encode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                    )]
                    pub enum Junctions {
                        #[codec(index = 0)]
                        Here,
                        #[codec(index = 1)]
                        X1(runtime_types::xcm::v3::junction::Junction),
                        #[codec(index = 2)]
                        X2(
                            runtime_types::xcm::v3::junction::Junction,
                            runtime_types::xcm::v3::junction::Junction,
                        ),
                        #[codec(index = 3)]
                        X3(
                            runtime_types::xcm::v3::junction::Junction,
                            runtime_types::xcm::v3::junction::Junction,
                            runtime_types::xcm::v3::junction::Junction,
                        ),
                        #[codec(index = 4)]
                        X4(
                            runtime_types::xcm::v3::junction::Junction,
                            runtime_types::xcm::v3::junction::Junction,
                            runtime_types::xcm::v3::junction::Junction,
                            runtime_types::xcm::v3::junction::Junction,
                        ),
                        #[codec(index = 5)]
                        X5(
                            runtime_types::xcm::v3::junction::Junction,
                            runtime_types::xcm::v3::junction::Junction,
                            runtime_types::xcm::v3::junction::Junction,
                            runtime_types::xcm::v3::junction::Junction,
                            runtime_types::xcm::v3::junction::Junction,
                        ),
                        #[codec(index = 6)]
                        X6(
                            runtime_types::xcm::v3::junction::Junction,
                            runtime_types::xcm::v3::junction::Junction,
                            runtime_types::xcm::v3::junction::Junction,
                            runtime_types::xcm::v3::junction::Junction,
                            runtime_types::xcm::v3::junction::Junction,
                            runtime_types::xcm::v3::junction::Junction,
                        ),
                        #[codec(index = 7)]
                        X7(
                            runtime_types::xcm::v3::junction::Junction,
                            runtime_types::xcm::v3::junction::Junction,
                            runtime_types::xcm::v3::junction::Junction,
                            runtime_types::xcm::v3::junction::Junction,
                            runtime_types::xcm::v3::junction::Junction,
                            runtime_types::xcm::v3::junction::Junction,
                            runtime_types::xcm::v3::junction::Junction,
                        ),
                        #[codec(index = 8)]
                        X8(
                            runtime_types::xcm::v3::junction::Junction,
                            runtime_types::xcm::v3::junction::Junction,
                            runtime_types::xcm::v3::junction::Junction,
                            runtime_types::xcm::v3::junction::Junction,
                            runtime_types::xcm::v3::junction::Junction,
                            runtime_types::xcm::v3::junction::Junction,
                            runtime_types::xcm::v3::junction::Junction,
                            runtime_types::xcm::v3::junction::Junction,
                        ),
                    }
                }
                pub mod multiasset {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                        :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                    #[codec(dumb_trait_bound)]
                    #[decode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                    )]
                    #[encode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                    )]
                    pub enum AssetId {
                        #[codec(index = 0)]
                        Concrete(runtime_types::staging_xcm::v3::multilocation::MultiLocation),
                        #[codec(index = 1)]
                        Abstract([::core::primitive::u8; 32usize]),
                    }
                    #[derive(
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                        :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                    #[codec(dumb_trait_bound)]
                    #[decode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                    )]
                    #[encode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                    )]
                    pub enum AssetInstance {
                        #[codec(index = 0)]
                        Undefined,
                        #[codec(index = 1)]
                        Index(#[codec(compact)] ::core::primitive::u128),
                        #[codec(index = 2)]
                        Array4([::core::primitive::u8; 4usize]),
                        #[codec(index = 3)]
                        Array8([::core::primitive::u8; 8usize]),
                        #[codec(index = 4)]
                        Array16([::core::primitive::u8; 16usize]),
                        #[codec(index = 5)]
                        Array32([::core::primitive::u8; 32usize]),
                    }
                    #[derive(
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                        :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                    #[codec(dumb_trait_bound)]
                    #[decode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                    )]
                    #[encode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                    )]
                    pub enum Fungibility {
                        #[codec(index = 0)]
                        Fungible(#[codec(compact)] ::core::primitive::u128),
                        #[codec(index = 1)]
                        NonFungible(runtime_types::xcm::v3::multiasset::AssetInstance),
                    }
                    #[derive(
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                        :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                    #[codec(dumb_trait_bound)]
                    #[decode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                    )]
                    #[encode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                    )]
                    pub struct MultiAsset {
                        pub id: runtime_types::xcm::v3::multiasset::AssetId,
                        pub fun: runtime_types::xcm::v3::multiasset::Fungibility,
                    }
                    #[derive(
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                        :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                    #[codec(dumb_trait_bound)]
                    #[decode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                    )]
                    #[encode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                    )]
                    pub enum MultiAssetFilter {
                        #[codec(index = 0)]
                        Definite(runtime_types::xcm::v3::multiasset::MultiAssets),
                        #[codec(index = 1)]
                        Wild(runtime_types::xcm::v3::multiasset::WildMultiAsset),
                    }
                    #[derive(
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                        :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                    #[codec(dumb_trait_bound)]
                    #[decode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                    )]
                    #[encode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                    )]
                    pub struct MultiAssets(
                        pub  ::subxt::ext::subxt_core::alloc::vec::Vec<
                            runtime_types::xcm::v3::multiasset::MultiAsset,
                        >,
                    );
                    #[derive(
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                        :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                    #[codec(dumb_trait_bound)]
                    #[decode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                    )]
                    #[encode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                    )]
                    pub enum WildFungibility {
                        #[codec(index = 0)]
                        Fungible,
                        #[codec(index = 1)]
                        NonFungible,
                    }
                    #[derive(
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                        :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                    #[codec(dumb_trait_bound)]
                    #[decode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                    )]
                    #[encode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                    )]
                    pub enum WildMultiAsset {
                        #[codec(index = 0)]
                        All,
                        #[codec(index = 1)]
                        AllOf {
                            id: runtime_types::xcm::v3::multiasset::AssetId,
                            fun: runtime_types::xcm::v3::multiasset::WildFungibility,
                        },
                        #[codec(index = 2)]
                        AllCounted(#[codec(compact)] ::core::primitive::u32),
                        #[codec(index = 3)]
                        AllOfCounted {
                            id: runtime_types::xcm::v3::multiasset::AssetId,
                            fun: runtime_types::xcm::v3::multiasset::WildFungibility,
                            #[codec(compact)]
                            count: ::core::primitive::u32,
                        },
                    }
                }
                pub mod traits {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                        :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                    #[codec(dumb_trait_bound)]
                    #[decode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                    )]
                    #[encode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                    )]
                    pub enum Error {
                        #[codec(index = 0)]
                        Overflow,
                        #[codec(index = 1)]
                        Unimplemented,
                        #[codec(index = 2)]
                        UntrustedReserveLocation,
                        #[codec(index = 3)]
                        UntrustedTeleportLocation,
                        #[codec(index = 4)]
                        LocationFull,
                        #[codec(index = 5)]
                        LocationNotInvertible,
                        #[codec(index = 6)]
                        BadOrigin,
                        #[codec(index = 7)]
                        InvalidLocation,
                        #[codec(index = 8)]
                        AssetNotFound,
                        #[codec(index = 9)]
                        FailedToTransactAsset,
                        #[codec(index = 10)]
                        NotWithdrawable,
                        #[codec(index = 11)]
                        LocationCannotHold,
                        #[codec(index = 12)]
                        ExceedsMaxMessageSize,
                        #[codec(index = 13)]
                        DestinationUnsupported,
                        #[codec(index = 14)]
                        Transport,
                        #[codec(index = 15)]
                        Unroutable,
                        #[codec(index = 16)]
                        UnknownClaim,
                        #[codec(index = 17)]
                        FailedToDecode,
                        #[codec(index = 18)]
                        MaxWeightInvalid,
                        #[codec(index = 19)]
                        NotHoldingFees,
                        #[codec(index = 20)]
                        TooExpensive,
                        #[codec(index = 21)]
                        Trap(::core::primitive::u64),
                        #[codec(index = 22)]
                        ExpectationFalse,
                        #[codec(index = 23)]
                        PalletNotFound,
                        #[codec(index = 24)]
                        NameMismatch,
                        #[codec(index = 25)]
                        VersionIncompatible,
                        #[codec(index = 26)]
                        HoldingWouldOverflow,
                        #[codec(index = 27)]
                        ExportError,
                        #[codec(index = 28)]
                        ReanchorFailed,
                        #[codec(index = 29)]
                        NoDeal,
                        #[codec(index = 30)]
                        FeesNotMet,
                        #[codec(index = 31)]
                        LockError,
                        #[codec(index = 32)]
                        NoPermission,
                        #[codec(index = 33)]
                        Unanchored,
                        #[codec(index = 34)]
                        NotDepositable,
                        #[codec(index = 35)]
                        UnhandledXcmVersion,
                        #[codec(index = 36)]
                        WeightLimitReached(runtime_types::sp_weights::weight_v2::Weight),
                        #[codec(index = 37)]
                        Barrier,
                        #[codec(index = 38)]
                        WeightNotComputable,
                        #[codec(index = 39)]
                        ExceedsStackLimit,
                    }
                    #[derive(
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                        :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                    #[codec(dumb_trait_bound)]
                    #[decode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                    )]
                    #[encode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                    )]
                    pub enum SendError {
                        #[codec(index = 0)]
                        NotApplicable,
                        #[codec(index = 1)]
                        Transport,
                        #[codec(index = 2)]
                        Unroutable,
                        #[codec(index = 3)]
                        DestinationUnsupported,
                        #[codec(index = 4)]
                        ExceedsMaxMessageSize,
                        #[codec(index = 5)]
                        MissingArgument,
                        #[codec(index = 6)]
                        Fees,
                    }
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub enum Instruction {
                    #[codec(index = 0)]
                    WithdrawAsset(runtime_types::xcm::v3::multiasset::MultiAssets),
                    #[codec(index = 1)]
                    ReserveAssetDeposited(runtime_types::xcm::v3::multiasset::MultiAssets),
                    #[codec(index = 2)]
                    ReceiveTeleportedAsset(runtime_types::xcm::v3::multiasset::MultiAssets),
                    #[codec(index = 3)]
                    QueryResponse {
                        #[codec(compact)]
                        query_id: ::core::primitive::u64,
                        response: runtime_types::xcm::v3::Response,
                        max_weight: runtime_types::sp_weights::weight_v2::Weight,
                        querier: ::core::option::Option<
                            runtime_types::staging_xcm::v3::multilocation::MultiLocation,
                        >,
                    },
                    #[codec(index = 4)]
                    TransferAsset {
                        assets: runtime_types::xcm::v3::multiasset::MultiAssets,
                        beneficiary: runtime_types::staging_xcm::v3::multilocation::MultiLocation,
                    },
                    #[codec(index = 5)]
                    TransferReserveAsset {
                        assets: runtime_types::xcm::v3::multiasset::MultiAssets,
                        dest: runtime_types::staging_xcm::v3::multilocation::MultiLocation,
                        xcm: runtime_types::xcm::v3::Xcm,
                    },
                    #[codec(index = 6)]
                    Transact {
                        origin_kind: runtime_types::xcm::v3::OriginKind,
                        require_weight_at_most: runtime_types::sp_weights::weight_v2::Weight,
                        call: runtime_types::xcm::double_encoded::DoubleEncoded,
                    },
                    #[codec(index = 7)]
                    HrmpNewChannelOpenRequest {
                        #[codec(compact)]
                        sender: ::core::primitive::u32,
                        #[codec(compact)]
                        max_message_size: ::core::primitive::u32,
                        #[codec(compact)]
                        max_capacity: ::core::primitive::u32,
                    },
                    #[codec(index = 8)]
                    HrmpChannelAccepted {
                        #[codec(compact)]
                        recipient: ::core::primitive::u32,
                    },
                    #[codec(index = 9)]
                    HrmpChannelClosing {
                        #[codec(compact)]
                        initiator: ::core::primitive::u32,
                        #[codec(compact)]
                        sender: ::core::primitive::u32,
                        #[codec(compact)]
                        recipient: ::core::primitive::u32,
                    },
                    #[codec(index = 10)]
                    ClearOrigin,
                    #[codec(index = 11)]
                    DescendOrigin(runtime_types::xcm::v3::junctions::Junctions),
                    #[codec(index = 12)]
                    ReportError(runtime_types::xcm::v3::QueryResponseInfo),
                    #[codec(index = 13)]
                    DepositAsset {
                        assets: runtime_types::xcm::v3::multiasset::MultiAssetFilter,
                        beneficiary: runtime_types::staging_xcm::v3::multilocation::MultiLocation,
                    },
                    #[codec(index = 14)]
                    DepositReserveAsset {
                        assets: runtime_types::xcm::v3::multiasset::MultiAssetFilter,
                        dest: runtime_types::staging_xcm::v3::multilocation::MultiLocation,
                        xcm: runtime_types::xcm::v3::Xcm,
                    },
                    #[codec(index = 15)]
                    ExchangeAsset {
                        give: runtime_types::xcm::v3::multiasset::MultiAssetFilter,
                        want: runtime_types::xcm::v3::multiasset::MultiAssets,
                        maximal: ::core::primitive::bool,
                    },
                    #[codec(index = 16)]
                    InitiateReserveWithdraw {
                        assets: runtime_types::xcm::v3::multiasset::MultiAssetFilter,
                        reserve: runtime_types::staging_xcm::v3::multilocation::MultiLocation,
                        xcm: runtime_types::xcm::v3::Xcm,
                    },
                    #[codec(index = 17)]
                    InitiateTeleport {
                        assets: runtime_types::xcm::v3::multiasset::MultiAssetFilter,
                        dest: runtime_types::staging_xcm::v3::multilocation::MultiLocation,
                        xcm: runtime_types::xcm::v3::Xcm,
                    },
                    #[codec(index = 18)]
                    ReportHolding {
                        response_info: runtime_types::xcm::v3::QueryResponseInfo,
                        assets: runtime_types::xcm::v3::multiasset::MultiAssetFilter,
                    },
                    #[codec(index = 19)]
                    BuyExecution {
                        fees: runtime_types::xcm::v3::multiasset::MultiAsset,
                        weight_limit: runtime_types::xcm::v3::WeightLimit,
                    },
                    #[codec(index = 20)]
                    RefundSurplus,
                    #[codec(index = 21)]
                    SetErrorHandler(runtime_types::xcm::v3::Xcm),
                    #[codec(index = 22)]
                    SetAppendix(runtime_types::xcm::v3::Xcm),
                    #[codec(index = 23)]
                    ClearError,
                    #[codec(index = 24)]
                    ClaimAsset {
                        assets: runtime_types::xcm::v3::multiasset::MultiAssets,
                        ticket: runtime_types::staging_xcm::v3::multilocation::MultiLocation,
                    },
                    #[codec(index = 25)]
                    Trap(#[codec(compact)] ::core::primitive::u64),
                    #[codec(index = 26)]
                    SubscribeVersion {
                        #[codec(compact)]
                        query_id: ::core::primitive::u64,
                        max_response_weight: runtime_types::sp_weights::weight_v2::Weight,
                    },
                    #[codec(index = 27)]
                    UnsubscribeVersion,
                    #[codec(index = 28)]
                    BurnAsset(runtime_types::xcm::v3::multiasset::MultiAssets),
                    #[codec(index = 29)]
                    ExpectAsset(runtime_types::xcm::v3::multiasset::MultiAssets),
                    #[codec(index = 30)]
                    ExpectOrigin(
                        ::core::option::Option<
                            runtime_types::staging_xcm::v3::multilocation::MultiLocation,
                        >,
                    ),
                    #[codec(index = 31)]
                    ExpectError(
                        ::core::option::Option<(
                            ::core::primitive::u32,
                            runtime_types::xcm::v3::traits::Error,
                        )>,
                    ),
                    #[codec(index = 32)]
                    ExpectTransactStatus(runtime_types::xcm::v3::MaybeErrorCode),
                    #[codec(index = 33)]
                    QueryPallet {
                        module_name:
                            ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                        response_info: runtime_types::xcm::v3::QueryResponseInfo,
                    },
                    #[codec(index = 34)]
                    ExpectPallet {
                        #[codec(compact)]
                        index: ::core::primitive::u32,
                        name: ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                        module_name:
                            ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                        #[codec(compact)]
                        crate_major: ::core::primitive::u32,
                        #[codec(compact)]
                        min_crate_minor: ::core::primitive::u32,
                    },
                    #[codec(index = 35)]
                    ReportTransactStatus(runtime_types::xcm::v3::QueryResponseInfo),
                    #[codec(index = 36)]
                    ClearTransactStatus,
                    #[codec(index = 37)]
                    UniversalOrigin(runtime_types::xcm::v3::junction::Junction),
                    #[codec(index = 38)]
                    ExportMessage {
                        network: runtime_types::xcm::v3::junction::NetworkId,
                        destination: runtime_types::xcm::v3::junctions::Junctions,
                        xcm: runtime_types::xcm::v3::Xcm,
                    },
                    #[codec(index = 39)]
                    LockAsset {
                        asset: runtime_types::xcm::v3::multiasset::MultiAsset,
                        unlocker: runtime_types::staging_xcm::v3::multilocation::MultiLocation,
                    },
                    #[codec(index = 40)]
                    UnlockAsset {
                        asset: runtime_types::xcm::v3::multiasset::MultiAsset,
                        target: runtime_types::staging_xcm::v3::multilocation::MultiLocation,
                    },
                    #[codec(index = 41)]
                    NoteUnlockable {
                        asset: runtime_types::xcm::v3::multiasset::MultiAsset,
                        owner: runtime_types::staging_xcm::v3::multilocation::MultiLocation,
                    },
                    #[codec(index = 42)]
                    RequestUnlock {
                        asset: runtime_types::xcm::v3::multiasset::MultiAsset,
                        locker: runtime_types::staging_xcm::v3::multilocation::MultiLocation,
                    },
                    #[codec(index = 43)]
                    SetFeesMode {
                        jit_withdraw: ::core::primitive::bool,
                    },
                    #[codec(index = 44)]
                    SetTopic([::core::primitive::u8; 32usize]),
                    #[codec(index = 45)]
                    ClearTopic,
                    #[codec(index = 46)]
                    AliasOrigin(runtime_types::staging_xcm::v3::multilocation::MultiLocation),
                    #[codec(index = 47)]
                    UnpaidExecution {
                        weight_limit: runtime_types::xcm::v3::WeightLimit,
                        check_origin: ::core::option::Option<
                            runtime_types::staging_xcm::v3::multilocation::MultiLocation,
                        >,
                    },
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub enum MaybeErrorCode {
                    #[codec(index = 0)]
                    Success,
                    #[codec(index = 1)]
                    Error(
                        runtime_types::bounded_collections::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                    ),
                    #[codec(index = 2)]
                    TruncatedError(
                        runtime_types::bounded_collections::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                    ),
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub enum OriginKind {
                    #[codec(index = 0)]
                    Native,
                    #[codec(index = 1)]
                    SovereignAccount,
                    #[codec(index = 2)]
                    Superuser,
                    #[codec(index = 3)]
                    Xcm,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct PalletInfo {
                    #[codec(compact)]
                    pub index: ::core::primitive::u32,
                    pub name: runtime_types::bounded_collections::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >,
                    pub module_name: runtime_types::bounded_collections::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >,
                    #[codec(compact)]
                    pub major: ::core::primitive::u32,
                    #[codec(compact)]
                    pub minor: ::core::primitive::u32,
                    #[codec(compact)]
                    pub patch: ::core::primitive::u32,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct QueryResponseInfo {
                    pub destination: runtime_types::staging_xcm::v3::multilocation::MultiLocation,
                    #[codec(compact)]
                    pub query_id: ::core::primitive::u64,
                    pub max_weight: runtime_types::sp_weights::weight_v2::Weight,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub enum Response {
                    #[codec(index = 0)]
                    Null,
                    #[codec(index = 1)]
                    Assets(runtime_types::xcm::v3::multiasset::MultiAssets),
                    #[codec(index = 2)]
                    ExecutionResult(
                        ::core::option::Option<(
                            ::core::primitive::u32,
                            runtime_types::xcm::v3::traits::Error,
                        )>,
                    ),
                    #[codec(index = 3)]
                    Version(::core::primitive::u32),
                    #[codec(index = 4)]
                    PalletsInfo(
                        runtime_types::bounded_collections::bounded_vec::BoundedVec<
                            runtime_types::xcm::v3::PalletInfo,
                        >,
                    ),
                    #[codec(index = 5)]
                    DispatchResult(runtime_types::xcm::v3::MaybeErrorCode),
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub enum WeightLimit {
                    #[codec(index = 0)]
                    Unlimited,
                    #[codec(index = 1)]
                    Limited(runtime_types::sp_weights::weight_v2::Weight),
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct Xcm(
                    pub  ::subxt::ext::subxt_core::alloc::vec::Vec<
                        runtime_types::xcm::v3::Instruction,
                    >,
                );
            }
            pub mod v5 {
                use super::runtime_types;
                pub mod traits {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                        :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                    #[codec(dumb_trait_bound)]
                    #[decode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                    )]
                    #[encode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                    )]
                    pub enum Error {
                        #[codec(index = 0)]
                        Overflow,
                        #[codec(index = 1)]
                        Unimplemented,
                        #[codec(index = 2)]
                        UntrustedReserveLocation,
                        #[codec(index = 3)]
                        UntrustedTeleportLocation,
                        #[codec(index = 4)]
                        LocationFull,
                        #[codec(index = 5)]
                        LocationNotInvertible,
                        #[codec(index = 6)]
                        BadOrigin,
                        #[codec(index = 7)]
                        InvalidLocation,
                        #[codec(index = 8)]
                        AssetNotFound,
                        #[codec(index = 9)]
                        FailedToTransactAsset,
                        #[codec(index = 10)]
                        NotWithdrawable,
                        #[codec(index = 11)]
                        LocationCannotHold,
                        #[codec(index = 12)]
                        ExceedsMaxMessageSize,
                        #[codec(index = 13)]
                        DestinationUnsupported,
                        #[codec(index = 14)]
                        Transport,
                        #[codec(index = 15)]
                        Unroutable,
                        #[codec(index = 16)]
                        UnknownClaim,
                        #[codec(index = 17)]
                        FailedToDecode,
                        #[codec(index = 18)]
                        MaxWeightInvalid,
                        #[codec(index = 19)]
                        NotHoldingFees,
                        #[codec(index = 20)]
                        TooExpensive,
                        #[codec(index = 21)]
                        Trap(::core::primitive::u64),
                        #[codec(index = 22)]
                        ExpectationFalse,
                        #[codec(index = 23)]
                        PalletNotFound,
                        #[codec(index = 24)]
                        NameMismatch,
                        #[codec(index = 25)]
                        VersionIncompatible,
                        #[codec(index = 26)]
                        HoldingWouldOverflow,
                        #[codec(index = 27)]
                        ExportError,
                        #[codec(index = 28)]
                        ReanchorFailed,
                        #[codec(index = 29)]
                        NoDeal,
                        #[codec(index = 30)]
                        FeesNotMet,
                        #[codec(index = 31)]
                        LockError,
                        #[codec(index = 32)]
                        NoPermission,
                        #[codec(index = 33)]
                        Unanchored,
                        #[codec(index = 34)]
                        NotDepositable,
                        #[codec(index = 35)]
                        TooManyAssets,
                        #[codec(index = 36)]
                        UnhandledXcmVersion,
                        #[codec(index = 37)]
                        WeightLimitReached(runtime_types::sp_weights::weight_v2::Weight),
                        #[codec(index = 38)]
                        Barrier,
                        #[codec(index = 39)]
                        WeightNotComputable,
                        #[codec(index = 40)]
                        ExceedsStackLimit,
                    }
                }
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[codec(dumb_trait_bound)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub enum VersionedAssetId {
                #[codec(index = 3)]
                V3(runtime_types::xcm::v3::multiasset::AssetId),
                #[codec(index = 4)]
                V4(runtime_types::staging_xcm::v4::asset::AssetId),
                #[codec(index = 5)]
                V5(runtime_types::staging_xcm::v5::asset::AssetId),
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[codec(dumb_trait_bound)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub enum VersionedAssets {
                #[codec(index = 3)]
                V3(runtime_types::xcm::v3::multiasset::MultiAssets),
                #[codec(index = 4)]
                V4(runtime_types::staging_xcm::v4::asset::Assets),
                #[codec(index = 5)]
                V5(runtime_types::staging_xcm::v5::asset::Assets),
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[codec(dumb_trait_bound)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub enum VersionedLocation {
                #[codec(index = 3)]
                V3(runtime_types::staging_xcm::v3::multilocation::MultiLocation),
                #[codec(index = 4)]
                V4(runtime_types::staging_xcm::v4::location::Location),
                #[codec(index = 5)]
                V5(runtime_types::staging_xcm::v5::location::Location),
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[codec(dumb_trait_bound)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub enum VersionedXcm {
                #[codec(index = 3)]
                V3(runtime_types::xcm::v3::Xcm),
                #[codec(index = 4)]
                V4(runtime_types::staging_xcm::v4::Xcm),
                #[codec(index = 5)]
                V5(runtime_types::staging_xcm::v5::Xcm),
            }
        }
    }
}
