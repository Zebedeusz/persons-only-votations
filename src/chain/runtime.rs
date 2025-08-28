use scale_codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_core::{H256, sr25519};
use sp_runtime::traits::BlakeTwo256;
use subxt::Config;

// Basic runtime types
pub type AccountId = sp_runtime::AccountId32;
pub type Signature = sp_runtime::MultiSignature;
pub type BlockNumber = u32;
pub type Header = sp_runtime::generic::Header<BlockNumber, BlakeTwo256>;
pub type Block = sp_runtime::generic::Block<Header, UncheckedExtrinsic>;

// People Multi pallet types
pub type PersonalId = u64;

#[derive(Clone, Debug, Decode, Encode, TypeInfo, PartialEq, Eq)]
pub struct PersonRecord<AccountId> {
    pub key: sr25519::Public,
    pub account: Option<AccountId>,
    pub position: PersonPosition,
}

#[derive(Clone, Debug, Decode, Encode, TypeInfo, PartialEq, Eq)]
pub enum PersonPosition {
    Onboarding,
    Included(u16), // Ring index
}

impl PersonPosition {
    pub fn ring_index(&self) -> Option<u16> {
        match self {
            PersonPosition::Onboarding => None,
            PersonPosition::Included(index) => Some(*index),
        }
    }
}

// Runtime call enum - simplified
#[derive(Clone, Debug, Decode, Encode, TypeInfo, PartialEq, Eq)]
pub enum RuntimeCall {
    PeopleMulti(PeopleMultiCall),
}

#[derive(Clone, Debug, Decode, Encode, TypeInfo, PartialEq, Eq)]
pub enum PeopleMultiCall {
    SetPersonalIdAccount { account: AccountId },
    UnsetPersonalIdAccount,
}

pub type UncheckedExtrinsic =
    sp_runtime::generic::UncheckedExtrinsic<AccountId, RuntimeCall, Signature, ()>;

// Custom config for the runtime
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct IndividualityConfig;

impl Config for IndividualityConfig {
    type Hash = H256;
    type AccountId = AccountId;
    type Address = sp_runtime::MultiAddress<Self::AccountId, u32>;
    type Signature = Signature;
    type Hasher = BlakeTwo256;
    type Header = Header;
    type ExtrinsicParams = subxt::config::substrate::SubstrateExtrinsicParams<Self>;
    type AssetId = ();
}
