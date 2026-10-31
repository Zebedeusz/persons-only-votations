use crate::chain::PeopleMultiClient;
use crate::e2e::config::E2eConfig;
use std::time::Duration;
use tokio::time::sleep;

pub struct E2eTestSetup {
    pub config: E2eConfig,
    pub chain: PeopleMultiClient,
    pub http_client: reqwest::Client,
}

impl E2eTestSetup {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let config = E2eConfig::from_env();
        let chain = PeopleMultiClient::new_with_retry(
            &config.people_chain_endpoint,
            config.chain_connection_retries,
        )
        .await?;
        tracing::info!(
            "Connected to people chain at {}",
            config.people_chain_endpoint
        );

        let http_client = reqwest::Client::builder()
            .timeout(Duration::from_secs(config.test_timeout_seconds))
            .build()?;

        Ok(Self {
            config,
            chain,
            http_client,
        })
    }

    pub async fn wait_for_server(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let health_endpoint = format!("{}/health", self.config.server_base_url);
        tracing::info!("Waiting for server at {}", health_endpoint);

        for attempt in 1..=10 {
            match self.http_client.get(&health_endpoint).send().await {
                Ok(response) if response.status().is_success() => return Ok(()),
                Ok(response) => tracing::warn!(
                    "Server health check failed (attempt {attempt}): HTTP {}",
                    response.status()
                ),
                Err(e) => tracing::warn!("Server health check error (attempt {attempt}): {e}"),
            }
            sleep(Duration::from_secs(1)).await;
        }
        Err(format!("Server not available at {health_endpoint}").into())
    }
}
