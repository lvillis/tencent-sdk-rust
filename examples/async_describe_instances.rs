#[cfg(feature = "async")]
#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), tencent_sdk::Error> {
    use std::time::Duration;
    use tencent_sdk::types::{Filter, cvm::DescribeInstancesRequest};
    use tencent_sdk::{Auth, Client};

    let secret_id = std::env::var("TENCENT_SECRET_ID").expect("missing TENCENT_SECRET_ID");
    let secret_key = std::env::var("TENCENT_SECRET_KEY").expect("missing TENCENT_SECRET_KEY");

    let client = Client::builder_tencent_cloud()?
        .auth(Auth::tc3(secret_id, secret_key))
        .default_region("ap-guangzhou")
        .retry(3, Duration::from_millis(200))
        .build()?;

    let request = DescribeInstancesRequest::new()
        .limit(20)
        .push_filter(Filter::new("instance-name", ["example"]));

    let response = client.cvm().describe_instances(&request).await?;
    println!("total_count: {:?}", response.response.total_count);
    Ok(())
}

#[cfg(not(feature = "async"))]
fn main() {
    eprintln!("This example requires the `async` feature.");
}
