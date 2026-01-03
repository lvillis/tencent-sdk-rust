use crate::{Error, client::endpoint::Endpoint, types::RequestId};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AccountBalanceResponse {
    #[serde(rename = "Response")]
    pub response: AccountBalance,
}

#[derive(Debug, Deserialize)]
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
    pub request_id: RequestId,
}

#[derive(Debug, Default)]
pub struct DescribeAccountBalanceRequest;

impl Endpoint for DescribeAccountBalanceRequest {
    type Output = AccountBalanceResponse;

    fn service(&self) -> &'static str {
        "billing"
    }

    fn action(&self) -> &'static str {
        "DescribeAccountBalance"
    }

    fn version(&self) -> &'static str {
        "2018-07-09"
    }

    fn payload(&self) -> Result<Option<serde_json::Value>, Error> {
        Ok(Some(serde_json::Value::Object(Default::default())))
    }
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
        assert_eq!(parsed.response.request_id.as_str(), "req-123");
        assert_eq!(parsed.response.uin, Some(123456789));
    }
}
