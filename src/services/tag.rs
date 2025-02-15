use crate::client::TencentCloudClient;
use serde_json::json;
use std::error::Error;

/// 查看标签项目列表 - DescribeProjects
///
/// **接口描述**：
/// - 接口请求域名：tag.tencentcloudapi.com
/// - 默认接口请求频率限制：20次/秒
///
/// **入参说明**：
///
/// | 参数    | 类型              | 说明                                                     |
/// |---------|-------------------|----------------------------------------------------------|
/// | Action  | String            | 固定为 `"DescribeProjects"`                              |
/// | Version | String            | 固定为 `"2018-08-13"`                                    |
/// | AllList | Option<Integer>   | 可选，默认值为 `1`，表示获取所有项目                      |
/// | Limit   | Option<Integer>   | 可选，默认值为 `1000`，返回项目数量上限                  |
/// | Offset  | Option<Integer>   | 可选，默认值为 `0`，偏移量                               |
/// | Body    | JSON              | 由上述参数构成的 JSON 字符串                              |
///
/// **出参说明**：
///
/// | 字段       | 类型    | 说明                                            |
/// |------------|---------|-------------------------------------------------|
/// | TotalCount | Integer | 符合条件的项目总数                              |
/// | ProjectSet | Array   | 标签项目列表，包含每个项目的详细信息              |
/// | RequestId  | String  | 唯一请求ID，用于问题定位                        |
pub async fn describe_projects(
    client: &TencentCloudClient,
    all_list: Option<i32>,
    limit: Option<i32>,
    offset: Option<i32>,
) -> Result<String, Box<dyn Error>> {
    let payload = json!({
        "AllList": all_list.unwrap_or(1),
        "Limit": limit.unwrap_or(1000),
        "Offset": offset.unwrap_or(0)
    })
        .to_string();

    client
        .request(
            "tag",
            "tag.tencentcloudapi.com",
            None,
            "2018-08-13",
            "DescribeProjects",
            &payload,
        )
        .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    const TEST_SECRET_ID: &str = "YourSecretId";
    const TEST_SECRET_KEY: &str = "YourSecretKey";

    #[tokio::test]
    async fn test_describe_projects_default() {
        let client = TencentCloudClient::new(TEST_SECRET_ID, TEST_SECRET_KEY, None);
        match describe_projects(&client, None, None, None).await {
            Ok(resp) => {
                println!("DescribeProjects 响应:\n{}", resp);
                assert!(!resp.is_empty());
            }
            Err(e) => eprintln!("调用 DescribeProjects 时出错: {}", e),
        }
    }
}
