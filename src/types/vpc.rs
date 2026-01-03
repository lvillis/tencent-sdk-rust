use crate::{
    Error,
    client::endpoint::Endpoint,
    types::{DomainName, Filter, Region, RequestId, SubnetId, Tag, VpcId},
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct DescribeVpcsResponse {
    #[serde(rename = "Response")]
    pub response: DescribeVpcsResult,
}

#[derive(Debug, Deserialize)]
pub struct DescribeVpcsResult {
    #[serde(rename = "TotalCount")]
    pub total_count: Option<u64>,
    #[serde(rename = "VpcSet")]
    #[serde(default)]
    pub vpc_set: Vec<VpcSummary>,
    #[serde(rename = "RequestId")]
    pub request_id: RequestId,
}

#[derive(Debug, Deserialize)]
pub struct VpcSummary {
    #[serde(rename = "VpcId")]
    pub vpc_id: Option<VpcId>,
    #[serde(rename = "VpcName")]
    pub vpc_name: Option<String>,
    #[serde(rename = "CidrBlock")]
    pub cidr_block: Option<String>,
    #[serde(rename = "IsDefault")]
    pub is_default: Option<bool>,
    #[serde(rename = "EnableMulticast")]
    pub enable_multicast: Option<bool>,
    #[serde(rename = "TagSet")]
    pub tag_set: Option<Vec<ResourceTag>>,
    #[serde(rename = "CreatedTime")]
    pub created_time: Option<String>,
    #[serde(rename = "VpcIdString")]
    pub vpc_id_string: Option<String>,
    #[serde(default)]
    pub ipv6_cidr_block: Option<String>,
    #[serde(flatten, default)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Deserialize)]
pub struct ResourceTag {
    #[serde(rename = "Key")]
    pub key: Option<String>,
    #[serde(rename = "Value")]
    pub value: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
struct DescribeVpcsPayload<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_ids: Option<&'a [VpcId]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filters: Option<&'a [Filter]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    offset: Option<u32>,
}

pub struct DescribeVpcsRequest {
    region: Option<Region>,
    filters: Vec<Filter>,
    vpc_ids: Vec<VpcId>,
    limit: Option<u32>,
    offset: Option<u32>,
}

impl Default for DescribeVpcsRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl DescribeVpcsRequest {
    pub fn new() -> Self {
        Self {
            region: None,
            filters: Vec::new(),
            vpc_ids: Vec::new(),
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

    pub fn push_vpc_id(mut self, vpc_id: impl Into<VpcId>) -> Self {
        self.vpc_ids.push(vpc_id.into());
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

impl Endpoint for DescribeVpcsRequest {
    type Output = DescribeVpcsResponse;

    fn service(&self) -> &'static str {
        "vpc"
    }

    fn action(&self) -> &'static str {
        "DescribeVpcs"
    }

    fn version(&self) -> &'static str {
        "2017-03-12"
    }

    fn region(&self) -> Option<&Region> {
        self.region.as_ref()
    }

    fn payload(&self) -> Result<Option<Value>, Error> {
        let vpc_ids = (!self.vpc_ids.is_empty()).then_some(self.vpc_ids.as_slice());
        let filters = (!self.filters.is_empty()).then_some(self.filters.as_slice());
        let payload = DescribeVpcsPayload {
            vpc_ids,
            filters,
            limit: self.limit,
            offset: self.offset,
        };

        let value = serde_json::to_value(payload).map_err(|source| {
            Error::invalid_request_with_source(
                "failed to serialize DescribeVpcs request payload",
                Box::new(source),
            )
        })?;
        Ok(Some(value))
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateVpcResponse {
    #[serde(rename = "Response")]
    pub response: CreateVpcResult,
}

#[derive(Debug, Deserialize)]
pub struct CreateVpcResult {
    #[serde(rename = "Vpc")]
    pub vpc: Option<VpcSummary>,
    #[serde(rename = "RequestId")]
    pub request_id: RequestId,
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
struct CreateVpcPayload<'a> {
    vpc_name: &'a str,
    cidr_block: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_multicast: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dns_servers: Option<&'a [String]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    domain_name: Option<&'a DomainName>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<&'a [Tag]>,
}

pub struct CreateVpcRequest {
    region: Option<Region>,
    vpc_name: String,
    cidr_block: String,
    enable_multicast: Option<bool>,
    dns_servers: Vec<String>,
    domain_name: Option<DomainName>,
    tags: Vec<Tag>,
}

impl CreateVpcRequest {
    pub fn new(vpc_name: impl Into<String>, cidr_block: impl Into<String>) -> Self {
        Self {
            region: None,
            vpc_name: vpc_name.into(),
            cidr_block: cidr_block.into(),
            enable_multicast: None,
            dns_servers: Vec::new(),
            domain_name: None,
            tags: Vec::new(),
        }
    }

    pub fn region(mut self, region: impl Into<Region>) -> Self {
        self.region = Some(region.into());
        self
    }

    pub fn enable_multicast(mut self, enabled: bool) -> Self {
        self.enable_multicast = Some(enabled);
        self
    }

    pub fn dns_servers<I, S>(mut self, dns_servers: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.dns_servers = dns_servers.into_iter().map(Into::into).collect();
        self
    }

    pub fn domain_name(mut self, domain_name: impl Into<DomainName>) -> Self {
        self.domain_name = Some(domain_name.into());
        self
    }

    pub fn push_tag(mut self, tag: Tag) -> Self {
        self.tags.push(tag);
        self
    }
}

impl Endpoint for CreateVpcRequest {
    type Output = CreateVpcResponse;

    fn service(&self) -> &'static str {
        "vpc"
    }

    fn action(&self) -> &'static str {
        "CreateVpc"
    }

    fn version(&self) -> &'static str {
        "2017-03-12"
    }

    fn region(&self) -> Option<&Region> {
        self.region.as_ref()
    }

    fn payload(&self) -> Result<Option<Value>, Error> {
        let dns_servers = (!self.dns_servers.is_empty()).then_some(self.dns_servers.as_slice());
        let tags = (!self.tags.is_empty()).then_some(self.tags.as_slice());
        let payload = CreateVpcPayload {
            vpc_name: &self.vpc_name,
            cidr_block: &self.cidr_block,
            enable_multicast: self.enable_multicast,
            dns_servers,
            domain_name: self.domain_name.as_ref(),
            tags,
        };

        let value = serde_json::to_value(payload).map_err(|source| {
            Error::invalid_request_with_source(
                "failed to serialize CreateVpc request payload",
                Box::new(source),
            )
        })?;
        Ok(Some(value))
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateSubnetResponse {
    #[serde(rename = "Response")]
    pub response: CreateSubnetResult,
}

#[derive(Debug, Deserialize)]
pub struct CreateSubnetResult {
    #[serde(rename = "Subnet")]
    pub subnet: Option<SubnetSummary>,
    #[serde(rename = "RequestId")]
    pub request_id: RequestId,
}

#[derive(Debug, Deserialize)]
pub struct SubnetSummary {
    #[serde(rename = "SubnetId")]
    pub subnet_id: Option<SubnetId>,
    #[serde(rename = "SubnetName")]
    pub subnet_name: Option<String>,
    #[serde(rename = "CidrBlock")]
    pub cidr_block: Option<String>,
    #[serde(rename = "IsDefault")]
    pub is_default: Option<bool>,
    #[serde(rename = "Zone")]
    pub zone: Option<String>,
    #[serde(flatten, default)]
    pub extra: HashMap<String, Value>,
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
struct CreateSubnetPayload<'a> {
    vpc_id: &'a VpcId,
    subnet_name: &'a str,
    cidr_block: &'a str,
    zone: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_default: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<&'a [Tag]>,
}

pub struct CreateSubnetRequest {
    region: Option<Region>,
    vpc_id: VpcId,
    subnet_name: String,
    cidr_block: String,
    zone: String,
    is_default: Option<bool>,
    tags: Vec<Tag>,
}

impl CreateSubnetRequest {
    pub fn new(
        vpc_id: impl Into<VpcId>,
        subnet_name: impl Into<String>,
        cidr_block: impl Into<String>,
        zone: impl Into<String>,
    ) -> Self {
        Self {
            region: None,
            vpc_id: vpc_id.into(),
            subnet_name: subnet_name.into(),
            cidr_block: cidr_block.into(),
            zone: zone.into(),
            is_default: None,
            tags: Vec::new(),
        }
    }

    pub fn region(mut self, region: impl Into<Region>) -> Self {
        self.region = Some(region.into());
        self
    }

    pub fn is_default(mut self, is_default: bool) -> Self {
        self.is_default = Some(is_default);
        self
    }

    pub fn push_tag(mut self, tag: Tag) -> Self {
        self.tags.push(tag);
        self
    }
}

impl Endpoint for CreateSubnetRequest {
    type Output = CreateSubnetResponse;

    fn service(&self) -> &'static str {
        "vpc"
    }

    fn action(&self) -> &'static str {
        "CreateSubnet"
    }

    fn version(&self) -> &'static str {
        "2017-03-12"
    }

    fn region(&self) -> Option<&Region> {
        self.region.as_ref()
    }

    fn payload(&self) -> Result<Option<Value>, Error> {
        let tags = (!self.tags.is_empty()).then_some(self.tags.as_slice());
        let payload = CreateSubnetPayload {
            vpc_id: &self.vpc_id,
            subnet_name: &self.subnet_name,
            cidr_block: &self.cidr_block,
            zone: &self.zone,
            is_default: self.is_default,
            tags,
        };

        let value = serde_json::to_value(payload).map_err(|source| {
            Error::invalid_request_with_source(
                "failed to serialize CreateSubnet request payload",
                Box::new(source),
            )
        })?;
        Ok(Some(value))
    }
}

#[derive(Debug, Deserialize)]
pub struct DescribeSubnetsResponse {
    #[serde(rename = "Response")]
    pub response: DescribeSubnetsResult,
}

#[derive(Debug, Deserialize)]
pub struct DescribeSubnetsResult {
    #[serde(rename = "TotalCount")]
    pub total_count: Option<u64>,
    #[serde(rename = "SubnetSet")]
    #[serde(default)]
    pub subnet_set: Vec<SubnetSummary>,
    #[serde(rename = "RequestId")]
    pub request_id: RequestId,
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
struct DescribeSubnetsPayload<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    subnet_ids: Option<&'a [SubnetId]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filters: Option<&'a [Filter]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_id: Option<&'a VpcId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    offset: Option<u32>,
}

pub struct DescribeSubnetsRequest {
    region: Option<Region>,
    filters: Vec<Filter>,
    subnet_ids: Vec<SubnetId>,
    vpc_id: Option<VpcId>,
    limit: Option<u32>,
    offset: Option<u32>,
}

impl Default for DescribeSubnetsRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl DescribeSubnetsRequest {
    pub fn new() -> Self {
        Self {
            region: None,
            filters: Vec::new(),
            subnet_ids: Vec::new(),
            vpc_id: None,
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

    pub fn push_subnet_id(mut self, subnet_id: impl Into<SubnetId>) -> Self {
        self.subnet_ids.push(subnet_id.into());
        self
    }

    pub fn vpc_id(mut self, vpc_id: impl Into<VpcId>) -> Self {
        self.vpc_id = Some(vpc_id.into());
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

impl Endpoint for DescribeSubnetsRequest {
    type Output = DescribeSubnetsResponse;

    fn service(&self) -> &'static str {
        "vpc"
    }

    fn action(&self) -> &'static str {
        "DescribeSubnets"
    }

    fn version(&self) -> &'static str {
        "2017-03-12"
    }

    fn region(&self) -> Option<&Region> {
        self.region.as_ref()
    }

    fn payload(&self) -> Result<Option<Value>, Error> {
        let subnet_ids = (!self.subnet_ids.is_empty()).then_some(self.subnet_ids.as_slice());
        let filters = (!self.filters.is_empty()).then_some(self.filters.as_slice());
        let payload = DescribeSubnetsPayload {
            subnet_ids,
            filters,
            vpc_id: self.vpc_id.as_ref(),
            limit: self.limit,
            offset: self.offset,
        };

        let value = serde_json::to_value(payload).map_err(|source| {
            Error::invalid_request_with_source(
                "failed to serialize DescribeSubnets request payload",
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
    fn describe_vpcs_payload_supports_filters() {
        let request = DescribeVpcsRequest::new()
            .region("ap-guangzhou")
            .push_vpc_id("vpc-123")
            .push_vpc_id("vpc-456")
            .push_filter(Filter::new("vpc-name", ["prod"]))
            .limit(50)
            .offset(10);

        let payload = request.payload().unwrap().unwrap();
        assert_eq!(payload["VpcIds"], json!(["vpc-123", "vpc-456"]));
        assert_eq!(payload["Filters"][0]["Name"], json!("vpc-name"));
        assert_eq!(payload["Filters"][0]["Values"], json!(["prod"]));
        assert_eq!(payload["Limit"], json!(50));
        assert_eq!(payload["Offset"], json!(10));
    }

    #[test]
    fn create_vpc_payload_includes_options() {
        let request = CreateVpcRequest::new("demo", "10.0.0.0/16")
            .region("ap-guangzhou")
            .enable_multicast(true)
            .dns_servers(["1.1.1.1", "8.8.8.8"])
            .domain_name("local")
            .push_tag(Tag::new("env", "test"));

        let payload = request.payload().unwrap().unwrap();
        assert_eq!(payload["VpcName"], json!("demo"));
        assert_eq!(payload["CidrBlock"], json!("10.0.0.0/16"));
        assert_eq!(payload["EnableMulticast"], json!(true));
        assert_eq!(payload["DnsServers"], json!(["1.1.1.1", "8.8.8.8"]));
        assert_eq!(payload["DomainName"], json!("local"));
        assert_eq!(payload["Tags"], json!([{"Key": "env", "Value": "test"}]));
    }

    #[test]
    fn create_subnet_payload_includes_tags() {
        let request =
            CreateSubnetRequest::new("vpc-123", "subnet-1", "10.0.1.0/24", "ap-beijing-1")
                .region("ap-beijing")
                .is_default(false)
                .push_tag(Tag::new("team", "core"));

        let payload = request.payload().unwrap().unwrap();
        assert_eq!(payload["VpcId"], json!("vpc-123"));
        assert_eq!(payload["SubnetName"], json!("subnet-1"));
        assert_eq!(payload["CidrBlock"], json!("10.0.1.0/24"));
        assert_eq!(payload["Zone"], json!("ap-beijing-1"));
        assert_eq!(payload["IsDefault"], json!(false));
        assert_eq!(payload["Tags"], json!([{"Key": "team", "Value": "core"}]));
    }

    #[test]
    fn deserialize_create_vpc_response() {
        let payload = r#"{
            "Response": {
                "Vpc": { "VpcId": "vpc-abc" },
                "RequestId": "req-123"
            }
        }"#;
        let parsed: CreateVpcResponse = serde_json::from_str(payload).unwrap();
        assert_eq!(parsed.response.request_id.as_str(), "req-123");
        assert_eq!(
            parsed
                .response
                .vpc
                .unwrap()
                .vpc_id
                .as_ref()
                .map(VpcId::as_str),
            Some("vpc-abc")
        );
    }

    #[test]
    fn deserialize_create_subnet_response() {
        let payload = r#"{
            "Response": {
                "Subnet": { "SubnetId": "subnet-xyz" },
                "RequestId": "req-456"
            }
        }"#;
        let parsed: CreateSubnetResponse = serde_json::from_str(payload).unwrap();
        assert_eq!(parsed.response.request_id.as_str(), "req-456");
        assert_eq!(
            parsed
                .response
                .subnet
                .unwrap()
                .subnet_id
                .as_ref()
                .map(SubnetId::as_str),
            Some("subnet-xyz")
        );
    }

    #[test]
    fn describe_subnets_payload_contains_vpc_id() {
        let request = DescribeSubnetsRequest::new()
            .region("ap-hongkong")
            .push_subnet_id("subnet-aaa")
            .vpc_id("vpc-zzz");

        let payload = request.payload().unwrap().unwrap();
        assert_eq!(payload["SubnetIds"], json!(["subnet-aaa"]));
        assert_eq!(payload["VpcId"], json!("vpc-zzz"));
    }

    #[test]
    fn describe_subnets_builder_eases_filters() {
        let request = DescribeSubnetsRequest::new()
            .region("ap-hongkong")
            .push_filter(Filter::new("subnet-name", ["blue"]))
            .push_subnet_id("subnet-aaa")
            .vpc_id("vpc-zzz");

        assert_eq!(
            request.region.as_ref().map(Region::as_str),
            Some("ap-hongkong")
        );
        assert_eq!(request.filters[0].name, "subnet-name");
        assert_eq!(request.subnet_ids[0].as_str(), "subnet-aaa");
        assert_eq!(request.vpc_id.as_ref().map(VpcId::as_str), Some("vpc-zzz"));
    }
}
