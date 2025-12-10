use crate::{
    client::{TencentCloudAsync, TencentCloudBlocking},
    core::{Endpoint, TencentCloudResult},
    services::dns::{RecordLine, RecordType},
};
use serde::Deserialize;
use serde_json::{json, Value};
use std::borrow::Cow;

#[derive(Debug, Deserialize)]
pub struct ModifyTXTRecordResponse {
    #[serde(rename = "Response")]
    pub response: ModifyTXTRecordResult,
}

#[derive(Debug, Deserialize)]
pub struct ModifyTXTRecordResult {
    #[serde(rename = "RecordId")]
    pub record_id: Option<u64>,
    #[serde(rename = "RequestId")]
    pub request_id: String,
}

/// Request parameters for modifying a TXT record.
pub struct ModifyTXTRecord<'a> {
    pub domain: &'a str,
    pub sub_domain: &'a str,
    pub record_line: RecordLine<'a>,
    pub value: &'a str,
    pub record_type: RecordType<'a>,
    pub record_id: u64,
    pub domain_id: Option<u64>,
    pub record_line_id: Option<&'a str>,
    pub ttl: Option<u32>,
    pub status: Option<&'a str>,
    pub remark: Option<&'a str>,
}

impl<'a> ModifyTXTRecord<'a> {
    /// Create a TXT record update request (defaults `RecordType` to `"TXT"`).
    pub fn new(
        domain: &'a str,
        sub_domain: &'a str,
        record_line: &'a str,
        value: &'a str,
        record_id: u64,
    ) -> Self {
        Self {
            domain,
            sub_domain,
            record_line: record_line.into(),
            value,
            record_type: RecordType::Txt,
            record_id,
            domain_id: None,
            record_line_id: None,
            ttl: None,
            status: None,
            remark: None,
        }
    }

    pub fn with_domain_id(mut self, domain_id: u64) -> Self {
        self.domain_id = Some(domain_id);
        self
    }

    pub fn with_record_line_id(mut self, record_line_id: &'a str) -> Self {
        self.record_line_id = Some(record_line_id);
        self
    }

    pub fn with_ttl(mut self, ttl: u32) -> Self {
        self.ttl = Some(ttl);
        self
    }

    pub fn with_status(mut self, status: &'a str) -> Self {
        self.status = Some(status);
        self
    }

    pub fn with_remark(mut self, remark: &'a str) -> Self {
        self.remark = Some(remark);
        self
    }

    pub fn with_sub_domain(mut self, sub_domain: &'a str) -> Self {
        self.sub_domain = sub_domain;
        self
    }

    pub fn with_record_type(mut self, record_type: RecordType<'a>) -> Self {
        self.record_type = record_type;
        self
    }

    pub fn with_record_line(mut self, record_line: RecordLine<'a>) -> Self {
        self.record_line = record_line;
        self
    }
}

impl<'a> Endpoint for ModifyTXTRecord<'a> {
    type Output = ModifyTXTRecordResponse;

    fn service(&self) -> Cow<'static, str> {
        Cow::Borrowed("dnspod")
    }

    fn action(&self) -> Cow<'static, str> {
        Cow::Borrowed("ModifyRecord")
    }

    fn version(&self) -> Cow<'static, str> {
        Cow::Borrowed("2021-03-23")
    }

    fn region(&self) -> Option<Cow<'_, str>> {
        None
    }

    fn payload(&self) -> Value {
        let mut payload = json!({
            "Domain": self.domain,
            "SubDomain": self.sub_domain,
            "RecordType": self.record_type,
            "RecordLine": self.record_line,
            "Value": self.value,
            "RecordId": self.record_id,
        });

        if let Some(domain_id) = self.domain_id {
            payload["DomainId"] = json!(domain_id);
        }
        if let Some(record_line_id) = self.record_line_id {
            payload["RecordLineId"] = json!(record_line_id);
        }
        if let Some(ttl) = self.ttl {
            payload["TTL"] = json!(ttl);
        }
        if let Some(status) = self.status {
            payload["Status"] = json!(status);
        }
        if let Some(remark) = self.remark {
            payload["Remark"] = json!(remark);
        }

        payload
    }
}

/// Call DNSPod `ModifyRecord` with the async client.
pub async fn modify_txt_record_async(
    client: &TencentCloudAsync,
    request: &ModifyTXTRecord<'_>,
) -> TencentCloudResult<ModifyTXTRecordResponse> {
    client.request(request).await
}

/// Call DNSPod `ModifyRecord` with the blocking client.
pub fn modify_txt_record_blocking(
    client: &TencentCloudBlocking,
    request: &ModifyTXTRecord<'_>,
) -> TencentCloudResult<ModifyTXTRecordResponse> {
    client.request(request)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_modify_txt_record_payload() {
        let request = ModifyTXTRecord::new("example.com", "www", "default", "new-value", 123)
            .with_ttl(300)
            .with_remark("updated record");

        let payload = request.payload();
        assert_eq!(payload["Domain"], json!("example.com"));
        assert_eq!(payload["SubDomain"], json!("www"));
        assert_eq!(payload["RecordType"], json!("TXT"));
        assert_eq!(payload["RecordLine"], json!(RecordLine::Default.as_str()));
        assert_eq!(payload["Value"], json!("new-value"));
        assert_eq!(payload["RecordId"], json!(123));
        assert_eq!(payload["TTL"], json!(300));
        assert_eq!(payload["Remark"], json!("updated record"));
    }

    #[test]
    fn test_deserialize_modify_response() {
        let json = r#"{
            "Response": {
                "RecordId": 456,
                "RequestId": "req-345678"
            }
        }"#;

        let response: ModifyTXTRecordResponse =
            serde_json::from_str(json).expect("deserialize ModifyTXTRecordResponse");
        assert_eq!(response.response.record_id, Some(456));
        assert_eq!(response.response.request_id, "req-345678");
    }

    #[test]
    fn test_endpoint_implementation() {
        let modify_request =
            ModifyTXTRecord::new("test.com", "_acme-challenge", "default", "value", 123);
        assert_eq!(modify_request.service().as_ref(), "dnspod");
        assert_eq!(modify_request.action().as_ref(), "ModifyRecord");
        assert_eq!(modify_request.version().as_ref(), "2021-03-23");
        assert!(modify_request.region().is_none());
    }
}
