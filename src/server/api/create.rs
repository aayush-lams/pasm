use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::types::{entry::RequestData, error::PasmResult, state::PasmState};

/// This function creates new entry in the database
/// if key doesnot exist returns Error:409, `CONFLICT`
pub async fn call(
    State(state): State<PasmState>,
    Json(payload): Json<RequestData>,
) -> impl IntoResponse {
    let db = &state.db;
    let auth_key = state.auth_key;

    let user_id = match db.get_user_id_by_authkey(&auth_key) {
        Ok(id) => id,
        Err(err) => {
            let error = format!("{err:?}");
            return (StatusCode::INTERNAL_SERVER_ERROR, error).into_response();
        }
    };

    if let Err(err) = db.add_entry(&user_id, &payload.key, payload.value) {
        if let PasmResult::ServerStatus(err, err_detail) = err {
            return (err, err_detail).into_response();
        }
        let error = format!("{err:?}");
        return (StatusCode::INTERNAL_SERVER_ERROR, error).into_response();
    }
    (StatusCode::CREATED, "Entry added").into_response()
}
