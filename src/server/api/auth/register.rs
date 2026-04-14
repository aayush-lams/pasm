use axum::{
    body::Body,
    extract::State,
    http::{header::AUTHORIZATION, Request, StatusCode},
    response::IntoResponse,
};

use crate::types::state::PasmState;

/// Registers a new user with the authentication key from the server state.
///
/// This endpoint creates a new user with a generated UUID and associates it
/// with the server's authentication key.
pub async fn call(
    State(state): State<PasmState>,
    req: Request<Body>, // mut so we can add extensions
) -> impl IntoResponse {
    let db = &state.db;
    let token = req
        .headers()
        .get(AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "));

    let Some(uid) = token else {
        return StatusCode::NOT_EXTENDED.into_response();
    };
    println!("registered user!");
    db.register_auth(&uid).into_response()
}
