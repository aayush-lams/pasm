use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};

use crate::types::PasmState;

/// This function deletes specified entry in the database
/// saves the json entry to sled with its name field as key if key doesnot exist, else return Error:404, `NOT_FOUND`
pub async fn call(Path(name): Path<String>, State(state): State<PasmState>) -> impl IntoResponse {
    let db = state.db;
    let key = format!("entry:{}", name);
    if let Some(_) = db.remove(key).unwrap() {
        let result = format!("Entry deleted : {:?}", name);
        (StatusCode::OK, result).into_response()
    } else {
        (StatusCode::NOT_FOUND, "Entry doesnt exist").into_response()
    }
}
