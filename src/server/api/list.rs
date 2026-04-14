use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};

use crate::{
    types::state::PasmState,
};

/// This function finds all entries in the database and returns the Vector of Json contents
pub async fn call(State(state): State<PasmState>) -> impl IntoResponse {
    println!("called list");
    let db = &state.db;
    let auth_key = &state.auth_key;
    let user = match db.get_user_id_by_authkey(&auth_key) {
        Ok(user_id) => user_id,
        Err(err) => return (StatusCode::INTERNAL_SERVER_ERROR, format!("{:#?}", err)).into_response(),
    };

    let result = match db.list_entries(&user) {
        Ok(entries) => {
            println!("listing list");
            entries
        }
        Err(err) => return (StatusCode::INTERNAL_SERVER_ERROR, format!("{:#?}", err)).into_response(),
    };

    Json(result).into_response()
}
