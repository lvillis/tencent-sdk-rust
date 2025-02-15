use chrono::{TimeZone, Utc};
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Client;
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::error::Error;

use crate::signing::hmac_sha256;
use hmac::{Hmac, Mac};

type HmacSha256 = Hmac<Sha256>;

/// Tencent Cloud SDK Client
///
/// This client is used to send authenticated requests to Tencent Cloud APIs.
/// It constructs the canonical request string, computes the TC3-HMAC-SHA256 signature,
/// builds the necessary headers, and sends an HTTPS POST request.
/// This version returns the response as a `serde_json::Value`, preserving Chinese characters.
pub struct TencentCloudClient {
    /// Your Tencent Cloud SecretId.
    pub secret_id: String,
    /// Your Tencent Cloud SecretKey.
    pub secret_key: String,
    /// Optional token.
    pub token: Option<String>,
}

impl TencentCloudClient {
    /// Creates a new TencentCloudClient.
    ///
    /// # Arguments
    ///
    /// * `secret_id` - Your Tencent Cloud SecretId.
    /// * `secret_key` - Your Tencent Cloud SecretKey.
    /// * `token` - An optional token.
    pub fn new(secret_id: &str, secret_key: &str, token: Option<&str>) -> Self {
        Self {
            secret_id: secret_id.to_owned(),
            secret_key: secret_key.to_owned(),
            token: token.map(|s| s.to_owned()),
        }
    }

    /// Asynchronous general request function.
    ///
    /// This method constructs the canonical request, computes the TC3-HMAC-SHA256 signature,
    /// builds the Authorization header, and sends an HTTPS POST request.
    ///
    /// Instead of returning plain text, this version parses the response as JSON
    /// and returns a `serde_json::Value`, which preserves Chinese characters.
    ///
    /// # Arguments
    ///
    /// * `service` - The service name (e.g., "cvm").
    /// * `host` - The request host (e.g., "cvm.tencentcloudapi.com").
    /// * `region` - Optional region string.
    /// * `version` - API version (e.g., "2017-03-12").
    /// * `action` - API action name (e.g., "DescribeInstances").
    /// * `payload` - The request body as a JSON string.
    ///
    /// # Returns
    ///
    /// A `Result` containing the response parsed as `serde_json::Value` on success,
    /// or a boxed error on failure.
    pub async fn request(
        &self,
        service: &str,
        host: &str,
        region: Option<&str>,
        version: &str,
        action: &str,
        payload: &str,
    ) -> Result<Value, Box<dyn Error>> {
        let algorithm = "TC3-HMAC-SHA256";
        let ct = "application/json; charset=utf-8";

        // Step 1: Construct the canonical request string.
        let http_request_method = "POST";
        let canonical_uri = "/";
        let canonical_querystring = "";
        let canonical_headers = format!(
            "content-type:{}\nhost:{}\nx-tc-action:{}\n",
            ct,
            host,
            action.to_lowercase()
        );
        let signed_headers = "content-type;host;x-tc-action";
        let hashed_request_payload = {
            let mut hasher = Sha256::new();
            hasher.update(payload.as_bytes());
            format!("{:x}", hasher.finalize())
        };
        let canonical_request = format!(
            "{}\n{}\n{}\n{}\n{}\n{}",
            http_request_method,
            canonical_uri,
            canonical_querystring,
            canonical_headers,
            signed_headers,
            hashed_request_payload
        );

        // Step 2: Construct the string to sign.
        let timestamp = Utc::now().timestamp();
        let date = Utc.timestamp(timestamp, 0).format("%Y-%m-%d").to_string();
        let credential_scope = format!("{}/{}/tc3_request", date, service);
        let hashed_canonical_request = {
            let mut hasher = Sha256::new();
            hasher.update(canonical_request.as_bytes());
            format!("{:x}", hasher.finalize())
        };
        let string_to_sign = format!(
            "{}\n{}\n{}\n{}",
            algorithm, timestamp, credential_scope, hashed_canonical_request
        );

        // Step 3: Compute the signature.
        let secret_date = hmac_sha256(format!("TC3{}", self.secret_key).as_bytes(), &date);
        let secret_service = hmac_sha256(&secret_date, service);
        let secret_signing = hmac_sha256(&secret_service, "tc3_request");
        let signature = {
            let mut mac = HmacSha256::new_from_slice(&secret_signing)
                .expect("HMAC can accept any key length");
            mac.update(string_to_sign.as_bytes());
            format!("{:x}", mac.finalize().into_bytes())
        };

        // Step 4: Construct the Authorization header.
        let authorization = format!(
            "{} Credential={}/{}, SignedHeaders={}, Signature={}",
            algorithm, self.secret_id, credential_scope, signed_headers, signature
        );

        // Step 5: Build headers and send the request.
        let mut headers = HeaderMap::new();
        headers.insert("Authorization", HeaderValue::from_str(&authorization)?);
        headers.insert("Content-Type", HeaderValue::from_static(ct));
        headers.insert("Host", HeaderValue::from_str(host)?);
        headers.insert("X-TC-Action", HeaderValue::from_str(action)?);
        headers.insert("X-TC-Timestamp", HeaderValue::from_str(&timestamp.to_string())?);
        headers.insert("X-TC-Version", HeaderValue::from_str(version)?);
        if let Some(r) = region {
            headers.insert("X-TC-Region", HeaderValue::from_str(r)?);
        }
        if let Some(t) = &self.token {
            headers.insert("X-TC-Token", HeaderValue::from_str(t)?);
        }

        let url = format!("https://{}", host);
        let client = Client::new();
        let resp_json: Value = client
            .post(&url)
            .headers(headers)
            .body(payload.to_owned())
            .send()
            .await?
            .json()
            .await?;
        Ok(resp_json)
    }
}
