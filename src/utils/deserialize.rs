use crate::{
    types::{detail::Details, error::PasmErrors},
    utils::decrypt::decrypt_string,
};
use sled::IVec;

/// This function deserializes the IVec content from sled database to `Details` object.
/// It uses `decrypt_string` hence requires encryption key
/// # Example
/// ```rust
/// let details = deserialise_entry(ivec_data, encr_string).unwrap()
/// ```
pub fn deserialize_entry(value: IVec, passkey: String) -> Result<Details, PasmErrors> {
    let cred =
        String::from_utf8(value.to_vec()).map_err(|err| PasmErrors::UTF8ConversionError { err })?;
    let decrypt_str = decrypt_string(cred, passkey)?;
    let details = serde_json::from_str(&decrypt_str)
        .map_err(|err| PasmErrors::DeserializationError { err })?;
    Ok(details)
}
