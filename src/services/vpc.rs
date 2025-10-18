use crate::{
    client::{TencentCloudAsync, TencentCloudBlocking},
    core::{Endpoint, TencentCloudResult},
    services::{Filter, Tag},
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::borrow::Cow;
use std::collections::HashMap;

/// Backwards compatible alias for service tags used across VPC APIs.
pub type TagRef<'a> = Tag<'a>;

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
    pub vpc_set: Option<Vec<VpcSummary>>,
    #[serde(rename = "RequestId")]
    pub request_id: String,
}

#[derive(Debug, Deserialize)]
pub struct VpcSummary {
    #[serde(rename = "VpcId")]
    pub vpc_id: Option<String>,
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
    #[serde(flatten)]
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
    vpc_ids: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filters: Option<&'a [Filter<'a>]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    offset: Option<u32>,
}

/// Request parameters for VPC `DescribeVpcs`.
pub struct DescribeVpcs<'a> {
    pub region: Option<&'a str>,
    pub filters: Option<Vec<Filter<'a>>>,
    pub vpc_ids: Option<Vec<&'a str>>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

impl<'a> Endpoint for DescribeVpcs<'a> {
    type Output = DescribeVpcsResponse;

    fn service(&self) -> Cow<'static, str> {
        Cow::Borrowed("vpc")
    }

    fn action(&self) -> Cow<'static, str> {
        Cow::Borrowed("DescribeVpcs")
    }

    fn version(&self) -> Cow<'static, str> {
        Cow::Borrowed("2017-03-12")
    }

    fn region(&self) -> Option<Cow<'_, str>> {
        self.region.map(Cow::Borrowed)
    }

    fn payload(&self) -> Value {
        let payload = DescribeVpcsPayload {
            vpc_ids: self.vpc_ids.as_deref(),
            filters: self.filters.as_deref(),
            limit: self.limit,
            offset: self.offset,
        };
        serde_json::to_value(payload).expect("serialize DescribeVpcs payload")
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
    pub request_id: String,
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
struct CreateVpcPayload<'a> {
    vpc_name: &'a str,
    cidr_block: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_multicast: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dns_servers: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    domain_name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<&'a [Tag<'a>]>,
}

/// Request parameters for VPC `CreateVpc`.
pub struct CreateVpc<'a> {
    pub region: Option<&'a str>,
    pub vpc_name: &'a str,
    pub cidr_block: &'a str,
    pub enable_multicast: Option<bool>,
    pub dns_servers: Option<Vec<&'a str>>,
    pub domain_name: Option<&'a str>,
    pub tags: Option<Vec<Tag<'a>>>,
}

impl<'a> Endpoint for CreateVpc<'a> {
    type Output = CreateVpcResponse;

    fn service(&self) -> Cow<'static, str> {
        Cow::Borrowed("vpc")
    }

    fn action(&self) -> Cow<'static, str> {
        Cow::Borrowed("CreateVpc")
    }

    fn version(&self) -> Cow<'static, str> {
        Cow::Borrowed("2017-03-12")
    }

    fn region(&self) -> Option<Cow<'_, str>> {
        self.region.map(Cow::Borrowed)
    }

    fn payload(&self) -> Value {
        let payload = CreateVpcPayload {
            vpc_name: self.vpc_name,
            cidr_block: self.cidr_block,
            enable_multicast: self.enable_multicast,
            dns_servers: self.dns_servers.as_deref(),
            domain_name: self.domain_name,
            tags: self.tags.as_deref(),
        };
        serde_json::to_value(payload).expect("serialize CreateVpc payload")
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
    pub request_id: String,
}

#[derive(Debug, Deserialize)]
pub struct SubnetSummary {
    #[serde(rename = "SubnetId")]
    pub subnet_id: Option<String>,
    #[serde(rename = "SubnetName")]
    pub subnet_name: Option<String>,
    #[serde(rename = "CidrBlock")]
    pub cidr_block: Option<String>,
    #[serde(rename = "Zone")]
    pub zone: Option<String>,
    #[serde(rename = "IsDefault")]
    pub is_default: Option<bool>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
struct CreateSubnetPayload<'a> {
    vpc_id: &'a str,
    subnet_name: &'a str,
    cidr_block: &'a str,
    zone: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_default: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<&'a [Tag<'a>]>,
}

/// Request parameters for VPC `CreateSubnet`.
pub struct CreateSubnet<'a> {
    pub region: Option<&'a str>,
    pub vpc_id: &'a str,
    pub subnet_name: &'a str,
    pub cidr_block: &'a str,
    pub zone: &'a str,
    pub is_default: Option<bool>,
    pub tags: Option<Vec<Tag<'a>>>,
}

impl<'a> Endpoint for CreateSubnet<'a> {
    type Output = CreateSubnetResponse;

    fn service(&self) -> Cow<'static, str> {
        Cow::Borrowed("vpc")
    }

    fn action(&self) -> Cow<'static, str> {
        Cow::Borrowed("CreateSubnet")
    }

    fn version(&self) -> Cow<'static, str> {
        Cow::Borrowed("2017-03-12")
    }

    fn region(&self) -> Option<Cow<'_, str>> {
        self.region.map(Cow::Borrowed)
    }

    fn payload(&self) -> Value {
        let payload = CreateSubnetPayload {
            vpc_id: self.vpc_id,
            subnet_name: self.subnet_name,
            cidr_block: self.cidr_block,
            zone: self.zone,
            is_default: self.is_default,
            tags: self.tags.as_deref(),
        };
        serde_json::to_value(payload).expect("serialize CreateSubnet payload")
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
    pub subnet_set: Option<Vec<SubnetSummary>>,
    #[serde(rename = "RequestId")]
    pub request_id: String,
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
struct DescribeSubnetsPayload<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    filters: Option<&'a [Filter<'a>]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subnet_ids: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_id: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    offset: Option<u32>,
}

/// Request parameters for VPC `DescribeSubnets`.
pub struct DescribeSubnets<'a> {
    pub region: Option<&'a str>,
    pub filters: Option<Vec<Filter<'a>>>,
    pub subnet_ids: Option<Vec<&'a str>>,
    pub vpc_id: Option<&'a str>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

impl<'a> Endpoint for DescribeSubnets<'a> {
    type Output = DescribeSubnetsResponse;

    fn service(&self) -> Cow<'static, str> {
        Cow::Borrowed("vpc")
    }

    fn action(&self) -> Cow<'static, str> {
        Cow::Borrowed("DescribeSubnets")
    }

    fn version(&self) -> Cow<'static, str> {
        Cow::Borrowed("2017-03-12")
    }

    fn region(&self) -> Option<Cow<'_, str>> {
        self.region.map(Cow::Borrowed)
    }

    fn payload(&self) -> Value {
        let payload = DescribeSubnetsPayload {
            filters: self.filters.as_deref(),
            subnet_ids: self.subnet_ids.as_deref(),
            vpc_id: self.vpc_id,
            limit: self.limit,
            offset: self.offset,
        };
        serde_json::to_value(payload).expect("serialize DescribeSubnets payload")
    }
}

/// Call `DescribeVpcs` asynchronously.
pub async fn describe_vpcs_async(
    client: &TencentCloudAsync,
    request: &DescribeVpcs<'_>,
) -> TencentCloudResult<DescribeVpcsResponse> {
    client.request(request).await
}

/// Call `DescribeVpcs` with the blocking client.
pub fn describe_vpcs_blocking(
    client: &TencentCloudBlocking,
    request: &DescribeVpcs<'_>,
) -> TencentCloudResult<DescribeVpcsResponse> {
    client.request(request)
}

/// Create a VPC asynchronously.
pub async fn create_vpc_async(
    client: &TencentCloudAsync,
    request: &CreateVpc<'_>,
) -> TencentCloudResult<CreateVpcResponse> {
    client.request(request).await
}

/// Create a VPC with the blocking client.
pub fn create_vpc_blocking(
    client: &TencentCloudBlocking,
    request: &CreateVpc<'_>,
) -> TencentCloudResult<CreateVpcResponse> {
    client.request(request)
}

/// Create a subnet asynchronously.
pub async fn create_subnet_async(
    client: &TencentCloudAsync,
    request: &CreateSubnet<'_>,
) -> TencentCloudResult<CreateSubnetResponse> {
    client.request(request).await
}

/// Create a subnet with the blocking client.
pub fn create_subnet_blocking(
    client: &TencentCloudBlocking,
    request: &CreateSubnet<'_>,
) -> TencentCloudResult<CreateSubnetResponse> {
    client.request(request)
}

/// Describe subnets asynchronously.
pub async fn describe_subnets_async(
    client: &TencentCloudAsync,
    request: &DescribeSubnets<'_>,
) -> TencentCloudResult<DescribeSubnetsResponse> {
    client.request(request).await
}

/// Describe subnets with the blocking client.
pub fn describe_subnets_blocking(
    client: &TencentCloudBlocking,
    request: &DescribeSubnets<'_>,
) -> TencentCloudResult<DescribeSubnetsResponse> {
    client.request(request)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn describe_vpcs_payload_contains_ids_and_filters() {
        let filters = vec![Filter::new("vpc-name", ["prod"])];
        let request = DescribeVpcs {
            region: Some("ap-shanghai"),
            filters: Some(filters.clone()),
            vpc_ids: Some(vec!["vpc-123", "vpc-456"]),
            limit: Some(50),
            offset: Some(10),
        };

        let payload = request.payload();
        assert_eq!(payload["VpcIds"], serde_json::json!(["vpc-123", "vpc-456"]));
        assert_eq!(payload["Filters"][0]["Name"], serde_json::json!("vpc-name"));
        assert_eq!(payload["Filters"][0]["Values"], serde_json::json!(["prod"]));
        assert_eq!(payload["Limit"], serde_json::json!(50));
        assert_eq!(payload["Offset"], serde_json::json!(10));
    }

    #[test]
    fn create_vpc_payload_includes_options() {
        let request = CreateVpc {
            region: Some("ap-guangzhou"),
            vpc_name: "demo",
            cidr_block: "10.0.0.0/16",
            enable_multicast: Some(true),
            dns_servers: Some(vec!["1.1.1.1", "8.8.8.8"]),
            domain_name: Some("local"),
            tags: Some(vec![Tag::new("env", "test")]),
        };

        let payload = request.payload();
        assert_eq!(payload["VpcName"], serde_json::json!("demo"));
        assert_eq!(payload["CidrBlock"], serde_json::json!("10.0.0.0/16"));
        assert_eq!(payload["EnableMulticast"], serde_json::json!(true));
        assert_eq!(
            payload["DnsServers"],
            serde_json::json!(["1.1.1.1", "8.8.8.8"])
        );
        assert_eq!(payload["DomainName"], serde_json::json!("local"));
        assert_eq!(
            payload["Tags"],
            serde_json::json!([{"Key": "env", "Value": "test"}])
        );
    }

    #[test]
    fn create_subnet_payload_includes_tags() {
        let request = CreateSubnet {
            region: Some("ap-beijing"),
            vpc_id: "vpc-123",
            subnet_name: "subnet-1",
            cidr_block: "10.0.1.0/24",
            zone: "ap-beijing-1",
            is_default: Some(false),
            tags: Some(vec![Tag::new("team", "core")]),
        };

        let payload = request.payload();
        assert_eq!(payload["VpcId"], serde_json::json!("vpc-123"));
        assert_eq!(payload["SubnetName"], serde_json::json!("subnet-1"));
        assert_eq!(payload["CidrBlock"], serde_json::json!("10.0.1.0/24"));
        assert_eq!(payload["Zone"], serde_json::json!("ap-beijing-1"));
        assert_eq!(payload["IsDefault"], serde_json::json!(false));
        assert_eq!(
            payload["Tags"],
            serde_json::json!([{"Key": "team", "Value": "core"}])
        );
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
        assert_eq!(parsed.response.request_id, "req-123");
        assert_eq!(
            parsed.response.vpc.unwrap().vpc_id.as_deref(),
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
        assert_eq!(parsed.response.request_id, "req-456");
        assert_eq!(
            parsed.response.subnet.unwrap().subnet_id.as_deref(),
            Some("subnet-xyz")
        );
    }

    #[test]
    fn describe_subnets_payload_contains_vpc_id() {
        let request = DescribeSubnets {
            region: Some("ap-hongkong"),
            filters: None,
            subnet_ids: Some(vec!["subnet-aaa"]),
            vpc_id: Some("vpc-zzz"),
            limit: None,
            offset: None,
        };

        let payload = request.payload();
        assert_eq!(payload["SubnetIds"], serde_json::json!(["subnet-aaa"]));
        assert_eq!(payload["VpcId"], serde_json::json!("vpc-zzz"));
    }
}
