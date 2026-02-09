use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
pub struct HealthResponse {
    pub status: &'static str,
    pub version: &'static str,
}

#[derive(Deserialize)]
pub struct Search {
    pub q: String,
}

#[derive(Serialize)]
pub struct SearchQuery {
    pub found: String,
    pub score: String,
    pub exist: bool
}