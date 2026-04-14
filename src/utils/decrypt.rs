use magic_crypt::{new_magic_crypt, MagicCryptTrait};

use crate::types::error::PasmErrors;

/// This function decrypts the content passed to it.
/// It takes some encrypted `String` content and encryption key to decrypt the content.
/// # Example
/// ```rust
/// let normal_string = decrypt_string(encr_string, encr_pass_key)
/// ```
pub fn decrypt_string<'a>(data: String, password: String) -> Result<String, PasmErrors> {
    let mcrypt = new_magic_crypt!(password.as_str(), 256);
    let binding = mcrypt
        .decrypt_base64_to_string(&data)
        .map_err(|err| PasmErrors::DecryptionError { err })?;
    Ok(binding)
}
