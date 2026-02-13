use std::collections::BinaryHeap;
use std::fs;
use std::sync::Arc;

use memmap2::Mmap;
use fst::{Map, automaton::Levenshtein, IntoStreamer, Streamer};

pub mod error;
pub mod handlers;
pub mod models;

use models::SearchQuery;

pub struct AppState {
    pub db: sqlx::PgPool,
    pub fst_index: Arc<Map<Mmap>>,
}

pub fn load_fst() -> Result<Arc<Map<Mmap>>, Box<dyn std::error::Error>> {
    // Loading the fst
    let data = fs::File::open("data/dict.fst")?;
    let mmap = unsafe {
        Mmap::map(&data)?
    };
    let map = Map::new(mmap)?;
    let fst_map = Arc::new(map);

    Ok(fst_map)
}

#[derive(PartialEq, Eq)]
struct SearchResult {
    key: Vec<u8>,
    value: u64,
    is_exact: bool,
}

impl PartialOrd for SearchResult {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SearchResult {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // In a BinaryHeap (MaxHeap), pop() removes the Greatest item.
        // We want to KEEP the best items and pop the worst.
        
        // 1. Exact Match: Non-exact is "Worse" (Greater)
        other
            .is_exact
            .cmp(&self.is_exact)
            // 2. Score: Lower score is "Worse" (Greater)
            .then_with(|| other.value.cmp(&self.value))
            // 3. Alphabetically
            .then_with(|| self.key.cmp(&other.key))
    }
}

/// Perform FST search using BinaryHeap for efficient top-10 filtering
pub fn perform_search(
    fst_map: &Arc<Map<Mmap>>,
    query: &str,
    max_distance: u32,
) -> Vec<SearchQuery> {
    let query_lower = query.to_lowercase();
    let query_lower = query_lower.trim();

    // Handle automaton creation
    let lev = match Levenshtein::new(query_lower, max_distance) {
        Ok(leven) => leven,
        Err(_) => return vec![],
    };

    let mut heap = BinaryHeap::with_capacity(11);
    let mut stream = fst_map.search(lev).into_stream();
    let target_bytes = query_lower.as_bytes();

    // Only keep top 10 and pop the worst one if hit 11
    while let Some((key_bytes, value)) = stream.next() {
        let is_exact = key_bytes == target_bytes;

        let res = SearchResult {
            key: key_bytes.to_vec(),
            value,
            is_exact,
        };

        heap.push(res);

        if heap.len() > 10 {
            heap.pop();
        }
    }
    
    let top_10: Vec<_> = heap.into_sorted_vec();

    // Convert to SearchQuery results
    top_10
        .into_iter()
        .map(|result| SearchQuery {
            found: String::from_utf8_lossy(&result.key).to_string(),
            score: result.value.to_string(),
            exist: true,
        })
        .collect()
}