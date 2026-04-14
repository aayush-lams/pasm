use axum::{extract::State, response::IntoResponse, Json};

use crate::types::{entry::RequestData, state::PasmState};

/// Updates the authentication key for the current user.
///
/// Replaces the existing auth key with a new one provided in the payload.
pub async fn call(
    State(state): State<PasmState>,
    Json(payload): Json<RequestData>,
) -> impl IntoResponse {
    let db = &state.db;
    let auth_key = &state.auth_key;
    let new_auth = &payload.value;

    db.update_auth(auth_key, new_auth).into_response()
}
