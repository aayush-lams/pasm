use axum::{extract::State, response::IntoResponse, Json, Extension};

use crate::types::state::PasmState;

/// Lists all users in the database.
pub async fn call(
    Extension(auth_key): Extension<String>,
    State(state): State<PasmState>,
) -> impl IntoResponse {
    let db = &state.db;

    match db.list_users() {
        Ok(users) => Json(users).into_response(),
        Err(err) => err.into_response(),
    }
}
