use crate::core::Endpoint;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::borrow::Cow;

#[derive(Debug, Deserialize)]
pub struct ApplyCertificateResponse {
    #[serde(rename = "Response")]
    pub response: ApplyCertificateResult,
}

#[derive(Debug, Deserialize)]
pub struct ApplyCertificateResult {
    #[serde(rename = "CertificateId")]
    pub certificate_id: Option<String>,
    #[serde(rename = "RequestId")]
    pub request_id: String,
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
struct ApplyCertificatePayload<'a> {
    dv_auth_method: &'a str,
    domain_name: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    project_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    package_type: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    contact_email: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    contact_phone: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    validity_period: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    csr_encrypt_algo: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    csr_key_parameter: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    csr_key_password: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    alias: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    old_certificate_id: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    package_id: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete_dns_auto_record: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dns_names: Option<&'a [&'a str]>,
}

/// Request payload for `ApplyCertificate`.
pub struct ApplyCertificate<'a> {
    pub dv_auth_method: &'a str,
    pub domain_name: &'a str,
    pub project_id: Option<i64>,
    pub package_type: Option<&'a str>,
    pub contact_email: Option<&'a str>,
    pub contact_phone: Option<&'a str>,
    pub validity_period: Option<&'a str>,
    pub csr_encrypt_algo: Option<&'a str>,
    pub csr_key_parameter: Option<&'a str>,
    pub csr_key_password: Option<&'a str>,
    pub alias: Option<&'a str>,
    pub old_certificate_id: Option<&'a str>,
    pub package_id: Option<&'a str>,
    pub delete_dns_auto_record: Option<bool>,
    pub dns_names: Option<&'a [&'a str]>,
}

impl<'a> ApplyCertificate<'a> {
    /// Create a new certificate application request
    pub fn new(dv_auth_method: &'a str, domain_name: &'a str) -> Self {
        Self {
            dv_auth_method,
            domain_name,
            project_id: None,
            package_type: None,
            contact_email: None,
            contact_phone: None,
            validity_period: None,
            csr_encrypt_algo: None,
            csr_key_parameter: None,
            csr_key_password: None,
            alias: None,
            old_certificate_id: None,
            package_id: None,
            delete_dns_auto_record: None,
            dns_names: None,
        }
    }

    /// Set project ID
    pub fn with_project_id(mut self, project_id: i64) -> Self {
        self.project_id = Some(project_id);
        self
    }

    /// Set package type (currently only "83" is supported)
    pub fn with_package_type(mut self, package_type: &'a str) -> Self {
        self.package_type = Some(package_type);
        self
    }

    /// Set contact email
    pub fn with_contact_email(mut self, contact_email: &'a str) -> Self {
        self.contact_email = Some(contact_email);
        self
    }

    /// Set contact phone
    pub fn with_contact_phone(mut self, contact_phone: &'a str) -> Self {
        self.contact_phone = Some(contact_phone);
        self
    }

    /// Set validity period (default "3")
    pub fn with_validity_period(mut self, validity_period: &'a str) -> Self {
        self.validity_period = Some(validity_period);
        self
    }

    /// Set CSR encrypt algorithm (RSA or ECC)
    pub fn with_csr_encrypt_algo(mut self, csr_encrypt_algo: &'a str) -> Self {
        self.csr_encrypt_algo = Some(csr_encrypt_algo);
        self
    }

    /// Set CSR key parameter (2048 for RSA, prime256v1 for ECC)
    pub fn with_csr_key_parameter(mut self, csr_key_parameter: &'a str) -> Self {
        self.csr_key_parameter = Some(csr_key_parameter);
        self
    }

    /// Set CSR key password
    pub fn with_csr_key_password(mut self, csr_key_password: &'a str) -> Self {
        self.csr_key_password = Some(csr_key_password);
        self
    }

    /// Set certificate alias
    pub fn with_alias(mut self, alias: &'a str) -> Self {
        self.alias = Some(alias);
        self
    }

    /// Set old certificate ID for renewal
    pub fn with_old_certificate_id(mut self, old_certificate_id: &'a str) -> Self {
        self.old_certificate_id = Some(old_certificate_id);
        self
    }

    /// Set package ID for free certificate expansion
    pub fn with_package_id(mut self, package_id: &'a str) -> Self {
        self.package_id = Some(package_id);
        self
    }

    /// Set whether to delete DNS auto record after issuance
    pub fn with_delete_dns_auto_record(mut self, delete_dns_auto_record: bool) -> Self {
        self.delete_dns_auto_record = Some(delete_dns_auto_record);
        self
    }

    /// Set DNS names for multi-domain certificates
    pub fn with_dns_names(mut self, dns_names: &'a [&'a str]) -> Self {
        self.dns_names = Some(dns_names);
        self
    }
}

impl<'a> Endpoint for ApplyCertificate<'a> {
    type Output = ApplyCertificateResponse;

    fn service(&self) -> Cow<'static, str> {
        Cow::Borrowed("ssl")
    }

    fn action(&self) -> Cow<'static, str> {
        Cow::Borrowed("ApplyCertificate")
    }

    fn version(&self) -> Cow<'static, str> {
        Cow::Borrowed("2019-12-05")
    }

    fn region(&self) -> Option<Cow<'_, str>> {
        // SSL接口不需要region参数
        None
    }

    fn payload(&self) -> Value {
        serde_json::to_value(ApplyCertificatePayload {
            dv_auth_method: self.dv_auth_method,
            domain_name: self.domain_name,
            project_id: self.project_id,
            package_type: self.package_type,
            contact_email: self.contact_email,
            contact_phone: self.contact_phone,
            validity_period: self.validity_period,
            csr_encrypt_algo: self.csr_encrypt_algo,
            csr_key_parameter: self.csr_key_parameter,
            csr_key_password: self.csr_key_password,
            alias: self.alias,
            old_certificate_id: self.old_certificate_id,
            package_id: self.package_id,
            delete_dns_auto_record: self.delete_dns_auto_record,
            dns_names: self.dns_names,
        })
        .expect("serialize ApplyCertificate payload")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn apply_certificate_payload_serialization() {
        let request = ApplyCertificate::new("DNS_AUTO", "example.com")
            .with_project_id(12345)
            .with_package_type("83")
            .with_contact_email("admin@example.com")
            .with_validity_period("3")
            .with_csr_encrypt_algo("RSA")
            .with_csr_key_parameter("2048")
            .with_alias("MyCertificate")
            .with_delete_dns_auto_record(true);

        let payload = request.payload();
        assert_eq!(payload["DvAuthMethod"], json!("DNS_AUTO"));
        assert_eq!(payload["DomainName"], json!("example.com"));
        assert_eq!(payload["ProjectId"], json!(12345));
        assert_eq!(payload["PackageType"], json!("83"));
        assert_eq!(payload["ContactEmail"], json!("admin@example.com"));
        assert_eq!(payload["ValidityPeriod"], json!("3"));
        assert_eq!(payload["CsrEncryptAlgo"], json!("RSA"));
        assert_eq!(payload["CsrKeyParameter"], json!("2048"));
        assert_eq!(payload["Alias"], json!("MyCertificate"));
        assert_eq!(payload["DeleteDnsAutoRecord"], json!(true));
    }

    #[test]
    fn apply_certificate_with_dns_names() {
        let dns_names = ["www.example.com", "api.example.com"];
        let request = ApplyCertificate::new("DNS", "example.com").with_dns_names(&dns_names);

        let payload = request.payload();
        let dns_names_array = payload["DnsNames"].as_array().unwrap();
        assert_eq!(dns_names_array[0], json!("www.example.com"));
        assert_eq!(dns_names_array[1], json!("api.example.com"));
    }

    #[test]
    fn deserialize_apply_certificate_response() {
        let payload = r#"{
            "Response": {
                "CertificateId": "cert-123456",
                "RequestId": "req-abc-123"
            }
        }"#;
        let parsed: ApplyCertificateResponse = serde_json::from_str(payload).unwrap();
        assert_eq!(
            parsed.response.certificate_id,
            Some("cert-123456".to_string())
        );
        assert_eq!(parsed.response.request_id, "req-abc-123");
    }

    #[test]
    fn builder_pattern_works_for_apply_certificate() {
        let request = ApplyCertificate::new("DNS_AUTO", "tencent.com")
            .with_project_id(0)
            .with_package_type("83")
            .with_contact_email("ssl@tencent.com")
            .with_contact_phone("18888888888")
            .with_validity_period("3")
            .with_csr_encrypt_algo("RSA")
            .with_csr_key_parameter("2048")
            .with_alias("生产证书")
            .with_delete_dns_auto_record(true);

        let payload = request.payload();
        assert_eq!(payload["DvAuthMethod"], json!("DNS_AUTO"));
        assert_eq!(payload["DomainName"], json!("tencent.com"));
        assert_eq!(payload["ProjectId"], json!(0));
        assert_eq!(payload["PackageType"], json!("83"));
        assert_eq!(payload["ContactEmail"], json!("ssl@tencent.com"));
        assert_eq!(payload["ContactPhone"], json!("18888888888"));
        assert_eq!(payload["ValidityPeriod"], json!("3"));
        assert_eq!(payload["CsrEncryptAlgo"], json!("RSA"));
        assert_eq!(payload["CsrKeyParameter"], json!("2048"));
        assert_eq!(payload["Alias"], json!("生产证书"));
        assert_eq!(payload["DeleteDnsAutoRecord"], json!(true));
    }

    #[test]
    fn apply_certificate_with_renewal() {
        let request = ApplyCertificate::new("DNS", "example.com")
            .with_old_certificate_id("LqQxgqUe")
            .with_validity_period("3");

        let payload = request.payload();
        assert_eq!(payload["DvAuthMethod"], json!("DNS"));
        assert_eq!(payload["DomainName"], json!("example.com"));
        assert_eq!(payload["OldCertificateId"], json!("LqQxgqUe"));
        assert_eq!(payload["ValidityPeriod"], json!("3"));
    }

    #[test]
    fn apply_certificate_with_ecc() {
        let request = ApplyCertificate::new("FILE", "example.com")
            .with_csr_encrypt_algo("ECC")
            .with_csr_key_parameter("prime256v1");

        let payload = request.payload();
        assert_eq!(payload["CsrEncryptAlgo"], json!("ECC"));
        assert_eq!(payload["CsrKeyParameter"], json!("prime256v1"));
    }
}
