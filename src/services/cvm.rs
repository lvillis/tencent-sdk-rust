use crate::client::TencentCloudClient;
use serde_json::{json, Map, Value};
use std::error::Error;

/// 查看实例列表 - DescribeInstances
///
/// **接口描述**：
/// - 接口请求域名：cvm.tencentcloudapi.com
/// - 默认接口请求频率限制：40次/秒
///
/// **入参说明**：
///
/// | 参数    | 类型   | 说明                                       |
/// |---------|--------|--------------------------------------------|
/// | Action  | String | 固定为 `"DescribeInstances"`               |
/// | Version | String | 固定为 `"2017-03-12"`                      |
/// | Region  | String | 必填，指定区域（例如："ap-shanghai"）       |
/// | Body    | JSON   | 固定为 `{}`（无业务参数）                  |
///
/// **出参说明**：
///
/// | 字段         | 类型    | 说明                                               |
/// |--------------|---------|----------------------------------------------------|
/// | TotalCount   | Integer | 符合条件的实例数量                                 |
/// | InstanceSet  | Array   | 实例详细信息列表（具体字段参见腾讯云文档）           |
/// | RequestId    | String  | 唯一请求ID，用于问题定位                           |
pub async fn describe_instances(client: &TencentCloudClient) -> Result<String, Box<dyn Error>> {
    client.request(
        "cvm",
        "cvm.tencentcloudapi.com",
        Some("ap-shanghai"),
        "2017-03-12",
        "DescribeInstances",
        "{}",
    ).await
}

/// 重置实例密码 - ResetInstancesPassword
///
/// **接口描述**：
/// - 接口请求域名：cvm.tencentcloudapi.com
/// - 默认接口请求频率限制：10次/秒
///
/// **入参说明**：
///
/// | 参数         | 类型    | 说明                                                                               |
/// |--------------|---------|------------------------------------------------------------------------------------|
/// | Action       | String  | 固定为 `"ResetInstancesPassword"`                                                 |
/// | Version      | String  | 固定为 `"2017-03-12"`                                                              |
/// | Region       | String  | 必填，指定区域（例如："ap-shanghai"）                                               |
/// | InstanceIds  | Array   | 必填，实例ID数组（每次最多支持100个实例ID）                                         |
/// | Password     | String  | 必填，重置后的登录密码（需符合系统密码复杂度要求）                                   |
/// | UserName     | String  | 可选，待重置密码的操作系统用户名（不传则使用默认管理员账号）                          |
/// | ForceStop    | Boolean | 可选，是否对运行中实例进行强制关机，默认 false                                      |
/// | Body         | JSON    | 由上述参数构成的 JSON 字符串                                                       |
///
/// **出参说明**：
///
/// | 字段     | 类型   | 说明           |
/// |----------|--------|----------------|
/// | RequestId| String | 唯一请求ID     |
pub async fn reset_instances_password(
    client: &TencentCloudClient,
    region: &str,
    instance_ids: Vec<&str>,
    password: &str,
    username: Option<&str>,
    force_stop: Option<bool>,
) -> Result<String, Box<dyn Error>> {
    let mut payload_map = Map::new();
    payload_map.insert("InstanceIds".to_string(), json!(instance_ids));
    payload_map.insert("Password".to_string(), json!(password));
    if let Some(user) = username {
        payload_map.insert("UserName".to_string(), json!(user));
    }
    if let Some(force) = force_stop {
        payload_map.insert("ForceStop".to_string(), json!(force));
    }
    let payload = Value::Object(payload_map).to_string();

    client.request(
        "cvm",
        "cvm.tencentcloudapi.com",
        Some(region),
        "2017-03-12",
        "ResetInstancesPassword",
        &payload,
    ).await
}

/// 查询实例管理终端地址 - DescribeInstanceVncUrl
///
/// **接口描述**：
/// - 接口请求域名：cvm.tencentcloudapi.com
/// - 默认接口请求频率限制：10次/秒
///
/// **入参说明**：
///
/// | 参数       | 类型   | 说明                                           |
/// |------------|--------|------------------------------------------------|
/// | Action     | String | 固定为 `"DescribeInstanceVncUrl"`               |
/// | Version    | String | 固定为 `"2017-03-12"`                          |
/// | Region     | String | 必填，指定区域（例如："ap-shanghai"）           |
/// | InstanceId | String | 必填，指定单个实例ID                           |
/// | Body       | JSON   | 固定为 `{"InstanceId": "<实例ID>"}`            |
///
/// **出参说明**：
///
/// | 字段           | 类型   | 说明                                               |
/// |----------------|--------|----------------------------------------------------|
/// | InstanceVncUrl | String | 实例的管理终端地址（VNC 地址）                     |
/// | RequestId      | String | 唯一请求ID，用于问题定位                           |
pub async fn describe_instance_vnc_url(
    client: &TencentCloudClient,
    region: &str,
    instance_id: &str,
) -> Result<String, Box<dyn Error>> {
    let payload = json!({ "InstanceId": instance_id }).to_string();

    client.request(
        "cvm",
        "cvm.tencentcloudapi.com",
        Some(region),
        "2017-03-12",
        "DescribeInstanceVncUrl",
        &payload,
    ).await
}

/// 启动实例 - StartInstances
///
/// **接口描述**：
/// - 接口请求域名：cvm.tencentcloudapi.com
/// - 默认接口请求频率限制：10次/秒
///
/// **入参说明**：
///
/// | 参数        | 类型   | 说明                                           |
/// |-------------|--------|------------------------------------------------|
/// | Action      | String | 固定为 `"StartInstances"`                       |
/// | Version     | String | 固定为 `"2017-03-12"`                           |
/// | Region      | String | 必填，指定区域（例如："ap-shanghai"）            |
/// | InstanceIds | Array  | 必填，实例ID数组（最多支持100个实例ID）          |
/// | Body        | JSON   | 由上述参数构成的 JSON 字符串                     |
///
/// **出参说明**：
///
/// | 字段     | 类型   | 说明           |
/// |----------|--------|----------------|
/// | RequestId| String | 唯一请求ID     |
pub async fn start_instances(
    client: &TencentCloudClient,
    region: &str,
    instance_ids: Vec<&str>,
) -> Result<String, Box<dyn Error>> {
    let payload = json!({ "InstanceIds": instance_ids }).to_string();

    client.request(
        "cvm",
        "cvm.tencentcloudapi.com",
        Some(region),
        "2017-03-12",
        "StartInstances",
        &payload,
    ).await
}

/// 重启实例 - RebootInstances
///
/// **接口描述**：
/// - 接口请求域名：cvm.tencentcloudapi.com
/// - 默认接口请求频率限制：10次/秒
///
/// **入参说明**：
///
/// | 参数        | 类型   | 说明                                               |
/// |-------------|--------|----------------------------------------------------|
/// | Action      | String | 固定为 `"RebootInstances"`                         |
/// | Version     | String | 固定为 `"2017-03-12"`                              |
/// | Region      | String | 必填，指定区域（例如："ap-shanghai"）               |
/// | InstanceIds | Array  | 必填，实例ID数组（最多支持100个实例ID）             |
/// | StopType    | String | 可选，实例关机类型，默认 `"SOFT"`                  |
/// | Body        | JSON   | 由上述参数构成的 JSON 字符串                       |
///
/// **出参说明**：
///
/// | 字段     | 类型   | 说明           |
/// |----------|--------|----------------|
/// | RequestId| String | 唯一请求ID     |
pub async fn reboot_instances(
    client: &TencentCloudClient,
    region: &str,
    instance_ids: Vec<&str>,
    stop_type: Option<&str>,
) -> Result<String, Box<dyn Error>> {
    let payload = {
        let mut map = Map::new();
        map.insert("InstanceIds".to_string(), json!(instance_ids));
        map.insert("StopType".to_string(), json!(stop_type.unwrap_or("SOFT")));
        Value::Object(map).to_string()
    };

    client.request(
        "cvm",
        "cvm.tencentcloudapi.com",
        Some(region),
        "2017-03-12",
        "RebootInstances",
        &payload,
    ).await
}

/// 关闭实例 - StopInstances
///
/// **接口描述**：
/// - 接口请求域名：cvm.tencentcloudapi.com
/// - 默认接口请求频率限制：10次/秒
///
/// **入参说明**：
///
/// | 参数         | 类型   | 说明                                                         |
/// |--------------|--------|--------------------------------------------------------------|
/// | Action       | String | 固定为 `"StopInstances"`                                     |
/// | Version      | String | 固定为 `"2017-03-12"`                                        |
/// | Region       | String | 必填，指定区域（例如："ap-shanghai"）                         |
/// | InstanceIds  | Array  | 必填，实例ID数组（最多支持100个实例ID）                       |
/// | StopType     | String | 可选，实例关闭模式，默认 `"SOFT"`                           |
/// | StoppedMode  | String | 可选，按量计费实例关机收费模式，默认 `"KEEP_CHARGING"`        |
/// | Body         | JSON   | 由上述参数构成的 JSON 字符串                                  |
///
/// **出参说明**：
///
/// | 字段     | 类型   | 说明           |
/// |----------|--------|----------------|
/// | RequestId| String | 唯一请求ID     |
pub async fn stop_instances(
    client: &TencentCloudClient,
    region: &str,
    instance_ids: Vec<&str>,
    stop_type: Option<&str>,
    stopped_mode: Option<&str>,
) -> Result<String, Box<dyn Error>> {
    let payload = {
        let mut map = Map::new();
        map.insert("InstanceIds".to_string(), json!(instance_ids));
        map.insert("StopType".to_string(), json!(stop_type.unwrap_or("SOFT")));
        map.insert("StoppedMode".to_string(), json!(stopped_mode.unwrap_or("KEEP_CHARGING")));
        Value::Object(map).to_string()
    };

    client.request(
        "cvm",
        "cvm.tencentcloudapi.com",
        Some(region),
        "2017-03-12",
        "StopInstances",
        &payload,
    ).await
}

/// 修改实例所属项目 - ModifyInstancesProject
///
/// **接口描述**：
/// - 接口请求域名：cvm.tencentcloudapi.com
/// - 默认接口请求频率限制：10次/秒
///
/// **入参说明**：
///
/// | 参数        | 类型    | 说明                                                         |
/// |-------------|---------|--------------------------------------------------------------|
/// | Action      | String  | 固定为 `"ModifyInstancesProject"`                            |
/// | Version     | String  | 固定为 `"2017-03-12"`                                         |
/// | Region      | String  | 必填，指定区域（例如："ap-shanghai"）                          |
/// | InstanceIds | Array   | 必填，实例ID数组（最多支持100个实例ID）                         |
/// | ProjectId   | Integer | 必填，目标项目ID                                             |
/// | Body        | JSON    | 由上述参数构成的 JSON 字符串                                  |
///
/// **出参说明**：
///
/// | 字段     | 类型   | 说明           |
/// |----------|--------|----------------|
/// | RequestId| String | 唯一请求ID     |
pub async fn modify_instances_project(
    client: &TencentCloudClient,
    region: &str,
    instance_ids: Vec<&str>,
    project_id: i32,
) -> Result<String, Box<dyn Error>> {
    let payload = {
        let mut map = Map::new();
        map.insert("InstanceIds".to_string(), json!(instance_ids));
        map.insert("ProjectId".to_string(), json!(project_id));
        Value::Object(map).to_string()
    };

    client.request(
        "cvm",
        "cvm.tencentcloudapi.com",
        Some(region),
        "2017-03-12",
        "ModifyInstancesProject",
        &payload,
    ).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    const TEST_SECRET_ID: &str = "YourSecretId";
    const TEST_SECRET_KEY: &str = "YourSecretKey";
    const TEST_REGION: &str = "ap-shanghai";
    // 示例实例ID、项目ID等，请根据实际情况修改
    const TEST_INSTANCE_ID: &str = "ins-r9hr2upy";
    const TEST_INSTANCE_IDS: &[&str] = &["ins-r8hr2upy", "ins-5d8a23rs"];
    const TEST_PROJECT_ID: i32 = 1045;

    #[tokio::test]
    async fn test_describe_instances() {
        let client = TencentCloudClient::new(TEST_SECRET_ID, TEST_SECRET_KEY, None);
        match describe_instances(&client).await {
            Ok(resp) => {
                println!("DescribeInstances 响应:\n{}", resp);
                assert!(!resp.is_empty());
            }
            Err(e) => eprintln!("调用 DescribeInstances 时出错: {}", e),
        }
    }

    #[tokio::test]
    async fn test_reset_instances_password() {
        let client = TencentCloudClient::new(TEST_SECRET_ID, TEST_SECRET_KEY, None);
        let password = "abc123ABC!@#";
        let username = Some("root"); // 根据实际系统调整
        let force_stop = Some(true);
        match reset_instances_password(
            &client,
            TEST_REGION,
            TEST_INSTANCE_IDS.to_vec(),
            password,
            username,
            force_stop,
        ).await {
            Ok(resp) => {
                println!("ResetInstancesPassword 响应:\n{}", resp);
                assert!(!resp.is_empty());
            }
            Err(e) => eprintln!("调用 ResetInstancesPassword 时出错: {}", e),
        }
    }

    #[tokio::test]
    async fn test_describe_instance_vnc_url() {
        let client = TencentCloudClient::new(TEST_SECRET_ID, TEST_SECRET_KEY, None);
        match describe_instance_vnc_url(&client, TEST_REGION, TEST_INSTANCE_ID).await {
            Ok(resp) => {
                println!("DescribeInstanceVncUrl 响应:\n{}", resp);
                assert!(!resp.is_empty());
            }
            Err(e) => eprintln!("调用 DescribeInstanceVncUrl 时出错: {}", e),
        }
    }

    #[tokio::test]
    async fn test_start_instances() {
        let client = TencentCloudClient::new(TEST_SECRET_ID, TEST_SECRET_KEY, None);
        match start_instances(&client, TEST_REGION, TEST_INSTANCE_IDS.to_vec()).await {
            Ok(resp) => {
                println!("StartInstances 响应:\n{}", resp);
                assert!(!resp.is_empty());
            }
            Err(e) => eprintln!("调用 StartInstances 时出错: {}", e),
        }
    }

    #[tokio::test]
    async fn test_reboot_instances() {
        let client = TencentCloudClient::new(TEST_SECRET_ID, TEST_SECRET_KEY, None);
        match reboot_instances(&client, TEST_REGION, TEST_INSTANCE_IDS.to_vec(), Some("SOFT")).await {
            Ok(resp) => {
                println!("RebootInstances 响应:\n{}", resp);
                assert!(!resp.is_empty());
            }
            Err(e) => eprintln!("调用 RebootInstances 时出错: {}", e),
        }
    }

    #[tokio::test]
    async fn test_stop_instances() {
        let client = TencentCloudClient::new(TEST_SECRET_ID, TEST_SECRET_KEY, None);
        match stop_instances(&client, TEST_REGION, TEST_INSTANCE_IDS.to_vec(), Some("SOFT"), Some("KEEP_CHARGING")).await {
            Ok(resp) => {
                println!("StopInstances 响应:\n{}", resp);
                assert!(!resp.is_empty());
            }
            Err(e) => eprintln!("调用 StopInstances 时出错: {}", e),
        }
    }

    #[tokio::test]
    async fn test_modify_instances_project() {
        let client = TencentCloudClient::new(TEST_SECRET_ID, TEST_SECRET_KEY, None);
        match modify_instances_project(&client, TEST_REGION, TEST_INSTANCE_IDS.to_vec(), TEST_PROJECT_ID).await {
            Ok(resp) => {
                println!("ModifyInstancesProject 响应:\n{}", resp);
                assert!(!resp.is_empty());
            }
            Err(e) => eprintln!("调用 ModifyInstancesProject 时出错: {}", e),
        }
    }
}
