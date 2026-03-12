use crate::types::Details;
use magic_crypt::new_magic_crypt;
use magic_crypt::MagicCryptTrait;
use sled::IVec;

/// This function decrypts the content passed to it.
/// It takes some encrypted `String` content and encryption key to decrypt the content.
/// # Example
/// ```rust
/// let normal_string = decrypt_string(encr_string, encr_pass_key)
/// ```
fn decrypt_string<'a>(data: String, password: String) -> String {
    let mcrypt = new_magic_crypt!(password.as_str(), 256);
    let binding = mcrypt
        .decrypt_base64_to_string(&data)
        .expect(&"error converting base64 to String!".to_string());
    binding
}

/// This function encrypts the content passed to it.
/// It takes some `String` content and encryption key to encrypt the content.
/// # Example
/// ```rust
/// let encr_string = encrypt_string(some_string, encr_pass_key)
/// ```
fn encrypt_string<'a>(data: String, password: String) -> String {
    let mcrypt = new_magic_crypt!(password.as_str(), 256);
    let binding = mcrypt.encrypt_str_to_base64(&data);
    binding
}

/// This function deserializes the IVec content from sled database to `Details` object.
/// It uses `decrypt_string` hence requires encryption key
/// # Example
/// ```rust
/// let details = deserialise_entry(ivec_data, encr_string).unwrap()
/// ```
pub fn deserialize_entry(value: IVec, passkey: String) -> Result<Details, String> {
    let cred = String::from_utf8(value.to_vec()).unwrap();
    let decrypt_str = decrypt_string(cred, passkey);
    let details = serde_json::from_str(&decrypt_str).expect("failed to deserialize entry");
    Ok(details)
}

/// This function serializes the `Details` object to sled IVec data.
/// It uses `encrypt_string` hence requires encryption key
/// # Example
/// ```rust
/// let ivec_content = encrypt_string(details, encr_string).unwrap()
/// ```
pub fn serialize_entry(details: Details, passkey: String) -> String {
    let detail_str = serde_json::to_string(&details).expect("error converting to string");
    let crypt_text = encrypt_string(detail_str, passkey);
    crypt_text
}
