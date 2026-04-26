use axum::{
    body::Body,
    extract::State,
    http::{header::AUTHORIZATION, Request, StatusCode},
    middleware::Next,
    response::Response,
};

use crate::types::state::PasmState;

/// This function handles authentication for server requests.
/// It takes `State<PasmState>`, request body for header.
/// If the header authentication is success, runs the remainder of middleware, else returns 401
pub async fn call(
    State(state): State<PasmState>,
    mut req: Request<Body>, // mut so we can add extensions
    next: Next,
) -> Result<Response, StatusCode> {
    let token = req
        .headers()
        .get(AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .ok_or(StatusCode::UNAUTHORIZED)?
        .to_string();

    let users = match state.db.users() {
        Ok(tree) => tree,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    let Ok(exists) = users.contains_key(&token) else {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    };
    if !exists {
        return Err(StatusCode::UNAUTHORIZED);
    };

    req.extensions_mut().insert(token);

    Ok(next.run(req).await)
}
