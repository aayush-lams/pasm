use axum::{
    body::Body,
    extract::State,
    http::{header::AUTHORIZATION, Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};

use crate::types::{db::Db, state::PasmState};

/// This function handles authentication for server requests.
/// It takes `State<PasmState>`, request body for header.
/// If the header authentication is success, runs the remainder of middleware, else returns 401
pub async fn call(
    State(state): State<PasmState>,
    mut req: Request<Body>, // mut so we can add extensions
    next: Next,
) -> Response {
    let token = match req
        .headers()
        .get(AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
    {
        Some(t) => t.to_string(),
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                "authorization failed — missing or malformed Bearer token",
            )
                .into_response();
        }
    };

    match state.db.auth_key_exists(&token).await {
        Ok(true) => {
            req.extensions_mut().insert(token);
            next.run(req).await
        }
        Ok(false) => (
            StatusCode::UNAUTHORIZED,
            "authorization failed — invalid auth key, run `pasm_client login` to re-authenticate",
        )
            .into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "authorization failed — database error",
        )
            .into_response(),
    }
}
