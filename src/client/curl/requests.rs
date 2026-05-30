use std::process::Command;

/// Base URL of the pasm server. Change this to point at a remote instance.
const SERVER_URL: &str = "http://localhost:3000";

/// Executes a `curl` command with the given arguments and returns the output.
///
/// Captures stdout on success. On curl failure (binary not found, transport
/// error, non-zero exit), returns a string prefixed with `"Error:"` containing
/// the curl exit status and stderr output.
///
/// HTTP-level errors (4xx, 5xx) are detected via `-w "%{http_code}"` and
/// returned as `"Error: HTTP {status}"` — this distinguishes auth failures
/// (401 with empty body) from successful empty responses.
///
/// # Arguments
/// * `args` - Arguments passed directly to `curl` (excluding `-w`)
///
/// # Returns
/// The response body (stdout) from curl on success, or an error string on failure.
fn run_curl(args: &[&str]) -> String {
    let mut curl_args: Vec<&str> = args.to_vec();
    curl_args.push("-w");
    curl_args.push("\n%{http_code}");
    let output = match Command::new("curl").args(&curl_args).output() {
        Ok(o) => o,
        Err(e) => return format!("Error: failed to run curl: {e}"),
    };
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return format!(
            "Error: curl exited with status {}: {}",
            output.status,
            stderr.trim()
        );
    }
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let (body, status_str) = match stdout.rsplit_once('\n') {
        Some((b, s)) => (b.to_string(), s.trim()),
        None => return stdout,
    };
    let status: u16 = match status_str.parse() {
        Ok(s) => s,
        Err(_) => return stdout,
    };
    if status >= 400 {
        return format!("Error: HTTP {status}");
    }
    body
}

/// Creates a new entry on the server.
///
/// Sends `POST /entry` with a JSON body `{"key": "<name>", "value": "<encrypted>"}`.
/// The `value` should be the encrypted entry data (see `serialize_entry`).
///
/// # Arguments
/// * `api_key` - Bearer token for API authentication
/// * `key` - The entry name (e.g., `"github"`)
/// * `value` - The encrypted entry content
///
/// # Returns
/// The server response body.
pub fn create_entry(api_key: &str, key: &str, value: &str) -> String {
    let payload = serde_json::json!({ "key": key, "value": value }).to_string();
    run_curl(&[
        "-s",
        "-X",
        "POST",
        &format!("{}/entry", SERVER_URL),
        "-H",
        &format!("Authorization: Bearer {api_key}"),
        "-H",
        "Content-Type: application/json",
        "-d",
        &payload,
    ])
}

/// Finds and returns the encrypted value for a named entry.
///
/// Sends `GET /entry/{name}`. The returned JSON has the encrypted value in
/// the `"value"` field — callers should decrypt it with `deserialize_entry`.
///
/// # Arguments
/// * `api_key` - Bearer token for API authentication
/// * `name` - The entry name to look up
///
/// # Returns
/// The server response body (JSON with `key` and `value` fields on success).
pub fn find_entry(api_key: &str, name: &str) -> String {
    run_curl(&[
        "-s",
        "-X",
        "GET",
        &format!("{}/entry/{name}", SERVER_URL),
        "-H",
        &format!("Authorization: Bearer {api_key}"),
    ])
}

/// Lists all entries for the authenticated user.
///
/// Sends `GET /entries`. The response is a JSON array of `{"key": "...", "value": "..."}`
/// objects where each `value` contains encrypted entry data.
///
/// # Arguments
/// * `api_key` - Bearer token for API authentication
///
/// # Returns
/// The server response body (JSON array of entry objects).
pub fn list_entries(api_key: &str) -> String {
    run_curl(&[
        "-s",
        "-X",
        "GET",
        &format!("{}/entries", SERVER_URL),
        "-H",
        &format!("Authorization: Bearer {api_key}"),
    ])
}

/// Deletes a named entry from the server.
///
/// Sends `DELETE /entry/{name}`.
///
/// # Arguments
/// * `api_key` - Bearer token for API authentication
/// * `name` - The entry name to delete
///
/// # Returns
/// The server response body.
pub fn delete_entry(api_key: &str, name: &str) -> String {
    run_curl(&[
        "-s",
        "-X",
        "DELETE",
        &format!("{}/entry/{name}", SERVER_URL),
        "-H",
        &format!("Authorization: Bearer {api_key}"),
    ])
}

/// Creates or overwrites an entry on the server.
///
/// Sends `POST /entry/amend` with a JSON body `{"key": "<name>", "value": "<encrypted>"}`.
/// Unlike `POST /entry`, this endpoint overwrites existing entries with the same name.
///
/// # Arguments
/// * `api_key` - Bearer token for API authentication
/// * `key` - The entry name (e.g., `"github"`)
/// * `value` - The encrypted entry content
///
/// # Returns
/// The server response body.
pub fn amend_entry(api_key: &str, key: &str, value: &str) -> String {
    let payload = serde_json::json!({ "key": key, "value": value }).to_string();
    run_curl(&[
        "-s",
        "-X",
        "POST",
        &format!("{}/entry/amend", SERVER_URL),
        "-H",
        &format!("Authorization: Bearer {api_key}"),
        "-H",
        "Content-Type: application/json",
        "-d",
        &payload,
    ])
}

/// Registers the given API key as a new user on the server.
///
/// Sends `POST /auth` with the Bearer token. The server creates a new user ID
/// and associates it with this auth key.
///
/// # Arguments
/// * `api_key` - The auth key to register (also used as the Bearer token)
///
/// # Returns
/// The server response body (`"registered new authentication token!"` on success,
/// `"auth key already exists"` on conflict).
pub fn register_auth(api_key: &str) -> String {
    run_curl(&[
        "-s",
        "-X",
        "POST",
        &format!("{}/auth", SERVER_URL),
        "-H",
        &format!("Authorization: Bearer {api_key}"),
    ])
}

/// Replaces the current auth key with a new one on the server.
///
/// Sends `POST /auth/update` with a JSON body `{"key": "", "value": "<new_key>"}`.
/// The current auth key is identified from the Bearer token.
///
/// # Arguments
/// * `api_key` - The current Bearer token (old auth key)
/// * `new_key` - The replacement auth key to associate with the user
///
/// # Returns
/// The server response body.
pub fn update_auth(api_key: &str, new_key: &str) -> String {
    let payload = serde_json::json!({ "key": "", "value": new_key }).to_string();
    run_curl(&[
        "-s",
        "-X",
        "POST",
        &format!("{}/auth/update", SERVER_URL),
        "-H",
        &format!("Authorization: Bearer {api_key}"),
        "-H",
        "Content-Type: application/json",
        "-d",
        &payload,
    ])
}

/// Removes the current user and all their data from the server.
///
/// Sends `DELETE /auth/remove` with the Bearer token identifying the user to remove.
/// This deletes both the auth key mapping and the user's entire entry tree.
///
/// # Arguments
/// * `api_key` - The auth key (Bearer token) of the user to remove
///
/// # Returns
/// The server response body.
pub fn remove_auth(api_key: &str) -> String {
    run_curl(&[
        "-s",
        "-X",
        "DELETE",
        &format!("{}/auth/remove", SERVER_URL),
        "-H",
        &format!("Authorization: Bearer {api_key}"),
    ])
}

/// Lists all registered auth keys on the server.
///
/// Sends `GET /auth/list`. The response is a JSON array of auth key strings.
///
/// # Arguments
/// * `api_key` - Bearer token (must have an existing session)
///
/// # Returns
/// A newline-separated list of registered auth keys, or the raw server response
/// if JSON parsing fails.
pub fn list_users(api_key: &str) -> String {
    let res = run_curl(&[
        "-s",
        "-X",
        "GET",
        &format!("{}/auth/list", SERVER_URL),
        "-H",
        &format!("Authorization: Bearer {api_key}"),
    ]);
    match serde_json::from_str::<Vec<String>>(&res) {
        Ok(users) => users.join("\n"),
        Err(_) => res,
    }
}
