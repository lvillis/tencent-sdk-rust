use crate::core::Endpoint;
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

/// 请求参数结构体 - 修改 TXT 记录
pub struct ModifyTXTRecord<'a> {
    pub domain: &'a str,
    pub record_line: &'a str,
    pub value: &'a str,
    pub record_id: u64,
    pub domain_id: Option<u64>,
    pub sub_domain: Option<&'a str>,
    pub record_line_id: Option<&'a str>,
    pub ttl: Option<u32>,
    pub status: Option<&'a str>,
    pub remark: Option<&'a str>,
}

impl<'a> ModifyTXTRecord<'a> {
    pub fn new(domain: &'a str, record_line: &'a str, value: &'a str, record_id: u64) -> Self {
        Self {
            domain,
            record_line,
            value,
            record_id,
            domain_id: None,
            sub_domain: None,
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

    pub fn with_sub_domain(mut self, sub_domain: &'a str) -> Self {
        self.sub_domain = Some(sub_domain);
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
}

impl<'a> Endpoint for ModifyTXTRecord<'a> {
    type Output = ModifyTXTRecordResponse;

    fn service(&self) -> Cow<'static, str> {
        Cow::Borrowed("dnspod")
    }

    fn action(&self) -> Cow<'static, str> {
        Cow::Borrowed("ModifyTXTRecord")
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
            "RecordLine": self.record_line,
            "Value": self.value,
            "RecordId": self.record_id,
        });

        if let Some(domain_id) = self.domain_id {
            payload["DomainId"] = json!(domain_id);
        }
        if let Some(sub_domain) = self.sub_domain {
            payload["SubDomain"] = json!(sub_domain);
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_modify_txt_record_payload() {
        let request = ModifyTXTRecord::new("example.com", "默认", "new-value", 123)
            .with_sub_domain("www")
            .with_ttl(300)
            .with_remark("更新后的记录");

        let payload = request.payload();
        assert_eq!(payload["Domain"], json!("example.com"));
        assert_eq!(payload["RecordLine"], json!("默认"));
        assert_eq!(payload["Value"], json!("new-value"));
        assert_eq!(payload["RecordId"], json!(123));
        assert_eq!(payload["SubDomain"], json!("www"));
        assert_eq!(payload["TTL"], json!(300));
        assert_eq!(payload["Remark"], json!("更新后的记录"));
    }

    #[test]
    fn test_deserialize_modify_response() {
        let json = r#"{
            "Response": {
                "RecordId": 456,
                "RequestId": "req-345678"
            }
        }"#;

        let response: ModifyTXTRecordResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.response.record_id, Some(456));
        assert_eq!(response.response.request_id, "req-345678");
    }

    #[test]
    fn test_endpoint_implementation() {
        let modify_request = ModifyTXTRecord::new("test.com", "默认", "value", 123);
        assert_eq!(modify_request.service().as_ref(), "dnspod");
        assert_eq!(modify_request.action().as_ref(), "ModifyTXTRecord");
        assert_eq!(modify_request.version().as_ref(), "2021-03-23");
        assert!(modify_request.region().is_none());
    }
}
