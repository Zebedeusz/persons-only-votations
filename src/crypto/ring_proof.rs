use thiserror::Error;
use verifiable::{
    GenerateVerifiable,
    ring_vrf_impl::{BandersnatchVrfVerifiable, MembersCommitment},
};

#[derive(Error, Debug)]
pub enum RingProofError {
    #[error("Invalid ring proof format: {0}")]
    InvalidFormat(String),
    #[error("Ring proof verification failed: {0}")]
    VerificationFailed(String),
    #[error("Invalid hex encoding: {0}")]
    InvalidHex(#[from] hex::FromHexError),
}

pub type Result<T> = std::result::Result<T, RingProofError>;

type Proof = <BandersnatchVrfVerifiable as GenerateVerifiable>::Proof;

/// Validates a ring inclusion proof against the ring's commitment and returns
/// the voter's alias, which is stable per (voter, context) and used for deduplication.
pub fn validate_ring_proof(
    ring_proof_hex: &str,
    ring_root: &MembersCommitment,
    context: &[u8],
    message: &[u8],
) -> Result<String> {
    let proof_bytes = hex::decode(ring_proof_hex)?;
    let proof: Proof = proof_bytes.as_slice().try_into().map_err(|_| {
        RingProofError::InvalidFormat(format!(
            "Invalid proof size: expected {} bytes, got {}",
            std::mem::size_of::<Proof>(),
            proof_bytes.len()
        ))
    })?;

    BandersnatchVrfVerifiable::validate(&proof, ring_root, context, message)
        .map(hex::encode)
        .map_err(|e| {
            RingProofError::VerificationFailed(format!(
                "Ring proof cryptographic verification failed: {e:?}"
            ))
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Commitment of an empty ring, enough for the error paths that never reach validation.
    fn empty_ring() -> MembersCommitment {
        BandersnatchVrfVerifiable::finish_members(BandersnatchVrfVerifiable::start_members())
    }

    #[test]
    fn rejects_non_hex_proof() {
        let result = validate_ring_proof("invalid_hex", &empty_ring(), &[0u8; 32], b"msg");
        assert!(matches!(result.unwrap_err(), RingProofError::InvalidHex(_)));
    }

    #[test]
    fn rejects_wrong_size_proof() {
        let result = validate_ring_proof("00", &empty_ring(), &[0u8; 32], b"msg");
        assert!(matches!(
            result.unwrap_err(),
            RingProofError::InvalidFormat(_)
        ));
    }
}
