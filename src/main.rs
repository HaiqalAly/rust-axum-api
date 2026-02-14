use std::sync::Arc;
use std::time::Duration;

use axum::{Router, http::StatusCode, routing::get};
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;
use tower_http::{timeout::TimeoutLayer, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use rust_api_axum::{AppState, handlers, load_fst};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!(
                    "{}=debug,tower_http=debug,axum::rejection=trace",
                    env!("CARGO_CRATE_NAME")
                )
                .into()
            }),
        )
        .with(tracing_subscriber::fmt::layer().without_time())
        .init();

    // Database connection init
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&db_url)
        .await?;

    // Load FST dictionary
    let fst_map = load_fst()?;
    tracing::info!("FST dictionary loaded successfully");

    let state = Arc::new(AppState {
        db: pool,
        fst_index: fst_map,
    });

    // Build the app route
    let app = Router::new()
        .route("/", get(handlers::root))
        .route("/health", get(handlers::health))
        .route("/search", get(handlers::search_handler))
        .route("/history", get(handlers::search_history))
        .layer(TraceLayer::new_for_http())
        .layer(TimeoutLayer::with_status_code(
            StatusCode::REQUEST_TIMEOUT,
            Duration::from_secs(10),
        ))
        .with_state(state);

    // Run app on port 8080
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();

    tracing::info!("Server running on: //{}", listener.local_addr().unwrap());
    axum::serve(listener, app)
        .with_graceful_shutdown(handlers::shutdown_signal())
        .await?;

    Ok(())
}
