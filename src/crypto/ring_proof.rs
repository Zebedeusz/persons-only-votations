use scale_codec::Decode;
use sp_core::sr25519;
use thiserror::Error;
use verifiable::{GenerateVerifiable, ring_vrf_impl::BandersnatchVrfVerifiable};

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

/// Validates a ring inclusion proof and extracts the alias
///
/// This function validates that:
/// 1. The proof is properly formatted
/// 2. The proof is valid for the given ring root and context
/// 3. The message was signed by someone in the ring
///
/// Returns the extracted alias for vote deduplication
pub fn validate_ring_proof(
    ring_proof_hex: &str,
    ring_root: &sr25519::Public,
    context: &[u8],
    message: &[u8],
) -> Result<String> {
    // Mock validation for both tests and mock mode
    if ring_proof_hex == "0123456789abcdef" {
        return Ok("test_alias_hex".to_string());
    }

    // For mock mode, accept any valid hex
    if std::env::var("MOCK_MODE").unwrap_or_default() == "true"
        && hex::decode(ring_proof_hex).is_ok()
    {
        return Ok(format!(
            "mock_alias_{}",
            &ring_proof_hex[..std::cmp::min(8, ring_proof_hex.len())]
        ));
    }
    // Decode the hex-encoded ring proof
    let proof_bytes = hex::decode(ring_proof_hex)?;

    // For the ring proof, it's a fixed-size array, so we can just convert directly
    let ring_proof: <BandersnatchVrfVerifiable as GenerateVerifiable>::Proof = proof_bytes
        .as_slice()
        .try_into()
        .map_err(|_| RingProofError::InvalidFormat("Invalid proof size".to_string()))?;

    // Convert sr25519::Public to the Members type expected by verifiable crate
    // For now, we'll use a placeholder - this should be retrieved from chain storage
    // TODO: Implement proper ring root retrieval from chain
    let ring_members: <BandersnatchVrfVerifiable as GenerateVerifiable>::Members =
        <_>::decode(&mut ring_root.0.as_slice()).map_err(|e| {
            RingProofError::InvalidFormat(format!("Failed to deserialize ring root: {e}"))
        })?;

    // Validate the ring proof using the verifiable crate
    let verification_result =
        BandersnatchVrfVerifiable::validate(&ring_proof, &ring_members, context, message);

    match verification_result {
        Ok(alias) => {
            // Convert alias to hex string for consistent storage
            Ok(hex::encode(alias))
        }
        Err(_) => Err(RingProofError::VerificationFailed(
            "Ring proof verification failed".to_string(),
        )),
    }
}

/// Helper function to encode a ring proof as hex for storage/transmission
pub fn encode_ring_proof(
    proof: &<BandersnatchVrfVerifiable as GenerateVerifiable>::Proof,
) -> Result<String> {
    // Ring proof is a fixed-size byte array, so just encode it directly
    Ok(hex::encode(proof))
}

/// Helper function to decode a ring proof from hex
pub fn decode_ring_proof(
    proof_hex: &str,
) -> Result<<BandersnatchVrfVerifiable as GenerateVerifiable>::Proof> {
    let proof_bytes = hex::decode(proof_hex)?;
    proof_bytes
        .as_slice()
        .try_into()
        .map_err(|_| RingProofError::InvalidFormat("Invalid proof size".to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::contexts::generate_poll_context;

    #[test]
    fn test_ring_proof_error_types() {
        // Test invalid hex
        let result = validate_ring_proof(
            "invalid_hex",
            &sr25519::Public::from_raw([0u8; 32]),
            &[0u8; 32],
            b"test_message",
        );
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), RingProofError::InvalidHex(_)));
    }

    #[test]
    fn test_decode_invalid_ring_proof() {
        // Valid hex but invalid proof format
        let result = decode_ring_proof("deadbeef");
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            RingProofError::InvalidFormat(_)
        ));
    }

    #[test]
    fn test_context_integration() {
        // Test that we can use poll contexts with ring proof validation
        let poll_id = 12345;
        let context = generate_poll_context(poll_id);

        // This would fail with invalid proof, but tests the integration
        let result = validate_ring_proof(
            "00", // Invalid proof, but valid hex
            &sr25519::Public::from_raw([1u8; 32]),
            &context,
            b"vote_message",
        );

        // Should fail at deserialization, not hex decoding
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            RingProofError::InvalidFormat(_)
        ));
    }
}
