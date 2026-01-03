<div align="right">

<span style="color:#999;">🇺🇸 English</span> ·
<a href="README.zh-CN.md">🇨🇳 中文</a>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;|&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; Table&nbsp;of&nbsp;Contents&nbsp;↗️

</div>

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

Tencent Cloud API SDK for Rust. Async-first with an optional blocking client, sharing the same service layer, types and
error model. Requests are authenticated using TC3-HMAC-SHA256.

## Usage

### Add the crate

```toml
[dependencies]
tencent-sdk = "0.1"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

Blocking-only (no Tokio):

```toml
[dependencies]
tencent-sdk = { version = "0.1", default-features = false, features = ["blocking-rustls"] }
```

### Configure credentials and create clients

```rust
use std::time::Duration;
use tencent_sdk::types::{cvm::DescribeInstancesRequest, Filter};
use tencent_sdk::{Auth, Client};

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), tencent_sdk::Error> {
    let secret_id = std::env::var("TENCENT_SECRET_ID").expect("missing TENCENT_SECRET_ID");
    let secret_key = std::env::var("TENCENT_SECRET_KEY").expect("missing TENCENT_SECRET_KEY");

    let client = Client::builder_tencent_cloud()?
        .auth(Auth::tc3(secret_id, secret_key))
        .default_region("ap-guangzhou")
        .no_system_proxy(true)
        .retry(3, Duration::from_millis(200))
        .build()?;

    let request = DescribeInstancesRequest::new()
        .limit(20)
        .push_filter(Filter::new("instance-name", ["example"]));

    let response = client.cvm().describe_instances(&request).await?;
    println!("instances: {:?}", response.response.total_count);
    Ok(())
}
```

The blocking client mirrors the async API (does not require Tokio):

```rust
use tencent_sdk::{Auth, BlockingClient};

fn main() -> Result<(), tencent_sdk::Error> {
    let secret_id = std::env::var("TENCENT_SECRET_ID").expect("missing TENCENT_SECRET_ID");
    let secret_key = std::env::var("TENCENT_SECRET_KEY").expect("missing TENCENT_SECRET_KEY");

    let client = BlockingClient::builder_tencent_cloud()?
        .auth(Auth::tc3(secret_id, secret_key))
        .no_system_proxy(true)
        .build()?;

    let result = client.billing().describe_account_balance()?;
    println!("balance: {:?}", result.response.real_balance);
    Ok(())
}
```

## Features

- **Feature flags**
  - `async` (default) with TLS backend: `rustls` (default) or `native-tls`
  - `blocking` via `blocking-rustls` or `blocking-native-tls`
  - Optional integrations: `tracing`, `metrics`
- **Async-first, optional blocking**: `Client` (async) + `BlockingClient` (feature gated), sharing the same services and types.
- **No HTTP types in public API**: the SDK does not expose reqwest/ureq types in public signatures.
- **TC3 signing**: built-in TC3-HMAC-SHA256 signing with credential redaction in `Debug` output.
- **Actionable errors**: structured `Error` with status / request_id / body snippet and service classification.

# Implemented Interfaces

- **CVM**
    - [x] DescribeInstances
    - [x] ResetInstancesPassword
    - [x] DescribeInstanceVncUrl
    - [x] StartInstances
    - [x] RebootInstances
    - [x] StopInstances
    - [x] ModifyInstancesProject
    - [x] RunInstances
    - [x] TerminateInstances
    - [x] DescribeImages

- **Tag**
    - [x] DescribeProjects

- **Billing**
    - [x] DescribeAccountBalance

- **CDN**
    - [x] UpdateDomainConfig (HTTPS certificate switch)

- **DNSPod**
    - [x] CreateRecord (TXT)
    - [x] ModifyRecord (TXT)
    - [x] DeleteRecord

- **SSL**
    - [x] ApplyCertificate
    - [x] DescribeCertificate
    - [x] DownloadCertificate
    - [x] UploadCertificate
