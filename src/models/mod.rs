use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct HealthResponse {
    pub status: &'static str,
    pub version: &'static str,
}
