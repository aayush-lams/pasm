use axum::{extract::State, response::IntoResponse, Extension, Json};

use crate::types::state::PasmState;

/// Lists all users in the database.
pub async fn call(
    Extension(_auth_key): Extension<String>,
    State(state): State<PasmState>,
) -> impl IntoResponse {
    let db = &state.db;

    match db.list_users() {
        Ok(users) => Json(users).into_response(),
        Err(err) => err.into_response(),
    }
}
