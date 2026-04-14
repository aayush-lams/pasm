use crate::types::db::PasmDb;

/// It is an axum state
/// It holds databse and keys at runtime
#[derive(Clone)]
pub struct PasmState {
    pub db: PasmDb,
    // pub auth_key: String,
}
