use serde::Deserialize;
use std::fs;

#[derive(Debug, Clone, Deserialize)]
pub struct TestUser {
    pub secret_scalar: String,
    pub member_hex: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TestUsers {
    pub recognized_persons: Vec<TestUser>,
    pub onboarding_queue: Vec<TestUser>,
}

#[derive(Debug, Clone)]
pub struct E2eConfig {
    pub people_chain_endpoint: String,
    pub server_base_url: String,
    pub test_timeout_seconds: u64,
    pub chain_connection_retries: u32,
    pub test_users: Option<TestUsers>,
}

impl E2eConfig {
    pub fn from_env() -> Self {
        let _ = dotenvy::from_filename(".env.e2e");
        let var =
            |name: &str, default: &str| std::env::var(name).unwrap_or_else(|_| default.to_string());

        Self {
            people_chain_endpoint: var("E2E_PEOPLE_CHAIN_ENDPOINT", "ws://127.0.0.1:9944"),
            server_base_url: var("E2E_SERVER_BASE_URL", "http://127.0.0.1:3000"),
            test_timeout_seconds: var("E2E_TEST_TIMEOUT_SECONDS", "30").parse().unwrap_or(30),
            chain_connection_retries: var("E2E_CHAIN_CONNECTION_RETRIES", "5")
                .parse()
                .unwrap_or(5),
            test_users: Self::load_test_users(&var(
                "E2E_TEST_USERS_CONFIG",
                "src/e2e/test_users.json",
            )),
        }
    }

    fn load_test_users(path: &str) -> Option<TestUsers> {
        let content = fs::read_to_string(path)
            .map_err(|e| {
                tracing::warn!(
                    "Test users config not readable at {path}: {e}. Generate it with devtools/scripts/generate-test-users.sh"
                )
            })
            .ok()?;
        serde_json::from_str(&content)
            .map_err(|e| tracing::error!("Failed to parse test users config: {e}"))
            .ok()
    }

    pub fn get_recognized_person(&self, index: usize) -> Option<&TestUser> {
        self.test_users.as_ref()?.recognized_persons.get(index)
    }

    pub fn get_onboarding_user(&self, index: usize) -> Option<&TestUser> {
        self.test_users.as_ref()?.onboarding_queue.get(index)
    }
}
