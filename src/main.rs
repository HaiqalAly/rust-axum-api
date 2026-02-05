use std::time::Duration;

use axum::{Router, routing::get, http::StatusCode};
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use tower_http::{trace::TraceLayer, timeout::TimeoutLayer};

mod models;
mod handlers;
mod error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    // Build the app route
    let app = Router::new()
        .route("/", get(handlers::root))
        .route("/health", get(handlers::health))
        .layer(TraceLayer::new_for_http())
        .layer(TimeoutLayer::with_status_code(StatusCode::REQUEST_TIMEOUT, Duration::from_secs(10)));

    // Run app on port 8080
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    axum::serve(listener, app)
        .with_graceful_shutdown(handlers::shutdown_signal())
        .await?;

    Ok(())
}