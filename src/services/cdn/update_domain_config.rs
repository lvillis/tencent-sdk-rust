use crate::core::Endpoint;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::borrow::Cow;

#[derive(Debug, Deserialize)]
pub struct UpdateDomainConfigResponse {
    #[serde(rename = "Response")]
    pub response: UpdateDomainConfigResult,
}

#[derive(Debug, Deserialize)]
pub struct UpdateDomainConfigResult {
    #[serde(rename = "RequestId")]
    pub request_id: String,
}

#[derive(Serialize)]
pub struct CertInfo<'a> {
    #[serde(rename = "CertId")]
    pub cert_id: &'a str,
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct HttpsInfo<'a> {
    switch: &'a str,
    cert_info: CertInfo<'a>,
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct UpdateDomainConfig<'a> {
    pub domain: &'a str,
    pub https: HttpsInfo<'a>,
}

impl<'a> UpdateDomainConfig<'a> {
    pub fn new(domain: &'a str, value: &'a str) -> Self {
        Self {
            domain,
            https: HttpsInfo {
                switch: "on",
                cert_info: CertInfo { cert_id: value },
            },
        }
    }
}

impl<'a> Endpoint for UpdateDomainConfig<'a> {
    type Output = UpdateDomainConfigResponse;

    fn service(&self) -> Cow<'static, str> {
        Cow::Borrowed("cdn")
    }

    fn action(&self) -> Cow<'static, str> {
        Cow::Borrowed("UpdateDomainConfig")
    }

    fn version(&self) -> Cow<'static, str> {
        Cow::Borrowed("2018-06-06")
    }

    fn region(&self) -> Option<Cow<'_, str>> {
        // DNS 接口通常不需要 region 参数
        None
    }

    fn payload(&self) -> Value {
        let payload = serde_json::to_value(&self).unwrap();
        payload
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_update_domain_config_payload() {
        let cert_id = "cert_001";
        let domain_name = "example.com";

        let request = UpdateDomainConfig::new(domain_name, cert_id);

        let payload = request.payload();
        assert_eq!(payload["Domain"], json!(domain_name));
        assert!(payload["Https"].is_object());

        let https_value = &payload["Https"];
        assert!(https_value["CertInfo"].is_object());

        let cert_info_value = &https_value["CertInfo"];
        assert_eq!(cert_info_value["CertId"], json!(cert_id),);

        let expected_payload = json!({
            "Domain": "example.com",
            "Https": {
                "Switch":"on",
                "CertInfo": {
                    "CertId": "cert_001"
                }
            }
        });

        assert_eq!(payload, expected_payload);
    }
}
