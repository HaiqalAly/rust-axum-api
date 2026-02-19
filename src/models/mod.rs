use serde::Serialize;

mod search_models;
pub use search_models::{Search, SearchQuery};

#[derive(Serialize, Debug)]
pub struct HealthResponse {
    pub status: &'static str,
    pub version: &'static str,
}
