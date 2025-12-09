```rust 
use crate::{
    client::{TencentCloudAsync, TencentCloudBlocking},
    core::{Endpoint, TencentCloudResult},
    services::Filter,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::borrow::Cow;
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
    pub request_id: String,
}

#[derive(Debug, Deserialize)]
pub struct InstanceSummary {
    #[serde(rename = "InstanceId")]
    pub instance_id: Option<String>,
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
    filters: Option<&'a [Filter<'a>]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    offset: Option<u32>,
}

/// Request wrapper for CVM `DescribeInstances`.
pub struct DescribeInstances<'a> {
    pub region: Option<&'a str>,
    pub filters: Option<Vec<Filter<'a>>>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

impl<'a> Default for DescribeInstances<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> DescribeInstances<'a> {
    /// Create an empty request configuration.
    pub fn new() -> Self {
        Self {
            region: None,
            filters: None,
            limit: None,
            offset: None,
        }
    }

    /// Set the target region for the request.
    pub fn with_region(mut self, region: &'a str) -> Self {
        self.region = Some(region);
        self
    }

    /// Add a filter to the request.
    pub fn push_filter(mut self, filter: Filter<'a>) -> Self {
        self.filters.get_or_insert_with(Vec::new).push(filter);
        self
    }

    /// Override the maximum number of items to return.
    pub fn with_limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Provide an offset for pagination.
    pub fn with_offset(mut self, offset: u32) -> Self {
        self.offset = Some(offset);
        self
    }
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
        let filters = self.filters.as_deref();
        serde_json::to_value(DescribeInstancesPayload {
            filters,
            limit: self.limit,
            offset: self.offset,
        })
        .expect("serialize DescribeInstances payload")
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
    pub request_id: String,
}

/// Request payload for `ResetInstancesPassword`.
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
        let mut payload = json!({
            "InstanceIds": self.instance_ids,
            "Password": self.password
        });

        if let Some(username) = self.username {
            payload["UserName"] = json!(username);
        }
        if let Some(force_stop) = self.force_stop {
            payload["ForceStop"] = json!(force_stop);
        }
        payload
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
pub struct RebootInstances<'a> {
    pub region: &'a str,
    pub instance_ids: &'a [&'a str],
    pub force_reboot: Option<bool>,
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
        let mut payload = json!({ "InstanceIds": self.instance_ids });
        if let Some(force_reboot) = self.force_reboot {
            payload["ForceReboot"] = json!(force_reboot);
        }
        payload
    }
}

/// Request payload for `StopInstances`.
pub struct StopInstances<'a> {
    pub region: &'a str,
    pub instance_ids: &'a [&'a str],
    pub stop_type: Option<&'a str>,
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
        let mut payload = json!({ "InstanceIds": self.instance_ids });
        if let Some(stop_type) = self.stop_type {
            payload["StopType"] = json!(stop_type);
        }
        payload
    }
}

/// Request payload for `ModifyInstancesProject`.
pub struct ModifyInstancesProject<'a> {
    pub region: &'a str,
    pub instance_ids: &'a [&'a str],
    pub project_id: u64,
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
            "ProjectId": self.project_id
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::Filter;

    #[test]
    fn describe_instances_payload_supports_filters() {
        let filters = vec![
            Filter::new("instance-id", ["ins-123"]),
            Filter::new("zone", ["ap-shanghai-1"]),
        ];
        let request = DescribeInstances {
            region: Some("ap-shanghai"),
            filters: Some(filters.clone()),
            limit: Some(20),
            offset: Some(0),
        };

        let payload = request.payload();
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

    #[test]
    fn describe_instances_builder_accumulates_filters() {
        let request = DescribeInstances::new()
            .with_region("ap-guangzhou")
            .push_filter(Filter::new("zone", ["ap-guangzhou-1"]))
            .with_limit(10)
            .with_offset(5);

        assert_eq!(request.region, Some("ap-guangzhou"));
        let filters = request.filters.as_ref().expect("filters set");
        assert_eq!(filters.len(), 1);
        assert_eq!(filters[0].name, "zone");
        assert_eq!(filters[0].values[0], "ap-guangzhou-1");
        assert_eq!(request.limit, Some(10));
        assert_eq!(request.offset, Some(5));
    }
}
```
dns 添加记录:
<<2. 输入参数
以下请求参数列表仅列出了接口请求参数和部分公共参数，完整公共参数列表见 公共请求参数。

参数名称	必选	类型	描述
Action	是	String	公共参数，本接口取值：CreateTXTRecord。
Version	是	String	公共参数，本接口取值：2021-03-23。
Region	否	String	公共参数，本接口不需要传递此参数。
Domain	是	String	域名
示例值：dnspod.cn
RecordLine	是	String	记录线路
示例值：默认
取值可参考：
获取等级允许的线路
按分类返回线路列表
Value	是	String	记录值，如 IP : 200.200.200.200， CNAME : cname.dnspod.com.， MX : mail.dnspod.com.。
示例值：200.200.200.200
DomainId	否	Integer	域名 ID 。参数 DomainId 优先级比参数 Domain 高，如果传递参数 DomainId 将忽略参数 Domain 。
示例值：1923
取值可参考：
获取域名列表
获取域名筛选列表
获取域名信息
SubDomain	否	String	主机记录，如 www，如果不传，默认为 @。
示例值：www
RecordLineId	否	String	线路的 ID，英文字符串，比如：10=1。参数RecordLineId优先级高于RecordLine，如果同时传递二者，优先使用RecordLineId参数。
示例值：10=1
取值可参考：
获取等级允许的线路
按分类返回线路列表
TTL	否	Integer	TTL，范围1-604800，不同套餐域名最小值不同。
示例值：600
Status	否	String	记录初始状态，取值范围为 ENABLE 和 DISABLE 。默认为 ENABLE ，如果传入 DISABLE，解析不会生效，也不会验证负载均衡的限制。
示例值：ENABLE
Remark	否	String	备注
示例值：官网主页
GroupId	否	Integer	记录分组 Id。
示例值：3430
取值可参考：
查询解析记录分组列表
3. 输出参数
参数名称	类型	描述
RecordId	Integer	记录ID
示例值：162
RequestId	String	唯一请求 ID，由服务端生成，每次请求都会返回（若请求因其他原因未能抵达服务端，则该次请求不会获得 RequestId）。定位问题时需要提供该次请求的 RequestId。>>

dns 删除记录:
<<2. 输入参数
以下请求参数列表仅列出了接口请求参数和部分公共参数，完整公共参数列表见 公共请求参数。

参数名称	必选	类型	描述
Action	是	String	公共参数，本接口取值：DeleteRecord。
Version	是	String	公共参数，本接口取值：2021-03-23。
Region	否	String	公共参数，本接口不需要传递此参数。
Domain	是	String	域名
示例值：dnspod.cn
RecordId	是	Integer	记录 ID 。可以通过接口DescribeRecordList查到所有的解析记录列表以及对应的RecordId
示例值：162
DomainId	否	Integer	域名 ID 。参数 DomainId 优先级比参数 Domain 高，如果传递参数 DomainId 将忽略参数 Domain 。可以通过接口DescribeDomainList查到所有的Domain以及DomainId
示例值：1923
3. 输出参数
参数名称	类型	描述
RequestId	String	唯一请求 ID，由服务端生成，每次请求都会返回（若请求因其他原因未能抵达服务端，则该次请求不会获得 RequestId）。定位问题时需要提供该次请求的 RequestId。>>

dns 修改记录:
<<2. 输入参数
以下请求参数列表仅列出了接口请求参数和部分公共参数，完整公共参数列表见 公共请求参数。

参数名称	必选	类型	描述
Action	是	String	公共参数，本接口取值：ModifyTXTRecord。
Version	是	String	公共参数，本接口取值：2021-03-23。
Region	否	String	公共参数，本接口不需要传递此参数。
Domain	是	String	域名
示例值：dnspod.cn
RecordLine	是	String	记录线路，通过 API 记录线路获得，中文，比如：默认。
示例值：默认
取值可参考：
获取等级允许的线路
按分类返回线路列表
Value	是	String	记录值，如 IP : 200.200.200.200， CNAME : cname.dnspod.com.， MX : mail.dnspod.com.。
示例值：200.200.200.200
RecordId	是	Integer	记录 ID 。
示例值：162
取值可参考：
添加记录
获取域名的解析记录列表
获取域名的解析记录筛选列表
DomainId	否	Integer	域名 ID 。参数 DomainId 优先级比参数 Domain 高，如果传递参数 DomainId 将忽略参数 Domain 。
示例值：1923
取值可参考：
获取域名列表
获取域名筛选列表
获取域名信息
SubDomain	否	String	主机记录，如 www，如果不传，默认为 @。
示例值：m
RecordLineId	否	String	线路的 ID，通过 API 记录线路获得，字符串，比如：10=1。参数RecordLineId优先级高于RecordLine，如果同时传递二者，优先使用RecordLineId参数。
示例值：10=1
取值可参考：
获取等级允许的线路
按分类返回线路列表
TTL	否	Integer	TTL，范围1-604800，不同等级域名最小值不同。单位：秒
示例值：600
Status	否	String	记录初始状态，取值范围为 ENABLE 和 DISABLE 。默认为 ENABLE ，如果传入 DISABLE，解析不会生效，也不会验证负载均衡的限制。
示例值：ENABLE
Remark	否	String	记录的备注信息。传空删除备注。
示例值：这是备注
3. 输出参数
参数名称	类型	描述
RecordId	Integer	记录ID
示例值：162
RequestId	String	唯一请求 ID，由服务端生成，每次请求都会返回（若请求因其他原因未能抵达服务端，则该次请求不会获得 RequestId）。定位问题时需要提供该次请求的 RequestId。>>

请根据以上文档和代码，输出 Rust 代码实现 Tencent Cloud DNS服务记录的添加、删除、修改。