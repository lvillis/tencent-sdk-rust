use crate::{
    client::{TencentCloudAsync, TencentCloudBlocking},
    core::{Endpoint, TencentCloudResult},
};
use serde::Deserialize;
use serde_json::{json, Value};
use std::borrow::Cow;

#[derive(Debug, Deserialize)]
/// Response payload returned by Tag `DescribeProjects`.
///
/// | Field | Type | Description |
/// |-------|------|-------------|
/// | `response` | [`DescribeProjectsResult`] | Result body containing project metadata. |
pub struct DescribeProjectsResponse {
    #[serde(rename = "Response")]
    pub response: DescribeProjectsResult,
}

#[derive(Debug, Deserialize)]
/// Detailed fields exposed by Tag project listings.
///
/// | Field | Type | Description |
/// |-------|------|-------------|
/// | `total_count` | `Option<u64>` | Number of projects matching the query. |
/// | `project_set` | `Option<Vec<Value>>` | Raw project array returned by Tencent Cloud. |
/// | `request_id` | `String` | Unique request identifier. |
pub struct DescribeProjectsResult {
    #[serde(rename = "TotalCount")]
    pub total_count: Option<u64>,
    #[serde(rename = "ProjectSet")]
    pub project_set: Option<Vec<Value>>,
    #[serde(rename = "RequestId")]
    pub request_id: String,
}

/// Request parameters for Tag `DescribeProjects`.
///
/// | Field | Type | Required | Description |
/// |-------|------|----------|-------------|
/// | `all_list` | `Option<i32>` | No | Whether to fetch all projects (defaults to `1`). |
/// | `limit` | `Option<i32>` | No | Maximum number of items per page (defaults to `1000`). |
/// | `offset` | `Option<i32>` | No | Pagination offset (defaults to `0`). |
pub struct DescribeProjects {
    pub all_list: Option<i32>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

impl Default for DescribeProjects {
    fn default() -> Self {
        Self {
            all_list: Some(1),
            limit: Some(1000),
            offset: Some(0),
        }
    }
}

impl Endpoint for DescribeProjects {
    type Output = DescribeProjectsResponse;

    fn service(&self) -> Cow<'static, str> {
        Cow::Borrowed("tag")
    }

    fn action(&self) -> Cow<'static, str> {
        Cow::Borrowed("DescribeProjects")
    }

    fn version(&self) -> Cow<'static, str> {
        Cow::Borrowed("2018-08-13")
    }

    fn payload(&self) -> Value {
        json!({
            "AllList": self.all_list.unwrap_or(1),
            "Limit": self.limit.unwrap_or(1000),
            "Offset": self.offset.unwrap_or(0)
        })
    }
}

/// List Tag service projects asynchronously via `DescribeProjects`.
///
/// # Tencent Cloud Reference
/// | Item | Value |
/// |------|-------|
/// | Service | `tag` |
/// | Action | `DescribeProjects` |
/// | Version | `2018-08-13` |
/// | Rate Limit | 20 req/s |
///
/// Returns [`DescribeProjectsResponse`].
pub async fn describe_projects_async(
    client: &TencentCloudAsync,
    request: &DescribeProjects,
) -> TencentCloudResult<DescribeProjectsResponse> {
    client.request(request).await
}

/// List Tag service projects with the blocking client.
///
/// Behaviour and parameters match [`describe_projects_async`].
pub fn describe_projects_blocking(
    client: &TencentCloudBlocking,
    request: &DescribeProjects,
) -> TencentCloudResult<DescribeProjectsResponse> {
    client.request(request)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_payload_with_defaults() {
        let request = DescribeProjects::default();
        let payload = request.payload();
        assert_eq!(payload["AllList"], json!(1));
        assert_eq!(payload["Limit"], json!(1000));
        assert_eq!(payload["Offset"], json!(0));
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
