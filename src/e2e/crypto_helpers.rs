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
        Self::from_entropy(entropy)
    }

    fn from_entropy(entropy: [u8; 32]) -> Self {
        let secret = BandersnatchVrfVerifiable::new_secret(entropy);
        let member = BandersnatchVrfVerifiable::member_from_secret(&secret);
        Self {
            member_hex: hex::encode(member.0),
            secret,
        }
    }

    /// Rebuilds the keypair of a generated test user.
    /// The generator seeds each user with `[personal_id as u8; 32]`, so the seed is found by search.
    pub fn from_secret_scalar(
        secret_scalar: &str,
        member_hex: &str,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        (0u8..=255)
            .map(|seed| Self::from_entropy([seed; 32]))
            .find(|pair| {
                pair.member_hex == member_hex && pair.secret.scalar.to_string() == secret_scalar
            })
            .ok_or_else(|| format!("No seed in 0..=255 reproduces member key {member_hex}").into())
    }

    pub fn sign_message(
        &self,
        message: &[u8],
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let signature = BandersnatchVrfVerifiable::sign(&self.secret, message)
            .map_err(|_| "Failed to create signature")?;
        Ok(hex::encode(signature))
    }

    /// Ring proof over `message` for the ring given by `ring_members`.
    pub fn generate_ring_proof<I>(
        &self,
        ring_members: I,
        context: &[u8],
        message: &[u8],
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>>
    where
        I: Iterator<Item = verifiable::ring_vrf_impl::EncodedPublicKey> + Clone,
    {
        let member = BandersnatchVrfVerifiable::member_from_secret(&self.secret);
        let commitment = BandersnatchVrfVerifiable::open(&member, ring_members)
            .map_err(|_| "Failed to create ring commitment")?;
        let (proof, _vrf_output) =
            BandersnatchVrfVerifiable::create(commitment, &self.secret, context, message)
                .map_err(|_| "Failed to generate ring proof")?;
        Ok(hex::encode(proof))
    }
}
