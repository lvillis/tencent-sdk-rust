use crate::core::Endpoint;
use serde::Deserialize;
use serde_json::{json, Value};
use std::borrow::Cow;

#[derive(Debug, Deserialize)]
pub struct CreateTXTRecordResponse {
    #[serde(rename = "Response")]
    pub response: CreateTXTRecordResult,
}

#[derive(Debug, Deserialize)]
pub struct CreateTXTRecordResult {
    #[serde(rename = "RecordId")]
    pub record_id: Option<u64>,
    #[serde(rename = "RequestId")]
    pub request_id: String,
}

/// 请求参数结构体 - 添加 TXT 记录
pub struct CreateTXTRecord<'a> {
    pub domain: &'a str,
    pub record_line: &'a str,
    pub value: &'a str,
    pub domain_id: Option<u64>,
    pub sub_domain: Option<&'a str>,
    pub record_line_id: Option<&'a str>,
    pub ttl: Option<u32>,
    pub status: Option<&'a str>,
    pub remark: Option<&'a str>,
    pub group_id: Option<u64>,
}

impl<'a> CreateTXTRecord<'a> {
    pub fn new(domain: &'a str, record_line: &'a str, value: &'a str) -> Self {
        Self {
            domain,
            record_line,
            value,
            domain_id: None,
            sub_domain: None,
            record_line_id: None,
            ttl: None,
            status: None,
            remark: None,
            group_id: None,
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

    pub fn with_group_id(mut self, group_id: u64) -> Self {
        self.group_id = Some(group_id);
        self
    }
}

impl<'a> Endpoint for CreateTXTRecord<'a> {
    type Output = CreateTXTRecordResponse;

    fn service(&self) -> Cow<'static, str> {
        Cow::Borrowed("dnspod")
    }

    fn action(&self) -> Cow<'static, str> {
        Cow::Borrowed("CreateTXTRecord")
    }

    fn version(&self) -> Cow<'static, str> {
        Cow::Borrowed("2021-03-23")
    }

    fn region(&self) -> Option<Cow<'_, str>> {
        // DNS 接口通常不需要 region 参数
        None
    }

    fn payload(&self) -> Value {
        let mut payload = json!({
            "Domain": self.domain,
            "RecordLine": self.record_line,
            "Value": self.value,
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
        if let Some(group_id) = self.group_id {
            payload["GroupId"] = json!(group_id);
        }

        payload
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_txt_record_builder() {
        let request = CreateTXTRecord::new("example.com", "默认", "test-value")
            .with_sub_domain("www")
            .with_ttl(600)
            .with_status("ENABLE")
            .with_remark("测试记录")
            .with_group_id(1234);

        assert_eq!(request.domain, "example.com");
        assert_eq!(request.record_line, "默认");
        assert_eq!(request.value, "test-value");
        assert_eq!(request.sub_domain, Some("www"));
        assert_eq!(request.ttl, Some(600));
        assert_eq!(request.status, Some("ENABLE"));
        assert_eq!(request.remark, Some("测试记录"));
        assert_eq!(request.group_id, Some(1234));
    }

    #[test]
    fn test_create_txt_record_payload() {
        let request = CreateTXTRecord::new("example.com", "默认", "test-value")
            .with_sub_domain("www")
            .with_ttl(600)
            .with_status("ENABLE");

        let payload = request.payload();
        assert_eq!(payload["Domain"], json!("example.com"));
        assert_eq!(payload["RecordLine"], json!("默认"));
        assert_eq!(payload["Value"], json!("test-value"));
        assert_eq!(payload["SubDomain"], json!("www"));
        assert_eq!(payload["TTL"], json!(600));
        assert_eq!(payload["Status"], json!("ENABLE"));
    }

    #[test]
    fn test_deserialize_create_response() {
        let json = r#"{
            "Response": {
                "RecordId": 123,
                "RequestId": "req-123456"
            }
        }"#;

        let response: CreateTXTRecordResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.response.record_id, Some(123));
        assert_eq!(response.response.request_id, "req-123456");
    }

    #[test]
    fn test_endpoint_implementation() {
        let create_request = CreateTXTRecord::new("test.com", "默认", "value");
        assert_eq!(create_request.service().as_ref(), "dnspod");
        assert_eq!(create_request.action().as_ref(), "CreateTXTRecord");
        assert_eq!(create_request.version().as_ref(), "2021-03-23");
        assert!(create_request.region().is_none());
    }
}
