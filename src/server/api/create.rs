use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::{
    types::{Details, PasmState},
    utils::helper_fxns::serialize_entry,
};

/// This function creates new entry in the database
/// if key doesnot exist returns Error:409, `CONFLICT`
pub async fn call(
    State(state): State<PasmState>,
    Json(payload): Json<Details>,
) -> impl IntoResponse {
    let db = state.db;
    let passkey = state.encr_key;
    let name = format!("entry:{}", payload.name);
    if let Some(_) = db.get(name.clone()).unwrap() {
        return (StatusCode::CONFLICT, "Key already exists").into_response();
    }
    let crypt_text = serialize_entry(payload, passkey.to_string());
    db.insert(name, crypt_text.as_bytes()).unwrap();
    (StatusCode::CREATED, "Entry added").into_response()
}
