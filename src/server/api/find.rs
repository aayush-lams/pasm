use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::{types::PasmState, utils::helper_fxns::deserialize_entry};

/// This function finds specified entry in the database and returns the Json content
/// If key doesnot exist, return Error:404, `NOT_FOUND`
pub async fn call(Path(name): Path<String>, State(state): State<PasmState>) -> impl IntoResponse {
    let db = state.db;
    let passkey = state.encr_key;
    let key = format!("entry:{}", name);
    if let Some(entry) = db.get(key).unwrap() {
        let result = deserialize_entry(entry, passkey.to_string());
        Json(result).into_response()
    } else {
        (StatusCode::NOT_FOUND, "Entry not found").into_response()
    }
}
