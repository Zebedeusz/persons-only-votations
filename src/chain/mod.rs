pub mod people_multi;
pub mod runtime;
pub mod subxt_generated_people_apis;

pub use people_multi::{ChainClient, ChainError, PeopleMultiClient, Result};
pub use runtime::{
    AccountId, IndividualityConfig, PeopleMultiCall, PersonPosition, PersonRecord, PersonalId,
};
