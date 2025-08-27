use std::sync::Arc;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tracing::{error, info};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use persons_only_votations::{
    api::routes::{AppState, create_router},
    chain::{ChainClient, PeopleMultiClient},
    config::Config,
    services::{poll_service::PollService, vote_service::VoteService},
    storage::repositories::{
        poll_repository::{PollRepository, Repository},
        vote_repository::{InMemoryVoteRepository, VoteRepository},
    },
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::load();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| config.log_level.clone().into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("Starting Persons-only Votations Server");
    info!("Server configuration: {:#?}", config);

    let chain_client = initialize_chain_client(&config).await?;

    let poll_repository: Arc<dyn Repository> = Arc::new(PollRepository::new());
    let vote_repository: Arc<dyn VoteRepository> = Arc::new(InMemoryVoteRepository::new());

    let poll_service = Arc::new(PollService::new(
        poll_repository.clone(),
        chain_client.clone(),
    ));
    let vote_service = Arc::new(VoteService::new(
        vote_repository,
        poll_repository,
        chain_client.clone(),
    ));

    let app_state = AppState {
        poll_service,
        vote_service,
    };

    let app = create_router(app_state).layer(
        ServiceBuilder::new()
            .layer(CorsLayer::permissive())
            .into_inner(),
    );
    let listener = TcpListener::bind(&config.server_address()).await?;
    info!("Server listening on {}", config.server_address());
    info!(
        "Health check available at: http://{}/health",
        config.server_address()
    );
    info!("People Chain endpoint: {}", config.people_chain_endpoint());

    axum::serve(listener, app).await?;

    Ok(())
}

async fn initialize_chain_client(
    config: &Config,
) -> Result<Arc<dyn ChainClient>, Box<dyn std::error::Error>> {
    if config.mock_mode {
        info!("Mock mode enabled - using mock chain client");
        return Ok(Arc::new(PeopleMultiClient::new_mock()) as Arc<dyn ChainClient>);
    }

    let chain_endpoint = config.people_chain_endpoint();
    info!("Connecting to People Chain at: {}", chain_endpoint);

    match PeopleMultiClient::new_with_retry(&chain_endpoint, 3).await {
        Ok(client) => match client.health_check().await {
            Ok(true) => {
                info!("Successfully connected to People Chain");
                Ok(Arc::new(client) as Arc<dyn ChainClient>)
            }
            Ok(false) => {
                error!("People Chain connection failed health check");
                Err("People Chain connection failed health check".into())
            }
            Err(e) => {
                error!("People Chain health check error: {}", e);
                Err(format!("People Chain health check error: {}", e).into())
            }
        },
        Err(e) => {
            error!("Failed to connect to People Chain: {}", e);
            Err(e.into())
        }
    }
}
