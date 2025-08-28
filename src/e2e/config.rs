use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestUser {
    pub id: u32,
    pub secret_scalar: String,
    pub member_bytes: Vec<u8>,
    pub member_hex: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestUsers {
    pub recognized_persons: Vec<TestUser>,
    pub onboarding_queue: Vec<TestUser>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct E2eConfig {
    pub people_chain_endpoint: String,
    pub server_base_url: String,
    pub test_timeout_seconds: u64,
    pub chain_connection_retries: u32,
    pub mock_mode: bool,
    pub test_users: Option<TestUsers>,
}

impl E2eConfig {
    pub fn from_env() -> Self {
        if std::env::var("MOCK_MODE").is_ok() {
            if let Err(_) = dotenvy::from_filename(".env.e2e.mock") {
                let _ = dotenvy::from_filename(".env.e2e");
            }
        } else {
            if let Err(_) = dotenvy::from_filename(".env.e2e") {
                let _ = dotenvy::from_filename(".env.e2e.mock");
            }
        }

        let test_users = Self::load_test_users();
        
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
                .unwrap_or_else(|_| "true".to_string()) // Default to true for E2E tests
                .parse()
                .unwrap_or(true),
            test_users,
        }
    }

    fn load_test_users() -> Option<TestUsers> {
        let config_path = std::env::var("E2E_TEST_USERS_CONFIG")
            .unwrap_or_else(|_| "src/e2e/test_users.json".to_string());
        
        if !Path::new(&config_path).exists() {
            eprintln!("Test users config file not found at: {}", config_path);
            eprintln!("Copy src/e2e/test_users.json.example to src/e2e/test_users.json and fill in the credentials");
            return None;
        }

        match fs::read_to_string(&config_path) {
            Ok(content) => match serde_json::from_str::<TestUsers>(&content) {
                Ok(users) => Some(users),
                Err(e) => {
                    eprintln!("Failed to parse test users config: {}", e);
                    None
                }
            },
            Err(e) => {
                eprintln!("Failed to read test users config file: {}", e);
                None
            }
        }
    }

    pub fn get_recognized_person(&self, index: usize) -> Option<&TestUser> {
        self.test_users
            .as_ref()?
            .recognized_persons
            .get(index)
    }

    pub fn get_onboarding_user(&self, index: usize) -> Option<&TestUser> {
        self.test_users
            .as_ref()?
            .onboarding_queue
            .get(index)
    }
}

impl Default for E2eConfig {
    fn default() -> Self {
        Self {
            people_chain_endpoint: "ws://127.0.0.1:9944".to_string(),
            server_base_url: "http://127.0.0.1:3000".to_string(),
            test_timeout_seconds: 30,
            chain_connection_retries: 5,
            mock_mode: true, // Default to mock mode for tests
            test_users: None,
        }
    }
}
