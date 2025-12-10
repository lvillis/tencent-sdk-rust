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

/// Invoke `DescribeInstances` asynchronously.
pub async fn describe_instances_async(
    client: &TencentCloudAsync,
    request: &DescribeInstances<'_>,
) -> TencentCloudResult<DescribeInstancesResponse> {
    client.request(request).await
}

/// Invoke `DescribeInstances` with the blocking client.
pub fn describe_instances_blocking(
    client: &TencentCloudBlocking,
    request: &DescribeInstances<'_>,
) -> TencentCloudResult<DescribeInstancesResponse> {
    client.request(request)
}

/// Reset instances password asynchronously.
pub async fn reset_instances_password_async(
    client: &TencentCloudAsync,
    request: &ResetInstancesPassword<'_>,
) -> TencentCloudResult<GenericActionResponse> {
    client.request(request).await
}

/// Reset instances password with the blocking client.
pub fn reset_instances_password_blocking(
    client: &TencentCloudBlocking,
    request: &ResetInstancesPassword<'_>,
) -> TencentCloudResult<GenericActionResponse> {
    client.request(request)
}

/// Fetch CVM VNC URL asynchronously.
pub async fn describe_instance_vnc_url_async(
    client: &TencentCloudAsync,
    request: &DescribeInstanceVncUrl<'_>,
) -> TencentCloudResult<DescribeInstanceVncUrlResponse> {
    client.request(request).await
}

/// Fetch CVM VNC URL with the blocking client.
pub fn describe_instance_vnc_url_blocking(
    client: &TencentCloudBlocking,
    request: &DescribeInstanceVncUrl<'_>,
) -> TencentCloudResult<DescribeInstanceVncUrlResponse> {
    client.request(request)
}

/// Start CVM instances asynchronously.
pub async fn start_instances_async(
    client: &TencentCloudAsync,
    request: &StartInstances<'_>,
) -> TencentCloudResult<GenericActionResponse> {
    client.request(request).await
}

/// Start CVM instances with the blocking client.
pub fn start_instances_blocking(
    client: &TencentCloudBlocking,
    request: &StartInstances<'_>,
) -> TencentCloudResult<GenericActionResponse> {
    client.request(request)
}

/// Reboot CVM instances asynchronously.
pub async fn reboot_instances_async(
    client: &TencentCloudAsync,
    request: &RebootInstances<'_>,
) -> TencentCloudResult<GenericActionResponse> {
    client.request(request).await
}

/// Reboot CVM instances with the blocking client.
pub fn reboot_instances_blocking(
    client: &TencentCloudBlocking,
    request: &RebootInstances<'_>,
) -> TencentCloudResult<GenericActionResponse> {
    client.request(request)
}

/// Stop CVM instances asynchronously.
pub async fn stop_instances_async(
    client: &TencentCloudAsync,
    request: &StopInstances<'_>,
) -> TencentCloudResult<GenericActionResponse> {
    client.request(request).await
}

/// Stop CVM instances with the blocking client.
pub fn stop_instances_blocking(
    client: &TencentCloudBlocking,
    request: &StopInstances<'_>,
) -> TencentCloudResult<GenericActionResponse> {
    client.request(request)
}

/// Change the project of CVM instances asynchronously.
pub async fn modify_instances_project_async(
    client: &TencentCloudAsync,
    request: &ModifyInstancesProject<'_>,
) -> TencentCloudResult<GenericActionResponse> {
    client.request(request).await
}

/// Change the project of CVM instances with the blocking client.
pub fn modify_instances_project_blocking(
    client: &TencentCloudBlocking,
    request: &ModifyInstancesProject<'_>,
) -> TencentCloudResult<GenericActionResponse> {
    client.request(request)
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
ssl 申请文档:
<<2. 输入参数
以下请求参数列表仅列出了接口请求参数和部分公共参数，完整公共参数列表见 公共请求参数。

参数名称	必选	类型	描述
Action	是	String	公共参数，本接口取值：ApplyCertificate。
Version	是	String	公共参数，本接口取值：2019-12-05。
Region	否	String	公共参数，本接口不需要传递此参数。
DvAuthMethod	是	String	证书域名验证方式：
DNS_AUTO： 自动添加域名DNS验证， 需用户域名解析托管在『云解析DNS』，且与申请证书归属同一个腾讯云账号
DNS：手动添加域名DNS验证，需用户手动去域名解析服务商添加验证值
FILE：手动添加域名文件验证。 需要用户手动在域名站点根目录添加指定路径文件进行文件验证， http&https任一通过即可；且域名站点需海外CA机构能访问， 具体访问白名单为：64.78.193.238，216.168.247.9，216.168.249.9，54.189.196.217
示例值：DNS_AUTO
DomainName	是	String	证书绑定的域名。
示例值：tencent.com
ProjectId	否	Integer	证书关联的项目 ID。 默认为0（默认项目）
示例值：0
PackageType	否	String	证书类型， 可不传，目前仅支持类型83。83 = TrustAsia C1 DV Free。
示例值：83
ContactEmail	否	String	证书订单关联邮箱。默认为腾讯云账号邮箱， 不存在则关联固定邮箱
示例值：ssl@tencent.com
ContactPhone	否	String	证书关联手机号码， 不存在则关联固定手机号码
示例值：188**778
ValidityPeriod	否	String	证书有效期，默认3（月），目前仅支持3个月。
示例值：3
CsrEncryptAlgo	否	String	加密算法，取值为ECC、RSA， 默认为RSA
示例值：RSA
CsrKeyParameter	否	String	密钥对参数，RSA仅支持2048。ECC仅支持prime256v1。加密算法选择ECC时，此参数必填
示例值：2048
CsrKeyPassword	否	String	私钥密码， 目前仅使用在生成jks、pfx格式证书时密码； 其他格式私钥证书未加密
示例值：**
Alias	否	String	证书别名
示例值：**
OldCertificateId	否	String	旧证书 ID，用于证书续费（证书有效期在30天内，且未过期），会建立续费关系， 可用于托管； 不传则表示新申请证书
示例值：LqQxgqUe
PackageId	否	String	权益包ID，用于免费证书扩容包使用， 免费证书扩容包已下线
示例值：pid****xsj
DeleteDnsAutoRecord	否	Boolean	签发后是否删除自动域名验证记录， 默认为否；仅域名为DNS_AUTO验证类型支持传参
示例值：true
DnsNames.N	否	Array of String	证书绑定的其他域名，待开放。目前不支持此参数
示例值：["www.tencent.com"]
3. 输出参数
参数名称	类型	描述
CertificateId	String	新申请成功的证书 ID。
示例值：LqQxgqUe
RequestId	String	唯一请求 ID，由服务端生成，每次请求都会返回（若请求因其他原因未能抵达服务端，则该次请求不会获得 RequestId）。定位问题时需要提供该次请求的 RequestId。>>

获取证书文档:
<<2. 输入参数
以下请求参数列表仅列出了接口请求参数和部分公共参数，完整公共参数列表见 公共请求参数。

参数名称	必选	类型	描述
Action	是	String	公共参数，本接口取值：DescribeCertificate。
Version	是	String	公共参数，本接口取值：2019-12-05。
Region	否	String	公共参数，本接口不需要传递此参数。
CertificateId	是	String	证书 ID。
示例值：heysh**hh
3. 输出参数
参数名称	类型	描述
OwnerUin	String	用户 UIN。
注意：此字段可能返回 null，表示取不到有效值。
示例值：278389292
ProjectId	String	项目 ID。
注意：此字段可能返回 null，表示取不到有效值。
示例值：7388392
From	String	证书来源：
trustasia：亚洲诚信，
upload：用户上传。
wosign：沃通
sheca：上海CA
注意：此字段可能返回 null，表示取不到有效值。
示例值：upload
CertificateType	String	证书类型：CA = 客户端证书，SVR = 服务器证书。
注意：此字段可能返回 null，表示取不到有效值。
示例值：CA
PackageType	String	证书套餐类型：
null：用户上传证书（没有套餐类型），
2：TrustAsia TLS RSA CA，
3：SecureSite 增强型企业版（EV Pro），
4：SecureSite 增强型（EV），
5：SecureSite 企业型专业版（OV Pro），
6：SecureSite 企业型（OV），
7：SecureSite 企业型（OV）通配符，
8：Geotrust 增强型（EV），
9：Geotrust 企业型（OV），
10：Geotrust 企业型（OV）通配符，
11：TrustAsia 域名型多域名 SSL 证书，
12：TrustAsia 域名型（DV）通配符，
13：TrustAsia 企业型通配符（OV）SSL 证书（D3），
14：TrustAsia 企业型（OV）SSL 证书（D3），
15：TrustAsia 企业型多域名 （OV）SSL 证书（D3），
16：TrustAsia 增强型 （EV）SSL 证书（D3），
17：TrustAsia 增强型多域名（EV）SSL 证书（D3），
18：GlobalSign 企业型（OV）SSL 证书，
19：GlobalSign 企业型通配符 （OV）SSL 证书，
20：GlobalSign 增强型 （EV）SSL 证书，
21：TrustAsia 企业型通配符多域名（OV）SSL 证书（D3），
22：GlobalSign 企业型多域名（OV）SSL 证书，
23：GlobalSign 企业型通配符多域名（OV）SSL 证书，
24：GlobalSign 增强型多域名（EV）SSL 证书，
25：Wotrus 域名型证书，
26：Wotrus 域名型多域名证书，
27：Wotrus 域名型通配符证书，
28：Wotrus 企业型证书，
29：Wotrus 企业型多域名证书，
30：Wotrus 企业型通配符证书，
31：Wotrus 增强型证书，
32：Wotrus 增强型多域名证书，
33：WoTrus-国密域名型证书，
34：WoTrus-国密域名型证书（多域名），
35：WoTrus-国密域名型证书（通配符），
37：WoTrus-国密企业型证书，
38：WoTrus-国密企业型证书（多域名），
39：WoTrus-国密企业型证书（通配符），
40：WoTrus-国密增强型证书，
41：WoTrus-国密增强型证书（多域名），
42：TrustAsia-域名型证书（通配符多域名），
43：DNSPod-企业型(OV)SSL证书
44：DNSPod-企业型(OV)通配符SSL证书
45：DNSPod-企业型(OV)多域名SSL证书
46：DNSPod-增强型(EV)SSL证书
47：DNSPod-增强型(EV)多域名SSL证书
48：DNSPod-域名型(DV)SSL证书
49：DNSPod-域名型(DV)通配符SSL证书
50：DNSPod-域名型(DV)多域名SSL证书
51：DNSPod（国密）-企业型(OV)SSL证书
52：DNSPod（国密）-企业型(OV)通配符SSL证书
53：DNSPod（国密）-企业型(OV)多域名SSL证书
54：DNSPod（国密）-域名型(DV)SSL证书
55：DNSPod（国密）-域名型(DV)通配符SSL证书
56：DNSPod（国密）-域名型(DV)多域名SSL证书
57：SecureSite 企业型专业版多域名(OV Pro)
58：SecureSite 企业型多域名(OV)
59：SecureSite 增强型专业版多域名(EV Pro)
60：SecureSite 增强型多域名(EV)
61：Geotrust 增强型多域名(EV)
75：SecureSite 企业型(OV)
76：SecureSite 企业型(OV)通配符
77：SecureSite 增强型(EV)
78：Geotrust 企业型(OV)
79：Geotrust 企业型(OV)通配符
80：Geotrust 增强型(EV)
81：GlobalSign 企业型（OV）SSL证书
82：GlobalSign 企业型通配符 （OV）SSL证书
83：TrustAsia C1 DV Free
85：GlobalSign 增强型 （EV）SSL证书
88：GlobalSign 企业型通配符多域名 （OV）SSL证书
89：GlobalSign 企业型多域名 （OV）SSL证书
90：GlobalSign 增强型多域名（EV） SSL证书
91：Geotrust 增强型多域名(EV)
92：SecureSite 企业型专业版多域名(OV Pro)
93：SecureSite 企业型多域名(OV)
94：SecureSite 增强型专业版多域名(EV Pro)
95：SecureSite 增强型多域名(EV)
96：SecureSite 增强型专业版(EV Pro)
97：SecureSite 企业型专业版(OV Pro)
98：CFCA 企业型(OV)SSL证书
99：CFCA 企业型多域名(OV)SSL证书
100：CFCA 企业型通配符(OV)SSL证书
101：CFCA 增强型(EV)SSL证书
注意：此字段可能返回 null，表示取不到有效值。
示例值：2
ProductZhName	String	证书产品名称
注意：此字段可能返回 null，表示取不到有效值。
示例值：TrustAsia C1 DV Free
Domain	String	域名。
注意：此字段可能返回 null，表示取不到有效值。
示例值：www.**.com
Alias	String	备注名称。
注意：此字段可能返回 null，表示取不到有效值。
示例值：hello
Status	Integer	证书状态：0 = 审核中，1 = 已通过，2 = 审核失败，3 = 已过期，4 = 自动添加DNS记录，5 = 企业证书，待提交资料，6 = 订单取消中，7 = 已取消，8 = 已提交资料， 待上传确认函，9 = 证书吊销中，10 = 已吊销，11 = 重颁发中，12 = 待上传吊销确认函，13 = 免费证书待提交资料。14 = 证书已退款。 15 = 证书迁移中
注意：此字段可能返回 null，表示取不到有效值。
示例值：0
StatusMsg	String	状态信息。 取值范围：
//通用状态信息
1、PRE-REVIEWING：预审核中
2、LEGAL-REVIEWING：法务审核中
3、CA-REVIEWING：CA审核中
4、PENDING-DCV：域名验证中
5、WAIT-ISSUE：等待签发（域名验证已通过）
//证书审核失败状态信息
1、订单审核失败
2、CA审核失败，域名未通过安全审查
3、域名验证超时，订单自动关闭，请您重新进行证书申请
4、证书资料未通过证书CA机构审核，审核人员会致电您证书预留的联系方式，请您留意来电。后续可通过“修改资料”重新提交资料
待持续完善
注意：此字段可能返回 null，表示取不到有效值。
示例值：CA-REVIEWING
VerifyType	String	验证类型：DNS_AUTO = 自动DNS验证，DNS = 手动DNS验证，FILE = 文件验证，DNS_PROXY = DNS代理验证。FILE_PROXY = 文件代理验证
注意：此字段可能返回 null，表示取不到有效值。
示例值：DNS_AUTO
VulnerabilityStatus	String	漏洞扫描状态。
注意：此字段可能返回 null，表示取不到有效值。
示例值：INACTIVE
CertBeginTime	String	证书生效时间。时区为GMT+8:00
注意：此字段可能返回 null，表示取不到有效值。
示例值：2024-11-27 08:00:00
CertEndTime	String	证书失效时间。时区为GMT+8:00
注意：此字段可能返回 null，表示取不到有效值。
示例值：2025-02-26 07:59:59
ValidityPeriod	String	证书有效期：单位(月)。
注意：此字段可能返回 null，表示取不到有效值。
示例值：3
InsertTime	String	申请时间。时区为GMT+8:00
注意：此字段可能返回 null，表示取不到有效值。
示例值：2024-11-27 17:44:36
OrderId	String	订单 ID。
注意：此字段可能返回 null，表示取不到有效值。
示例值：HyJ4b85G_mZOLxSuw
CertificateExtra	CertificateExtra	证书扩展信息。
注意：此字段可能返回 null，表示取不到有效值。
DvAuthDetail	DvAuthDetail	DV 认证信息。
注意：此字段可能返回 null，表示取不到有效值。
VulnerabilityReport	String	漏洞扫描评估报告。
注意：此字段可能返回 null，表示取不到有效值。
示例值：no problem
CertificateId	String	证书 ID。
注意：此字段可能返回 null，表示取不到有效值。
示例值：heysh**he
PackageTypeName	String	证书类型名称。
注意：此字段可能返回 null，表示取不到有效值。
示例值：TrustAsia C1 DV Free
StatusName	String	状态描述。
注意：此字段可能返回 null，表示取不到有效值。
示例值：已颁发
SubjectAltName	Array of String	证书包含的多个域名（包含主域名）。
注意：此字段可能返回 null，表示取不到有效值。
示例值：["www.****.online"]
IsVip	Boolean	是否为 VIP 客户。
注意：此字段可能返回 null，表示取不到有效值。
示例值：true
IsWildcard	Boolean	是否为泛域名证书。
注意：此字段可能返回 null，表示取不到有效值。
示例值：true
IsDv	Boolean	是否为 DV 版证书。
注意：此字段可能返回 null，表示取不到有效值。
示例值：true
IsVulnerability	Boolean	是否启用了漏洞扫描功能。
注意：此字段可能返回 null，表示取不到有效值。
示例值：true
RenewAble	Boolean	是否可重颁发证书。
注意：此字段可能返回 null，表示取不到有效值。
示例值：true
SubmittedData	SubmittedData	提交的资料信息。
注意：此字段可能返回 null，表示取不到有效值。
Deployable	Boolean	是否可部署。
注意：此字段可能返回 null，表示取不到有效值。
示例值：true
Tags	Array of Tags	标签列表
注意：此字段可能返回 null，表示取不到有效值。
CAEncryptAlgorithms	Array of String	CA证书的所有加密方式。仅证书类型CertificateType为CA有效
注意：此字段可能返回 null，表示取不到有效值。
示例值：["RSA"]
CACommonNames	Array of String	CA证书的所有通用名称。仅证书类型CertificateType为CA有效
注意：此字段可能返回 null，表示取不到有效值。
示例值：["TrustAsia C1 DV Free"]
CAEndTimes	Array of String	CA证书所有的到期时间。仅证书类型CertificateType为CA有效，时区为GMT+8:00
注意：此字段可能返回 null，表示取不到有效值。
示例值：["2024-11-27 17:44:36"]
DvRevokeAuthDetail	Array of DvAuths	DV证书吊销验证值
注意：此字段可能返回 null，表示取不到有效值。
RequestId	String	唯一请求 ID，由服务端生成，每次请求都会返回（若请求因其他原因未能抵达服务端，则该次请求不会获得 RequestId）。定位问题时需要提供该次请求的 RequestId。>>

证书下载文档:
<<2. 输入参数
以下请求参数列表仅列出了接口请求参数和部分公共参数，完整公共参数列表见 公共请求参数。

参数名称	必选	类型	描述
Action	是	String	公共参数，本接口取值：DownloadCertificate。
Version	是	String	公共参数，本接口取值：2019-12-05。
Region	否	String	公共参数，本接口不需要传递此参数。
CertificateId	是	String	证书 ID。
示例值："hehs**jjsj"
3. 输出参数
参数名称	类型	描述
Content	String	ZIP base64 编码内容，base64 解码后可保存为 ZIP 文件。
示例值："hdejcjueujake**hdhhs"
ContentType	String	MIME 类型：application/zip = ZIP 压缩文件。
示例值："application/zip"
RequestId	String	唯一请求 ID，由服务端生成，每次请求都会返回（若请求因其他原因未能抵达服务端，则该次请求不会获得 RequestId）。定位问题时需要提供该次请求的 RequestId。>>

请根据以上文档和代码，输出 Rust 代码实现 Tencent Cloud SSL 证书服务的申请、获取和下载功能。