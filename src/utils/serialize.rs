use crate::{
    types::{detail::Details, error::PasmErrors},
    utils::encrypt::encrypt_string,
};

/// This function serializes the `Details` object to sled IVec data.
/// It uses `encrypt_string` hence requires encryption key
/// # Example
/// ```rust
/// let ivec_content = encrypt_string(details, encr_string).unwrap()
/// ```
pub fn serialize_entry(details: Details, passkey: String) -> Result<String, PasmErrors> {
    let detail_str =
        serde_json::to_string(&details).map_err(|err| PasmErrors::SerializationError { err })?;
    let crypt_text = encrypt_string(detail_str, passkey);
    Ok(crypt_text)
}
