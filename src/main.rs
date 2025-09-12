use std::net::SocketAddr;
use tokio;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use rust_compress_api::{
    AppConfig, AppState,
    api::create_router,
    core::database::{create_pool, init_database},
};

#[tokio::main]
async fn main() {
    // Load .env file first
    dotenvy::dotenv().ok();

    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rust_compress_api=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    let config = AppConfig::from_env().expect("Failed to load configuration");

    info!("Starting server with config: {:?}", config);
    info!("Database URL: {}", config.database.url);

    // Create database connection pool
    let db_pool = create_pool(&config.database.url)
        .await
        .expect("Failed to create database pool");

    // Initialize database
    init_database(&db_pool)
        .await
        .expect("Failed to initialize database");

    info!("Database connected and initialized");

    // Create application state
    let state = AppState::new(db_pool);

    // Build our application with routes
    let app = create_router().with_state(state);

    // Run our app with hyper, listening on the configured host and port
    let addr = SocketAddr::from(([0, 0, 0, 0], config.server.port));
    info!("Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
