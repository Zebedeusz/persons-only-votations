use verifiable::{GenerateVerifiable, ring_vrf_impl::BandersnatchVrfVerifiable};

pub struct TestKeyPair {
    pub member_hex: String,
    pub secret: <BandersnatchVrfVerifiable as GenerateVerifiable>::Secret,
}

impl TestKeyPair {
    pub fn generate() -> Self {
        let mut entropy = [0u8; 32];
        for byte in entropy.iter_mut() {
            *byte = fastrand::u8(..);
        }
        let secret = BandersnatchVrfVerifiable::new_secret(entropy);
        let member = BandersnatchVrfVerifiable::member_from_secret(&secret);
        let member_hex = hex::encode(member.0);

        Self { member_hex, secret }
    }

    pub fn from_secret_scalar(
        secret_scalar: &str,
        member_hex: &str,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        // The chain script creates entropy as [personal_id as u8; 32]
        // We need to reverse-engineer which personal_id was used

        // Try different personal_id values to find the one that produces the expected member_hex
        for personal_id in 0u8..=255 {
            let entropy = [personal_id; 32];
            let secret = BandersnatchVrfVerifiable::new_secret(entropy);
            let derived_member = BandersnatchVrfVerifiable::member_from_secret(&secret);
            let derived_hex = hex::encode(derived_member.0);

            if derived_hex == member_hex {
                // Verify the scalar matches too
                if secret.scalar.to_string() == secret_scalar {
                    return Ok(Self {
                        member_hex: member_hex.to_string(),
                        secret,
                    });
                }
            }
        }

        // If we can't find a matching personal_id, fall back to the member_bytes as entropy
        // This allows for flexibility with different key generation methods
        let member_bytes = hex::decode(member_hex)?;
        if member_bytes.len() != 32 {
            return Err("Member hex must decode to 32 bytes".into());
        }
        let mut entropy = [0u8; 32];
        entropy.copy_from_slice(&member_bytes);

        let secret = BandersnatchVrfVerifiable::new_secret(entropy);
        let derived_member = BandersnatchVrfVerifiable::member_from_secret(&secret);
        let derived_hex = hex::encode(derived_member.0);

        if derived_hex != member_hex {
            return Err(format!("Generated secret doesn't produce expected member key. Expected: {member_hex}, Got: {derived_hex}").into());
        }

        Ok(Self {
            member_hex: member_hex.to_string(),
            secret,
        })
    }

    pub fn sign_message(
        &self,
        message: &[u8],
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let signature = BandersnatchVrfVerifiable::sign(&self.secret, message)
            .map_err(|_| "Failed to create signature")?;
        Ok(hex::encode(signature))
    }

    /// Generate a ring proof for the given ring members, context, and message
    pub fn generate_ring_proof<I>(
        &self,
        ring_members: I,
        context: &[u8],
        message: &[u8],
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>>
    where
        I: Iterator<Item = verifiable::ring_vrf_impl::EncodedPublicKey> + Clone,
    {
        // First get the public member from the secret
        let member = BandersnatchVrfVerifiable::member_from_secret(&self.secret);

        // Create the commitment using the open method
        let commitment = BandersnatchVrfVerifiable::open(&member, ring_members.clone())
            .map_err(|_| "Failed to create ring commitment")?;

        // Generate the ring proof using the create method
        let (proof, _vrf_output) =
            BandersnatchVrfVerifiable::create(commitment, &self.secret, context, message)
                .map_err(|_| "Failed to generate ring proof")?;

        Ok(hex::encode(proof))
    }
}

pub fn create_poll_signature_message(
    poll_name: &str,
    poll_description: &str,
    ends_at: i64,
) -> Vec<u8> {
    use crate::storage::models::poll::PollData;

    let poll_data = PollData {
        name: poll_name.to_string(),
        description: poll_description.to_string(),
        ends_at,
    };

    serde_json::to_string(&poll_data)
        .unwrap()
        .as_bytes()
        .to_vec()
}

pub fn create_vote_signature_message(
    poll_id: u32,
    _voter_member_hex: &str,
    vote_type: &str,
    poll_context: &str,
) -> Vec<u8> {
    let vote_data = serde_json::json!({
        "poll_id": poll_id,
        "vote_type": vote_type,
        "context": poll_context
    });

    serde_json::to_string(&vote_data)
        .unwrap()
        .as_bytes()
        .to_vec()
}

pub fn create_commit_signature_message(
    poll_id: u32,
    vote_commitment: &str,
    poll_context: &str,
) -> Vec<u8> {
    let commit_data = serde_json::json!({
        "poll_id": poll_id,
        "vote_commitment": vote_commitment,
        "context": poll_context
    });

    let message_json = serde_json::to_string(&commit_data).unwrap();
    message_json.as_bytes().to_vec()
}
