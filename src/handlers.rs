use axum::Json;
use tracing::info;

use crate::models::HealthResponse;
use crate::error::AppError;

// Handler that respond with static string
pub async fn root() -> &'static str {
    "Hello, World!"
}

// Handler for health check
pub async fn health() -> Result<Json<HealthResponse>, AppError> {
    info!("Everything's fine!");
    
    let response = HealthResponse {
        status: "Server healthy",
        version: env!("CARGO_PKG_VERSION"),
    };

    Ok(Json(response))
}