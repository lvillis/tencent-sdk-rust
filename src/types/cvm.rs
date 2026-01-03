use crate::{
    Error,
    client::endpoint::Endpoint,
    types::{Filter, ImageId, InstanceId, Region, RequestId, SecurityGroupId, SubnetId, VpcId},
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

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
    #[serde(default)]
    pub instance_set: Vec<InstanceSummary>,
    #[serde(rename = "RequestId")]
    pub request_id: RequestId,
}

#[derive(Debug, Deserialize)]
pub struct InstanceSummary {
    #[serde(rename = "InstanceId")]
    pub instance_id: Option<InstanceId>,
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,
    #[serde(rename = "InstanceState")]
    pub instance_state: Option<String>,
    #[serde(rename = "InstanceType")]
    pub instance_type: Option<String>,
    #[serde(rename = "Cpu")]
    pub cpu: Option<u64>,
    #[serde(rename = "Memory")]
    pub memory: Option<u64>,
    #[serde(rename = "PrivateIpAddresses")]
    pub private_ip_addresses: Option<Vec<String>>,
    #[serde(rename = "PublicIpAddresses")]
    pub public_ip_addresses: Option<Vec<String>>,
    #[serde(default)]
    pub placement: Option<InstancePlacement>,
    #[serde(default)]
    pub system_disk: Option<DiskSummary>,
    #[serde(default)]
    pub data_disks: Option<Vec<DiskSummary>>,
    #[serde(flatten, default)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Deserialize)]
pub struct InstancePlacement {
    #[serde(rename = "Zone")]
    pub zone: Option<String>,
    #[serde(rename = "ProjectId")]
    pub project_id: Option<i64>,
    #[serde(flatten, default)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Deserialize)]
pub struct DiskSummary {
    #[serde(rename = "DiskType")]
    pub disk_type: Option<String>,
    #[serde(rename = "DiskSize")]
    pub disk_size: Option<u64>,
    #[serde(flatten, default)]
    pub extra: HashMap<String, Value>,
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
struct DescribeInstancesPayload<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    filters: Option<&'a [Filter]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    offset: Option<u32>,
}

pub struct DescribeInstancesRequest {
    region: Option<Region>,
    filters: Vec<Filter>,
    limit: Option<u32>,
    offset: Option<u32>,
}

impl Default for DescribeInstancesRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl DescribeInstancesRequest {
    pub fn new() -> Self {
        Self {
            region: None,
            filters: Vec::new(),
            limit: None,
            offset: None,
        }
    }

    pub fn region(mut self, region: impl Into<Region>) -> Self {
        self.region = Some(region.into());
        self
    }

    pub fn push_filter(mut self, filter: Filter) -> Self {
        self.filters.push(filter);
        self
    }

    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn offset(mut self, offset: u32) -> Self {
        self.offset = Some(offset);
        self
    }
}

impl Endpoint for DescribeInstancesRequest {
    type Output = DescribeInstancesResponse;

    fn service(&self) -> &'static str {
        "cvm"
    }

    fn action(&self) -> &'static str {
        "DescribeInstances"
    }

    fn version(&self) -> &'static str {
        "2017-03-12"
    }

    fn region(&self) -> Option<&Region> {
        self.region.as_ref()
    }

    fn payload(&self) -> Result<Option<Value>, Error> {
        let filters = (!self.filters.is_empty()).then_some(self.filters.as_slice());
        let payload = DescribeInstancesPayload {
            filters,
            limit: self.limit,
            offset: self.offset,
        };
        let value = serde_json::to_value(payload).map_err(|source| {
            Error::invalid_request_with_source(
                "failed to serialize DescribeInstances request payload",
                Box::new(source),
            )
        })?;
        Ok(Some(value))
    }
}

#[derive(Debug, Deserialize)]
pub struct GenericActionResponse {
    #[serde(rename = "Response")]
    pub response: GenericActionResult,
}

#[derive(Debug, Deserialize)]
pub struct GenericActionResult {
    #[serde(rename = "RequestId")]
    pub request_id: RequestId,
}

pub struct ResetInstancesPasswordRequest {
    region: Region,
    instance_ids: Vec<InstanceId>,
    password: String,
    username: Option<String>,
    force_stop: Option<bool>,
}

impl ResetInstancesPasswordRequest {
    pub fn new(
        region: impl Into<Region>,
        instance_ids: impl IntoIterator<Item = impl Into<InstanceId>>,
        password: impl Into<String>,
    ) -> Self {
        Self {
            region: region.into(),
            instance_ids: instance_ids.into_iter().map(Into::into).collect(),
            password: password.into(),
            username: None,
            force_stop: None,
        }
    }

    pub fn username(mut self, username: impl Into<String>) -> Self {
        self.username = Some(username.into());
        self
    }

    pub fn force_stop(mut self, enabled: bool) -> Self {
        self.force_stop = Some(enabled);
        self
    }
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
struct ResetInstancesPasswordPayload<'a> {
    instance_ids: &'a [InstanceId],
    password: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    force_stop: Option<bool>,
}

impl Endpoint for ResetInstancesPasswordRequest {
    type Output = GenericActionResponse;

    fn service(&self) -> &'static str {
        "cvm"
    }

    fn action(&self) -> &'static str {
        "ResetInstancesPassword"
    }

    fn version(&self) -> &'static str {
        "2017-03-12"
    }

    fn region(&self) -> Option<&Region> {
        Some(&self.region)
    }

    fn payload(&self) -> Result<Option<Value>, Error> {
        let payload = ResetInstancesPasswordPayload {
            instance_ids: &self.instance_ids,
            password: &self.password,
            user_name: self.username.as_deref(),
            force_stop: self.force_stop,
        };
        let value = serde_json::to_value(payload).map_err(|source| {
            Error::invalid_request_with_source(
                "failed to serialize ResetInstancesPassword request payload",
                Box::new(source),
            )
        })?;
        Ok(Some(value))
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
    pub request_id: RequestId,
}

pub struct DescribeInstanceVncUrlRequest {
    region: Region,
    instance_id: InstanceId,
}

impl DescribeInstanceVncUrlRequest {
    pub fn new(region: impl Into<Region>, instance_id: impl Into<InstanceId>) -> Self {
        Self {
            region: region.into(),
            instance_id: instance_id.into(),
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
struct DescribeInstanceVncUrlPayload<'a> {
    instance_id: &'a InstanceId,
}

impl Endpoint for DescribeInstanceVncUrlRequest {
    type Output = DescribeInstanceVncUrlResponse;

    fn service(&self) -> &'static str {
        "cvm"
    }

    fn action(&self) -> &'static str {
        "DescribeInstanceVncUrl"
    }

    fn version(&self) -> &'static str {
        "2017-03-12"
    }

    fn region(&self) -> Option<&Region> {
        Some(&self.region)
    }

    fn payload(&self) -> Result<Option<Value>, Error> {
        let payload = DescribeInstanceVncUrlPayload {
            instance_id: &self.instance_id,
        };
        let value = serde_json::to_value(payload).map_err(|source| {
            Error::invalid_request_with_source(
                "failed to serialize DescribeInstanceVncUrl request payload",
                Box::new(source),
            )
        })?;
        Ok(Some(value))
    }
}

#[derive(Debug, Deserialize)]
pub struct RunInstancesResponse {
    #[serde(rename = "Response")]
    pub response: RunInstancesResult,
}

#[derive(Debug, Deserialize)]
pub struct RunInstancesResult {
    #[serde(rename = "InstanceIdSet")]
    pub instance_id_set: Option<Vec<InstanceId>>,
    #[serde(rename = "RequestId")]
    pub request_id: RequestId,
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
struct RunInstancesPayload<'a> {
    image_id: &'a ImageId,
    instance_type: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_token: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subnet_id: Option<&'a SubnetId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_id: Option<&'a VpcId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_group_ids: Option<&'a [SecurityGroupId]>,
}

pub struct RunInstancesRequest {
    region: Region,
    image_id: ImageId,
    instance_type: String,
    instance_name: Option<String>,
    instance_count: Option<u32>,
    client_token: Option<String>,
    subnet_id: Option<SubnetId>,
    vpc_id: Option<VpcId>,
    security_group_ids: Vec<SecurityGroupId>,
}

impl RunInstancesRequest {
    pub fn new(
        region: impl Into<Region>,
        image_id: impl Into<ImageId>,
        instance_type: impl Into<String>,
    ) -> Self {
        Self {
            region: region.into(),
            image_id: image_id.into(),
            instance_type: instance_type.into(),
            instance_name: None,
            instance_count: None,
            client_token: None,
            subnet_id: None,
            vpc_id: None,
            security_group_ids: Vec::new(),
        }
    }

    pub fn instance_name(mut self, name: impl Into<String>) -> Self {
        self.instance_name = Some(name.into());
        self
    }

    pub fn instance_count(mut self, count: u32) -> Self {
        self.instance_count = Some(count);
        self
    }

    pub fn client_token(mut self, token: impl Into<String>) -> Self {
        self.client_token = Some(token.into());
        self
    }

    pub fn subnet_id(mut self, subnet_id: impl Into<SubnetId>) -> Self {
        self.subnet_id = Some(subnet_id.into());
        self
    }

    pub fn vpc_id(mut self, vpc_id: impl Into<VpcId>) -> Self {
        self.vpc_id = Some(vpc_id.into());
        self
    }

    pub fn security_group_ids<I, S>(mut self, groups: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<SecurityGroupId>,
    {
        self.security_group_ids = groups.into_iter().map(Into::into).collect();
        self
    }
}

impl Endpoint for RunInstancesRequest {
    type Output = RunInstancesResponse;

    fn service(&self) -> &'static str {
        "cvm"
    }

    fn action(&self) -> &'static str {
        "RunInstances"
    }

    fn version(&self) -> &'static str {
        "2017-03-12"
    }

    fn region(&self) -> Option<&Region> {
        Some(&self.region)
    }

    fn payload(&self) -> Result<Option<Value>, Error> {
        let security_group_ids =
            (!self.security_group_ids.is_empty()).then_some(self.security_group_ids.as_slice());

        let payload = RunInstancesPayload {
            image_id: &self.image_id,
            instance_type: &self.instance_type,
            instance_name: self.instance_name.as_deref(),
            instance_count: self.instance_count,
            client_token: self.client_token.as_deref(),
            subnet_id: self.subnet_id.as_ref(),
            vpc_id: self.vpc_id.as_ref(),
            security_group_ids,
        };

        let value = serde_json::to_value(payload).map_err(|source| {
            Error::invalid_request_with_source(
                "failed to serialize RunInstances request payload",
                Box::new(source),
            )
        })?;
        Ok(Some(value))
    }
}

pub struct StartInstancesRequest {
    region: Region,
    instance_ids: Vec<InstanceId>,
}

impl StartInstancesRequest {
    pub fn new(
        region: impl Into<Region>,
        instance_ids: impl IntoIterator<Item = impl Into<InstanceId>>,
    ) -> Self {
        Self {
            region: region.into(),
            instance_ids: instance_ids.into_iter().map(Into::into).collect(),
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
struct InstanceIdsPayload<'a> {
    instance_ids: &'a [InstanceId],
}

impl Endpoint for StartInstancesRequest {
    type Output = GenericActionResponse;

    fn service(&self) -> &'static str {
        "cvm"
    }

    fn action(&self) -> &'static str {
        "StartInstances"
    }

    fn version(&self) -> &'static str {
        "2017-03-12"
    }

    fn region(&self) -> Option<&Region> {
        Some(&self.region)
    }

    fn payload(&self) -> Result<Option<Value>, Error> {
        let payload = InstanceIdsPayload {
            instance_ids: &self.instance_ids,
        };
        let value = serde_json::to_value(payload).map_err(|source| {
            Error::invalid_request_with_source(
                "failed to serialize StartInstances request payload",
                Box::new(source),
            )
        })?;
        Ok(Some(value))
    }
}

pub struct RebootInstancesRequest {
    region: Region,
    instance_ids: Vec<InstanceId>,
    force_reboot: Option<bool>,
}

impl RebootInstancesRequest {
    pub fn new(
        region: impl Into<Region>,
        instance_ids: impl IntoIterator<Item = impl Into<InstanceId>>,
    ) -> Self {
        Self {
            region: region.into(),
            instance_ids: instance_ids.into_iter().map(Into::into).collect(),
            force_reboot: None,
        }
    }

    pub fn force_reboot(mut self, enabled: bool) -> Self {
        self.force_reboot = Some(enabled);
        self
    }
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
struct RebootInstancesPayload<'a> {
    instance_ids: &'a [InstanceId],
    #[serde(skip_serializing_if = "Option::is_none")]
    force_reboot: Option<bool>,
}

impl Endpoint for RebootInstancesRequest {
    type Output = GenericActionResponse;

    fn service(&self) -> &'static str {
        "cvm"
    }

    fn action(&self) -> &'static str {
        "RebootInstances"
    }

    fn version(&self) -> &'static str {
        "2017-03-12"
    }

    fn region(&self) -> Option<&Region> {
        Some(&self.region)
    }

    fn payload(&self) -> Result<Option<Value>, Error> {
        let payload = RebootInstancesPayload {
            instance_ids: &self.instance_ids,
            force_reboot: self.force_reboot,
        };
        let value = serde_json::to_value(payload).map_err(|source| {
            Error::invalid_request_with_source(
                "failed to serialize RebootInstances request payload",
                Box::new(source),
            )
        })?;
        Ok(Some(value))
    }
}

pub struct StopInstancesRequest {
    region: Region,
    instance_ids: Vec<InstanceId>,
    stop_type: Option<String>,
}

impl StopInstancesRequest {
    pub fn new(
        region: impl Into<Region>,
        instance_ids: impl IntoIterator<Item = impl Into<InstanceId>>,
    ) -> Self {
        Self {
            region: region.into(),
            instance_ids: instance_ids.into_iter().map(Into::into).collect(),
            stop_type: None,
        }
    }

    pub fn stop_type(mut self, stop_type: impl Into<String>) -> Self {
        self.stop_type = Some(stop_type.into());
        self
    }
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
struct StopInstancesPayload<'a> {
    instance_ids: &'a [InstanceId],
    #[serde(skip_serializing_if = "Option::is_none")]
    stop_type: Option<&'a str>,
}

impl Endpoint for StopInstancesRequest {
    type Output = GenericActionResponse;

    fn service(&self) -> &'static str {
        "cvm"
    }

    fn action(&self) -> &'static str {
        "StopInstances"
    }

    fn version(&self) -> &'static str {
        "2017-03-12"
    }

    fn region(&self) -> Option<&Region> {
        Some(&self.region)
    }

    fn payload(&self) -> Result<Option<Value>, Error> {
        let payload = StopInstancesPayload {
            instance_ids: &self.instance_ids,
            stop_type: self.stop_type.as_deref(),
        };
        let value = serde_json::to_value(payload).map_err(|source| {
            Error::invalid_request_with_source(
                "failed to serialize StopInstances request payload",
                Box::new(source),
            )
        })?;
        Ok(Some(value))
    }
}

pub struct ModifyInstancesProjectRequest {
    region: Region,
    instance_ids: Vec<InstanceId>,
    project_id: u64,
}

impl ModifyInstancesProjectRequest {
    pub fn new(
        region: impl Into<Region>,
        instance_ids: impl IntoIterator<Item = impl Into<InstanceId>>,
        project_id: u64,
    ) -> Self {
        Self {
            region: region.into(),
            instance_ids: instance_ids.into_iter().map(Into::into).collect(),
            project_id,
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
struct ModifyInstancesProjectPayload<'a> {
    instance_ids: &'a [InstanceId],
    project_id: u64,
}

impl Endpoint for ModifyInstancesProjectRequest {
    type Output = GenericActionResponse;

    fn service(&self) -> &'static str {
        "cvm"
    }

    fn action(&self) -> &'static str {
        "ModifyInstancesProject"
    }

    fn version(&self) -> &'static str {
        "2017-03-12"
    }

    fn region(&self) -> Option<&Region> {
        Some(&self.region)
    }

    fn payload(&self) -> Result<Option<Value>, Error> {
        let payload = ModifyInstancesProjectPayload {
            instance_ids: &self.instance_ids,
            project_id: self.project_id,
        };
        let value = serde_json::to_value(payload).map_err(|source| {
            Error::invalid_request_with_source(
                "failed to serialize ModifyInstancesProject request payload",
                Box::new(source),
            )
        })?;
        Ok(Some(value))
    }
}

pub struct TerminateInstancesRequest {
    region: Region,
    instance_ids: Vec<InstanceId>,
}

impl TerminateInstancesRequest {
    pub fn new(
        region: impl Into<Region>,
        instance_ids: impl IntoIterator<Item = impl Into<InstanceId>>,
    ) -> Self {
        Self {
            region: region.into(),
            instance_ids: instance_ids.into_iter().map(Into::into).collect(),
        }
    }
}

impl Endpoint for TerminateInstancesRequest {
    type Output = GenericActionResponse;

    fn service(&self) -> &'static str {
        "cvm"
    }

    fn action(&self) -> &'static str {
        "TerminateInstances"
    }

    fn version(&self) -> &'static str {
        "2017-03-12"
    }

    fn region(&self) -> Option<&Region> {
        Some(&self.region)
    }

    fn payload(&self) -> Result<Option<Value>, Error> {
        let payload = InstanceIdsPayload {
            instance_ids: &self.instance_ids,
        };
        let value = serde_json::to_value(payload).map_err(|source| {
            Error::invalid_request_with_source(
                "failed to serialize TerminateInstances request payload",
                Box::new(source),
            )
        })?;
        Ok(Some(value))
    }
}

#[derive(Debug, Deserialize)]
pub struct DescribeImagesResponse {
    #[serde(rename = "Response")]
    pub response: DescribeImagesResult,
}

#[derive(Debug, Deserialize)]
pub struct DescribeImagesResult {
    #[serde(rename = "TotalCount")]
    pub total_count: Option<u64>,
    #[serde(rename = "ImageSet")]
    pub image_set: Vec<ImageSummary>,
    #[serde(rename = "RequestId")]
    pub request_id: RequestId,
}

#[derive(Debug, Deserialize)]
pub struct ImageSummary {
    #[serde(rename = "ImageId")]
    pub image_id: Option<ImageId>,
    #[serde(rename = "ImageName")]
    pub image_name: Option<String>,
    #[serde(rename = "ImageType")]
    pub image_type: Option<String>,
    #[serde(rename = "CreatedTime")]
    pub created_time: Option<String>,
    #[serde(flatten, default)]
    pub extra: HashMap<String, Value>,
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
struct DescribeImagesPayload<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    image_ids: Option<&'a [ImageId]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filters: Option<&'a [Filter]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    offset: Option<u32>,
}

pub struct DescribeImagesRequest {
    region: Option<Region>,
    image_ids: Vec<ImageId>,
    filters: Vec<Filter>,
    limit: Option<u32>,
    offset: Option<u32>,
}

impl Default for DescribeImagesRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl DescribeImagesRequest {
    pub fn new() -> Self {
        Self {
            region: None,
            image_ids: Vec::new(),
            filters: Vec::new(),
            limit: None,
            offset: None,
        }
    }

    pub fn region(mut self, region: impl Into<Region>) -> Self {
        self.region = Some(region.into());
        self
    }

    pub fn push_image_id(mut self, id: impl Into<ImageId>) -> Self {
        self.image_ids.push(id.into());
        self
    }

    pub fn push_filter(mut self, filter: Filter) -> Self {
        self.filters.push(filter);
        self
    }

    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn offset(mut self, offset: u32) -> Self {
        self.offset = Some(offset);
        self
    }
}

impl Endpoint for DescribeImagesRequest {
    type Output = DescribeImagesResponse;

    fn service(&self) -> &'static str {
        "cvm"
    }

    fn action(&self) -> &'static str {
        "DescribeImages"
    }

    fn version(&self) -> &'static str {
        "2017-03-12"
    }

    fn region(&self) -> Option<&Region> {
        self.region.as_ref()
    }

    fn payload(&self) -> Result<Option<Value>, Error> {
        let image_ids = (!self.image_ids.is_empty()).then_some(self.image_ids.as_slice());
        let filters = (!self.filters.is_empty()).then_some(self.filters.as_slice());
        let payload = DescribeImagesPayload {
            image_ids,
            filters,
            limit: self.limit,
            offset: self.offset,
        };

        let value = serde_json::to_value(payload).map_err(|source| {
            Error::invalid_request_with_source(
                "failed to serialize DescribeImages request payload",
                Box::new(source),
            )
        })?;
        Ok(Some(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{Value, json};

    #[test]
    fn describe_instances_payload_supports_filters() {
        let request = DescribeInstancesRequest::new()
            .region("ap-shanghai")
            .limit(20)
            .offset(0)
            .push_filter(Filter::new("instance-id", ["ins-123"]))
            .push_filter(Filter::new("zone", ["ap-shanghai-1"]));

        let payload = request.payload().unwrap().unwrap();
        assert_eq!(payload["Filters"][0]["Name"], json!("instance-id"));
        assert_eq!(payload["Filters"][1]["Values"], json!(["ap-shanghai-1"]));
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
        assert_eq!(parsed.response.request_id.as_str(), "req-abc");
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

    #[test]
    fn describe_instances_builder_accumulates_filters() {
        let request = DescribeInstancesRequest::new()
            .region("ap-guangzhou")
            .push_filter(Filter::new("zone", ["ap-guangzhou-1"]))
            .limit(10)
            .offset(5);

        assert_eq!(
            request.region.as_ref().map(Region::as_str),
            Some("ap-guangzhou")
        );
        assert_eq!(request.filters.len(), 1);
        assert_eq!(request.filters[0].name, "zone");
        assert_eq!(request.filters[0].values[0], "ap-guangzhou-1");
        assert_eq!(request.limit, Some(10));
        assert_eq!(request.offset, Some(5));
    }

    #[test]
    fn run_instances_payload_includes_optional_fields() {
        let request = RunInstancesRequest::new("ap-beijing", "img-123", "S4.SMALL1")
            .instance_name("demo")
            .instance_count(2)
            .client_token("token")
            .subnet_id("subnet-123")
            .security_group_ids(["sg-1", "sg-2"]);

        let payload = request.payload().unwrap().unwrap();
        assert_eq!(payload["ImageId"], json!("img-123"));
        assert_eq!(payload["InstanceType"], json!("S4.SMALL1"));
        assert_eq!(payload["InstanceName"], json!("demo"));
        assert_eq!(payload["InstanceCount"], json!(2));
        assert_eq!(payload["ClientToken"], json!("token"));
        assert_eq!(payload["SubnetId"], json!("subnet-123"));
        assert_eq!(payload["SecurityGroupIds"], json!(["sg-1", "sg-2"]));
    }

    #[test]
    fn describe_images_payload_supports_filters() {
        let request = DescribeImagesRequest::new()
            .region("ap-beijing")
            .push_image_id("img-123")
            .push_filter(Filter::new("image-type", ["PUBLIC_IMAGE"]))
            .limit(10);

        let payload = request.payload().unwrap().unwrap();
        assert_eq!(payload["ImageIds"], json!(["img-123"]));
        assert_eq!(
            payload["Filters"],
            json!([{ "Name": "image-type", "Values": ["PUBLIC_IMAGE"] }])
        );
        assert_eq!(payload["Limit"], json!(10));
    }

    #[test]
    fn deserialize_run_instances_response() {
        let payload = r#"{
            "Response": {
                "InstanceIdSet": ["ins-1", "ins-2"],
                "RequestId": "req-789"
            }
        }"#;
        let parsed: RunInstancesResponse = serde_json::from_str(payload).unwrap();
        assert_eq!(
            parsed.response.instance_id_set.clone().unwrap(),
            vec![InstanceId::from("ins-1"), InstanceId::from("ins-2")]
        );
    }

    #[test]
    fn deserialize_describe_images_response() {
        let payload = r#"{
            "Response": {
                "TotalCount": 1,
                "ImageSet": [{
                    "ImageId": "img-1",
                    "ImageName": "test"
                }],
                "RequestId": "req-111"
            }
        }"#;
        let parsed: DescribeImagesResponse = serde_json::from_str(payload).unwrap();
        assert_eq!(parsed.response.total_count, Some(1));
        assert_eq!(
            parsed.response.image_set[0]
                .image_id
                .as_ref()
                .map(ImageId::as_str),
            Some("img-1")
        );
    }

    #[test]
    fn deserialize_instance_summary_preserves_unknown_fields() {
        let payload = r#"{
            "Response": {
                "TotalCount": 1,
                "InstanceSet": [{
                    "InstanceId": "ins-1",
                    "UnknownField": "extra"
                }],
                "RequestId": "req-xyz"
            }
        }"#;

        let parsed: DescribeInstancesResponse = serde_json::from_str(payload).unwrap();
        assert_eq!(
            parsed.response.instance_set[0]
                .instance_id
                .as_ref()
                .map(InstanceId::as_str),
            Some("ins-1")
        );
        assert_eq!(
            parsed.response.instance_set[0]
                .extra
                .get("UnknownField")
                .unwrap(),
            &Value::String("extra".to_string())
        );
    }
}
