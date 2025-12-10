use crate::core::Endpoint;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::borrow::Cow;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct CheckCertificateResponse {
    #[serde(rename = "Response")]
    pub response: CheckCertificateResult,
}

#[derive(Debug, Deserialize)]
pub struct CheckCertificateResult {
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
    pub certificate_id: Option<String>,
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
    pub tags: Option<Vec<Tags>>,
    #[serde(rename = "CAEncryptAlgorithms")]
    pub ca_encrypt_algorithms: Option<Vec<String>>,
    #[serde(rename = "CACommonNames")]
    pub ca_common_names: Option<Vec<String>>,
    #[serde(rename = "CAEndTimes")]
    pub ca_end_times: Option<Vec<String>>,
    #[serde(rename = "DvRevokeAuthDetail")]
    pub dv_revoke_auth_detail: Option<Vec<DvAuths>>,
    #[serde(rename = "RequestId")]
    pub request_id: String,
}

#[derive(Debug, Deserialize)]
pub struct CertificateExtra {
    // 根据实际情况定义具体字段
    #[serde(flatten, default)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DvAuthDetail {
    /// 证书域名验证记录Key
    /// 注意：此字段可能返回 null，表示取不到有效值。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dv_auth_key: Option<String>,

    /// 证书域名验证记录值
    /// 注意：此字段可能返回 null，表示取不到有效值。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dv_auth_value: Option<String>,

    /// 证书域名验证域名值
    /// 注意：此字段可能返回 null，表示取不到有效值。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dv_auth_domain: Option<String>,

    /// 证书域名验证文件路径，仅FILE、FILE_PROXY使用
    /// 注意：此字段可能返回 null，表示取不到有效值。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dv_auth_path: Option<String>,

    /// 证书域名验证子域名
    /// 注意：此字段可能返回 null，表示取不到有效值。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dv_auth_key_sub_domain: Option<String>,

    /// 证书域名验证信息，存在多个域名验证使用本字段
    /// 注意：此字段可能返回 null，表示取不到有效值。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dv_auths: Option<Vec<DvAuths>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DvAuths {
    /// 证书域名验证记录Key
    pub dv_auth_key: String,

    /// 证书域名验证记录值
    pub dv_auth_value: String,

    /// 证书域名验证域名值
    pub dv_auth_domain: String,

    /// 证书域名验证文件路径，仅FILE、FILE_PROXY使用
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dv_auth_path: Option<String>,

    /// 证书域名验证子域名
    pub dv_auth_sub_domain: String,

    /// 证书域名验证类型，取值：
    /// TXT：DNS域名验证添加TXT记录
    /// FILE：域名文件验证
    /// CNAME：DNS域名验证添加CNAME记录
    pub dv_auth_verify_type: String,
}

#[derive(Debug, Deserialize)]
pub struct SubmittedData {
    // 根据实际情况定义具体字段
    #[serde(flatten, default)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Deserialize)]
pub struct Tags {
    #[serde(rename = "TagKey")]
    pub tag_key: Option<String>,
    #[serde(rename = "TagValue")]
    pub tag_value: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
struct CheckCertificatePayload<'a> {
    certificate_id: &'a str,
}

/// Request payload for `CheckCertificate`.
pub struct CheckCertificate<'a> {
    pub certificate_id: &'a str,
}

impl<'a> CheckCertificate<'a> {
    /// Create a new certificate description request
    pub fn new(certificate_id: &'a str) -> Self {
        Self { certificate_id }
    }
}

impl<'a> Endpoint for CheckCertificate<'a> {
    type Output = CheckCertificateResponse;

    fn service(&self) -> Cow<'static, str> {
        Cow::Borrowed("ssl")
    }

    fn action(&self) -> Cow<'static, str> {
        Cow::Borrowed("DescribeCertificate")
    }

    fn version(&self) -> Cow<'static, str> {
        Cow::Borrowed("2019-12-05")
    }

    fn region(&self) -> Option<Cow<'_, str>> {
        // SSL接口不需要region参数
        None
    }

    fn payload(&self) -> Value {
        serde_json::to_value(CheckCertificatePayload {
            certificate_id: self.certificate_id,
        })
        .expect("serialize CheckCertificate payload")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn check_certificate_payload_serialization() {
        let request = CheckCertificate::new("cert-abc123");
        let payload = request.payload();
        assert_eq!(payload["CertificateId"], json!("cert-abc123"));
    }

    #[test]
    fn check_certificate_response() {
        let payload = r#"{
            "Response": {
                "OwnerUin": "1234567890",
                "ProjectId": "10086",
                "From": "trustasia",
                "CertificateType": "SVR",
                "PackageType": "83",
                "ProductZhName": "TrustAsia C1 DV Free",
                "Domain": "example.com",
                "Status": 1,
                "StatusMsg": "已颁发",
                "VerifyType": "DNS_AUTO",
                "CertBeginTime": "2024-01-01 00:00:00",
                "CertEndTime": "2024-04-01 23:59:59",
                "ValidityPeriod": "3",
                "InsertTime": "2023-12-31 10:30:00",
                "CertificateId": "cert-abc123",
                "PackageTypeName": "TrustAsia C1 DV Free",
                "StatusName": "已颁发",
                "SubjectAltName": ["www.example.com"],
                "IsVip": false,
                "IsWildcard": false,
                "IsDv": true,
                "RenewAble": true,
                "Deployable": true,
                "RequestId": "req-xyz-789"
            }
        }"#;
        let parsed: CheckCertificateResponse = serde_json::from_str(payload).unwrap();
        let resp = &parsed.response;
        assert_eq!(resp.owner_uin, Some("1234567890".to_string()));
        assert_eq!(resp.project_id, Some("10086".to_string()));
        assert_eq!(resp.from, Some("trustasia".to_string()));
        assert_eq!(resp.certificate_type, Some("SVR".to_string()));
        assert_eq!(resp.package_type, Some("83".to_string()));
        assert_eq!(
            resp.product_zh_name,
            Some("TrustAsia C1 DV Free".to_string())
        );
        assert_eq!(resp.domain, Some("example.com".to_string()));
        assert_eq!(resp.status, Some(1));
        assert_eq!(resp.status_msg, Some("已颁发".to_string()));
        assert_eq!(resp.verify_type, Some("DNS_AUTO".to_string()));
        assert_eq!(
            resp.cert_begin_time,
            Some("2024-01-01 00:00:00".to_string())
        );
        assert_eq!(resp.cert_end_time, Some("2024-04-01 23:59:59".to_string()));
        assert_eq!(resp.validity_period, Some("3".to_string()));
        assert_eq!(resp.insert_time, Some("2023-12-31 10:30:00".to_string()));
        assert_eq!(resp.certificate_id, Some("cert-abc123".to_string()));
        assert_eq!(resp.status_name, Some("已颁发".to_string()));
        assert_eq!(resp.is_vip, Some(false));
        assert_eq!(resp.is_wildcard, Some(false));
        assert_eq!(resp.is_dv, Some(true));
        assert_eq!(resp.renew_able, Some(true));
        assert_eq!(resp.deployable, Some(true));
        assert_eq!(resp.request_id, "req-xyz-789");
    }
}
