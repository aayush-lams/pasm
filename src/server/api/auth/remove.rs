use axum::{extract::State, response::IntoResponse};

use crate::types::state::PasmState;

/// Removes the current user and all their data from the database.
///
/// This endpoint deletes the user's authentication key and their entire
/// encrypted password entry tree.
pub async fn call(State(state): State<PasmState>) -> impl IntoResponse {
    let db = &state.db;
    let auth_key = &state.auth_key;

    db.remove_user(auth_key).into_response()
}
