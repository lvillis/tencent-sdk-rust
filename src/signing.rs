use crate::core::Credentials;
use chrono::{TimeZone, Utc};
use hmac::{Hmac, Mac};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use thiserror::Error;

pub type HmacSha256 = Hmac<Sha256>;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum SigningError {
    #[error("timestamp {0} is out of range for chrono")]
    InvalidTimestamp(i64),
}

pub struct SigningInput<'a> {
    pub service: &'a str,
    pub host: &'a str,
    pub path: &'a str,
    pub region: Option<&'a str>,
    pub action: &'a str,
    pub version: &'a str,
    pub payload: &'a str,
    pub timestamp: i64,
}

pub fn build_tc3_headers(
    credentials: &Credentials,
    input: &SigningInput<'_>,
) -> Result<HashMap<String, String>, SigningError> {
    let SigningInput {
        service,
        host,
        path,
        region,
        action,
        version,
        payload,
        timestamp,
    } = input;

    let algorithm = "TC3-HMAC-SHA256";
    let canonical_query = "";
    let content_type = "application/json; charset=utf-8";
    let lower_action = action.to_ascii_lowercase();
    let canonical_headers = format!(
        "content-type:{}\nhost:{}\nx-tc-action:{}\n",
        content_type, host, lower_action
    );
    let signed_headers = "content-type;host;x-tc-action";

    let hashed_payload = {
        let mut hasher = Sha256::new();
        hasher.update(payload.as_bytes());
        format!("{:x}", hasher.finalize())
    };

    let canonical_request = format!(
        "POST\n{}\n{}\n{}\n{}\n{}",
        path, canonical_query, canonical_headers, signed_headers, hashed_payload
    );

    let datetime = Utc
        .timestamp_opt(*timestamp, 0)
        .single()
        .ok_or(SigningError::InvalidTimestamp(*timestamp))?;
    let date = datetime.format("%Y-%m-%d").to_string();

    let hashed_canonical_request = {
        let mut hasher = Sha256::new();
        hasher.update(canonical_request.as_bytes());
        format!("{:x}", hasher.finalize())
    };

    let credential_scope = format!("{}/{}/tc3_request", date, service);
    let string_to_sign = format!(
        "{}\n{}\n{}\n{}",
        algorithm, timestamp, credential_scope, hashed_canonical_request
    );

    let secret_date = hmac_sha256(format!("TC3{}", credentials.secret_key).as_bytes(), &date);
    let secret_service = hmac_sha256(&secret_date, service);
    let secret_signing = hmac_sha256(&secret_service, "tc3_request");

    let signature = {
        let mut mac =
            HmacSha256::new_from_slice(&secret_signing).expect("HMAC can accept any key length");
        mac.update(string_to_sign.as_bytes());
        format!("{:x}", mac.finalize().into_bytes())
    };

    let authorization = format!(
        "{} Credential={}/{}, SignedHeaders={}, Signature={}",
        algorithm, credentials.secret_id, credential_scope, signed_headers, signature
    );

    let mut headers = HashMap::new();
    headers.insert("Authorization".to_string(), authorization);
    headers.insert("Content-Type".to_string(), content_type.to_string());
    headers.insert("Host".to_string(), (*host).to_string());
    headers.insert("X-TC-Action".to_string(), (*action).to_string());
    headers.insert("X-TC-Version".to_string(), (*version).to_string());
    headers.insert("X-TC-Timestamp".to_string(), timestamp.to_string());
    if let Some(value) = region {
        headers.insert("X-TC-Region".to_string(), (*value).to_string());
    }
    if let Some(token) = credentials.token.as_deref() {
        headers.insert("X-TC-Token".to_string(), token.to_string());
    }

    Ok(headers)
}

fn hmac_sha256(key: &[u8], msg: &str) -> Vec<u8> {
    let mut mac = HmacSha256::new_from_slice(key).expect("HMAC can accept any key length");
    mac.update(msg.as_bytes());
    mac.finalize().into_bytes().to_vec()
}
