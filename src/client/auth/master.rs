use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

use colored::Colorize;
use magic_crypt::{new_magic_crypt, MagicCryptTrait};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::client::curl::requests;
use crate::client::input::prompts::prompt_hidden;

const CONFIG_DIR: &str = ".config/pasm";
const HASH_FILE: &str = "master.hash";
const SESSION_FILE: &str = "session";
const VERIFY_PLAINTEXT: &str = "pasm::verify";

/// Data stored in the session JSON file.
///
/// Fields:
/// - `api_key`: Bearer token sent with API requests (derived from master password)
/// - `encr_key`: AES-256 key for entry encryption/decryption (derived from master password)
#[derive(Serialize, Deserialize)]
struct SessionData {
    api_key: String,
    encr_key: String,
}

/// Returns the `$HOME/.config/pasm` directory path.
///
/// Falls back to `/tmp/.config/pasm` if `$HOME` is unset.
fn config_dir() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
    PathBuf::from(home).join(CONFIG_DIR)
}

/// Returns the path to the master password hash file.
fn hash_path() -> PathBuf {
    config_dir().join(HASH_FILE)
}

/// Returns the path to the session file.
fn session_path() -> PathBuf {
    config_dir().join(SESSION_FILE)
}

/// Encodes a byte slice as a lowercase hex string.
///
/// # Arguments
/// * `bytes` - The raw bytes to encode
///
/// # Returns
/// A hex-encoded string (2 characters per byte).
fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

/// Derives a 64-character hex key from a password using SHA-256 with a context salt.
///
/// The key is computed as `SHA-256(context || password)` where `||` is byte
/// concatenation. Using different context values (e.g., `"pasm-auth"`,
/// `"pasm-encr"`) produces independent keys from the same password.
///
/// # Arguments
/// * `password` - The master password string
/// * `context` - A context string used as a salt to namespace the derivation
///
/// # Returns
/// A 64-character lowercase hex string (SHA-256 digest).
fn derive_key(password: &str, context: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(context.as_bytes());
    hasher.update(password.as_bytes());
    hex_encode(&hasher.finalize())
}

/// Hashes a key string with a second SHA-256 pass.
///
/// Used to produce the `api_key` from the intermediate `auth_key`:
/// `api_key = SHA-256(auth_key)`. This double-hashing ensures the raw
/// `auth_key` is never directly exposed over the wire.
///
/// # Arguments
/// * `key` - The key string to hash
///
/// # Returns
/// A 64-character lowercase hex string.
fn hash_key(key: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(key.as_bytes());
    hex_encode(&hasher.finalize())
}

/// Returns whether the master password hash file exists on disk.
fn password_exists() -> bool {
    hash_path().exists()
}

/// Returns whether the session file exists on disk.
fn session_exists() -> bool {
    session_path().exists()
}

/// Sets Unix permissions to 0600 (owner read/write only) on the given path.
///
/// This is a no-op on non-Unix platforms.
///
/// # Arguments
/// * `path` - Path to the file whose permissions to restrict
fn set_restricted_permissions(path: &std::path::Path) {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if let Ok(meta) = fs::metadata(path) {
            let mut perms = meta.permissions();
            perms.set_mode(0o600);
            fs::set_permissions(path, perms).ok();
        }
    }
}

/// Stores an encrypted verification of the master password to disk.
///
/// Encrypts the known plaintext `"pasm::verify"` using the password as the
/// encryption key, then writes the base64 ciphertext to the hash file.
/// The file is created with 0600 permissions. This can be used later to
/// verify the password without storing a password-equivalent hash.
///
/// # Arguments
/// * `password` - The master password to store a verification for
///
/// # Errors
/// Returns an error string if the config directory cannot be created or
/// the hash file cannot be written.
fn store_password_hash(password: &str) -> Result<(), String> {
    fs::create_dir_all(config_dir()).map_err(|e| format!("failed to create config dir: {e}"))?;
    let mcrypt = new_magic_crypt!(password, 256);
    let hash = mcrypt.encrypt_str_to_base64(VERIFY_PLAINTEXT);
    fs::write(hash_path(), hash.as_bytes()).map_err(|e| format!("failed to write hash: {e}"))?;
    set_restricted_permissions(&hash_path());
    Ok(())
}

/// Verifies a password against the stored hash file.
///
/// Decrypts the hash file content using the provided password and checks
/// whether the decrypted plaintext matches the expected verification string.
///
/// # Arguments
/// * `password` - The password to verify
///
/// # Returns
/// `true` if the password matches, `false` if the hash file is missing,
/// the password is wrong, or decryption fails for any reason.
fn verify_password(password: &str) -> bool {
    let hash = match fs::read_to_string(hash_path()) {
        Ok(h) => h.trim().to_string(),
        Err(_) => return false,
    };
    let mcrypt = new_magic_crypt!(password, 256);
    match mcrypt.decrypt_base64_to_string(&hash) {
        Ok(s) => s == VERIFY_PLAINTEXT,
        Err(_) => false,
    }
}

/// Writes the session keys to the session JSON file with 0600 permissions.
///
/// # Arguments
/// * `api_key` - The API key (Bearer token) to store
/// * `encr_key` - The encryption key to store
///
/// # Errors
/// Returns an error string if serialization or file I/O fails.
fn store_session(api_key: &str, encr_key: &str) -> Result<(), String> {
    let data = SessionData {
        api_key: api_key.to_string(),
        encr_key: encr_key.to_string(),
    };
    let json = serde_json::to_string(&data).map_err(|e| format!("session serialization: {e}"))?;
    fs::write(session_path(), json.as_bytes())
        .map_err(|e| format!("failed to write session: {e}"))?;
    set_restricted_permissions(&session_path());
    Ok(())
}

/// Deletes the session file from disk.
///
/// This does not invalidate the server-side API key; it just removes
/// local credentials, requiring the user to log in again.
fn destroy_session() -> Result<(), String> {
    if session_path().exists() {
        fs::remove_file(session_path()).map_err(|e| format!("failed to remove session: {e}"))
    } else {
        Ok(())
    }
}

/// Derives the full API key from the master password.
///
/// Two-step derivation:
/// 1. `auth_key = SHA-256("pasm-auth" || password)`
/// 2. `api_key = SHA-256(auth_key)`
///
/// The intermediate `auth_key` is never sent over the wire; only the
/// final `api_key` is registered with the server and used as Bearer token.
///
/// # Arguments
/// * `password` - The master password
///
/// # Returns
/// A 64-character hex string used as the API Bearer token.
fn derive_api_key(password: &str) -> String {
    let auth_key = derive_key(password, "pasm-auth");
    hash_key(&auth_key)
}

/// Reads and returns the session keys from the session file, if it exists.
///
/// # Returns
/// `Some((api_key, encr_key))` if the session file exists and is valid JSON,
/// `None` otherwise.
pub fn get_session_keys() -> Option<(String, String)> {
    let path = session_path();
    if !path.exists() {
        return None;
    }
    let content = fs::read_to_string(path).ok()?;
    let data: SessionData = serde_json::from_str(&content).ok()?;
    Some((data.api_key, data.encr_key))
}

/// Logs the user in by creating or verifying the master password and setting up a session.
///
/// **First-time flow:**
/// 1. Displays a password criticality warning
/// 2. Prompts for and confirms a new master password
/// 3. Stores a password verification hash on disk
/// 4. Derives `api_key` and `encr_key` from the password
/// 5. Registers the `api_key` with the server via `POST /auth`
/// 6. Saves the session keys to the session file
///
/// **Subsequent flow:**
/// 1. If a session already exists, offers auto-login (skip re-entering password)
/// 2. Otherwise, prompts for the master password (up to 3 attempts)
/// 3. Verifies the password against the stored hash
/// 4. Re-derives keys and re-creates the session file
///
/// # Returns
/// A status message string indicating success or describing the failure.
pub fn login() -> String {
    if !password_exists() {
        println!(
            "\n{} {}\n{}\n{}\n",
            "⚠".yellow().bold(),
            "MASTER PASSWORD SETUP".yellow().bold(),
            "This password protects ALL your stored credentials."
                .red()
                .bold(),
            "If you lose it, your data CANNOT be recovered."
                .red()
                .bold(),
        );

        let password = prompt_hidden("Create master password: ");
        if password.is_empty() {
            return "password cannot be empty".to_string();
        }
        let confirm = prompt_hidden("Confirm master password: ");
        if password != confirm {
            return "passwords do not match".to_string();
        }

        if let Err(e) = store_password_hash(&password) {
            return format!("failed to store password: {e}");
        }

        let api_key = derive_api_key(&password);
        let encr_key = derive_key(&password, "pasm-encr");

        let reg_res = requests::register_auth(&api_key);
        if reg_res.starts_with("Error") {
            eprintln!("Warning: server registration failed. API calls may not work.");
        }

        if let Err(e) = store_session(&api_key, &encr_key) {
            return format!("failed to create session: {e}");
        }

        return "master password created and logged in".to_string();
    }

    if session_exists() {
        let answer = prompt_visible("Auto-login using existing session? [Y/n]: ");
        if !answer.starts_with('n') && !answer.starts_with('N') {
            return "auto-logged in".to_string();
        }
    }

    for attempt in 1..=3 {
        let password = prompt_hidden("Master password: ");
        if verify_password(&password) {
            let api_key = derive_api_key(&password);
            let encr_key = derive_key(&password, "pasm-encr");
            if let Err(e) = store_session(&api_key, &encr_key) {
                return format!("failed to create session: {e}");
            }
            return "logged in".to_string();
        }
        if attempt < 3 {
            println!("incorrect password (attempt {attempt}/3)");
        }
    }

    "too many failed attempts".to_string()
}

/// Logs the user out by deleting the local session file.
///
/// The server-side API key remains registered and is not invalidated.
///
/// # Returns
/// A status message string indicating success or failure.
pub fn logout() -> String {
    match destroy_session() {
        Ok(_) => "logged out".to_string(),
        Err(e) => format!("logout failed: {e}"),
    }
}

/// Prompts the user for a yes/no response with visible input.
///
/// # Arguments
/// * `msg` - The prompt message (e.g., `"Auto-login? [Y/n]: "`)
///
/// # Returns
/// The trimmed response string, or `"y"` if the input was empty.
fn prompt_visible(msg: &str) -> String {
    print!("{msg}");
    io::stdout().flush().ok();
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok();
    input.trim().to_string()
}
