use axum::{extract::State, response::IntoResponse, Extension, Json};

use crate::types::{entry::RequestData, state::PasmState};

/// Updates the authentication key for the current user.
///
/// Replaces the existing auth key with a new one provided in the payload.
pub async fn call(
    State(state): State<PasmState>,
    Extension(uid): Extension<String>,
    Json(payload): Json<RequestData>,
) -> impl IntoResponse {
    let db = &state.db;
    let new_auth = &payload.value;

    println!("updated user!");
    db.update_auth(&uid, &new_auth).into_response()
}
