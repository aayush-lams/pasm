use crate::types::db::PasmDb;

/// Axum application state holding the database handle.
#[derive(Clone)]
pub struct PasmState {
    pub db: PasmDb,
}
