use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::{
    types::{Details, PasmState},
    utils::helper_fxns::serialize_entry,
};

/// This function edits the content to sled database
/// if value already exists in databse overwrites it else return error:404, `NOT_FOUND`
pub async fn call(
    State(state): State<PasmState>,
    Json(payload): Json<Details>,
) -> impl IntoResponse {
    let db = state.db;
    let passkey = state.encr_key;
    let name = format!("entry:{}", payload.name);
    if let Some(_) = db.get(name.clone()).unwrap() {
        let crypt_text = serialize_entry(payload, passkey.to_string());
        db.insert(name, crypt_text.as_bytes()).unwrap();
        return (StatusCode::CREATED, "Updated the entry").into_response();
    }
    (StatusCode::NOT_FOUND, "Key not found").into_response()
}
