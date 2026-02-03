use axum::{Json, Router, routing::get};
use serde::Serialize;
use tokio::net::TcpListener;

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
    version: &'static str,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Build the app route
    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health));

    // Run app on port 8080
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

// Handler that respond with static string
async fn root() -> &'static str {
    "Hello, World!"
}

// Handler for health check
async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "Server healthy",
        version: env!("CARGO_PKG_VERSION"),
    })
}
