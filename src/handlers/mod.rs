use std::sync::Arc;

use axum::{Json, extract::Query, extract::State};
use tracing::info;

use crate::AppState;
use crate::error::AppError;
use crate::models::{HealthResponse, Search, SearchQuery};

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

// Search query for the fst file
pub async fn search_handler(
    State(state): State<Arc<AppState>>,
    Query(params): Query<Search>,
) -> Json<Vec<SearchQuery>> {
    let query = params.q.to_lowercase();
    let query = query.trim();

    let mut result = Vec::new();

    if let Some(score_value) = state.fst_index.get(query.as_bytes()) {
        result.push(SearchQuery {
            found: query.to_string(),
            score: score_value.to_string(),
            exist: true,
        });
    }

    Json(result)
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
