use crate::{
    Error,
    client::endpoint::Endpoint,
    types::{DomainName, Region, RequestId},
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum RecordType {
    Txt,
    A,
    Cname,
    Custom(String),
}

impl RecordType {
    pub fn as_str(&self) -> &str {
        match self {
            RecordType::Txt => "TXT",
            RecordType::A => "A",
            RecordType::Cname => "CNAME",
            RecordType::Custom(value) => value.as_str(),
        }
    }
}

impl From<&str> for RecordType {
    fn from(value: &str) -> Self {
        match value.to_ascii_uppercase().as_str() {
            "TXT" => RecordType::Txt,
            "A" => RecordType::A,
            "CNAME" => RecordType::Cname,
            _ => RecordType::Custom(value.to_string()),
        }
    }
}

impl Serialize for RecordType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum RecordLine {
    Default,
    Custom(String),
}

impl RecordLine {
    pub fn as_str(&self) -> &str {
        match self {
            RecordLine::Default => "默认",
            RecordLine::Custom(value) => value.as_str(),
        }
    }
}

impl From<&str> for RecordLine {
    fn from(value: &str) -> Self {
        match value {
            "默认" => RecordLine::Default,
            other if other.eq_ignore_ascii_case("default") => RecordLine::Default,
            other => RecordLine::Custom(other.to_string()),
        }
    }
}

impl Serialize for RecordLine {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateTxtRecordResponse {
    #[serde(rename = "Response")]
    pub response: CreateTxtRecordResult,
}

#[derive(Debug, Deserialize)]
pub struct CreateTxtRecordResult {
    #[serde(rename = "RecordId")]
    pub record_id: Option<u64>,
    #[serde(rename = "RequestId")]
    pub request_id: RequestId,
}

#[derive(Debug, Clone)]
pub struct CreateTxtRecordRequest {
    domain: DomainName,
    sub_domain: String,
    record_line: RecordLine,
    value: String,
    record_type: RecordType,
    domain_id: Option<u64>,
    record_line_id: Option<String>,
    ttl: Option<u32>,
    status: Option<String>,
    remark: Option<String>,
    group_id: Option<u64>,
}

impl CreateTxtRecordRequest {
    pub fn new(
        domain: impl Into<DomainName>,
        sub_domain: impl Into<String>,
        record_line: impl Into<RecordLine>,
        value: impl Into<String>,
    ) -> Self {
        Self {
            domain: domain.into(),
            sub_domain: sub_domain.into(),
            record_line: record_line.into(),
            value: value.into(),
            record_type: RecordType::Txt,
            domain_id: None,
            record_line_id: None,
            ttl: None,
            status: None,
            remark: None,
            group_id: None,
        }
    }

    pub fn domain_id(mut self, domain_id: u64) -> Self {
        self.domain_id = Some(domain_id);
        self
    }

    pub fn record_line_id(mut self, record_line_id: impl Into<String>) -> Self {
        self.record_line_id = Some(record_line_id.into());
        self
    }

    pub fn ttl(mut self, ttl: u32) -> Self {
        self.ttl = Some(ttl);
        self
    }

    pub fn status(mut self, status: impl Into<String>) -> Self {
        self.status = Some(status.into());
        self
    }

    pub fn remark(mut self, remark: impl Into<String>) -> Self {
        self.remark = Some(remark.into());
        self
    }

    pub fn group_id(mut self, group_id: u64) -> Self {
        self.group_id = Some(group_id);
        self
    }

    pub fn sub_domain(mut self, sub_domain: impl Into<String>) -> Self {
        self.sub_domain = sub_domain.into();
        self
    }

    pub fn record_type(mut self, record_type: impl Into<RecordType>) -> Self {
        self.record_type = record_type.into();
        self
    }

    pub fn record_line(mut self, record_line: impl Into<RecordLine>) -> Self {
        self.record_line = record_line.into();
        self
    }
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
struct CreateRecordPayload<'a> {
    domain: &'a DomainName,
    sub_domain: &'a str,
    record_type: &'a RecordType,
    record_line: &'a RecordLine,
    value: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    domain_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    record_line_id: Option<&'a str>,
    #[serde(rename = "TTL", skip_serializing_if = "Option::is_none")]
    ttl: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    remark: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    group_id: Option<u64>,
}

impl Endpoint for CreateTxtRecordRequest {
    type Output = CreateTxtRecordResponse;

    fn service(&self) -> &'static str {
        "dnspod"
    }

    fn action(&self) -> &'static str {
        "CreateRecord"
    }

    fn version(&self) -> &'static str {
        "2021-03-23"
    }

    fn region(&self) -> Option<&Region> {
        None
    }

    fn payload(&self) -> Result<Option<Value>, Error> {
        let payload = CreateRecordPayload {
            domain: &self.domain,
            sub_domain: &self.sub_domain,
            record_type: &self.record_type,
            record_line: &self.record_line,
            value: &self.value,
            domain_id: self.domain_id,
            record_line_id: self.record_line_id.as_deref(),
            ttl: self.ttl,
            status: self.status.as_deref(),
            remark: self.remark.as_deref(),
            group_id: self.group_id,
        };

        let value = serde_json::to_value(payload).map_err(|source| {
            Error::invalid_request_with_source(
                "failed to serialize CreateRecord request payload",
                Box::new(source),
            )
        })?;
        Ok(Some(value))
    }
}

#[derive(Debug, Deserialize)]
pub struct ModifyTxtRecordResponse {
    #[serde(rename = "Response")]
    pub response: ModifyTxtRecordResult,
}

#[derive(Debug, Deserialize)]
pub struct ModifyTxtRecordResult {
    #[serde(rename = "RecordId")]
    pub record_id: Option<u64>,
    #[serde(rename = "RequestId")]
    pub request_id: RequestId,
}

#[derive(Debug, Clone)]
pub struct ModifyTxtRecordRequest {
    domain: DomainName,
    sub_domain: String,
    record_line: RecordLine,
    value: String,
    record_type: RecordType,
    record_id: u64,
    domain_id: Option<u64>,
    record_line_id: Option<String>,
    ttl: Option<u32>,
    status: Option<String>,
    remark: Option<String>,
}

impl ModifyTxtRecordRequest {
    pub fn new(
        domain: impl Into<DomainName>,
        sub_domain: impl Into<String>,
        record_line: impl Into<RecordLine>,
        value: impl Into<String>,
        record_id: u64,
    ) -> Self {
        Self {
            domain: domain.into(),
            sub_domain: sub_domain.into(),
            record_line: record_line.into(),
            value: value.into(),
            record_type: RecordType::Txt,
            record_id,
            domain_id: None,
            record_line_id: None,
            ttl: None,
            status: None,
            remark: None,
        }
    }

    pub fn domain_id(mut self, domain_id: u64) -> Self {
        self.domain_id = Some(domain_id);
        self
    }

    pub fn record_line_id(mut self, record_line_id: impl Into<String>) -> Self {
        self.record_line_id = Some(record_line_id.into());
        self
    }

    pub fn ttl(mut self, ttl: u32) -> Self {
        self.ttl = Some(ttl);
        self
    }

    pub fn status(mut self, status: impl Into<String>) -> Self {
        self.status = Some(status.into());
        self
    }

    pub fn remark(mut self, remark: impl Into<String>) -> Self {
        self.remark = Some(remark.into());
        self
    }

    pub fn sub_domain(mut self, sub_domain: impl Into<String>) -> Self {
        self.sub_domain = sub_domain.into();
        self
    }

    pub fn record_type(mut self, record_type: impl Into<RecordType>) -> Self {
        self.record_type = record_type.into();
        self
    }

    pub fn record_line(mut self, record_line: impl Into<RecordLine>) -> Self {
        self.record_line = record_line.into();
        self
    }
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
struct ModifyRecordPayload<'a> {
    domain: &'a DomainName,
    sub_domain: &'a str,
    record_type: &'a RecordType,
    record_line: &'a RecordLine,
    value: &'a str,
    record_id: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    domain_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    record_line_id: Option<&'a str>,
    #[serde(rename = "TTL", skip_serializing_if = "Option::is_none")]
    ttl: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    remark: Option<&'a str>,
}

impl Endpoint for ModifyTxtRecordRequest {
    type Output = ModifyTxtRecordResponse;

    fn service(&self) -> &'static str {
        "dnspod"
    }

    fn action(&self) -> &'static str {
        "ModifyRecord"
    }

    fn version(&self) -> &'static str {
        "2021-03-23"
    }

    fn region(&self) -> Option<&Region> {
        None
    }

    fn payload(&self) -> Result<Option<Value>, Error> {
        let payload = ModifyRecordPayload {
            domain: &self.domain,
            sub_domain: &self.sub_domain,
            record_type: &self.record_type,
            record_line: &self.record_line,
            value: &self.value,
            record_id: self.record_id,
            domain_id: self.domain_id,
            record_line_id: self.record_line_id.as_deref(),
            ttl: self.ttl,
            status: self.status.as_deref(),
            remark: self.remark.as_deref(),
        };

        let value = serde_json::to_value(payload).map_err(|source| {
            Error::invalid_request_with_source(
                "failed to serialize ModifyRecord request payload",
                Box::new(source),
            )
        })?;
        Ok(Some(value))
    }
}

#[derive(Debug, Deserialize)]
pub struct DeleteRecordResponse {
    #[serde(rename = "Response")]
    pub response: DeleteRecordResult,
}

#[derive(Debug, Deserialize)]
pub struct DeleteRecordResult {
    #[serde(rename = "RequestId")]
    pub request_id: RequestId,
}

#[derive(Debug, Clone)]
pub struct DeleteRecordRequest {
    domain: DomainName,
    record_id: u64,
    domain_id: Option<u64>,
}

impl DeleteRecordRequest {
    pub fn new(domain: impl Into<DomainName>, record_id: u64) -> Self {
        Self {
            domain: domain.into(),
            record_id,
            domain_id: None,
        }
    }

    pub fn domain_id(mut self, domain_id: u64) -> Self {
        self.domain_id = Some(domain_id);
        self
    }
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
struct DeleteRecordPayload<'a> {
    domain: &'a DomainName,
    record_id: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    domain_id: Option<u64>,
}

impl Endpoint for DeleteRecordRequest {
    type Output = DeleteRecordResponse;

    fn service(&self) -> &'static str {
        "dnspod"
    }

    fn action(&self) -> &'static str {
        "DeleteRecord"
    }

    fn version(&self) -> &'static str {
        "2021-03-23"
    }

    fn region(&self) -> Option<&Region> {
        None
    }

    fn payload(&self) -> Result<Option<Value>, Error> {
        let payload = DeleteRecordPayload {
            domain: &self.domain,
            record_id: self.record_id,
            domain_id: self.domain_id,
        };

        let value = serde_json::to_value(payload).map_err(|source| {
            Error::invalid_request_with_source(
                "failed to serialize DeleteRecord request payload",
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
    fn create_txt_record_builder_roundtrip() {
        let request = CreateTxtRecordRequest::new("example.com", "www", "default", "test-value")
            .ttl(600)
            .status("ENABLE")
            .remark("test record")
            .group_id(1234);

        assert_eq!(request.domain.as_str(), "example.com");
        assert_eq!(request.sub_domain, "www");
        assert_eq!(request.record_line, RecordLine::Default);
        assert_eq!(request.value, "test-value");
        assert_eq!(request.record_type, RecordType::Txt);
        assert_eq!(request.ttl, Some(600));
        assert_eq!(request.status.as_deref(), Some("ENABLE"));
        assert_eq!(request.remark.as_deref(), Some("test record"));
        assert_eq!(request.group_id, Some(1234));
    }

    #[test]
    fn create_txt_record_payload() {
        let request = CreateTxtRecordRequest::new("example.com", "www", "default", "test-value")
            .ttl(600)
            .status("ENABLE");

        let payload = request.payload().unwrap().unwrap();
        assert_eq!(payload["Domain"], json!("example.com"));
        assert_eq!(payload["SubDomain"], json!("www"));
        assert_eq!(payload["RecordType"], json!("TXT"));
        assert_eq!(payload["RecordLine"], json!(RecordLine::Default.as_str()));
        assert_eq!(payload["Value"], json!("test-value"));
        assert_eq!(payload["TTL"], json!(600));
        assert_eq!(payload["Status"], json!("ENABLE"));
    }

    #[test]
    fn deserialize_create_response() {
        let payload = r#"{
            "Response": {
                "RecordId": 123,
                "RequestId": "req-123456"
            }
        }"#;

        let response: CreateTxtRecordResponse = serde_json::from_str(payload).unwrap();
        assert_eq!(response.response.record_id, Some(123));
        assert_eq!(response.response.request_id.as_str(), "req-123456");
    }

    #[test]
    fn custom_record_type_and_line_serialize() {
        let request = CreateTxtRecordRequest::new("example.com", "www", "default", "value")
            .record_type("CNAME")
            .record_line("custom-line");

        let payload = request.payload().unwrap().unwrap();
        assert_eq!(payload["RecordType"], json!("CNAME"));
        assert_eq!(payload["RecordLine"], json!("custom-line"));
    }

    #[test]
    fn modify_txt_record_payload() {
        let request =
            ModifyTxtRecordRequest::new("example.com", "www", "default", "new-value", 123)
                .ttl(300)
                .remark("updated record");

        let payload = request.payload().unwrap().unwrap();
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
    fn delete_record_payload() {
        let request = DeleteRecordRequest::new("example.com", 123).domain_id(456);

        let payload = request.payload().unwrap().unwrap();
        assert_eq!(payload["Domain"], json!("example.com"));
        assert_eq!(payload["RecordId"], json!(123));
        assert_eq!(payload["DomainId"], json!(456));
    }
}
