use axum::{extract::State, response::IntoResponse};

use crate::types::state::PasmState;

/// Registers a new user with the authentication key from the server state.
///
/// This endpoint creates a new user with a generated UUID and associates it
/// with the server's authentication key.
pub async fn call(State(state): State<PasmState>) -> impl IntoResponse {
    let db = &state.db;
    let auth_key = &state.auth_key;

    db.register_auth(auth_key).into_response()
}
