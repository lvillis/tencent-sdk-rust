use crate::{Error, auth::Auth};
use chrono::{TimeZone, Utc};
use hmac::{Hmac, Mac};
use http::{HeaderMap, HeaderValue, Method};
use sha2::{Digest, Sha256};
use thiserror::Error as ThisError;

pub(crate) type HmacSha256 = Hmac<Sha256>;

#[derive(Debug, ThisError)]
#[non_exhaustive]
pub(crate) enum SigningError {
    #[error("missing credentials")]
    MissingCredentials,

    #[error("timestamp {0} is out of range for chrono")]
    InvalidTimestamp(i64),
}

pub(crate) struct SigningInput<'a> {
    pub(crate) method: &'a Method,
    pub(crate) service: &'a str,
    pub(crate) host: &'a str,
    pub(crate) path: &'a str,
    pub(crate) canonical_query: &'a str,
    pub(crate) region: Option<&'a str>,
    pub(crate) action: &'a str,
    pub(crate) version: &'a str,
    pub(crate) payload: &'a str,
    pub(crate) timestamp: i64,
}

pub(crate) fn build_tc3_headers(auth: &Auth, input: &SigningInput<'_>) -> Result<HeaderMap, Error> {
    let Auth::Tc3(credentials) = auth else {
        return Err(Error::signing(Box::new(SigningError::MissingCredentials)));
    };

    let SigningInput {
        method,
        service,
        host,
        path,
        canonical_query,
        region,
        action,
        version,
        payload,
        timestamp,
    } = input;

    let algorithm = "TC3-HMAC-SHA256";
    let content_type = "application/json; charset=utf-8";
    let lower_action = action.to_ascii_lowercase();
    let canonical_headers =
        format!("content-type:{content_type}\nhost:{host}\nx-tc-action:{lower_action}\n");
    let signed_headers = "content-type;host;x-tc-action";

    let hashed_payload = sha256_hex(payload.as_bytes());
    let canonical_request = format!(
        "{}\n{}\n{}\n{}\n{}\n{}",
        method.as_str(),
        path,
        canonical_query,
        canonical_headers,
        signed_headers,
        hashed_payload
    );

    let datetime = Utc
        .timestamp_opt(*timestamp, 0)
        .single()
        .ok_or(SigningError::InvalidTimestamp(*timestamp))
        .map_err(|source| Error::signing(Box::new(source)))?;
    let date = datetime.format("%Y-%m-%d").to_string();

    let hashed_canonical_request = sha256_hex(canonical_request.as_bytes());
    let credential_scope = format!("{date}/{service}/tc3_request");
    let string_to_sign =
        format!("{algorithm}\n{timestamp}\n{credential_scope}\n{hashed_canonical_request}");

    let secret_date = hmac_sha256(format!("TC3{}", credentials.secret_key()).as_bytes(), &date)?;
    let secret_service = hmac_sha256(&secret_date, service)?;
    let secret_signing = hmac_sha256(&secret_service, "tc3_request")?;

    let mut mac = HmacSha256::new_from_slice(&secret_signing)
        .map_err(|source| Error::signing(Box::new(source)))?;
    mac.update(string_to_sign.as_bytes());
    let signature = format!("{:x}", mac.finalize().into_bytes());

    let authorization = format!(
        "{algorithm} Credential={}/{credential_scope}, SignedHeaders={signed_headers}, Signature={signature}",
        credentials.secret_id(),
    );

    let mut headers = HeaderMap::new();
    headers.insert(
        "Authorization",
        HeaderValue::from_str(&authorization).map_err(|source| Error::signing(Box::new(source)))?,
    );
    headers.insert(
        "Content-Type",
        HeaderValue::from_str(content_type).map_err(|source| Error::signing(Box::new(source)))?,
    );
    headers.insert(
        "Host",
        HeaderValue::from_str(host).map_err(|source| Error::signing(Box::new(source)))?,
    );
    headers.insert(
        "X-TC-Action",
        HeaderValue::from_str(action).map_err(|source| Error::signing(Box::new(source)))?,
    );
    headers.insert(
        "X-TC-Version",
        HeaderValue::from_str(version).map_err(|source| Error::signing(Box::new(source)))?,
    );
    headers.insert(
        "X-TC-Timestamp",
        HeaderValue::from_str(&timestamp.to_string())
            .map_err(|source| Error::signing(Box::new(source)))?,
    );
    if let Some(region) = region {
        headers.insert(
            "X-TC-Region",
            HeaderValue::from_str(region).map_err(|source| Error::signing(Box::new(source)))?,
        );
    }
    if let Some(token) = credentials.token() {
        headers.insert(
            "X-TC-Token",
            HeaderValue::from_str(token).map_err(|source| Error::signing(Box::new(source)))?,
        );
    }

    Ok(headers)
}

fn sha256_hex(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    format!("{:x}", hasher.finalize())
}

fn hmac_sha256(key: &[u8], msg: &str) -> Result<Vec<u8>, Error> {
    let mut mac =
        HmacSha256::new_from_slice(key).map_err(|source| Error::signing(Box::new(source)))?;
    mac.update(msg.as_bytes());
    Ok(mac.finalize().into_bytes().to_vec())
}

#[cfg(test)]
mod tests {
    use super::*;
    use http::Method;
    use serde_json::json;

    #[test]
    fn tc3_authorization_snapshot_matches_reference() {
        let auth = Auth::tc3(
            "AKIDz8krbsJ5yKBZQpn74WFkmLPx3xxxx",
            "Gu5t9xGARNpq86cd98joQYCN3Cozxxxx",
        );
        let method = Method::POST;
        let payload = json!({
            "Limit": 1,
            "Filters": [
                { "Name": "zone", "Values": ["ap-guangzhou-1"] }
            ]
        })
        .to_string();
        let headers = build_tc3_headers(
            &auth,
            &SigningInput {
                method: &method,
                service: "cvm",
                host: "cvm.tencentcloudapi.com",
                path: "/",
                canonical_query: "",
                region: Some("ap-guangzhou"),
                action: "DescribeInstances",
                version: "2017-03-12",
                payload: &payload,
                timestamp: 1551113065,
            },
        )
        .expect("build headers");

        let authorization = headers
            .get("Authorization")
            .expect("authorization header exists")
            .to_str()
            .expect("authorization header is valid utf-8");
        assert_eq!(
            authorization,
            "TC3-HMAC-SHA256 Credential=AKIDz8krbsJ5yKBZQpn74WFkmLPx3xxxx/2019-02-25/cvm/tc3_request, SignedHeaders=content-type;host;x-tc-action, Signature=fb562f0e44f0c7f0afa9eff2998c6fc41e053d0efa3741b068332b545afdb587"
        );
    }
}
