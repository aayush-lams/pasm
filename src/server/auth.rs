use axum::{
    body::Body,
    extract::State,
    http::{Request, StatusCode, header::AUTHORIZATION},
    middleware::Next,
    response::Response,
};

use crate::types::state::PasmState;

/// This function handles authentication for server requests.
/// It takes `State<PasmState>`, request body for header.
/// If the header authentication is success, runs the remainder of middleware, else returns 401
pub async fn call(
    State(state): State<PasmState>,
    req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let auth_key = state.auth_key;
    let auth_header = req
        .headers()
        .get(AUTHORIZATION)
        .and_then(|value| value.to_str().ok());
    match auth_header {
        Some(value) if value == format!("Bearer {}", auth_key) => {
            println!("verified user");
            Ok(next.run(req).await)
        }
        _ => {
            println!("failed to authorised!");
            Err(StatusCode::UNAUTHORIZED)
        }
    }
}
