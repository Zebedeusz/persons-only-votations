use crate::storage::models::vote::VoteType;
use tfhe::core_crypto::prelude::{CiphertextModulus, LweSize, lwe_ciphertext_add_assign};
use tfhe::shortint::parameters::PARAM_MESSAGE_1_CARRY_7_COMPACT_PK_KS_PBS;
use tfhe::shortint::{Ciphertext, ClientKey, CompactPublicKey};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum HomomorphicError {
    #[error("Invalid ciphertext: {0}")]
    InvalidCiphertext(String),
    #[error("Key serialization failed: {0}")]
    KeySerialization(String),
}

/// Tallies TFHE-encrypted yes/no ballots.
/// Holds the secret key; only the compact public key ever leaves the server.
pub struct TfheVotingSystem {
    client_key: ClientKey,
    public_key: CompactPublicKey,
    lwe_size: LweSize,
    ciphertext_modulus: CiphertextModulus<u64>,
}

impl TfheVotingSystem {
    pub fn new() -> Self {
        let client_key = ClientKey::new(PARAM_MESSAGE_1_CARRY_7_COMPACT_PK_KS_PBS);
        let public_key = CompactPublicKey::new(&client_key);
        // Reference ciphertext fixes the shape every submitted ballot must match
        let reference = public_key.encrypt(0);
        Self {
            client_key,
            public_key,
            lwe_size: reference.ct.lwe_size(),
            ciphertext_modulus: reference.ct.ciphertext_modulus(),
        }
    }

    pub fn public_key_hex(&self) -> Result<String, HomomorphicError> {
        bincode::serialize(&self.public_key)
            .map(hex::encode)
            .map_err(|e| HomomorphicError::KeySerialization(e.to_string()))
    }

    /// Decodes a submitted ballot and checks it has the shape of a ciphertext under this key.
    pub fn decode_ciphertext(&self, hex_str: &str) -> Result<Ciphertext, HomomorphicError> {
        let bytes = hex::decode(hex_str)
            .map_err(|e| HomomorphicError::InvalidCiphertext(format!("invalid hex: {e}")))?;
        let ciphertext: Ciphertext = bincode::deserialize(&bytes)
            .map_err(|e| HomomorphicError::InvalidCiphertext(format!("not a ciphertext: {e}")))?;
        if ciphertext.ct.lwe_size() != self.lwe_size
            || ciphertext.ct.ciphertext_modulus() != self.ciphertext_modulus
        {
            return Err(HomomorphicError::InvalidCiphertext(
                "ciphertext does not match the server key parameters".to_string(),
            ));
        }
        Ok(ciphertext)
    }

    /// Sums the ballots homomorphically and decrypts only the total.
    /// No bootstrapping, so the whole plaintext space is available for the count.
    // ponytail: exact up to 255 yay votes (8-bit plaintext space of 1_CARRY_7); move to tfhe::integer radix if polls grow
    pub fn tally(&self, votes: &[Ciphertext]) -> u64 {
        let Some((first, rest)) = votes.split_first() else {
            return 0;
        };
        let mut sum = first.clone();
        for vote in rest {
            lwe_ciphertext_add_assign(&mut sum.ct, &vote.ct);
        }
        self.client_key.decrypt_message_and_carry(&sum)
    }
}

impl Default for TfheVotingSystem {
    fn default() -> Self {
        Self::new()
    }
}

/// Encrypts a ballot under the server's compact public key (client side), returning the hex to submit.
pub fn encrypt_vote_with_public_key(
    public_key_hex: &str,
    vote: &VoteType,
) -> Result<String, HomomorphicError> {
    let bytes = hex::decode(public_key_hex)
        .map_err(|e| HomomorphicError::KeySerialization(format!("invalid hex: {e}")))?;
    let public_key: CompactPublicKey = bincode::deserialize(&bytes)
        .map_err(|e| HomomorphicError::KeySerialization(format!("not a public key: {e}")))?;
    let value = match vote {
        VoteType::Yay => 1,
        VoteType::Nay => 0,
    };
    bincode::serialize(&public_key.encrypt(value))
        .map(hex::encode)
        .map_err(|e| HomomorphicError::InvalidCiphertext(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::OnceLock;

    fn system() -> &'static TfheVotingSystem {
        static SYSTEM: OnceLock<TfheVotingSystem> = OnceLock::new();
        SYSTEM.get_or_init(TfheVotingSystem::new)
    }

    fn encrypted(votes: &[VoteType]) -> Vec<Ciphertext> {
        let pk = system().public_key_hex().unwrap();
        votes
            .iter()
            .map(|v| {
                system()
                    .decode_ciphertext(&encrypt_vote_with_public_key(&pk, v).unwrap())
                    .unwrap()
            })
            .collect()
    }

    #[test]
    fn tally_counts_yay_votes() {
        let votes = encrypted(&[VoteType::Yay, VoteType::Nay, VoteType::Yay]);
        assert_eq!(system().tally(&votes), 2);
    }

    #[test]
    fn tally_of_nothing_is_zero() {
        assert_eq!(system().tally(&[]), 0);
    }

    #[test]
    fn tally_does_not_wrap_at_message_modulus() {
        let votes = encrypted(&vec![VoteType::Yay; 20]);
        assert_eq!(system().tally(&votes), 20);
    }

    #[test]
    fn published_key_is_not_the_secret_key() {
        let bytes = hex::decode(system().public_key_hex().unwrap()).unwrap();
        assert!(bincode::deserialize::<ClientKey>(&bytes).is_err());
    }

    #[test]
    fn malformed_ballots_are_rejected() {
        assert!(system().decode_ciphertext("not hex").is_err());
        assert!(system().decode_ciphertext("deadbeef").is_err());
    }
}
