use crate::{
    client::{TencentCloudAsync, TencentCloudBlocking},
    core::{Endpoint, TencentCloudResult},
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::borrow::Cow;

#[derive(Debug, Deserialize)]
pub struct UploadCertificateResponse {
    #[serde(rename = "Response")]
    pub response: UploadCertificateResult,
}

#[derive(Debug, Deserialize)]
pub struct UploadCertificateResult {
    #[serde(rename = "CertificateId")]
    pub certificate_id: Option<String>,
    #[serde(rename = "RepeatCertId")]
    pub repeat_cert_id: Option<String>,
    #[serde(rename = "RequestId")]
    pub request_id: String,
}

#[derive(Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum CertificateType<'a> {
    #[serde(rename = "CA")]
    Ca,
    #[serde(rename = "SVR")]
    Svr,
    Custom(&'a str),
}

impl<'a> From<&'a str> for CertificateType<'a> {
    fn from(s: &'a str) -> Self {
        match s.to_uppercase().as_str() {
            "CA" => CertificateType::Ca,
            "SVR" => CertificateType::Svr,
            _ => CertificateType::Custom(s),
        }
    }
}

impl<'a> CertificateType<'a> {
    pub fn as_str(&self) -> &str {
        match self {
            CertificateType::Ca => "CA",
            CertificateType::Svr => "SVR",
            CertificateType::Custom(value) => value,
        }
    }
}

#[derive(Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum CertificateUse<'a> {
    #[serde(rename = "CLB")]
    Clb,
    #[serde(rename = "CDN")]
    Cdn,
    #[serde(rename = "WAF")]
    Waf,
    #[serde(rename = "LIVE")]
    Live,
    #[serde(rename = "DDOS")]
    Ddos,
    Custom(&'a str),
}

impl<'a> From<&'a str> for CertificateUse<'a> {
    fn from(s: &'a str) -> Self {
        match s.to_uppercase().as_str() {
            "CLB" => CertificateUse::Clb,
            "CDN" => CertificateUse::Cdn,
            "WAF" => CertificateUse::Waf,
            "LIVE" => CertificateUse::Live,
            "DDOS" => CertificateUse::Ddos,
            _ => CertificateUse::Custom(s),
        }
    }
}

impl<'a> CertificateUse<'a> {
    pub fn as_str(&self) -> &str {
        match self {
            CertificateUse::Clb => "CLB",
            CertificateUse::Cdn => "CDN",
            CertificateUse::Waf => "WAF",
            CertificateUse::Live => "LIVE",
            CertificateUse::Ddos => "DDOS",
            CertificateUse::Custom(value) => value,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Tag<'a> {
    #[serde(rename = "TagKey")]
    pub key: &'a str,
    #[serde(rename = "TagValue")]
    pub value: &'a str,
}

/// Request payload for `UploadCertificate`.
pub struct UploadCertificate<'a> {
    pub region: Option<&'a str>,
    pub certificate_public_key: &'a str,
    pub certificate_private_key: Option<&'a str>,
    pub certificate_type: Option<CertificateType<'a>>,
    pub alias: Option<&'a str>,
    pub project_id: Option<u64>,
    pub certificate_use: Option<CertificateUse<'a>>,
    pub tags: Option<Vec<Tag<'a>>>,
    pub repeatable: Option<bool>,
    pub key_password: Option<&'a str>,
}

impl<'a> UploadCertificate<'a> {
    /// Create a new upload certificate request with the required public key.
    pub fn new(certificate_public_key: &'a str) -> Self {
        Self {
            region: None,
            certificate_public_key,
            certificate_private_key: None,
            certificate_type: None,
            alias: None,
            project_id: None,
            certificate_use: None,
            tags: None,
            repeatable: None,
            key_password: None,
        }
    }

    /// Set the region (not required according to documentation, but kept for consistency).
    pub fn with_region(mut self, region: &'a str) -> Self {
        self.region = Some(region);
        self
    }

    /// Set the private key (required for SVR certificates).
    pub fn with_private_key(mut self, private_key: &'a str) -> Self {
        self.certificate_private_key = Some(private_key);
        self
    }

    /// Set the certificate type (CA or SVR).
    pub fn with_certificate_type(mut self, cert_type: &'a str) -> Self {
        self.certificate_type = Some(cert_type.into());
        self
    }

    /// Set an alias/name for the certificate.
    pub fn with_alias(mut self, alias: &'a str) -> Self {
        self.alias = Some(alias);
        self
    }

    /// Set the project ID.
    pub fn with_project_id(mut self, project_id: u64) -> Self {
        self.project_id = Some(project_id);
        self
    }

    /// Set the certificate use/source (CLB, CDN, WAF, LIVE, DDOS).
    pub fn with_certificate_use(mut self, cert_use: &'a str) -> Self {
        self.certificate_use = Some(cert_use.into());
        self
    }

    /// Add a tag to the certificate.
    pub fn with_tag(mut self, key: &'a str, value: &'a str) -> Self {
        let tag = Tag { key, value };
        self.tags.get_or_insert_with(Vec::new).push(tag);
        self
    }

    /// Set whether duplicate certificates are allowed.
    pub fn with_repeatable(mut self, repeatable: bool) -> Self {
        self.repeatable = Some(repeatable);
        self
    }

    /// Set the private key password.
    pub fn with_key_password(mut self, password: &'a str) -> Self {
        self.key_password = Some(password);
        self
    }
}

impl<'a> Endpoint for UploadCertificate<'a> {
    type Output = UploadCertificateResponse;

    fn service(&self) -> Cow<'static, str> {
        Cow::Borrowed("ssl")
    }

    fn action(&self) -> Cow<'static, str> {
        Cow::Borrowed("UploadCertificate")
    }

    fn version(&self) -> Cow<'static, str> {
        Cow::Borrowed("2019-12-05")
    }

    fn region(&self) -> Option<Cow<'_, str>> {
        self.region.map(Cow::Borrowed)
    }

    fn payload(&self) -> Value {
        let mut payload = serde_json::json!({
            "CertificatePublicKey": self.certificate_public_key,
        });

        if let Some(private_key) = self.certificate_private_key {
            payload["CertificatePrivateKey"] = serde_json::json!(private_key);
        }
        if let Some(cert_type) = &self.certificate_type {
            payload["CertificateType"] = serde_json::json!(cert_type.as_str());
        }
        if let Some(alias) = self.alias {
            payload["Alias"] = serde_json::json!(alias);
        }
        if let Some(project_id) = self.project_id {
            payload["ProjectId"] = serde_json::json!(project_id);
        }
        if let Some(cert_use) = &self.certificate_use {
            payload["CertificateUse"] = serde_json::json!(cert_use.as_str());
        }
        if let Some(tags) = &self.tags {
            payload["Tags"] = serde_json::json!(tags);
        }
        if let Some(repeatable) = self.repeatable {
            payload["Repeatable"] = serde_json::json!(repeatable);
        }
        if let Some(key_password) = self.key_password {
            payload["KeyPassword"] = serde_json::json!(key_password);
        }

        payload
    }
}

/// Upload a certificate with the async client.
pub async fn upload_certificate_async(
    client: &TencentCloudAsync,
    request: &UploadCertificate<'_>,
) -> TencentCloudResult<UploadCertificateResponse> {
    client.request(request).await
}

/// Upload a certificate with the blocking client.
pub fn upload_certificate_blocking(
    client: &TencentCloudBlocking,
    request: &UploadCertificate<'_>,
) -> TencentCloudResult<UploadCertificateResponse> {
    client.request(request)
}

#[cfg(test)]
mod tests {

    use super::*;
    use serde_json::json;

    #[test]
    fn upload_certificate_payload_basic() {
        let public_key = "-----BEGIN CERTIFICATE-----\n...\n-----END CERTIFICATE-----";
        let private_key = "-----BEGIN RSA PRIVATE KEY-----\n...\n-----END RSA PRIVATE KEY-----";

        let request = UploadCertificate::new(public_key)
            .with_private_key(private_key)
            .with_certificate_type("SVR")
            .with_alias("my-website-cert")
            .with_project_id(123456)
            .with_certificate_use("CLB")
            .with_repeatable(false);

        let payload = request.payload();

        assert!(payload["CertificatePublicKey"]
            .as_str()
            .expect("certificate public key should be string")
            .contains("BEGIN CERTIFICATE"));
        assert!(payload["CertificatePrivateKey"]
            .as_str()
            .expect("certificate private key should be string")
            .contains("BEGIN RSA PRIVATE KEY"));
        assert_eq!(payload["CertificateType"], json!("SVR"));
        assert_eq!(payload["Alias"], json!("my-website-cert"));
        assert_eq!(payload["ProjectId"], json!(123456));
        assert_eq!(payload["CertificateUse"], json!("CLB"));
        assert_eq!(payload["Repeatable"], json!(false));
    }

    #[test]
    fn upload_certificate_payload_ca() {
        let public_key = "-----BEGIN CERTIFICATE-----\nCA_CERT\n-----END CERTIFICATE-----";

        let request = UploadCertificate::new(public_key)
            .with_certificate_type("CA")
            .with_alias("root-ca")
            .with_repeatable(true);

        let payload = request.payload();

        assert!(payload["CertificatePublicKey"]
            .as_str()
            .expect("certificate public key should be string")
            .contains("CA_CERT"));
        assert_eq!(payload["CertificateType"], json!("CA"));
        assert_eq!(payload["Alias"], json!("root-ca"));
        assert_eq!(payload["Repeatable"], json!(true));
        // CA certificates should not include a private key
        assert!(payload.get("CertificatePrivateKey").is_none());
    }

    #[test]
    fn upload_certificate_with_tags() {
        let public_key = "-----BEGIN CERTIFICATE-----\n...\n-----END CERTIFICATE-----";

        let request = UploadCertificate::new(public_key)
            .with_tag("environment", "production")
            .with_tag("application", "web-server")
            .with_tag("owner", "team-a");

        let payload = request.payload();

        let tags = payload["Tags"].as_array().expect("Tags should be an array");
        assert_eq!(tags.len(), 3);

        // First tag
        assert_eq!(tags[0]["TagKey"], json!("environment"));
        assert_eq!(tags[0]["TagValue"], json!("production"));

        // Second tag
        assert_eq!(tags[1]["TagKey"], json!("application"));
        assert_eq!(tags[1]["TagValue"], json!("web-server"));
    }

    #[test]
    fn deserialize_upload_certificate_response() {
        let payload = r#"{
            "Response": {
                "CertificateId": "cert-abc123xyz",
                "RepeatCertId": "cert-duplicate456",
                "RequestId": "req-789def"
            }
        }"#;

        let parsed: UploadCertificateResponse =
            serde_json::from_str(payload).expect("deserialize UploadCertificateResponse");

        assert_eq!(
            parsed.response.certificate_id.as_deref(),
            Some("cert-abc123xyz")
        );
        assert_eq!(
            parsed.response.repeat_cert_id.as_deref(),
            Some("cert-duplicate456")
        );
        assert_eq!(parsed.response.request_id, "req-789def");
    }

    #[test]
    fn deserialize_upload_certificate_response_without_duplicate() {
        let payload = r#"{
            "Response": {
                "CertificateId": "cert-new-unique",
                "RequestId": "req-123abc"
            }
        }"#;

        let parsed: UploadCertificateResponse =
            serde_json::from_str(payload).expect("deserialize UploadCertificateResponse");

        assert_eq!(
            parsed.response.certificate_id.as_deref(),
            Some("cert-new-unique")
        );
        assert!(parsed.response.repeat_cert_id.is_none());
        assert_eq!(parsed.response.request_id, "req-123abc");
    }

    #[test]
    fn certificate_type_enum_conversion() {
        let ca: CertificateType = "CA".into();
        let svr: CertificateType = "SVR".into();
        let custom: CertificateType = "CUSTOM_TYPE".into();

        assert!(matches!(ca, CertificateType::Ca));
        assert!(matches!(svr, CertificateType::Svr));
        assert!(matches!(custom, CertificateType::Custom("CUSTOM_TYPE")));
    }

    #[test]
    fn certificate_use_enum_conversion() {
        let uses = ["CLB", "CDN", "WAF", "LIVE", "DDOS", "CUSTOM"];

        for use_str in uses {
            let cert_use: CertificateUse = use_str.into();

            match use_str {
                "CLB" => assert!(matches!(cert_use, CertificateUse::Clb)),
                "CDN" => assert!(matches!(cert_use, CertificateUse::Cdn)),
                "WAF" => assert!(matches!(cert_use, CertificateUse::Waf)),
                "LIVE" => assert!(matches!(cert_use, CertificateUse::Live)),
                "DDOS" => assert!(matches!(cert_use, CertificateUse::Ddos)),
                _ => assert!(matches!(cert_use, CertificateUse::Custom("CUSTOM"))),
            }
        }
    }

    #[test]
    fn certificate_enums_serialize_as_bare_strings() {
        let custom_type: CertificateType = "any-type".into();
        let custom_use: CertificateUse = "custom-use".into();

        let payload = serde_json::json!({
            "CertificateType": custom_type,
            "CertificateUse": custom_use
        });

        assert_eq!(payload["CertificateType"], json!("any-type"));
        assert_eq!(payload["CertificateUse"], json!("custom-use"));
    }
}
