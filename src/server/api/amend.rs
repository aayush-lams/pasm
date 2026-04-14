use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::types::{entry::RequestData, error::PasmResult, state::PasmState};

/// This function edits the content to sled database
/// if value already exists in databse overwrites it else return error:404, `NOT_FOUND`
pub async fn call(
    State(state): State<PasmState>,
    Json(payload): Json<RequestData>,
) -> impl IntoResponse {
    let db = &state.db;
    let auth_key = &state.auth_key;

    let user = match db.get_user_id_by_authkey(&auth_key) {
        Ok(user_id) => user_id,
        Err(err) => return (StatusCode::INTERNAL_SERVER_ERROR, format!("{:#?}", err)).into_response(),
    };

    if let Err(err) = db.amend_entry(&user, &payload.key, payload.value) {
        if let PasmResult::ServerStatus(err, err_detail) = err {
            return (err, err_detail).into_response();
        }
        let error = format!("{err:?}");
        return (StatusCode::INTERNAL_SERVER_ERROR, error).into_response();
    }
    (StatusCode::OK).into_response()
}
