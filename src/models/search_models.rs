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

#[derive(Serialize)]
pub struct SearchHistory {
    pub id: i32,
    pub query: String,
    pub found: bool,
    pub searched_at: Option<chrono::NaiveDateTime>,
}
