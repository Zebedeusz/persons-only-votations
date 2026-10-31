pub mod commitments;
pub mod contexts;
pub mod homomorphic;
pub mod ring_proof;

pub fn hex_to_bytes32(hex_str: &str) -> Result<[u8; 32], hex::FromHexError> {
    let bytes = hex::decode(hex_str)?;
    bytes
        .as_slice()
        .try_into()
        .map_err(|_| hex::FromHexError::InvalidStringLength)
}
