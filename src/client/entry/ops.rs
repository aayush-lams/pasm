use colored::Colorize;

use crate::client::{curl::requests, input::prompts};
use crate::types::detail::Details;
use crate::utils::{deserialize::deserialize_entry, serialize::serialize_entry};

/// Creates a new encrypted entry on the server.
///
/// Collects entry details via interactive prompts, encrypts them with
/// `serialize_entry`, and sends the encrypted data to the server via
/// `POST /entry`.
///
/// # Arguments
/// * `api_key` - Bearer token for server API authentication
/// * `encr_key` - AES-256 key used to encrypt the entry data before upload
///
/// # Returns
/// A success message like `"entry 'github' created"`, or the raw error
/// response from the server if creation fails.
pub fn create(api_key: &str, encr_key: &str) -> String {
    let details = prompts::collect_details();
    let encrypted = match serialize_entry(&details, encr_key) {
        Ok(e) => e,
        Err(e) => return e.to_string(),
    };
    let res = requests::create_entry(api_key, &details.name, &encrypted);
    if res.is_empty() || res.contains("Error") {
        return res;
    }
    format!("entry '{}' created", details.name)
}

/// Finds and displays a decrypted entry by name.
///
/// Fetches the encrypted entry from the server via `GET /entry/{name}`,
/// then decrypts and formats it using `deserialize_entry`.
///
/// # Arguments
/// * `api_key` - Bearer token for server API authentication
/// * `encr_key` - AES-256 key used to decrypt the fetched entry data
/// * `name` - The entry name to look up
///
/// # Returns
/// A formatted display string with the decrypted entry details, or an
/// error message if the entry is not found or decryption fails.
pub fn find(api_key: &str, encr_key: &str, name: &str) -> String {
    let res = requests::find_entry(api_key, name);
    let parsed: serde_json::Value = match serde_json::from_str(&res) {
        Ok(v) => v,
        Err(_) => return res,
    };
    let value = match parsed["value"].as_str() {
        Some(v) => v,
        None => return res,
    };
    match deserialize_entry(value, encr_key) {
        Ok(details) => format_details(&details),
        Err(e) => e.to_string(),
    }
}

/// Lists all entries with their decrypted details.
///
/// Fetches all entries from the server via `GET /entries`, then decrypts
/// and formats each one. Internal server key prefixes (e.g., `entry:`) are
/// stripped before display.
///
/// # Arguments
/// * `api_key` - Bearer token for server API authentication
/// * `encr_key` - AES-256 key used to decrypt each entry
///
/// # Returns
/// A formatted string showing all entries separated by headers, or
/// `"(no entries)"` if the list is empty. Decryption failures are noted
/// per-entry rather than aborting the entire listing.
pub fn list(api_key: &str, encr_key: &str) -> String {
    let res = requests::list_entries(api_key);
    let items: Vec<serde_json::Value> = match serde_json::from_str(&res) {
        Ok(v) => v,
        Err(_) => return res,
    };
    if items.is_empty() {
        return "(no entries)".to_string();
    }
    let mut out = String::new();
    for item in &items {
        let raw_name = item["key"].as_str().unwrap_or("?");
        let name = raw_name.strip_prefix("entry:").unwrap_or(raw_name);
        let value = item["value"].as_str().unwrap_or("");
        match deserialize_entry(value, encr_key) {
            Ok(details) => {
                out.push_str(&format!("--- {} ---\n", name.bright_blue()));
                out.push_str(&format_detail_lines(&details));
            }
            Err(e) => {
                out.push_str(&format!("--- {} --- ({})\n", name.bright_blue(), e));
            }
        }
    }
    out.trim_end().to_string()
}

/// Updates (or creates) an encrypted entry on the server.
///
/// Collects entry details via interactive prompts, encrypts them, and
/// sends them to the server via `POST /entry/amend`. Unlike `create`,
/// this will overwrite an existing entry with the same name.
///
/// # Arguments
/// * `api_key` - Bearer token for server API authentication
/// * `encr_key` - AES-256 key used to encrypt the entry data before upload
///
/// # Returns
/// A success message like `"entry 'github' updated"`, or the raw error
/// response from the server if the operation fails.
pub fn amend(api_key: &str, encr_key: &str) -> String {
    let details = prompts::collect_details();
    let encrypted = match serialize_entry(&details, encr_key) {
        Ok(e) => e,
        Err(e) => return e.to_string(),
    };
    let res = requests::amend_entry(api_key, &details.name, &encrypted);
    if res.is_empty() || res.contains("Error") {
        return res;
    }
    format!("entry '{}' updated", details.name)
}

/// Deletes an entry by name.
///
/// Sends `DELETE /entry/{name}` — no encryption/decryption is needed since
/// only the entry name is used.
///
/// # Arguments
/// * `api_key` - Bearer token for server API authentication
/// * `name` - The entry name to delete
///
/// # Returns
/// The server response body.
pub fn delete(api_key: &str, name: &str) -> String {
    requests::delete_entry(api_key, name)
}

/// Formats a `Details` struct into a multi-line display string for single-entry view.
///
/// Each field is labelled with a green prefix and displayed on its own line.
fn format_details(d: &Details) -> String {
    format!(
        "{} {}\n{} {}\n{} {}\n{} {}\n{} {}",
        "Name:".bright_green(),
        d.name,
        "Site:".bright_green(),
        d.site,
        "Username:".bright_green(),
        d.uname,
        "Password:".bright_green(),
        d.pword,
        "Note:".bright_green(),
        d.note,
    )
}

/// Formats a `Details` struct into an indented sub-list for list-entry view.
///
/// Used within the `list` function to display each entry's non-name fields
/// under a header. Each field is indented with two spaces.
fn format_detail_lines(d: &Details) -> String {
    format!(
        "  {} {}\n  {} {}\n  {} {}\n  {} {}\n",
        "Site:".bright_green(),
        d.site,
        "Username:".bright_green(),
        d.uname,
        "Password:".bright_green(),
        d.pword,
        "Note:".bright_green(),
        d.note,
    )
}
