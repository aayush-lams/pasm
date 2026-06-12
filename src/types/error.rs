use std::fmt;
use std::string::FromUtf8Error;

use axum::{http::StatusCode, response::IntoResponse};

/// Unified error type used throughout pasm for both client and server.
///
/// # Variants
/// * `DatabaseError` — Sled database operation failed
/// * `UTF8ConversionError` — Failed to convert bytes to a UTF-8 string
/// * `EncryptionError` — AES-256 encryption failed
/// * `DecryptionError` — AES-256 decryption failed (wrong key or corrupted data)
/// * `SerializationError` — Failed to serialize a value to JSON
/// * `DeserializationError` — Failed to parse JSON into the expected type
/// * `ServerStatus` — Server returned an HTTP status + message (client-side) or
///   represents a controlled error response (server-side)
#[derive(Debug)]
pub enum PasmResult {
    DatabaseError { err: sled::Error },
    UTF8ConversionError { err: FromUtf8Error },
    EncryptionError { err: String },
    DecryptionError { err: String },
    SerializationError { err: String },
    DeserializationError { err: String },
    ServerStatus(StatusCode, String),
}

impl fmt::Display for PasmResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PasmResult::DatabaseError { err } => write!(f, "database error: {err}"),
            PasmResult::UTF8ConversionError { err } => write!(f, "utf8 conversion error: {err}"),
            PasmResult::EncryptionError { err } => write!(f, "encryption error: {err}"),
            PasmResult::DecryptionError { err } => write!(f, "decryption error: {err}"),
            PasmResult::SerializationError { err } => write!(f, "serialization error: {err}"),
            PasmResult::DeserializationError { err } => write!(f, "deserialization error: {err}"),
            PasmResult::ServerStatus(_, msg) => write!(f, "{msg}"),
        }
    }
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
            PasmResult::EncryptionError { err } => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("encryption error: {err}"),
            )
                .into_response(),
            PasmResult::DecryptionError { err } => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("decryption error: {err}"),
            )
                .into_response(),
            PasmResult::SerializationError { err } => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("serialization error: {err}"),
            )
                .into_response(),
            PasmResult::DeserializationError { err } => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("deserialization error: {err}"),
            )
                .into_response(),
            PasmResult::ServerStatus(status, msg) => (status, msg).into_response(),
        }
    }
}
