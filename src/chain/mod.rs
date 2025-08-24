pub mod people_multi;
pub mod runtime;
pub mod subxt_generated_people_apis;

pub use people_multi::{ChainClient, PeopleMultiClient, ChainError, Result};
pub use runtime::{
    IndividualityConfig, PersonalId, PersonRecord, RingRecord, 
    PersonPosition, RuntimeCall, PeopleMultiCall, AccountId
};