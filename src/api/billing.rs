use crate::{
    Result,
    client::RequestOptions,
    types::billing::{AccountBalanceResponse, DescribeAccountBalanceRequest},
};

#[cfg(feature = "async")]
use crate::client::Client;

#[cfg(feature = "async")]
#[derive(Clone)]
pub struct BillingService {
    client: Client,
}

#[cfg(feature = "async")]
impl BillingService {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn describe_account_balance(&self) -> Result<AccountBalanceResponse> {
        self.client
            .execute(&DescribeAccountBalanceRequest, None)
            .await
    }

    pub async fn describe_account_balance_with_options(
        &self,
        options: &RequestOptions,
    ) -> Result<AccountBalanceResponse> {
        self.client
            .execute(&DescribeAccountBalanceRequest, Some(options))
            .await
    }
}

#[cfg(feature = "blocking")]
#[derive(Clone)]
pub struct BlockingBillingService {
    client: BlockingClient,
}

#[cfg(feature = "blocking")]
use crate::client::BlockingClient;

#[cfg(feature = "blocking")]
impl BlockingBillingService {
    pub(crate) fn new(client: BlockingClient) -> Self {
        Self { client }
    }

    pub fn describe_account_balance(&self) -> Result<AccountBalanceResponse> {
        self.client.execute(&DescribeAccountBalanceRequest, None)
    }

    pub fn describe_account_balance_with_options(
        &self,
        options: &RequestOptions,
    ) -> Result<AccountBalanceResponse> {
        self.client
            .execute(&DescribeAccountBalanceRequest, Some(options))
    }
}
