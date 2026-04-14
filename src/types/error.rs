use std::string::FromUtf8Error;

use axum::{http::StatusCode, response::IntoResponse};

/// Custom errors used throughout pasm
#[derive(Debug)]
pub enum PasmResult {
    DatabaseError { err: sled::Error },
    UTF8ConversionError { err: FromUtf8Error },
    ServerStatus(StatusCode, String),
}

impl IntoResponse for PasmResult {
    fn into_response(self) -> axum::response::Response {
        match self {
            PasmResult::DatabaseError { err } => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("database error: {err}"),
            )
                .into_response(),
            PasmResult::UTF8ConversionError { err } => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("utf8 conversion error: {err}"),
            )
                .into_response(),
            PasmResult::ServerStatus(status, msg) => (status, msg).into_response(),
        }
    }
}
