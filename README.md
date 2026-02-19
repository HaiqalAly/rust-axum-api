# Rust API with Axum

A high-performance REST API built with Axum for searching words in an FST (Finite State Transducer) dictionary with fuzzy matching capabilities. This project connects the [FST Experiment](https://github.com/HaiqalAly/rust-fst-exp) to the web, providing a fast and efficient word search service.

## Features

- **Health Monitoring**: Health check endpoint with version information
- **Fast Search**: Lightning-fast word search using FST with memory-mapped files
- **Fuzzy Matching**: Levenshtein automaton-based fuzzy search (edit distance: 1)
- **Smart Ranking**: Top 10 results with intelligent prioritization:
  - 🥇 Exact matches ranked first
  - 🥈 Higher scores ranked second
  - 🥉 Alphabetical ordering as tiebreaker
- **Observability**: Request tracing and structured logging with `tracing`
- **Reliability**: Request timeout (10s) and graceful shutdown support
- **Zero Dependencies**: No database required - pure in-memory FST search

## Why No Database?

This API was originally designed with PostgreSQL for search history logging, but the database added unnecessary latency and complexity for a simple search API. The FST data structure provides:

- **Instant Response Times**: Memory-mapped FST operations complete in microseconds
- **Simplified Architecture**: No connection pooling, migrations, or database maintenance
- **Reduced Infrastructure**: No Docker containers or external services to manage
- **Better Resource Efficiency**: Lower memory footprint and no database connection overhead

## Prerequisites

Before you begin, ensure you have the following installed:

- **Rust**: Version 1.75 or higher (Rust edition 2024)
- **FST Dictionary**: Place your dictionary file at `data/dict.fst`

## Quick Start

### 1. Dictionary Setup

Place your FST dictionary file at:
```
data/dict.fst
```

### 2. Run the Server

Start the API server:
```bash
cargo run --release
```

**Server is ready!** Access it at: `http://127.0.0.1:8080`