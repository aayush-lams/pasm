use magic_crypt::{new_magic_crypt, MagicCryptTrait};

/// This function encrypts the content passed to it.
/// It takes some `String` content and encryption key to encrypt the content.
/// # Example
/// ```rust
/// let encr_string = encrypt_string(some_string, encr_pass_key)
/// ```
pub fn encrypt_string<'a>(data: String, password: String) -> String {
    let mcrypt = new_magic_crypt!(password.as_str(), 256);
    let binding = mcrypt.encrypt_str_to_base64(&data);
    binding
}
