use std::time::{SystemTime, UNIX_EPOCH};

use axum::{extract::State, http::StatusCode, response::IntoResponse, Extension};

use crate::types::{db::Db, state::PasmState};

/// Creates a JSON dump of all encrypted entries for the authenticated user.
///
/// Writes to `/tmp/pasm/backups/<user_id>_<unix_timestamp>.json` and
/// returns the file path along with entry count and size.
pub async fn call(
    Extension(auth_key): Extension<String>,
    State(state): State<PasmState>,
) -> impl IntoResponse {
    let db = &state.db;

    let user_id = match db.get_user_id_by_authkey(&auth_key).await {
        Ok(id) => id,
        Err(err) => return err.into_response(),
    };

    let entries = match db.list_entries(&user_id).await {
        Ok(e) => e,
        Err(err) => return err.into_response(),
    };

    let backup_dir = "/tmp/pasm/backups";
    if let Err(e) = std::fs::create_dir_all(backup_dir) {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("failed to create backup directory: {e}"),
        )
            .into_response();
    }

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    let file_name = format!("{user_id}_{timestamp}.json");
    let backup_path = std::path::Path::new(backup_dir).join(&file_name);

    let json = match serde_json::to_string_pretty(&entries) {
        Ok(j) => j,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("failed to serialize entries: {e}"),
            )
                .into_response();
        }
    };

    if let Err(e) = std::fs::write(&backup_path, &json) {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("failed to write backup file: {e}"),
        )
            .into_response();
    }

    let path_str = backup_path.to_string_lossy().to_string();
    let size = json.len();
    let count = entries.len();

    (
        StatusCode::OK,
        format!("Backup created: {path_str} ({count} entries, {size} bytes)"),
    )
        .into_response()
}
