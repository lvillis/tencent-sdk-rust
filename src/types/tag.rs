use crate::{Error, client::endpoint::Endpoint, types::RequestId};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct DescribeProjectsResponse {
    #[serde(rename = "Response")]
    pub response: DescribeProjectsResult,
}

#[derive(Debug, Deserialize)]
pub struct DescribeProjectsResult {
    #[serde(rename = "TotalCount")]
    pub total_count: Option<u64>,
    #[serde(rename = "ProjectSet")]
    #[serde(default)]
    pub project_set: Vec<Value>,
    #[serde(rename = "RequestId")]
    pub request_id: RequestId,
}

#[derive(Debug, Clone)]
pub struct DescribeProjectsRequest {
    all_list: Option<i32>,
    limit: Option<i32>,
    offset: Option<i32>,
}

impl Default for DescribeProjectsRequest {
    fn default() -> Self {
        Self {
            all_list: Some(1),
            limit: Some(1000),
            offset: Some(0),
        }
    }
}

impl DescribeProjectsRequest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn include_all(mut self, yes: bool) -> Self {
        self.all_list = Some(if yes { 1 } else { 0 });
        self
    }

    pub fn limit(mut self, limit: i32) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn offset(mut self, offset: i32) -> Self {
        self.offset = Some(offset);
        self
    }
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
struct DescribeProjectsPayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    all_list: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    offset: Option<i32>,
}

impl Endpoint for DescribeProjectsRequest {
    type Output = DescribeProjectsResponse;

    fn service(&self) -> &'static str {
        "tag"
    }

    fn action(&self) -> &'static str {
        "DescribeProjects"
    }

    fn version(&self) -> &'static str {
        "2018-08-13"
    }

    fn payload(&self) -> Result<Option<Value>, Error> {
        let payload = DescribeProjectsPayload {
            all_list: self.all_list,
            limit: self.limit,
            offset: self.offset,
        };
        let value = serde_json::to_value(payload).map_err(|source| {
            Error::invalid_request_with_source(
                "failed to serialize DescribeProjects request payload",
                Box::new(source),
            )
        })?;
        Ok(Some(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_payload_with_defaults() {
        let request = DescribeProjectsRequest::new()
            .include_all(true)
            .limit(1000)
            .offset(0);
        let payload = request.payload().unwrap().unwrap();
        assert_eq!(payload["AllList"], serde_json::json!(1));
        assert_eq!(payload["Limit"], serde_json::json!(1000));
        assert_eq!(payload["Offset"], serde_json::json!(0));
    }

    #[test]
    fn deserialize_projects_response() {
        let payload = r#"{
            "Response": {
                "TotalCount": 2,
                "ProjectSet": [
                    { "ProjectId": 1, "Name": "sample" },
                    { "ProjectId": 2, "Name": "test" }
                ],
                "RequestId": "req-123"
            }
        }"#;
        let parsed: DescribeProjectsResponse = serde_json::from_str(payload).unwrap();
        assert_eq!(parsed.response.total_count, Some(2));
    }
}
