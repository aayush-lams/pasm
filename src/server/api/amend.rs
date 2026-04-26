use axum::{extract::State, http::StatusCode, response::IntoResponse, Extension, Json};

use crate::types::{entry::RequestData, state::PasmState};

/// This function edits the content to sled database
/// if value already exists in databse overwrites it else return error:404, `NOT_FOUND`
pub async fn call(
    State(state): State<PasmState>,
    Extension(auth_key): Extension<String>,
    Json(payload): Json<RequestData>,
) -> impl IntoResponse {
    let db = &state.db;

    let user = match db.get_user_id_by_authkey(&auth_key) {
        Ok(user_id) => user_id,
        Err(err) => return err.into_response(),
    };

    if let Err(err) = db.amend_entry(&user, &payload.key, payload.value) {
        return err.into_response();
    }
    (StatusCode::OK, "Entry updated").into_response()
}
