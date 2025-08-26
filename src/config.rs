use clap::Parser;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[derive(Parser, Debug, Clone, Serialize, Deserialize)]
#[command(name = "persons-only-votations")]
#[command(about = "A voting system that requires proof of personhood")]
pub struct Config {
    #[arg(long, env = "SERVER_HOST", default_value = "127.0.0.1")]
    pub host: String,

    #[arg(long, env = "SERVER_PORT", default_value = "3000")]
    pub port: u16,

    #[arg(long, env = "PEOPLE_CHAIN_HOST", default_value = "127.0.0.1")]
    pub people_chain_host: String,

    #[arg(long, env = "PEOPLE_CHAIN_PORT")]
    pub people_chain_port: u16,

    #[arg(long, env = "PEOPLE_CHAIN_PROTOCOL", default_value = "ws")]
    pub people_chain_protocol: String,

    #[arg(long, env = "LOG_LEVEL", default_value = "info")]
    pub log_level: String,

    #[arg(long, env = "MOCK_MODE", default_value = "false")]
    pub mock_mode: bool,
}

impl Config {
    pub fn server_address(&self) -> SocketAddr {
        format!("{}:{}", self.host, self.port)
            .parse()
            .expect("Invalid server address")
    }

    pub fn people_chain_endpoint(&self) -> String {
        format!(
            "{}://{}:{}",
            self.people_chain_protocol, self.people_chain_host, self.people_chain_port
        )
    }

    pub fn load() -> Self {
        // Load from .env file if it exists
        if let Err(e) = dotenvy::dotenv() {
            tracing::debug!("No .env file found or error loading it: {}", e);
        }

        Config::parse()
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 3000,
            people_chain_host: "127.0.0.1".to_string(),
            people_chain_port: 9944, // Default Substrate WebSocket port
            people_chain_protocol: "ws".to_string(),
            log_level: "info".to_string(),
            mock_mode: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_server_address() {
        let config = Config {
            host: "0.0.0.0".to_string(),
            port: 8080,
            ..Default::default()
        };
        assert_eq!(config.server_address().to_string(), "0.0.0.0:8080");
    }

    #[test]
    fn test_people_chain_endpoint() {
        let config = Config {
            people_chain_protocol: "wss".to_string(),
            people_chain_host: "people-chain.example.com".to_string(),
            people_chain_port: 443,
            ..Default::default()
        };
        assert_eq!(
            config.people_chain_endpoint(),
            "wss://people-chain.example.com:443"
        );
    }

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.host, "127.0.0.1");
        assert_eq!(config.port, 3000);
        assert_eq!(config.people_chain_host, "127.0.0.1");
        assert_eq!(config.people_chain_port, 9944);
        assert_eq!(config.people_chain_protocol, "ws");
        assert_eq!(config.people_chain_endpoint(), "ws://127.0.0.1:9944");
        assert_eq!(config.log_level, "info");
        assert_eq!(config.mock_mode, false);
    }
}
