use axum::{extract::State, http::StatusCode, response::IntoResponse, Extension, Json};

use crate::types::{db::Db, entry::RequestData, state::PasmState};

/// Updates or creates a password entry.
/// If the entry already exists it is overwritten.
pub async fn call(
    State(state): State<PasmState>,
    Extension(auth_key): Extension<String>,
    Json(payload): Json<RequestData>,
) -> impl IntoResponse {
    let db = &state.db;

    let user = match db.get_user_id_by_authkey(&auth_key).await {
        Ok(user_id) => user_id,
        Err(err) => return err.into_response(),
    };

    if let Err(err) = db.amend_entry(&user, &payload.key, &payload.value).await {
        return err.into_response();
    }
    (StatusCode::OK, "Entry updated").into_response()
}
