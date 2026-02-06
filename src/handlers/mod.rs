use axum::Json;
use tracing::info;

use crate::error::AppError;
use crate::models::HealthResponse;

// Handler that respond with static string
pub async fn root() -> &'static str {
    "Hello, World!"
}

// Handler for health check
#[tracing::instrument(level = "info", ret)]
pub async fn health() -> Result<Json<HealthResponse>, AppError> {
    let response = HealthResponse {
        status: "Server healthy",
        version: env!("CARGO_PKG_VERSION"),
    };

    Ok(Json(response))
}

// Graceful shutdown
pub async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    info!("Signal received, starting graceful shutdown...");
}
