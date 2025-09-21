use crate::{
    client::{TencentCloudAsync, TencentCloudBlocking},
    core::{Endpoint, TencentCloudResult},
};
use serde::Deserialize;
use serde_json::{json, Map, Value};
use std::borrow::Cow;

#[derive(Debug, Deserialize)]
pub struct DescribeInstancesResponse {
    #[serde(rename = "Response")]
    pub response: DescribeInstancesResult,
}

#[derive(Debug, Deserialize)]
pub struct DescribeInstancesResult {
    #[serde(rename = "TotalCount")]
    pub total_count: Option<u64>,
    #[serde(rename = "InstanceSet")]
    pub instance_set: Option<Vec<Value>>,
    #[serde(rename = "RequestId")]
    pub request_id: String,
}

/// Request wrapper for CVM `DescribeInstances`.
///
/// | Field | Type | Required | Description |
/// |-------|------|----------|-------------|
/// | `region` | `Option<&str>` | Yes* | Specify an explicit region or rely on the client default. |
/// | `filters` | `Option<Value>` | No | Tencent Cloud filter objects to scope the query. |
/// | `limit` | `Option<u32>` | No | Page size, maximum 100. |
/// | `offset` | `Option<u32>` | No | Pagination offset. |
///
/// *Required unless `TencentCloudAsync::with_default_region` (or blocking equivalent) was set.
pub struct DescribeInstances<'a> {
    pub region: Option<&'a str>,
    pub filters: Option<Value>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

impl<'a> Endpoint for DescribeInstances<'a> {
    type Output = DescribeInstancesResponse;

    fn service(&self) -> Cow<'static, str> {
        Cow::Borrowed("cvm")
    }

    fn action(&self) -> Cow<'static, str> {
        Cow::Borrowed("DescribeInstances")
    }

    fn version(&self) -> Cow<'static, str> {
        Cow::Borrowed("2017-03-12")
    }

    fn region(&self) -> Option<Cow<'_, str>> {
        self.region.map(Cow::Borrowed)
    }

    fn payload(&self) -> Value {
        let mut map = Map::new();
        if let Some(filters) = &self.filters {
            map.insert("Filters".to_string(), filters.clone());
        }
        if let Some(limit) = self.limit {
            map.insert("Limit".to_string(), json!(limit));
        }
        if let Some(offset) = self.offset {
            map.insert("Offset".to_string(), json!(offset));
        }
        Value::Object(map)
    }
}

#[derive(Debug, Deserialize)]
/// Generic response envelope returned by CVM mutation APIs.
///
/// | Field | Type | Description |
/// |-------|------|-------------|
/// | `response` | [`GenericActionResult`] | Minimal result set from the platform. |
pub struct GenericActionResponse {
    #[serde(rename = "Response")]
    pub response: GenericActionResult,
}

#[derive(Debug, Deserialize)]
/// Minimal response fields extracted from generic CVM actions.
///
/// | Field | Type | Description |
/// |-------|------|-------------|
/// | `request_id` | `String` | Unique request identifier. |
pub struct GenericActionResult {
    #[serde(rename = "RequestId")]
    pub request_id: String,
}

/// Request payload for `ResetInstancesPassword`.
///
/// | Field | Type | Required | Description |
/// |-------|------|----------|-------------|
/// | `region` | `&str` | Yes | Target region. |
/// | `instance_ids` | `&[&str]` | Yes | Instance ID list (<= 100 IDs). |
/// | `password` | `&str` | Yes | New login password respecting password policy. |
/// | `username` | `Option<&str>` | No | Custom user account to reset. |
/// | `force_stop` | `Option<bool>` | No | Whether to force shutdown before resetting. |
pub struct ResetInstancesPassword<'a> {
    pub region: &'a str,
    pub instance_ids: &'a [&'a str],
    pub password: &'a str,
    pub username: Option<&'a str>,
    pub force_stop: Option<bool>,
}

impl<'a> Endpoint for ResetInstancesPassword<'a> {
    type Output = GenericActionResponse;

    fn service(&self) -> Cow<'static, str> {
        Cow::Borrowed("cvm")
    }

    fn action(&self) -> Cow<'static, str> {
        Cow::Borrowed("ResetInstancesPassword")
    }

    fn version(&self) -> Cow<'static, str> {
        Cow::Borrowed("2017-03-12")
    }

    fn region(&self) -> Option<Cow<'_, str>> {
        Some(Cow::Borrowed(self.region))
    }

    fn payload(&self) -> Value {
        let mut map = Map::new();
        map.insert("InstanceIds".to_string(), json!(self.instance_ids));
        map.insert("Password".to_string(), json!(self.password));
        if let Some(username) = self.username {
            map.insert("UserName".to_string(), json!(username));
        }
        if let Some(force_stop) = self.force_stop {
            map.insert("ForceStop".to_string(), json!(force_stop));
        }
        Value::Object(map)
    }
}

#[derive(Debug, Deserialize)]
pub struct DescribeInstanceVncUrlResponse {
    #[serde(rename = "Response")]
    pub response: DescribeInstanceVncUrlResult,
}

#[derive(Debug, Deserialize)]
pub struct DescribeInstanceVncUrlResult {
    #[serde(rename = "InstanceVncUrl")]
    pub instance_vnc_url: Option<String>,
    #[serde(rename = "RequestId")]
    pub request_id: String,
}

/// Request payload for `DescribeInstanceVncUrl`.
///
/// | Field | Type | Required | Description |
/// |-------|------|----------|-------------|
/// | `region` | `&str` | Yes | Target region. |
/// | `instance_id` | `&str` | Yes | Instance ID whose web console URL is requested. |
pub struct DescribeInstanceVncUrl<'a> {
    pub region: &'a str,
    pub instance_id: &'a str,
}

impl<'a> Endpoint for DescribeInstanceVncUrl<'a> {
    type Output = DescribeInstanceVncUrlResponse;

    fn service(&self) -> Cow<'static, str> {
        Cow::Borrowed("cvm")
    }

    fn action(&self) -> Cow<'static, str> {
        Cow::Borrowed("DescribeInstanceVncUrl")
    }

    fn version(&self) -> Cow<'static, str> {
        Cow::Borrowed("2017-03-12")
    }

    fn region(&self) -> Option<Cow<'_, str>> {
        Some(Cow::Borrowed(self.region))
    }

    fn payload(&self) -> Value {
        json!({ "InstanceId": self.instance_id })
    }
}

/// Request payload for `StartInstances`.
///
/// | Field | Type | Required | Description |
/// |-------|------|----------|-------------|
/// | `region` | `&str` | Yes | Target region. |
/// | `instance_ids` | `&[&str]` | Yes | Instance ID list to start. |
pub struct StartInstances<'a> {
    pub region: &'a str,
    pub instance_ids: &'a [&'a str],
}

impl<'a> Endpoint for StartInstances<'a> {
    type Output = GenericActionResponse;

    fn service(&self) -> Cow<'static, str> {
        Cow::Borrowed("cvm")
    }

    fn action(&self) -> Cow<'static, str> {
        Cow::Borrowed("StartInstances")
    }

    fn version(&self) -> Cow<'static, str> {
        Cow::Borrowed("2017-03-12")
    }

    fn region(&self) -> Option<Cow<'_, str>> {
        Some(Cow::Borrowed(self.region))
    }

    fn payload(&self) -> Value {
        json!({ "InstanceIds": self.instance_ids })
    }
}

/// Request payload for `RebootInstances`.
///
/// | Field | Type | Required | Description |
/// |-------|------|----------|-------------|
/// | `region` | `&str` | Yes | Target region. |
/// | `instance_ids` | `&[&str]` | Yes | Instance ID list to reboot. |
/// | `reboot_type` | `Option<&str>` | No | Reboot strategy (`SOFT` or `HARD`). |
pub struct RebootInstances<'a> {
    pub region: &'a str,
    pub instance_ids: &'a [&'a str],
    pub reboot_type: Option<&'a str>,
}

impl<'a> Endpoint for RebootInstances<'a> {
    type Output = GenericActionResponse;

    fn service(&self) -> Cow<'static, str> {
        Cow::Borrowed("cvm")
    }

    fn action(&self) -> Cow<'static, str> {
        Cow::Borrowed("RebootInstances")
    }

    fn version(&self) -> Cow<'static, str> {
        Cow::Borrowed("2017-03-12")
    }

    fn region(&self) -> Option<Cow<'_, str>> {
        Some(Cow::Borrowed(self.region))
    }

    fn payload(&self) -> Value {
        let mut map = Map::new();
        map.insert("InstanceIds".to_string(), json!(self.instance_ids));
        if let Some(value) = self.reboot_type {
            map.insert("RebootType".to_string(), json!(value));
        }
        Value::Object(map)
    }
}

/// Request payload for `StopInstances`.
///
/// | Field | Type | Required | Description |
/// |-------|------|----------|-------------|
/// | `region` | `&str` | Yes | Target region. |
/// | `instance_ids` | `&[&str]` | Yes | Instance ID list to stop. |
/// | `stop_type` | `Option<&str>` | No | Stop strategy (`SOFT` or `HARD`). |
/// | `stopped_mode` | `Option<&str>` | No | Billing mode after stop (e.g. `KEEP_CHARGING`). |
pub struct StopInstances<'a> {
    pub region: &'a str,
    pub instance_ids: &'a [&'a str],
    pub stop_type: Option<&'a str>,
    pub stopped_mode: Option<&'a str>,
}

impl<'a> Endpoint for StopInstances<'a> {
    type Output = GenericActionResponse;

    fn service(&self) -> Cow<'static, str> {
        Cow::Borrowed("cvm")
    }

    fn action(&self) -> Cow<'static, str> {
        Cow::Borrowed("StopInstances")
    }

    fn version(&self) -> Cow<'static, str> {
        Cow::Borrowed("2017-03-12")
    }

    fn region(&self) -> Option<Cow<'_, str>> {
        Some(Cow::Borrowed(self.region))
    }

    fn payload(&self) -> Value {
        let mut map = Map::new();
        map.insert("InstanceIds".to_string(), json!(self.instance_ids));
        if let Some(value) = self.stop_type {
            map.insert("StopType".to_string(), json!(value));
        }
        if let Some(value) = self.stopped_mode {
            map.insert("StoppedMode".to_string(), json!(value));
        }
        Value::Object(map)
    }
}

/// Request payload for `ModifyInstancesProject`.
///
/// | Field | Type | Required | Description |
/// |-------|------|----------|-------------|
/// | `region` | `&str` | Yes | Target region. |
/// | `instance_ids` | `&[&str]` | Yes | Instance ID list to reassign. |
/// | `project_id` | `i32` | Yes | Target project ID. |
pub struct ModifyInstancesProject<'a> {
    pub region: &'a str,
    pub instance_ids: &'a [&'a str],
    pub project_id: i32,
}

impl<'a> Endpoint for ModifyInstancesProject<'a> {
    type Output = GenericActionResponse;

    fn service(&self) -> Cow<'static, str> {
        Cow::Borrowed("cvm")
    }

    fn action(&self) -> Cow<'static, str> {
        Cow::Borrowed("ModifyInstancesProject")
    }

    fn version(&self) -> Cow<'static, str> {
        Cow::Borrowed("2017-03-12")
    }

    fn region(&self) -> Option<Cow<'_, str>> {
        Some(Cow::Borrowed(self.region))
    }

    fn payload(&self) -> Value {
        json!({
            "InstanceIds": self.instance_ids,
            "ProjectId": self.project_id,
        })
    }
}

/// Execute CVM `DescribeInstances` asynchronously.
///
/// # Tencent Cloud Reference
/// | Item | Value |
/// |------|-------|
/// | Service | `cvm` |
/// | Action | `DescribeInstances` |
/// | Version | `2017-03-12` |
/// | Rate Limit | 40 req/s |
///
/// # Request Parameters
/// Mirrors [`DescribeInstances`] fields.
///
/// Returns [`DescribeInstancesResponse`].
pub async fn describe_instances_async(
    client: &TencentCloudAsync,
    request: &DescribeInstances<'_>,
) -> TencentCloudResult<DescribeInstancesResponse> {
    client.request(request).await
}

/// Execute CVM `DescribeInstances` with the blocking client.
///
/// Behaviour and parameters match [`describe_instances_async`].
pub fn describe_instances_blocking(
    client: &TencentCloudBlocking,
    request: &DescribeInstances<'_>,
) -> TencentCloudResult<DescribeInstancesResponse> {
    client.request(request)
}

/// Reset CVM instance passwords asynchronously.
///
/// # Tencent Cloud Reference
/// | Item | Value |
/// |------|-------|
/// | Service | `cvm` |
/// | Action | `ResetInstancesPassword` |
/// | Version | `2017-03-12` |
/// | Rate Limit | 10 req/s |
///
/// # Request Parameters
/// Mirrors [`ResetInstancesPassword`] fields.
///
/// Returns [`GenericActionResponse`].
pub async fn reset_instances_password_async(
    client: &TencentCloudAsync,
    request: &ResetInstancesPassword<'_>,
) -> TencentCloudResult<GenericActionResponse> {
    client.request(request).await
}

/// Reset CVM instance passwords with the blocking client.
///
/// Behaviour and parameters match [`reset_instances_password_async`].
pub fn reset_instances_password_blocking(
    client: &TencentCloudBlocking,
    request: &ResetInstancesPassword<'_>,
) -> TencentCloudResult<GenericActionResponse> {
    client.request(request)
}

/// Query VNC URLs for CVM instances asynchronously.
///
/// # Tencent Cloud Reference
/// | Item | Value |
/// |------|-------|
/// | Service | `cvm` |
/// | Action | `DescribeInstanceVncUrl` |
/// | Version | `2017-03-12` |
/// | Rate Limit | 10 req/s |
///
/// Returns [`DescribeInstanceVncUrlResponse`].
pub async fn describe_instance_vnc_url_async(
    client: &TencentCloudAsync,
    request: &DescribeInstanceVncUrl<'_>,
) -> TencentCloudResult<DescribeInstanceVncUrlResponse> {
    client.request(request).await
}

/// Query VNC URLs for CVM instances with the blocking client.
///
/// Behaviour and parameters match [`describe_instance_vnc_url_async`].
pub fn describe_instance_vnc_url_blocking(
    client: &TencentCloudBlocking,
    request: &DescribeInstanceVncUrl<'_>,
) -> TencentCloudResult<DescribeInstanceVncUrlResponse> {
    client.request(request)
}

/// Start CVM instances asynchronously.
///
/// # Tencent Cloud Reference
/// | Item | Value |
/// |------|-------|
/// | Service | `cvm` |
/// | Action | `StartInstances` |
/// | Version | `2017-03-12` |
///
/// Returns [`GenericActionResponse`].
pub async fn start_instances_async(
    client: &TencentCloudAsync,
    request: &StartInstances<'_>,
) -> TencentCloudResult<GenericActionResponse> {
    client.request(request).await
}

/// Start CVM instances with the blocking client.
///
/// Behaviour and parameters match [`start_instances_async`].
pub fn start_instances_blocking(
    client: &TencentCloudBlocking,
    request: &StartInstances<'_>,
) -> TencentCloudResult<GenericActionResponse> {
    client.request(request)
}

/// Reboot CVM instances asynchronously.
///
/// # Tencent Cloud Reference
/// | Item | Value |
/// |------|-------|
/// | Service | `cvm` |
/// | Action | `RebootInstances` |
/// | Version | `2017-03-12` |
///
/// Returns [`GenericActionResponse`].
pub async fn reboot_instances_async(
    client: &TencentCloudAsync,
    request: &RebootInstances<'_>,
) -> TencentCloudResult<GenericActionResponse> {
    client.request(request).await
}

/// Reboot CVM instances with the blocking client.
///
/// Behaviour and parameters match [`reboot_instances_async`].
pub fn reboot_instances_blocking(
    client: &TencentCloudBlocking,
    request: &RebootInstances<'_>,
) -> TencentCloudResult<GenericActionResponse> {
    client.request(request)
}

/// Stop CVM instances asynchronously.
///
/// # Tencent Cloud Reference
/// | Item | Value |
/// |------|-------|
/// | Service | `cvm` |
/// | Action | `StopInstances` |
/// | Version | `2017-03-12` |
///
/// Returns [`GenericActionResponse`].
pub async fn stop_instances_async(
    client: &TencentCloudAsync,
    request: &StopInstances<'_>,
) -> TencentCloudResult<GenericActionResponse> {
    client.request(request).await
}

/// Stop CVM instances with the blocking client.
///
/// Behaviour and parameters match [`stop_instances_async`].
pub fn stop_instances_blocking(
    client: &TencentCloudBlocking,
    request: &StopInstances<'_>,
) -> TencentCloudResult<GenericActionResponse> {
    client.request(request)
}

/// Change the project of CVM instances asynchronously.
///
/// # Tencent Cloud Reference
/// | Item | Value |
/// |------|-------|
/// | Service | `cvm` |
/// | Action | `ModifyInstancesProject` |
/// | Version | `2017-03-12` |
///
/// Returns [`GenericActionResponse`].
pub async fn modify_instances_project_async(
    client: &TencentCloudAsync,
    request: &ModifyInstancesProject<'_>,
) -> TencentCloudResult<GenericActionResponse> {
    client.request(request).await
}

/// Change the project of CVM instances with the blocking client.
///
/// Behaviour and parameters match [`modify_instances_project_async`].
pub fn modify_instances_project_blocking(
    client: &TencentCloudBlocking,
    request: &ModifyInstancesProject<'_>,
) -> TencentCloudResult<GenericActionResponse> {
    client.request(request)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn describe_instances_payload_supports_filters() {
        let filters = json!([
            { "Name": "instance-id", "Values": ["ins-123"] },
            { "Name": "zone", "Values": ["ap-shanghai-1"] }
        ]);
        let request = DescribeInstances {
            region: Some("ap-shanghai"),
            filters: Some(filters.clone()),
            limit: Some(20),
            offset: Some(0),
        };

        let payload = request.payload();
        assert_eq!(payload["Filters"], filters);
        assert_eq!(payload["Limit"], json!(20));
        assert_eq!(payload["Offset"], json!(0));
    }

    #[test]
    fn deserialize_generic_action_response() {
        let payload = r#"{
            "Response": {
                "RequestId": "req-abc"
            }
        }"#;
        let parsed: GenericActionResponse = serde_json::from_str(payload).unwrap();
        assert_eq!(parsed.response.request_id, "req-abc");
    }

    #[test]
    fn deserialize_vnc_url_response() {
        let payload = r#"{
            "Response": {
                "InstanceVncUrl": "https://example.com",
                "RequestId": "req-xyz"
            }
        }"#;
        let parsed: DescribeInstanceVncUrlResponse = serde_json::from_str(payload).unwrap();
        assert_eq!(
            parsed.response.instance_vnc_url.as_deref(),
            Some("https://example.com")
        );
    }
}
