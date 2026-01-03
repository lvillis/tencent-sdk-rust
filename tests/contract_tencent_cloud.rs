#[cfg(feature = "async")]
mod async_contract {
    use tencent_sdk::{Auth, Client};

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    #[ignore]
    async fn billing_describe_account_balance() -> Result<(), tencent_sdk::Error> {
        let secret_id = std::env::var("TENCENT_SECRET_ID")
            .expect("set env TENCENT_SECRET_ID to run this contract test");
        let secret_key = std::env::var("TENCENT_SECRET_KEY")
            .expect("set env TENCENT_SECRET_KEY to run this contract test");

        let region = std::env::var("TENCENT_REGION").unwrap_or_else(|_| "ap-guangzhou".to_string());

        let client = Client::builder_tencent_cloud()?
            .auth(Auth::tc3(secret_id, secret_key))
            .default_region(region)
            .build()?;

        let response = client.billing().describe_account_balance().await?;
        assert!(!response.response.request_id.as_str().is_empty());
        Ok(())
    }
}

#[cfg(feature = "blocking")]
mod blocking_contract {
    use tencent_sdk::{Auth, BlockingClient};

    #[test]
    #[ignore]
    fn billing_describe_account_balance() -> Result<(), tencent_sdk::Error> {
        let secret_id = std::env::var("TENCENT_SECRET_ID")
            .expect("set env TENCENT_SECRET_ID to run this contract test");
        let secret_key = std::env::var("TENCENT_SECRET_KEY")
            .expect("set env TENCENT_SECRET_KEY to run this contract test");

        let region = std::env::var("TENCENT_REGION").unwrap_or_else(|_| "ap-guangzhou".to_string());

        let client = BlockingClient::builder_tencent_cloud()?
            .auth(Auth::tc3(secret_id, secret_key))
            .default_region(region)
            .build()?;

        let response = client.billing().describe_account_balance()?;
        assert!(!response.response.request_id.as_str().is_empty());
        Ok(())
    }
}
