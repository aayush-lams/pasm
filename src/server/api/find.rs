use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::types::state::PasmState;

/// This function finds specified entry in the database and returns the Json content
/// `#Error`
/// * Error:404, `NOT_FOUND` if key doesnt exist
/// * Error::500, `INTERNAL_SERVER_ERROR` if decryption failed
pub async fn call(Path(name): Path<String>, State(state): State<PasmState>) -> impl IntoResponse {
    let db = &state.db;
    let auth_key = &state.auth_key;

    let user_id = match db.get_user_id_by_authkey(auth_key) {
        Ok(id) => id,
        Err(err) => {
            let error = format!("{err:?}");
            return (StatusCode::INTERNAL_SERVER_ERROR, error).into_response();
        }
    };

    let result = match db.get_entry(&user_id, &name) {
        Ok(entries) => entries,
        // better error handling, error also contains rest error
        Err(err) => return (StatusCode::INTERNAL_SERVER_ERROR, format!("{:#?}", err)).into_response(),
    };
    Json(result).into_response()
}
