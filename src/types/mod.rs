use std::sync::Arc;

use serde::{Deserialize, Serialize};
use sled::Db;

/// User detail
///
/// Implements Clone and Debug
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Details {
    pub name: String,
    pub site: String,
    pub uname: String,
    pub pword: String,
    pub note: String,
}

// It is an axum state
// It holds databse and keys at runtime
#[derive(Clone)]
pub struct PasmState {
    pub db: Db,
    pub api_key: Arc<String>,
    pub encr_key: Arc<String>,
}
