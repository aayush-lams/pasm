use axum::{extract::State, response::IntoResponse, Json};

use crate::{
    types::{Details, PasmState},
    utils::helper_fxns::deserialize_entry,
};

/// This function finds all entries in the database and returns the Vector of Json contents
pub async fn call(State(state): State<PasmState>) -> impl IntoResponse {
    let db = state.db;
    let passkey = state.encr_key;
    let mut result: Vec<Details> = Vec::new();
    for item in db.scan_prefix("entry:") {
        match item {
            Ok((_key, value)) => {
                if let Ok(details) = deserialize_entry(value, passkey.to_string()) {
                    result.push(details);
                }
            }
            Err(_) => continue,
        }
    }
    Json(result).into_response()
}
