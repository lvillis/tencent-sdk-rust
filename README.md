<div align=right>Table of Contents↗️</div>

<h1 align=center><code>tencent-sdk-rust</code></h1>

<p align=center>📦 Tencent Cloud API SDK written in Rust</p>

<div align=center>
  <a href="https://crates.io/crates/tencent-sdk">
    <img src="https://img.shields.io/crates/v/tencent-sdk.svg" alt="crates.io version">
  </a>
  <a href="https://crates.io/crates/tencent-sdk">
    <img src="https://img.shields.io/crates/dr/tencent-sdk?color=ba86eb&logo=Handshake&logoColor=ea6aa6" alt="crates.io version">
  </a>
  <a href="https://crates.io/crates/tencent-sdk">
    <img src="https://img.shields.io/github/repo-size/lvillis/tencent-sdk-rust?style=flat-square&color=328657" alt="crates.io version">
  </a>
  <a href="https://github.com/lvillis/tencent-sdk-rust/actions">
    <img src="https://github.com/lvillis/tencent-sdk-rust/actions/workflows/ci.yaml/badge.svg" alt="build status">
  </a>
  <a href="mailto:lvillis@outlook.com?subject=Thanks%20for%20tencent-sdk-rust!">
    <img src="https://img.shields.io/badge/Say%20Thanks-!-1EAEDB.svg" alt="say thanks">
  </a>

</div>

---

This project is a Tencent Cloud API SDK written in Rust, designed to help developers integrate Tencent Cloud services
easily. The SDK uses asynchronous programming (via Tokio) and encapsulates functionalities such as request signing (
TC3-HMAC-SHA256), unified request handling, and modular service interfaces (e.g., CVM, Billing, Tag, etc.).

## Usage

### Add the crate

```toml
[dependencies]
tencent-sdk = "0.1"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

### Configure credentials and create clients

```rust
use tencent_sdk::{
    client::TencentCloudAsync,
    core::{TencentCloudError, TencentCloudResult},
    services::{
        cvm::{DescribeInstances, DescribeInstancesResponse},
        Filter,
    },
};

async fn describe_instances() -> TencentCloudResult<DescribeInstancesResponse> {
    let secret_id = std::env::var("TENCENT_SECRET_ID").expect("missing TENCENT_SECRET_ID");
    let secret_key = std::env::var("TENCENT_SECRET_KEY").expect("missing TENCENT_SECRET_KEY");

    let client = TencentCloudAsync::builder(secret_id, secret_key)?
        .no_system_proxy() // optional convenience helper
        .with_default_region("ap-guangzhou")
        .with_retry(3, std::time::Duration::from_millis(200))
        .build()?;

    let request = DescribeInstances::new()
        .with_region("ap-guangzhou")
        .with_limit(20)
        .push_filter(Filter::new("instance-name", ["example"]));

    client.request(&request).await
}

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), TencentCloudError> {
    let response = describe_instances().await?;
    println!("instances: {:?}", response.response.total_count);
    Ok(())
}
```

The blocking client mirrors the async API:

```rust
use tencent_sdk::{
    client::TencentCloudBlocking,
    services::billing::describe_account_balance_blocking,
};

fn fetch_balance() -> tencent_sdk::core::TencentCloudResult<()> {
    let client = TencentCloudBlocking::builder("secret", "key")?
        .no_system_proxy()
        .with_default_region("ap-guangzhou")
        .build()?;

    let result = describe_account_balance_blocking(&client)?;
    println!("balance: {:?}", result.response.real_balance);
    Ok(())
}
```

## Features

- **Asynchronous & Blocking Clients**: Tokio-powered async client plus a reqwest blocking client sharing configuration and retry middleware.
- **TC3 Signing Utilities**: Reusable helpers to construct compliant TC3-HMAC-SHA256 headers.
- **Strongly Typed Services**: Service modules expose typed request/response models and ergonomic builders for filters, tags, and pagination.
- **Actionable Error Taxonomy**: Service errors are classified (auth, throttled, forbidden, etc.) via `ServiceErrorKind` for easier recovery logic.
- **Expanded Test Coverage**: Wiremock-backed integration flows and deterministic signing snapshots keep regressions in check.

# Implemented Interfaces

- **CVM Module**
    - [x] DescribeInstances
    - [x] ResetInstancesPassword
    - [x] DescribeInstanceVncUrl
    - [x] StartInstances
    - [x] RebootInstances
    - [x] StopInstances
    - [x] ModifyInstancesProject

- **Tag Module**
    - [x] DescribeProjects

- **Billing Module**
    - [x] DescribeAccountBalance
