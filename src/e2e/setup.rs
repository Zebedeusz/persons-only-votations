use crate::e2e::config::E2eConfig;
use std::time::Duration;
use subxt::{OnlineClient, PolkadotConfig};
use tokio::time::{sleep, timeout};

pub struct E2eTestSetup {
    pub config: E2eConfig,
    pub people_client: Option<OnlineClient<PolkadotConfig>>,
    pub http_client: reqwest::Client,
}

impl E2eTestSetup {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let config = E2eConfig::from_env();

        let people_client = if config.mock_mode {
            tracing::info!("Mock mode enabled - skipping people chain connection");
            None
        } else {
            Some(Self::connect_to_people_chain(&config).await?)
        };

        let http_client = reqwest::Client::builder()
            .timeout(Duration::from_secs(config.test_timeout_seconds))
            .build()?;

        Ok(Self {
            config,
            people_client,
            http_client,
        })
    }

    async fn connect_to_people_chain(
        config: &E2eConfig,
    ) -> Result<OnlineClient<PolkadotConfig>, Box<dyn std::error::Error + Send + Sync>> {
        let mut retries = 0;

        loop {
            match timeout(
                Duration::from_secs(10),
                OnlineClient::<PolkadotConfig>::from_url(&config.people_chain_endpoint),
            )
            .await
            {
                Ok(Ok(client)) => {
                    tracing::info!(
                        "Connected to people chain at {}",
                        config.people_chain_endpoint
                    );
                    return Ok(client);
                }
                Ok(Err(e)) => {
                    retries += 1;
                    if retries > config.chain_connection_retries {
                        return Err(format!(
                            "Failed to connect to people chain after {} retries: {}",
                            config.chain_connection_retries, config.people_chain_endpoint
                        )
                        .into());
                    }
                    tracing::warn!(
                        "Failed to connect to people chain (attempt {}/{}): {:?}",
                        retries,
                        config.chain_connection_retries,
                        e
                    );
                }
                Err(timeout_err) => {
                    retries += 1;
                    if retries > config.chain_connection_retries {
                        return Err(format!(
                            "Failed to connect to people chain after {} retries: timeout",
                            config.chain_connection_retries
                        )
                        .into());
                    }
                    tracing::warn!(
                        "Failed to connect to people chain (attempt {}/{}): timeout - {:?}",
                        retries,
                        config.chain_connection_retries,
                        timeout_err
                    );
                    sleep(Duration::from_secs(2)).await;
                }
            }
        }
    }

    pub async fn wait_for_server(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let health_endpoint = format!("{}/health", self.config.server_base_url);
        let mut retries = 0;

        loop {
            match self.http_client.get(&health_endpoint).send().await {
                Ok(response) if response.status().is_success() => {
                    tracing::info!("Server is ready at {}", self.config.server_base_url);
                    return Ok(());
                }
                _ => {
                    retries += 1;
                    if retries > 10 {
                        return Err("Server not available after 10 retries".into());
                    }
                    sleep(Duration::from_secs(1)).await;
                }
            }
        }
    }

    pub fn people_client(&self) -> Option<&OnlineClient<PolkadotConfig>> {
        self.people_client.as_ref()
    }

    pub fn is_mock_mode(&self) -> bool {
        self.config.mock_mode
    }
}
