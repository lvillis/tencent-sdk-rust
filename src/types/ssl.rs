use crate::{
    Error,
    client::endpoint::Endpoint,
    types::{CertificateId, DomainName, Region, RequestId},
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct ApplyCertificateResponse {
    #[serde(rename = "Response")]
    pub response: ApplyCertificateResult,
}

#[derive(Debug, Deserialize)]
pub struct ApplyCertificateResult {
    #[serde(rename = "CertificateId")]
    pub certificate_id: Option<CertificateId>,
    #[serde(rename = "RequestId")]
    pub request_id: RequestId,
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum DvAuthMethod {
    DnsAuto,
    Dns,
    File,
    FileProxy,
    Custom(String),
}

impl DvAuthMethod {
    pub fn as_str(&self) -> &str {
        match self {
            DvAuthMethod::DnsAuto => "DNS_AUTO",
            DvAuthMethod::Dns => "DNS",
            DvAuthMethod::File => "FILE",
            DvAuthMethod::FileProxy => "FILE_PROXY",
            DvAuthMethod::Custom(value) => value.as_str(),
        }
    }
}

impl From<&str> for DvAuthMethod {
    fn from(value: &str) -> Self {
        match value.to_ascii_uppercase().as_str() {
            "DNS_AUTO" => DvAuthMethod::DnsAuto,
            "DNS" => DvAuthMethod::Dns,
            "FILE" => DvAuthMethod::File,
            "FILE_PROXY" => DvAuthMethod::FileProxy,
            _ => DvAuthMethod::Custom(value.to_string()),
        }
    }
}

impl Serialize for DvAuthMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

pub struct ApplyCertificateRequest {
    dv_auth_method: DvAuthMethod,
    domain_name: DomainName,
    project_id: Option<i64>,
    package_type: Option<String>,
    contact_email: Option<String>,
    contact_phone: Option<String>,
    validity_period: Option<String>,
    csr_encrypt_algo: Option<String>,
    csr_key_parameter: Option<String>,
    csr_key_password: Option<String>,
    alias: Option<String>,
    old_certificate_id: Option<CertificateId>,
    package_id: Option<String>,
    delete_dns_auto_record: Option<bool>,
    dns_names: Vec<DomainName>,
}

impl ApplyCertificateRequest {
    pub fn new(
        dv_auth_method: impl Into<DvAuthMethod>,
        domain_name: impl Into<DomainName>,
    ) -> Self {
        Self {
            dv_auth_method: dv_auth_method.into(),
            domain_name: domain_name.into(),
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
            dns_names: Vec::new(),
        }
    }

    pub fn project_id(mut self, project_id: i64) -> Self {
        self.project_id = Some(project_id);
        self
    }

    pub fn package_type(mut self, package_type: impl Into<String>) -> Self {
        self.package_type = Some(package_type.into());
        self
    }

    pub fn contact_email(mut self, contact_email: impl Into<String>) -> Self {
        self.contact_email = Some(contact_email.into());
        self
    }

    pub fn contact_phone(mut self, contact_phone: impl Into<String>) -> Self {
        self.contact_phone = Some(contact_phone.into());
        self
    }

    pub fn validity_period(mut self, validity_period: impl Into<String>) -> Self {
        self.validity_period = Some(validity_period.into());
        self
    }

    pub fn csr_encrypt_algo(mut self, csr_encrypt_algo: impl Into<String>) -> Self {
        self.csr_encrypt_algo = Some(csr_encrypt_algo.into());
        self
    }

    pub fn csr_key_parameter(mut self, csr_key_parameter: impl Into<String>) -> Self {
        self.csr_key_parameter = Some(csr_key_parameter.into());
        self
    }

    pub fn csr_key_password(mut self, csr_key_password: impl Into<String>) -> Self {
        self.csr_key_password = Some(csr_key_password.into());
        self
    }

    pub fn alias(mut self, alias: impl Into<String>) -> Self {
        self.alias = Some(alias.into());
        self
    }

    pub fn old_certificate_id(mut self, old_certificate_id: impl Into<CertificateId>) -> Self {
        self.old_certificate_id = Some(old_certificate_id.into());
        self
    }

    pub fn package_id(mut self, package_id: impl Into<String>) -> Self {
        self.package_id = Some(package_id.into());
        self
    }

    pub fn delete_dns_auto_record(mut self, enabled: bool) -> Self {
        self.delete_dns_auto_record = Some(enabled);
        self
    }

    pub fn dns_names<I, S>(mut self, dns_names: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<DomainName>,
    {
        self.dns_names = dns_names.into_iter().map(Into::into).collect();
        self
    }

    pub fn dv_auth_method(mut self, dv_auth_method: impl Into<DvAuthMethod>) -> Self {
        self.dv_auth_method = dv_auth_method.into();
        self
    }
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
struct ApplyCertificatePayload<'a> {
    dv_auth_method: &'a DvAuthMethod,
    domain_name: &'a DomainName,
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
    old_certificate_id: Option<&'a CertificateId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    package_id: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete_dns_auto_record: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dns_names: Option<&'a [DomainName]>,
}

impl Endpoint for ApplyCertificateRequest {
    type Output = ApplyCertificateResponse;

    fn service(&self) -> &'static str {
        "ssl"
    }

    fn action(&self) -> &'static str {
        "ApplyCertificate"
    }

    fn version(&self) -> &'static str {
        "2019-12-05"
    }

    fn region(&self) -> Option<&Region> {
        None
    }

    fn payload(&self) -> Result<Option<Value>, Error> {
        let dns_names = (!self.dns_names.is_empty()).then_some(self.dns_names.as_slice());
        let payload = ApplyCertificatePayload {
            dv_auth_method: &self.dv_auth_method,
            domain_name: &self.domain_name,
            project_id: self.project_id,
            package_type: self.package_type.as_deref(),
            contact_email: self.contact_email.as_deref(),
            contact_phone: self.contact_phone.as_deref(),
            validity_period: self.validity_period.as_deref(),
            csr_encrypt_algo: self.csr_encrypt_algo.as_deref(),
            csr_key_parameter: self.csr_key_parameter.as_deref(),
            csr_key_password: self.csr_key_password.as_deref(),
            alias: self.alias.as_deref(),
            old_certificate_id: self.old_certificate_id.as_ref(),
            package_id: self.package_id.as_deref(),
            delete_dns_auto_record: self.delete_dns_auto_record,
            dns_names,
        };

        let value = serde_json::to_value(payload).map_err(|source| {
            Error::invalid_request_with_source(
                "failed to serialize ApplyCertificate request payload",
                Box::new(source),
            )
        })?;
        Ok(Some(value))
    }
}

#[derive(Debug, Deserialize)]
pub struct DescribeCertificateResponse {
    #[serde(rename = "Response")]
    pub response: DescribeCertificateResult,
}

#[derive(Debug, Deserialize)]
pub struct DescribeCertificateResult {
    #[serde(rename = "OwnerUin")]
    pub owner_uin: Option<String>,
    #[serde(rename = "ProjectId")]
    pub project_id: Option<String>,
    #[serde(rename = "From")]
    pub from: Option<String>,
    #[serde(rename = "CertificateType")]
    pub certificate_type: Option<String>,
    #[serde(rename = "PackageType")]
    pub package_type: Option<String>,
    #[serde(rename = "ProductZhName")]
    pub product_zh_name: Option<String>,
    #[serde(rename = "Domain")]
    pub domain: Option<String>,
    #[serde(rename = "Alias")]
    pub alias: Option<String>,
    #[serde(rename = "Status")]
    pub status: Option<i32>,
    #[serde(rename = "StatusMsg")]
    pub status_msg: Option<String>,
    #[serde(rename = "VerifyType")]
    pub verify_type: Option<String>,
    #[serde(rename = "VulnerabilityStatus")]
    pub vulnerability_status: Option<String>,
    #[serde(rename = "CertBeginTime")]
    pub cert_begin_time: Option<String>,
    #[serde(rename = "CertEndTime")]
    pub cert_end_time: Option<String>,
    #[serde(rename = "ValidityPeriod")]
    pub validity_period: Option<String>,
    #[serde(rename = "InsertTime")]
    pub insert_time: Option<String>,
    #[serde(rename = "OrderId")]
    pub order_id: Option<String>,
    #[serde(rename = "CertificateExtra")]
    pub certificate_extra: Option<CertificateExtra>,
    #[serde(rename = "DvAuthDetail")]
    pub dv_auth_detail: Option<DvAuthDetail>,
    #[serde(rename = "VulnerabilityReport")]
    pub vulnerability_report: Option<String>,
    #[serde(rename = "CertificateId")]
    pub certificate_id: Option<CertificateId>,
    #[serde(rename = "PackageTypeName")]
    pub package_type_name: Option<String>,
    #[serde(rename = "StatusName")]
    pub status_name: Option<String>,
    #[serde(rename = "SubjectAltName")]
    pub subject_alt_name: Option<Vec<String>>,
    #[serde(rename = "IsVip")]
    pub is_vip: Option<bool>,
    #[serde(rename = "IsWildcard")]
    pub is_wildcard: Option<bool>,
    #[serde(rename = "IsDv")]
    pub is_dv: Option<bool>,
    #[serde(rename = "IsVulnerability")]
    pub is_vulnerability: Option<bool>,
    #[serde(rename = "RenewAble")]
    pub renew_able: Option<bool>,
    #[serde(rename = "SubmittedData")]
    pub submitted_data: Option<SubmittedData>,
    #[serde(rename = "Deployable")]
    pub deployable: Option<bool>,
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<CertificateTag>>,
    #[serde(rename = "CAEncryptAlgorithms")]
    pub ca_encrypt_algorithms: Option<Vec<String>>,
    #[serde(rename = "CACommonNames")]
    pub ca_common_names: Option<Vec<String>>,
    #[serde(rename = "CAEndTimes")]
    pub ca_end_times: Option<Vec<String>>,
    #[serde(rename = "DvRevokeAuthDetail")]
    pub dv_revoke_auth_detail: Option<Vec<DvAuths>>,
    #[serde(rename = "RequestId")]
    pub request_id: RequestId,
}

#[derive(Debug, Deserialize)]
pub struct CertificateExtra {
    #[serde(flatten, default)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DvAuthDetail {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dv_auth_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dv_auth_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dv_auth_domain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dv_auth_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dv_auth_key_sub_domain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dv_auths: Option<Vec<DvAuths>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DvAuths {
    pub dv_auth_key: String,
    pub dv_auth_value: String,
    pub dv_auth_domain: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dv_auth_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dv_auth_key_sub_domain: Option<String>,
    #[serde(flatten, default)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Deserialize)]
pub struct SubmittedData {
    #[serde(flatten, default)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Deserialize)]
pub struct CertificateTag {
    #[serde(rename = "TagKey")]
    pub tag_key: Option<String>,
    #[serde(rename = "TagValue")]
    pub tag_value: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
struct DescribeCertificatePayload<'a> {
    certificate_id: &'a CertificateId,
}

pub struct DescribeCertificateRequest {
    certificate_id: CertificateId,
}

impl DescribeCertificateRequest {
    pub fn new(certificate_id: impl Into<CertificateId>) -> Self {
        Self {
            certificate_id: certificate_id.into(),
        }
    }
}

impl Endpoint for DescribeCertificateRequest {
    type Output = DescribeCertificateResponse;

    fn service(&self) -> &'static str {
        "ssl"
    }

    fn action(&self) -> &'static str {
        "DescribeCertificate"
    }

    fn version(&self) -> &'static str {
        "2019-12-05"
    }

    fn region(&self) -> Option<&Region> {
        None
    }

    fn payload(&self) -> Result<Option<Value>, Error> {
        let payload = DescribeCertificatePayload {
            certificate_id: &self.certificate_id,
        };
        let value = serde_json::to_value(payload).map_err(|source| {
            Error::invalid_request_with_source(
                "failed to serialize DescribeCertificate request payload",
                Box::new(source),
            )
        })?;
        Ok(Some(value))
    }
}

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
    pub request_id: RequestId,
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
struct DownloadCertificatePayload<'a> {
    certificate_id: &'a CertificateId,
}

pub struct DownloadCertificateRequest {
    certificate_id: CertificateId,
}

impl DownloadCertificateRequest {
    pub fn new(certificate_id: impl Into<CertificateId>) -> Self {
        Self {
            certificate_id: certificate_id.into(),
        }
    }
}

impl Endpoint for DownloadCertificateRequest {
    type Output = DownloadCertificateResponse;

    fn service(&self) -> &'static str {
        "ssl"
    }

    fn action(&self) -> &'static str {
        "DownloadCertificate"
    }

    fn version(&self) -> &'static str {
        "2019-12-05"
    }

    fn region(&self) -> Option<&Region> {
        None
    }

    fn payload(&self) -> Result<Option<Value>, Error> {
        let payload = DownloadCertificatePayload {
            certificate_id: &self.certificate_id,
        };
        let value = serde_json::to_value(payload).map_err(|source| {
            Error::invalid_request_with_source(
                "failed to serialize DownloadCertificate request payload",
                Box::new(source),
            )
        })?;
        Ok(Some(value))
    }
}

#[derive(Debug, Deserialize)]
pub struct UploadCertificateResponse {
    #[serde(rename = "Response")]
    pub response: UploadCertificateResult,
}

#[derive(Debug, Deserialize)]
pub struct UploadCertificateResult {
    #[serde(rename = "CertificateId")]
    pub certificate_id: Option<CertificateId>,
    #[serde(rename = "RepeatCertId")]
    pub repeat_cert_id: Option<CertificateId>,
    #[serde(rename = "RequestId")]
    pub request_id: RequestId,
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum CertificateType {
    Ca,
    Svr,
    Custom(String),
}

impl CertificateType {
    pub fn as_str(&self) -> &str {
        match self {
            CertificateType::Ca => "CA",
            CertificateType::Svr => "SVR",
            CertificateType::Custom(value) => value.as_str(),
        }
    }
}

impl From<&str> for CertificateType {
    fn from(value: &str) -> Self {
        match value.to_ascii_uppercase().as_str() {
            "CA" => CertificateType::Ca,
            "SVR" => CertificateType::Svr,
            _ => CertificateType::Custom(value.to_string()),
        }
    }
}

impl Serialize for CertificateType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum CertificateUse {
    Clb,
    Cdn,
    Waf,
    Live,
    Ddos,
    Custom(String),
}

impl CertificateUse {
    pub fn as_str(&self) -> &str {
        match self {
            CertificateUse::Clb => "CLB",
            CertificateUse::Cdn => "CDN",
            CertificateUse::Waf => "WAF",
            CertificateUse::Live => "LIVE",
            CertificateUse::Ddos => "DDOS",
            CertificateUse::Custom(value) => value.as_str(),
        }
    }
}

impl From<&str> for CertificateUse {
    fn from(value: &str) -> Self {
        match value.to_ascii_uppercase().as_str() {
            "CLB" => CertificateUse::Clb,
            "CDN" => CertificateUse::Cdn,
            "WAF" => CertificateUse::Waf,
            "LIVE" => CertificateUse::Live,
            "DDOS" => CertificateUse::Ddos,
            _ => CertificateUse::Custom(value.to_string()),
        }
    }
}

impl Serialize for CertificateUse {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct UploadTag {
    #[serde(rename = "TagKey")]
    pub key: String,
    #[serde(rename = "TagValue")]
    pub value: String,
}

pub struct UploadCertificateRequest {
    region: Option<Region>,
    certificate_public_key: String,
    certificate_private_key: Option<String>,
    certificate_type: Option<CertificateType>,
    alias: Option<String>,
    project_id: Option<u64>,
    certificate_use: Option<CertificateUse>,
    tags: Vec<UploadTag>,
    repeatable: Option<bool>,
    key_password: Option<String>,
}

impl UploadCertificateRequest {
    pub fn new(certificate_public_key: impl Into<String>) -> Self {
        Self {
            region: None,
            certificate_public_key: certificate_public_key.into(),
            certificate_private_key: None,
            certificate_type: None,
            alias: None,
            project_id: None,
            certificate_use: None,
            tags: Vec::new(),
            repeatable: None,
            key_password: None,
        }
    }

    pub fn region(mut self, region: impl Into<Region>) -> Self {
        self.region = Some(region.into());
        self
    }

    pub fn private_key(mut self, private_key: impl Into<String>) -> Self {
        self.certificate_private_key = Some(private_key.into());
        self
    }

    pub fn certificate_type(mut self, certificate_type: impl Into<CertificateType>) -> Self {
        self.certificate_type = Some(certificate_type.into());
        self
    }

    pub fn alias(mut self, alias: impl Into<String>) -> Self {
        self.alias = Some(alias.into());
        self
    }

    pub fn project_id(mut self, project_id: u64) -> Self {
        self.project_id = Some(project_id);
        self
    }

    pub fn certificate_use(mut self, certificate_use: impl Into<CertificateUse>) -> Self {
        self.certificate_use = Some(certificate_use.into());
        self
    }

    pub fn push_tag(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.tags.push(UploadTag {
            key: key.into(),
            value: value.into(),
        });
        self
    }

    pub fn repeatable(mut self, repeatable: bool) -> Self {
        self.repeatable = Some(repeatable);
        self
    }

    pub fn key_password(mut self, key_password: impl Into<String>) -> Self {
        self.key_password = Some(key_password.into());
        self
    }
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
struct UploadCertificatePayload<'a> {
    certificate_public_key: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    certificate_private_key: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    certificate_type: Option<&'a CertificateType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    alias: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    certificate_use: Option<&'a CertificateUse>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<&'a [UploadTag]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    repeatable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key_password: Option<&'a str>,
}

impl Endpoint for UploadCertificateRequest {
    type Output = UploadCertificateResponse;

    fn service(&self) -> &'static str {
        "ssl"
    }

    fn action(&self) -> &'static str {
        "UploadCertificate"
    }

    fn version(&self) -> &'static str {
        "2019-12-05"
    }

    fn region(&self) -> Option<&Region> {
        self.region.as_ref()
    }

    fn payload(&self) -> Result<Option<Value>, Error> {
        let tags = (!self.tags.is_empty()).then_some(self.tags.as_slice());
        let payload = UploadCertificatePayload {
            certificate_public_key: &self.certificate_public_key,
            certificate_private_key: self.certificate_private_key.as_deref(),
            certificate_type: self.certificate_type.as_ref(),
            alias: self.alias.as_deref(),
            project_id: self.project_id,
            certificate_use: self.certificate_use.as_ref(),
            tags,
            repeatable: self.repeatable,
            key_password: self.key_password.as_deref(),
        };

        let value = serde_json::to_value(payload).map_err(|source| {
            Error::invalid_request_with_source(
                "failed to serialize UploadCertificate request payload",
                Box::new(source),
            )
        })?;
        Ok(Some(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn apply_certificate_payload_serialization() {
        let request = ApplyCertificateRequest::new(DvAuthMethod::DnsAuto, "example.com")
            .project_id(12345)
            .package_type("83")
            .contact_email("admin@example.com")
            .validity_period("3")
            .csr_encrypt_algo("RSA")
            .csr_key_parameter("2048")
            .alias("MyCertificate")
            .delete_dns_auto_record(true);

        let payload = request.payload().unwrap().unwrap();
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
        let request = ApplyCertificateRequest::new(DvAuthMethod::Dns, "example.com")
            .dns_names(["www.example.com", "api.example.com"]);

        let payload = request.payload().unwrap().unwrap();
        let dns_names_array = payload["DnsNames"]
            .as_array()
            .expect("DnsNames should be an array");
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
            parsed
                .response
                .certificate_id
                .as_ref()
                .map(CertificateId::as_str),
            Some("cert-123456")
        );
        assert_eq!(parsed.response.request_id.as_str(), "req-abc-123");
    }

    #[test]
    fn describe_certificate_payload_serialization() {
        let request = DescribeCertificateRequest::new("cert-abc123");
        let payload = request.payload().unwrap().unwrap();
        assert_eq!(payload["CertificateId"], json!("cert-abc123"));
    }

    #[test]
    fn download_certificate_payload_serialization() {
        let request = DownloadCertificateRequest::new("cert-xyz789");
        let payload = request.payload().unwrap().unwrap();
        assert_eq!(payload["CertificateId"], json!("cert-xyz789"));
    }

    #[test]
    fn upload_certificate_payload_basic() {
        let public_key = "-----BEGIN CERTIFICATE-----\n...\n-----END CERTIFICATE-----";
        let private_key = "-----BEGIN RSA PRIVATE KEY-----\n...\n-----END RSA PRIVATE KEY-----";

        let request = UploadCertificateRequest::new(public_key)
            .private_key(private_key)
            .certificate_type("SVR")
            .alias("my-website-cert")
            .project_id(123456)
            .certificate_use("CLB")
            .repeatable(false);

        let payload = request.payload().unwrap().unwrap();

        assert!(
            payload["CertificatePublicKey"]
                .as_str()
                .expect("certificate public key should be string")
                .contains("BEGIN CERTIFICATE")
        );
        assert!(
            payload["CertificatePrivateKey"]
                .as_str()
                .expect("certificate private key should be string")
                .contains("BEGIN RSA PRIVATE KEY")
        );
        assert_eq!(payload["CertificateType"], json!("SVR"));
        assert_eq!(payload["Alias"], json!("my-website-cert"));
        assert_eq!(payload["ProjectId"], json!(123456));
        assert_eq!(payload["CertificateUse"], json!("CLB"));
        assert_eq!(payload["Repeatable"], json!(false));
    }

    #[test]
    fn upload_certificate_with_tags() {
        let public_key = "-----BEGIN CERTIFICATE-----\n...\n-----END CERTIFICATE-----";

        let request = UploadCertificateRequest::new(public_key)
            .push_tag("environment", "production")
            .push_tag("application", "web-server")
            .push_tag("owner", "team-a");

        let payload = request.payload().unwrap().unwrap();
        let tags = payload["Tags"].as_array().expect("Tags should be an array");
        assert_eq!(tags.len(), 3);
        assert_eq!(tags[0]["TagKey"], json!("environment"));
        assert_eq!(tags[0]["TagValue"], json!("production"));
        assert_eq!(tags[1]["TagKey"], json!("application"));
        assert_eq!(tags[1]["TagValue"], json!("web-server"));
    }
}
