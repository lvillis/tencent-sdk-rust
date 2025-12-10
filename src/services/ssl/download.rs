use crate::{
    client::{TencentCloudAsync, TencentCloudBlocking},
    core::{Endpoint, TencentCloudResult},
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::borrow::Cow;

#[derive(Debug, Deserialize)]
pub struct DownloadCertificateResponse {
    #[serde(rename = "Response")]
    pub response: DownloadCertificateResult,
}

#[derive(Debug, Deserialize)]
pub struct DownloadCertificateResult {
    #[serde(rename = "Content")]
    pub content: Option<String>,
    #[serde(rename = "ContentType")]
    pub content_type: Option<String>,
    #[serde(rename = "RequestId")]
    pub request_id: String,
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
struct DownloadCertificatePayload<'a> {
    certificate_id: &'a str,
}

/// Request payload for `DownloadCertificate`.
pub struct DownloadCertificate<'a> {
    pub certificate_id: &'a str,
}

impl<'a> DownloadCertificate<'a> {
    /// Create a new certificate download request
    pub fn new(certificate_id: &'a str) -> Self {
        Self { certificate_id }
    }
}

impl<'a> Endpoint for DownloadCertificate<'a> {
    type Output = DownloadCertificateResponse;

    fn service(&self) -> Cow<'static, str> {
        Cow::Borrowed("ssl")
    }

    fn action(&self) -> Cow<'static, str> {
        Cow::Borrowed("DownloadCertificate")
    }

    fn version(&self) -> Cow<'static, str> {
        Cow::Borrowed("2019-12-05")
    }

    fn region(&self) -> Option<Cow<'_, str>> {
        // SSL APIs do not require a region parameter
        None
    }

    fn payload(&self) -> Value {
        serde_json::to_value(DownloadCertificatePayload {
            certificate_id: self.certificate_id,
        })
        .expect("serialize DownloadCertificate payload")
    }
}

/// Call SSL `DownloadCertificate` with the async client.
pub async fn download_certificate_async(
    client: &TencentCloudAsync,
    request: &DownloadCertificate<'_>,
) -> TencentCloudResult<DownloadCertificateResponse> {
    client.request(request).await
}

/// Call SSL `DownloadCertificate` with the blocking client.
pub fn download_certificate_blocking(
    client: &TencentCloudBlocking,
    request: &DownloadCertificate<'_>,
) -> TencentCloudResult<DownloadCertificateResponse> {
    client.request(request)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn download_certificate_payload_serialization() {
        let request = DownloadCertificate::new("cert-xyz789");
        let payload = request.payload();
        assert_eq!(payload["CertificateId"], json!("cert-xyz789"));
    }

    #[test]
    fn deserialize_download_certificate_response() {
        let payload = r#"{
            "Response": {
                "Content": "UEsDBBQACAgIABdFg1YAAAAAAAAAAAAAAA...",
                "ContentType": "application/zip",
                "RequestId": "req-download-123"
            }
        }"#;
        let parsed: DownloadCertificateResponse =
            serde_json::from_str(payload).expect("deserialize DownloadCertificateResponse");
        assert!(parsed
            .response
            .content
            .as_deref()
            .expect("content present")
            .starts_with("UEsDBBQACAgIABdFg1Y"));
        assert_eq!(
            parsed.response.content_type,
            Some("application/zip".to_string())
        );
        assert_eq!(parsed.response.request_id, "req-download-123");
    }
}
