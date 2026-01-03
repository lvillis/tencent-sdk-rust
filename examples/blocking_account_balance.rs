#[cfg(feature = "blocking")]
fn main() -> Result<(), tencent_sdk::Error> {
    use tencent_sdk::{Auth, BlockingClient};

    let secret_id = std::env::var("TENCENT_SECRET_ID").expect("missing TENCENT_SECRET_ID");
    let secret_key = std::env::var("TENCENT_SECRET_KEY").expect("missing TENCENT_SECRET_KEY");

    let client = BlockingClient::builder_tencent_cloud()?
        .auth(Auth::tc3(secret_id, secret_key))
        .build()?;

    let response = client.billing().describe_account_balance()?;
    println!("real_balance: {:?}", response.response.real_balance);
    Ok(())
}

#[cfg(not(feature = "blocking"))]
fn main() {
    eprintln!("This example requires the `blocking` feature.");
}
