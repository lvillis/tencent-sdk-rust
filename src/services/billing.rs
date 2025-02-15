use crate::client::TencentCloudClient;
use std::error::Error;

/// 获取云账户余额信息 - DescribeAccountBalance
///
/// **接口描述**：
///
/// - 接口请求域名： billing.tencentcloudapi.com
/// - 默认接口请求频率限制：20次/秒
///
/// **入参说明**：
///
/// | 参数    | 类型   | 说明                                                      |
/// |---------|--------|-----------------------------------------------------------|
/// | Action  | String | 固定为 `"DescribeAccountBalance"`                         |
/// | Version | String | 固定为 `"2018-07-09"`                                     |
/// | Region  | String | 可选参数，指定区域（如不传则使用默认区域）                 |
/// | Body    | JSON   | 固定为 `{}`（无业务参数）                                  |
///
/// **出参说明**：
///
/// | 字段                        | 类型    | 说明                                                        |
/// |-----------------------------|---------|-------------------------------------------------------------|
/// | Uin                         | Integer | 查询的用户 Uin                                              |
/// | RealBalance                 | Float   | 当前真实可用余额（单位：分）                                  |
/// | CashAccountBalance          | Float   | 现金账户余额（单位：分）                                      |
/// | IncomeIntoAccountBalance    | Float   | 收益转入账户余额（单位：分）                                  |
/// | PresentAccountBalance       | Float   | 赠送账户余额（单位：分）                                      |
/// | FreezeAmount                | Float   | 冻结金额（单位：分）                                          |
/// | OweAmount                   | Float   | 欠费金额（单位：分）                                          |
/// | CreditAmount                | Float   | 信用额度（单位：分）                                          |
/// | CreditBalance               | Float   | 可用信用额度（单位：分）                                      |
/// | RealCreditBalance           | Float   | 真实可用信用额度（单位：分）                                  |
/// | Balance                     | Integer | 当前真实可用余额（单位：分，与 RealBalance 相同）             |
/// | RequestId                   | String  | 唯一请求 ID，用于问题定位                                    |
pub async fn describe_account_balance(
    client: &TencentCloudClient,
) -> Result<String, Box<dyn Error>> {
    client
        .request(
            "billing",
            "billing.tencentcloudapi.com",
            None,
            "2018-07-09",
            "DescribeAccountBalance",
            "{}",
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
    async fn test_describe_account_balance() {
        let client = TencentCloudClient::new(TEST_SECRET_ID, TEST_SECRET_KEY, None);
        match describe_account_balance(&client).await {
            Ok(resp) => {
                println!("DescribeAccountBalance 响应:\n{}", resp);
                assert!(!resp.is_empty());
            }
            Err(e) => eprintln!("调用 DescribeAccountBalance 时出错: {}", e),
        }
    }
}
