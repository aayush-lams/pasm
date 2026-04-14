use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Extension, Json,
};

use crate::types::state::PasmState;

/// This function finds specified entry in the database and returns the Json content
pub async fn call(
    Path(name): Path<String>,
    Extension(auth_key): Extension<String>,
    State(state): State<PasmState>,
) -> impl IntoResponse {
    let db = &state.db;

    let user_id = match db.get_user_id_by_authkey(&auth_key) {
        Ok(id) => id,
        Err(err) => return err.into_response(),
    };

    let result = match db.get_entry(&user_id, &name) {
        Ok(entries) => entries,
        Err(err) => return err.into_response(),
    };
    Json(result).into_response()
}
