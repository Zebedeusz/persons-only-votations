use crate::chain::runtime::RingRoot;
use scale_codec::Decode;
use verifiable::ring_vrf_impl::{MembersCommitment, MembersSet};

/// Creates a mock ring root for testing purposes only
/// This should never be used in production code
#[cfg(test)]
pub fn create_mock_ring_root() -> RingRoot {
    let encoded_commitment = vec![0u8; 384]; // Correct size
    let encoded_intermediate = vec![0u8; 432]; // Correct size

    let members_commitment = match MembersCommitment::decode(&mut &encoded_commitment[..]) {
        Ok(commitment) => commitment,
        Err(_) => unsafe { std::mem::zeroed() }, // Only safe in test mode
    };

    let members_set = match MembersSet::decode(&mut &encoded_intermediate[..]) {
        Ok(set) => set,
        Err(_) => unsafe { std::mem::zeroed() }, // Only safe in test mode
    };

    RingRoot {
        root: members_commitment,
        revision: 0,
        intermediate: members_set,
    }
}
