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
        _secret_scalar: &str,
        member_hex: &str,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let entropy = match member_hex {
            "5e465beb01dbafe160ce8216047f2155dd0569f058afd52dcea601025a8d161d" => [0u8; 32], // User 1
            "3f9828dfc4e8fdc12ed5f7e6f380b5d30ff328613e6d3e4a27e24ca3e64474b9" => [1u8; 32], // User 2
            "7539d69a545904e39cabc487a63ae3b9f863afefede3f49c786a5047c5189cdb" => [2u8; 32], // User 3
            _ => {
                let member_bytes = hex::decode(member_hex)?;
                if member_bytes.len() != 32 {
                    return Err("Member hex must decode to 32 bytes".into());
                }
                let mut entropy = [0u8; 32];
                entropy.copy_from_slice(&member_bytes);
                entropy
            }
        };

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
