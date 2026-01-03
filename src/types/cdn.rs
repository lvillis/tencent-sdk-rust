use crate::{
    Error,
    client::endpoint::Endpoint,
    types::{CertificateId, DomainName, Region, RequestId},
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct UpdateDomainConfigResponse {
    #[serde(rename = "Response")]
    pub response: UpdateDomainConfigResult,
}

#[derive(Debug, Deserialize)]
pub struct UpdateDomainConfigResult {
    #[serde(rename = "RequestId")]
    pub request_id: RequestId,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct CertInfo {
    pub cert_id: CertificateId,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct HttpsInfo {
    pub switch: String,
    pub cert_info: CertInfo,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct UpdateDomainConfigRequest {
    pub domain: DomainName,
    pub https: HttpsInfo,
}

impl UpdateDomainConfigRequest {
    pub fn new(domain: impl Into<DomainName>, cert_id: impl Into<CertificateId>) -> Self {
        Self {
            domain: domain.into(),
            https: HttpsInfo {
                switch: "on".to_string(),
                cert_info: CertInfo {
                    cert_id: cert_id.into(),
                },
            },
        }
    }
}

impl Endpoint for UpdateDomainConfigRequest {
    type Output = UpdateDomainConfigResponse;

    fn service(&self) -> &'static str {
        "cdn"
    }

    fn action(&self) -> &'static str {
        "UpdateDomainConfig"
    }

    fn version(&self) -> &'static str {
        "2018-06-06"
    }

    fn region(&self) -> Option<&Region> {
        None
    }

    fn payload(&self) -> Result<Option<Value>, Error> {
        let value = serde_json::to_value(self).map_err(|source| {
            Error::invalid_request_with_source(
                "failed to serialize UpdateDomainConfig request payload",
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
    fn update_domain_config_payload() {
        let request = UpdateDomainConfigRequest::new("example.com", "cert_001");
        let payload = request.payload().unwrap().unwrap();

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
