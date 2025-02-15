use hmac::{Hmac, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

/// Computes an HMAC-SHA256 signature for the provided message using the given key.
///
/// # Arguments
///
/// * `key` - A byte slice representing the secret key.
/// * `msg` - The message to be signed as a string slice.
///
/// # Returns
///
/// A vector of bytes containing the HMAC-SHA256 signature.
///
/// # Panics
///
/// This function panics if the key is invalid for HMAC initialization.
pub fn hmac_sha256(key: &[u8], msg: &str) -> Vec<u8> {
    let mut mac = HmacSha256::new_from_slice(key).expect("HMAC can accept any key length");
    mac.update(msg.as_bytes());
    mac.finalize().into_bytes().to_vec()
}
