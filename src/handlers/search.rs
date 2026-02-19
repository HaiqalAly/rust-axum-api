use std::sync::Arc;

use axum::{Json, extract::Query, extract::State};

use crate::AppState;
use crate::models::{Search, SearchQuery};

// Search query for the fst file
pub async fn search_handler(
    State(state): State<Arc<AppState>>,
    Query(params): Query<Search>,
) -> Json<Vec<SearchQuery>> {
    let query = params.q.trim().to_string();

    // Use the search logic from lib.rs with BinaryHeap
    let fst_index = Arc::clone(&state.fst_index);
    let result = tokio::task::spawn_blocking(move || crate::perform_search(&fst_index, &query, 1))
        .await
        .unwrap_or_default();

    Json(result)
}
