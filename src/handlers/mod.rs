use axum::Json;

use crate::error::AppError;
use crate::models::HealthResponse;

mod graceful_shutdown;
mod search;
pub use graceful_shutdown::shutdown_signal;
pub use search::search_handler;

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
