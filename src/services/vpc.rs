use crate::{
    client::{TencentCloudAsync, TencentCloudBlocking},
    core::{Endpoint, TencentCloudResult},
};
use serde::Deserialize;
use serde_json::{json, Map, Value};
use std::borrow::Cow;

#[derive(Clone, Debug)]
/// Tag key-value pair used by create APIs.
///
/// | Field | Type | Description |
/// |-------|------|-------------|
/// | `key` | `&str` | Tag key. |
/// | `value` | `&str` | Tag value. |
pub struct TagRef<'a> {
    pub key: &'a str,
    pub value: &'a str,
}

#[derive(Debug, Deserialize)]
/// Response payload returned by VPC `DescribeVpcs`.
///
/// | Field | Type | Description |
/// |-------|------|-------------|
/// | `response` | [`DescribeVpcsResult`] | Result body containing VPC metadata. |
pub struct DescribeVpcsResponse {
    #[serde(rename = "Response")]
    pub response: DescribeVpcsResult,
}

#[derive(Debug, Deserialize)]
/// Detailed fields exposed by VPC listings.
///
/// | Field | Type | Description |
/// |-------|------|-------------|
/// | `total_count` | `Option<u64>` | Number of VPCs matching the query. |
/// | `vpc_set` | `Option<Vec<Value>>` | Raw VPC array returned by Tencent Cloud. |
/// | `request_id` | `String` | Unique request identifier. |
pub struct DescribeVpcsResult {
    #[serde(rename = "TotalCount")]
    pub total_count: Option<u64>,
    #[serde(rename = "VpcSet")]
    pub vpc_set: Option<Vec<Value>>,
    #[serde(rename = "RequestId")]
    pub request_id: String,
}

/// Request parameters for VPC `DescribeVpcs`.
///
/// | Field | Type | Required | Description |
/// |-------|------|----------|-------------|
/// | `region` | `Option<&str>` | Yes* | Target region or use client default. |
/// | `filters` | `Option<Value>` | No | Tencent Cloud filter objects (name/value). |
/// | `vpc_ids` | `Option<Vec<&str>>` | No | Explicit VPC ID list. |
/// | `limit` | `Option<u32>` | No | Page size (default 20, max 100). |
/// | `offset` | `Option<u32>` | No | Pagination offset. |
///
/// *Required unless a default region is configured on the client builder.
pub struct DescribeVpcs<'a> {
    pub region: Option<&'a str>,
    pub filters: Option<Value>,
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
        let mut map = Map::new();
        if let Some(ids) = &self.vpc_ids {
            map.insert("VpcIds".to_string(), json!(ids));
        }
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
/// Response payload returned by VPC `CreateVpc`.
///
/// | Field | Type | Description |
/// |-------|------|-------------|
/// | `response` | [`CreateVpcResult`] | Result body containing the created VPC descriptor. |
pub struct CreateVpcResponse {
    #[serde(rename = "Response")]
    pub response: CreateVpcResult,
}

#[derive(Debug, Deserialize)]
/// Result fields produced by VPC `CreateVpc`.
///
/// | Field | Type | Description |
/// |-------|------|-------------|
/// | `vpc` | `Option<Value>` | Raw VPC data returned by Tencent Cloud. |
/// | `request_id` | `String` | Unique request identifier. |
pub struct CreateVpcResult {
    #[serde(rename = "Vpc")]
    pub vpc: Option<Value>,
    #[serde(rename = "RequestId")]
    pub request_id: String,
}

/// Request parameters for VPC `CreateVpc`.
///
/// | Field | Type | Required | Description |
/// |-------|------|----------|-------------|
/// | `region` | `Option<&str>` | Yes* | Target region or use client default. |
/// | `vpc_name` | `&str` | Yes | Custom VPC name. |
/// | `cidr_block` | `&str` | Yes | CIDR block, e.g. `10.0.0.0/16`. |
/// | `enable_multicast` | `Option<bool>` | No | Whether to enable multicast (defaults to false). |
/// | `dns_servers` | `Option<Vec<&str>>` | No | Custom DNS servers. |
/// | `domain_name` | `Option<&str>` | No | Custom domain name. |
/// | `tags` | `Option<Vec<TagRef<'a>>>` | No | Tag list to associate with the VPC. |
///
/// *Required unless a default region is configured on the client builder.
pub struct CreateVpc<'a> {
    pub region: Option<&'a str>,
    pub vpc_name: &'a str,
    pub cidr_block: &'a str,
    pub enable_multicast: Option<bool>,
    pub dns_servers: Option<Vec<&'a str>>,
    pub domain_name: Option<&'a str>,
    pub tags: Option<Vec<TagRef<'a>>>,
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
        let mut map = Map::new();
        map.insert("VpcName".to_string(), json!(self.vpc_name));
        map.insert("CidrBlock".to_string(), json!(self.cidr_block));
        if let Some(multicast) = self.enable_multicast {
            map.insert("EnableMulticast".to_string(), json!(multicast));
        }
        if let Some(dns) = &self.dns_servers {
            map.insert("DnsServers".to_string(), json!(dns));
        }
        if let Some(domain) = self.domain_name {
            map.insert("DomainName".to_string(), json!(domain));
        }
        if let Some(tags) = &self.tags {
            let arr: Vec<Value> = tags
                .iter()
                .map(|tag| json!({ "Key": tag.key, "Value": tag.value }))
                .collect();
            map.insert("Tags".to_string(), Value::Array(arr));
        }
        Value::Object(map)
    }
}

#[derive(Debug, Deserialize)]
/// Response payload returned by VPC `CreateSubnet`.
///
/// | Field | Type | Description |
/// |-------|------|-------------|
/// | `response` | [`CreateSubnetResult`] | Result body containing subnet descriptor. |
pub struct CreateSubnetResponse {
    #[serde(rename = "Response")]
    pub response: CreateSubnetResult,
}

#[derive(Debug, Deserialize)]
/// Result fields produced by VPC `CreateSubnet`.
///
/// | Field | Type | Description |
/// |-------|------|-------------|
/// | `subnet` | `Option<Value>` | Raw subnet data returned by Tencent Cloud. |
/// | `request_id` | `String` | Unique request identifier. |
pub struct CreateSubnetResult {
    #[serde(rename = "Subnet")]
    pub subnet: Option<Value>,
    #[serde(rename = "RequestId")]
    pub request_id: String,
}

/// Request parameters for VPC `CreateSubnet`.
///
/// | Field | Type | Required | Description |
/// |-------|------|----------|-------------|
/// | `region` | `Option<&str>` | Yes* | Target region or use client default. |
/// | `vpc_id` | `&str` | Yes | VPC identifier containing the subnet. |
/// | `subnet_name` | `&str` | Yes | Subnet name. |
/// | `cidr_block` | `&str` | Yes | Subnet CIDR block. |
/// | `zone` | `&str` | Yes | Availability zone. |
/// | `is_default` | `Option<bool>` | No | Whether subnet is default. |
/// | `tags` | `Option<Vec<TagRef<'a>>>` | No | Tags to attach. |
///
/// *Required unless a default region is configured on the client builder.
pub struct CreateSubnet<'a> {
    pub region: Option<&'a str>,
    pub vpc_id: &'a str,
    pub subnet_name: &'a str,
    pub cidr_block: &'a str,
    pub zone: &'a str,
    pub is_default: Option<bool>,
    pub tags: Option<Vec<TagRef<'a>>>,
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
        let mut map = Map::new();
        map.insert("VpcId".to_string(), json!(self.vpc_id));
        map.insert("SubnetName".to_string(), json!(self.subnet_name));
        map.insert("CidrBlock".to_string(), json!(self.cidr_block));
        map.insert("Zone".to_string(), json!(self.zone));
        if let Some(is_default) = self.is_default {
            map.insert("IsDefault".to_string(), json!(is_default));
        }
        if let Some(tags) = &self.tags {
            let arr: Vec<Value> = tags
                .iter()
                .map(|tag| json!({ "Key": tag.key, "Value": tag.value }))
                .collect();
            map.insert("Tags".to_string(), Value::Array(arr));
        }
        Value::Object(map)
    }
}
#[derive(Debug, Deserialize)]
/// Response payload returned by VPC `DescribeSubnets`.
///
/// | Field | Type | Description |
/// |-------|------|-------------|
/// | `response` | [`DescribeSubnetsResult`] | Result body containing subnet metadata. |
pub struct DescribeSubnetsResponse {
    #[serde(rename = "Response")]
    pub response: DescribeSubnetsResult,
}

#[derive(Debug, Deserialize)]
/// Detailed fields exposed by subnet listings.
///
/// | Field | Type | Description |
/// |-------|------|-------------|
/// | `total_count` | `Option<u64>` | Number of subnets matching the query. |
/// | `subnet_set` | `Option<Vec<Value>>` | Raw subnet array returned by Tencent Cloud. |
/// | `request_id` | `String` | Unique request identifier. |
pub struct DescribeSubnetsResult {
    #[serde(rename = "TotalCount")]
    pub total_count: Option<u64>,
    #[serde(rename = "SubnetSet")]
    pub subnet_set: Option<Vec<Value>>,
    #[serde(rename = "RequestId")]
    pub request_id: String,
}

/// Request parameters for VPC `DescribeSubnets`.
///
/// | Field | Type | Required | Description |
/// |-------|------|----------|-------------|
/// | `region` | `Option<&str>` | Yes* | Target region or use client default. |
/// | `filters` | `Option<Value>` | No | Tencent Cloud filters to scope subnets. |
/// | `subnet_ids` | `Option<Vec<&str>>` | No | Explicit subnet ID list. |
/// | `vpc_id` | `Option<&str>` | No | Restrict results to a specific VPC. |
/// | `limit` | `Option<u32>` | No | Page size (default 20, max 100). |
/// | `offset` | `Option<u32>` | No | Pagination offset. |
///
/// *Required unless a default region is configured on the client builder.
pub struct DescribeSubnets<'a> {
    pub region: Option<&'a str>,
    pub filters: Option<Value>,
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
        let mut map = Map::new();
        if let Some(ids) = &self.subnet_ids {
            map.insert("SubnetIds".to_string(), json!(ids));
        }
        if let Some(filters) = &self.filters {
            map.insert("Filters".to_string(), filters.clone());
        }
        if let Some(vpc_id) = self.vpc_id {
            map.insert("VpcId".to_string(), json!(vpc_id));
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

/// List VPCs asynchronously via `DescribeVpcs`.
///
/// # Tencent Cloud Reference
/// | Item | Value |
/// |------|-------|
/// | Service | `vpc` |
/// | Action | `DescribeVpcs` |
/// | Version | `2017-03-12` |
/// | Rate Limit | 20 req/s |
///
/// Returns [`DescribeVpcsResponse`].
pub async fn describe_vpcs_async(
    client: &TencentCloudAsync,
    request: &DescribeVpcs<'_>,
) -> TencentCloudResult<DescribeVpcsResponse> {
    client.request(request).await
}

/// List VPCs with the blocking client.
///
/// Behaviour and parameters match [`describe_vpcs_async`].
pub fn describe_vpcs_blocking(
    client: &TencentCloudBlocking,
    request: &DescribeVpcs<'_>,
) -> TencentCloudResult<DescribeVpcsResponse> {
    client.request(request)
}

/// List subnets asynchronously via `DescribeSubnets`.
///
/// # Tencent Cloud Reference
/// | Item | Value |
/// |------|-------|
/// | Service | `vpc` |
/// | Action | `DescribeSubnets` |
/// | Version | `2017-03-12` |
/// | Rate Limit | 20 req/s |
///
/// Returns [`DescribeSubnetsResponse`].
pub async fn describe_subnets_async(
    client: &TencentCloudAsync,
    request: &DescribeSubnets<'_>,
) -> TencentCloudResult<DescribeSubnetsResponse> {
    client.request(request).await
}

/// List subnets with the blocking client.
///
/// Behaviour and parameters match [`describe_subnets_async`].
pub fn describe_subnets_blocking(
    client: &TencentCloudBlocking,
    request: &DescribeSubnets<'_>,
) -> TencentCloudResult<DescribeSubnetsResponse> {
    client.request(request)
}
/// Create a VPC asynchronously via `CreateVpc`.
///
/// # Tencent Cloud Reference
/// | Item | Value |
/// |------|-------|
/// | Service | `vpc` |
/// | Action | `CreateVpc` |
/// | Version | `2017-03-12` |
/// | Rate Limit | 20 req/s |
///
/// Returns [`CreateVpcResponse`].
pub async fn create_vpc_async(
    client: &TencentCloudAsync,
    request: &CreateVpc<'_>,
) -> TencentCloudResult<CreateVpcResponse> {
    client.request(request).await
}

/// Create a VPC with the blocking client.
///
/// Behaviour and parameters match [`create_vpc_async`].
pub fn create_vpc_blocking(
    client: &TencentCloudBlocking,
    request: &CreateVpc<'_>,
) -> TencentCloudResult<CreateVpcResponse> {
    client.request(request)
}

/// Create a subnet asynchronously via `CreateSubnet`.
///
/// # Tencent Cloud Reference
/// | Item | Value |
/// |------|-------|
/// | Service | `vpc` |
/// | Action | `CreateSubnet` |
/// | Version | `2017-03-12` |
/// | Rate Limit | 20 req/s |
///
/// Returns [`CreateSubnetResponse`].
pub async fn create_subnet_async(
    client: &TencentCloudAsync,
    request: &CreateSubnet<'_>,
) -> TencentCloudResult<CreateSubnetResponse> {
    client.request(request).await
}

/// Create a subnet with the blocking client.
///
/// Behaviour and parameters match [`create_subnet_async`].
pub fn create_subnet_blocking(
    client: &TencentCloudBlocking,
    request: &CreateSubnet<'_>,
) -> TencentCloudResult<CreateSubnetResponse> {
    client.request(request)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn describe_vpcs_payload_contains_ids_and_filters() {
        let filters = json!([
            { "Name": "vpc-name", "Values": ["prod"] }
        ]);
        let request = DescribeVpcs {
            region: Some("ap-shanghai"),
            filters: Some(filters.clone()),
            vpc_ids: Some(vec!["vpc-123", "vpc-456"]),
            limit: Some(50),
            offset: Some(10),
        };

        let payload = request.payload();
        assert_eq!(payload["VpcIds"], json!(["vpc-123", "vpc-456"]));
        assert_eq!(payload["Filters"], filters);
        assert_eq!(payload["Limit"], json!(50));
        assert_eq!(payload["Offset"], json!(10));
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
            tags: Some(vec![TagRef {
                key: "env",
                value: "test",
            }]),
        };

        let payload = request.payload();
        assert_eq!(payload["VpcName"], json!("demo"));
        assert_eq!(payload["CidrBlock"], json!("10.0.0.0/16"));
        assert_eq!(payload["EnableMulticast"], json!(true));
        assert_eq!(payload["DnsServers"], json!(["1.1.1.1", "8.8.8.8"]));
        assert_eq!(payload["DomainName"], json!("local"));
        assert_eq!(payload["Tags"], json!([{"Key": "env", "Value": "test"}]));
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
            tags: Some(vec![TagRef {
                key: "team",
                value: "core",
            }]),
        };

        let payload = request.payload();
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
        assert_eq!(parsed.response.request_id, "req-123");
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
        assert_eq!(payload["SubnetIds"], json!(["subnet-aaa"]));
        assert_eq!(payload["VpcId"], json!("vpc-zzz"));
    }
}
