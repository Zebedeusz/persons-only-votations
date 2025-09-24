use hex;

pub mod commitments;
pub mod contexts;
pub mod homomorphic;
pub mod ring_proof;

pub fn bytes32_to_hex(bytes: &[u8; 32]) -> String {
    hex::encode(bytes)
}

pub fn hex_to_bytes32(hex_str: &str) -> Result<[u8; 32], hex::FromHexError> {
    let bytes = hex::decode(hex_str)?;
    if bytes.len() != 32 {
        return Err(hex::FromHexError::InvalidStringLength);
    }
    let mut array = [0u8; 32];
    array.copy_from_slice(&bytes);
    Ok(array)
}

#[cfg(test)]
pub mod test_helpers;
