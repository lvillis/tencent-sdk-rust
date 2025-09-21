use crate::{
    client::{TencentCloudAsync, TencentCloudBlocking},
    core::{Endpoint, TencentCloudResult},
};
use serde::Deserialize;
use std::borrow::Cow;

#[derive(Debug, Deserialize)]
/// Response payload returned by `DescribeAccountBalance`.
///
/// | Field | Type | Description |
/// |-------|------|-------------|
/// | `response` | [`AccountBalance`] | Raw response content from Tencent Cloud.
pub struct AccountBalanceResponse {
    #[serde(rename = "Response")]
    pub response: AccountBalance,
}

#[derive(Debug, Deserialize)]
/// Detailed billing attributes mapped from Tencent Cloud billing service.
///
/// | Field | Type | Description |
/// |-------|------|-------------|
/// | `uin` | `Option<u64>` | UIN of the queried account. |
/// | `real_balance` | `Option<f64>` | Actual available balance (cents). |
/// | `cash_account_balance` | `Option<f64>` | Cash balance (cents). |
/// | `income_into_account_balance` | `Option<f64>` | Income balance (cents). |
/// | `present_account_balance` | `Option<f64>` | Promotional balance (cents). |
/// | `freeze_amount` | `Option<f64>` | Frozen amount (cents). |
/// | `owe_amount` | `Option<f64>` | Outstanding amount (cents). |
/// | `credit_amount` | `Option<f64>` | Credit limit (cents). |
/// | `credit_balance` | `Option<f64>` | Remaining credit (cents). |
/// | `real_credit_balance` | `Option<f64>` | Actual usable credit (cents). |
/// | `balance` | `Option<f64>` | Deprecated alias of `real_balance`. |
/// | `request_id` | `String` | Unique request identifier.
pub struct AccountBalance {
    #[serde(rename = "Uin")]
    pub uin: Option<u64>,
    #[serde(rename = "RealBalance")]
    pub real_balance: Option<f64>,
    #[serde(rename = "CashAccountBalance")]
    pub cash_account_balance: Option<f64>,
    #[serde(rename = "IncomeIntoAccountBalance")]
    pub income_into_account_balance: Option<f64>,
    #[serde(rename = "PresentAccountBalance")]
    pub present_account_balance: Option<f64>,
    #[serde(rename = "FreezeAmount")]
    pub freeze_amount: Option<f64>,
    #[serde(rename = "OweAmount")]
    pub owe_amount: Option<f64>,
    #[serde(rename = "CreditAmount")]
    pub credit_amount: Option<f64>,
    #[serde(rename = "CreditBalance")]
    pub credit_balance: Option<f64>,
    #[serde(rename = "RealCreditBalance")]
    pub real_credit_balance: Option<f64>,
    #[serde(rename = "Balance")]
    pub balance: Option<f64>,
    #[serde(rename = "RequestId")]
    pub request_id: String,
}

/// TencentCloud Billing `DescribeAccountBalance` endpoint definition.
///
/// | Item | Value |
/// |------|-------|
/// | Service | `billing` |
/// | Action | `DescribeAccountBalance` |
/// | Version | `2018-07-09` |
/// | Host | `billing.tencentcloudapi.com` |
/// | Rate Limit | 20 req/s |
pub struct DescribeAccountBalance;

impl Endpoint for DescribeAccountBalance {
    type Output = AccountBalanceResponse;

    fn service(&self) -> Cow<'static, str> {
        Cow::Borrowed("billing")
    }

    fn action(&self) -> Cow<'static, str> {
        Cow::Borrowed("DescribeAccountBalance")
    }

    fn version(&self) -> Cow<'static, str> {
        Cow::Borrowed("2018-07-09")
    }
}

/// Execute the Billing `DescribeAccountBalance` action with the async client.
///
/// # Tencent Cloud Reference
/// | Item | Value |
/// |------|-------|
/// | Service | `billing` |
/// | Action | `DescribeAccountBalance` |
/// | Version | `2018-07-09` |
/// | Rate Limit | 20 req/s |
///
/// # Request Payload
/// The API expects an empty JSON object (`{}`).
///
/// # Response Highlights
/// | Field | Description |
/// |-------|-------------|
/// | `RealBalance` | Current available balance expressed in cents. |
/// | `CashAccountBalance` | Cash balance expressed in cents. |
/// | `PresentAccountBalance` | Promotional balance expressed in cents. |
/// | `RequestId` | Unique identifier for troubleshooting. |
///
/// Returns [`AccountBalanceResponse`].
pub async fn describe_account_balance_async(
    client: &TencentCloudAsync,
) -> TencentCloudResult<AccountBalanceResponse> {
    client.request(&DescribeAccountBalance).await
}

/// Execute the Billing `DescribeAccountBalance` action with the blocking client.
///
/// This helper mirrors [`describe_account_balance_async`] but executes synchronously.
pub fn describe_account_balance_blocking(
    client: &TencentCloudBlocking,
) -> TencentCloudResult<AccountBalanceResponse> {
    client.request(&DescribeAccountBalance)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_account_balance_response() {
        let payload = r#"{
            "Response": {
                "Uin": 123456789,
                "RealBalance": 100.0,
                "CashAccountBalance": 80.0,
                "IncomeIntoAccountBalance": 10.0,
                "PresentAccountBalance": 10.0,
                "FreezeAmount": 0.0,
                "OweAmount": 0.0,
                "CreditAmount": 0.0,
                "CreditBalance": 0.0,
                "RealCreditBalance": 0.0,
                "Balance": 100.0,
                "RequestId": "req-123"
            }
        }"#;

        let parsed: AccountBalanceResponse = serde_json::from_str(payload).unwrap();
        assert_eq!(parsed.response.request_id, "req-123");
        assert_eq!(parsed.response.uin, Some(123456789));
    }
}
