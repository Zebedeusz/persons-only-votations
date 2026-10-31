use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use persons_only_votations::{
    api::routes::{AppState, create_router},
    chain::{ChainClient, PeopleMultiClient},
    config::Config,
    services::{poll_service::PollService, vote_service::VoteService},
    storage::repositories::{poll_repository::PollRepository, vote_repository::VoteRepository},
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

    let chain_endpoint = config.people_chain_endpoint();
    info!("Connecting to People Chain at: {}", chain_endpoint);
    let chain_client: Arc<dyn ChainClient> =
        Arc::new(PeopleMultiClient::new_with_retry(&chain_endpoint, 3).await?);
    info!("Connected to People Chain");

    let poll_repository = Arc::new(PollRepository::default());
    let vote_repository = Arc::new(VoteRepository::default());
    let state = AppState {
        poll_service: PollService::new(poll_repository.clone(), chain_client.clone()),
        vote_service: VoteService::new(vote_repository, poll_repository, chain_client),
    };

    let app = create_router(state).layer(CorsLayer::permissive());
    let listener = TcpListener::bind(&config.server_address()).await?;
    info!("Server listening on {}", config.server_address());
    info!(
        "Health check available at: http://{}/health",
        config.server_address()
    );

    axum::serve(listener, app).await?;
    Ok(())
}
