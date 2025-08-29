use super::{bytes32_to_hex, hex_to_bytes32};
use blake2::{Blake2b512, Digest};

pub fn generate_poll_context(poll_id: u32) -> [u8; 32] {
    let context_string = format!("PERSONS_POLL_{poll_id}");
    let mut hasher = Blake2b512::new();
    hasher.update(context_string.as_bytes());
    let result = hasher.finalize();

    let mut context = [0u8; 32];
    context.copy_from_slice(&result[..32]);
    context
}

pub fn context_to_string(context: &[u8; 32]) -> String {
    bytes32_to_hex(context)
}

pub fn string_to_context(hex_str: &str) -> Result<[u8; 32], hex::FromHexError> {
    hex_to_bytes32(hex_str)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_poll_context() {
        let context1 = generate_poll_context(123);
        let context2 = generate_poll_context(123);
        let context3 = generate_poll_context(124);

        assert_eq!(context1, context2);
        assert_ne!(context1, context3);
    }

    #[test]
    fn test_context_conversion() {
        let original = generate_poll_context(456);
        let hex_str = context_to_string(&original);
        let recovered = string_to_context(&hex_str).unwrap();

        assert_eq!(original, recovered);
    }
}
