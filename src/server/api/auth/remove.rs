use axum::{extract::State, response::IntoResponse, Extension};

use crate::types::state::PasmState;

/// Removes the current user and all their data from the database.
///
/// This endpoint deletes the user's authentication key and their entire
/// encrypted password entry tree.
pub async fn call(
    Extension(uid): Extension<String>,
    State(state): State<PasmState>,
) -> impl IntoResponse {
    let db = &state.db;

    println!("removed user!");
    db.remove_user(&uid).into_response()
}
