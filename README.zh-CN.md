<div align="right">

<a href="README.md">🇺🇸 English</a> ·
<span style="color:#999;">🇨🇳 中文</span>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;|&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; 目录&nbsp;↗️

</div>

<p align=center>📦 使用 Rust 编写的腾讯云 API SDK</p>

<div align=center>
  <a href="https://crates.io/crates/tencent-sdk">
    <img src="https://img.shields.io/crates/v/tencent-sdk.svg" alt="crates.io version">
  </a>
  <a href="https://crates.io/crates/tencent-sdk">
    <img src="https://img.shields.io/crates/dr/tencent-sdk?color=ba86eb&logo=Handshake&logoColor=ea6aa6" alt="downloads">
  </a>
  <a href="https://crates.io/crates/tencent-sdk">
    <img src="https://img.shields.io/github/repo-size/lvillis/tencent-sdk-rust?style=flat-square&color=328657" alt="repo size">
  </a>
  <a href="https://github.com/lvillis/tencent-sdk-rust/actions">
    <img src="https://github.com/lvillis/tencent-sdk-rust/actions/workflows/ci.yaml/badge.svg" alt="build status">
  </a>
  <a href="mailto:lvillis@outlook.com?subject=Thanks%20for%20tencent-sdk-rust!">
    <img src="https://img.shields.io/badge/Say%20Thanks-!-1EAEDB.svg" alt="say thanks">
  </a>
</div>

---

本项目是一个用 Rust 编写的腾讯云 API SDK，默认提供异步客户端，并通过 feature 提供可选的阻塞客户端；两者共享相同的 service 层、types 与错误模型。请求鉴权使用 TC3-HMAC-SHA256。

## Usage

### 添加依赖

```toml
[dependencies]
tencent-sdk = "0.1"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

仅使用阻塞客户端（不依赖 Tokio）：

```toml
[dependencies]
tencent-sdk = { version = "0.1", default-features = false, features = ["blocking-rustls"] }
```

### 配置凭证并创建客户端

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
        .no_system_proxy(true) // 可选：跳过系统代理
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

阻塞客户端与异步接口一致（不依赖 Tokio）：

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
  - `async`（默认）TLS 后端：`rustls`（默认）或 `native-tls`
  - `blocking`：`blocking-rustls` 或 `blocking-native-tls`
  - 可选集成：`tracing`、`metrics`
- **默认 async，可选 blocking**：`Client`（异步）+ `BlockingClient`（feature gated），共享 service 与 types。
- **Public API 不暴露底层 HTTP 类型**：对外签名不包含 reqwest/ureq 的类型。
- **TC3 签名**：内置 TC3-HMAC-SHA256 签名，`Debug` 输出默认脱敏凭证。
- **可诊断错误模型**：`Error` 提供 status / request_id / body snippet 与 service 错误分类。

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
