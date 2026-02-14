use std::sync::Arc;

use axum::{Json, extract::Query, extract::State};

use crate::AppState;
use crate::error::AppError;
use crate::models::{Search, SearchHistory, SearchQuery};

// Search query for the fst file
pub async fn search_handler(
    State(state): State<Arc<AppState>>,
    Query(params): Query<Search>,
) -> Json<Vec<SearchQuery>> {
    let query = params.q.trim().to_string();

    // Use the search logic from lib.rs with BinaryHeap
    let fst_index = Arc::clone(&state.fst_index);
    let query_for_search = query.clone();
    let result = tokio::task::spawn_blocking(move || {
        crate::perform_search(&fst_index, &query_for_search, 1)
    })
    .await
    .unwrap_or_default();

    // Log search (way to justify setting up database for now x_x)
    let db = state.db.clone();
    let log_query = query.clone();
    let has_results = !result.is_empty();

    tokio::spawn(async move {
        sqlx::query!(
            "INSERT INTO search_history (query, found) VALUES ($1, $2)",
            log_query,
            has_results
        )
        .execute(&db)
        .await
        .ok();
    });

    Json(result)
}

// Search history
pub async fn search_history(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<SearchHistory>>, AppError> {
    let history = sqlx::query_as!(
        SearchHistory,
        "SELECT id, query, found, searched_at FROM search_history ORDER BY searched_at DESC LIMIT 100"
    )
    .fetch_all(&state.db)
    .await?;

    Ok(Json(history))
}
