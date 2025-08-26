use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct E2eConfig {
    pub people_chain_endpoint: String,
    pub server_base_url: String,
    pub test_timeout_seconds: u64,
    pub chain_connection_retries: u32,
    pub mock_mode: bool,
}

impl E2eConfig {
    pub fn from_env() -> Self {
        Self {
            people_chain_endpoint: std::env::var("E2E_PEOPLE_CHAIN_ENDPOINT")
                .unwrap_or_else(|_| "ws://127.0.0.1:9944".to_string()),
            server_base_url: std::env::var("E2E_SERVER_BASE_URL")
                .unwrap_or_else(|_| "http://127.0.0.1:3000".to_string()),
            test_timeout_seconds: std::env::var("E2E_TEST_TIMEOUT_SECONDS")
                .unwrap_or_else(|_| "30".to_string())
                .parse()
                .unwrap_or(30),
            chain_connection_retries: std::env::var("E2E_CHAIN_CONNECTION_RETRIES")
                .unwrap_or_else(|_| "5".to_string())
                .parse()
                .unwrap_or(5),
            mock_mode: std::env::var("MOCK_MODE")
                .unwrap_or_else(|_| "false".to_string())
                .parse()
                .unwrap_or(false),
        }
    }
}

impl Default for E2eConfig {
    fn default() -> Self {
        Self {
            people_chain_endpoint: "ws://127.0.0.1:9944".to_string(),
            server_base_url: "http://127.0.0.1:3000".to_string(),
            test_timeout_seconds: 30,
            chain_connection_retries: 5,
            mock_mode: false,
        }
    }
}