use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};

use crate::types::{error::PasmResult, state::PasmState};

/// This function deletes specified entry in the database
/// saves the json entry to sled with its name field as key if key doesnot exist, else return Error:404, `NOT_FOUND`
///
/// `#Error`
/// * Error:404, `NOT_FOUND` if key doesnt exist
/// * Error::500, `INTERNAL_SERVER_ERROR` if decryption failed
pub async fn call(Path(name): Path<String>, State(state): State<PasmState>) -> impl IntoResponse {
    let db = &state.db;
    let auth_key = &state.auth_key;

    let user_id = match db.get_user_id_by_authkey(&auth_key) {
        Ok(id) => id,
        Err(err) => {
            let error = format!("{err:?}");
            return (StatusCode::INTERNAL_SERVER_ERROR, error).into_response();
        }
    };

    if let Err(err) = db.remove_entry(&user_id, &name) {
        if let PasmResult::ServerStatus(err, err_detail) = err {
            return (err, err_detail).into_response();
        }
        let error = format!("{err:?}");
        return (StatusCode::INTERNAL_SERVER_ERROR, error).into_response();
    }
    (StatusCode::OK, "Entry added").into_response()
}
