use axum::{
    body::Body,
    extract::State,
    http::{header::AUTHORIZATION, Request, StatusCode},
    middleware::Next,
    response::Response,
};

use crate::types::PasmState;

/// This function handles authentication for server requests.
/// It takes `State<PasmState>`, request body for header.
/// If the header authentication is success, runs the remainder of middleware, else returns 401
pub async fn call(
    State(state): State<PasmState>,
    req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let api_key = state.api_key;
    let auth_header = req
        .headers()
        .get(AUTHORIZATION)
        .and_then(|value| value.to_str().ok());
    match auth_header {
        Some(value) if value == format!("Bearer {}", api_key) => Ok(next.run(req).await),
        _ => Err(StatusCode::UNAUTHORIZED),
    }
}
