use std::time::Instant;

use crate::types::db::PgDb;

/// Axum application state holding the database handle and server metadata.
#[derive(Clone)]
pub struct PasmState {
    pub db: PgDb,
    pub started_at: Instant,
}
