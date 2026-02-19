use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Search {
    pub q: String,
}

#[derive(Serialize)]
pub struct SearchQuery {
    pub found: String,
    pub score: String,
    pub exist: bool,
}
