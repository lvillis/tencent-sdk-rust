use crate::{
    client::{TencentCloudAsync, TencentCloudBlocking},
    core::{Endpoint, TencentCloudResult},
};
use serde::Deserialize;
use serde_json::{json, Value};
use std::borrow::Cow;

#[derive(Debug, Deserialize)]
pub struct DeleteRecordResponse {
    #[serde(rename = "Response")]
    pub response: DeleteRecordResult,
}

#[derive(Debug, Deserialize)]
pub struct DeleteRecordResult {
    #[serde(rename = "RequestId")]
    pub request_id: String,
}

/// Request parameters for deleting a record.
pub struct DeleteRecord<'a> {
    pub domain: &'a str,
    pub record_id: u64,
    pub domain_id: Option<u64>,
}

impl<'a> DeleteRecord<'a> {
    pub fn new(domain: &'a str, record_id: u64) -> Self {
        Self {
            domain,
            record_id,
            domain_id: None,
        }
    }

    pub fn with_domain_id(mut self, domain_id: u64) -> Self {
        self.domain_id = Some(domain_id);
        self
    }
}

impl<'a> Endpoint for DeleteRecord<'a> {
    type Output = DeleteRecordResponse;

    fn service(&self) -> Cow<'static, str> {
        Cow::Borrowed("dnspod")
    }

    fn action(&self) -> Cow<'static, str> {
        Cow::Borrowed("DeleteRecord")
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
            "RecordId": self.record_id,
        });

        if let Some(domain_id) = self.domain_id {
            payload["DomainId"] = json!(domain_id);
        }

        payload
    }
}

/// Call DNSPod `DeleteRecord` with the async client.
pub async fn delete_record_async(
    client: &TencentCloudAsync,
    request: &DeleteRecord<'_>,
) -> TencentCloudResult<DeleteRecordResponse> {
    client.request(request).await
}

/// Call DNSPod `DeleteRecord` with the blocking client.
pub fn delete_record_blocking(
    client: &TencentCloudBlocking,
    request: &DeleteRecord<'_>,
) -> TencentCloudResult<DeleteRecordResponse> {
    client.request(request)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delete_record_payload() {
        let request = DeleteRecord::new("example.com", 123).with_domain_id(456);

        let payload = request.payload();
        assert_eq!(payload["Domain"], json!("example.com"));
        assert_eq!(payload["RecordId"], json!(123));
        assert_eq!(payload["DomainId"], json!(456));
    }

    #[test]
    fn test_deserialize_delete_response() {
        let json = r#"{
            "Response": {
                "RequestId": "req-789012"
            }
        }"#;

        let response: DeleteRecordResponse =
            serde_json::from_str(json).expect("deserialize DeleteRecordResponse");
        assert_eq!(response.response.request_id, "req-789012");
    }

    #[test]
    fn test_endpoint_implementation() {
        let delete_request = DeleteRecord::new("test.com", 123);
        assert_eq!(delete_request.service().as_ref(), "dnspod");
        assert_eq!(delete_request.action().as_ref(), "DeleteRecord");
        assert_eq!(delete_request.version().as_ref(), "2021-03-23");
        assert!(delete_request.region().is_none());
    }
}
