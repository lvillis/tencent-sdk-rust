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

本项目是一个用 Rust 编写的腾讯云 API SDK，帮助开发者轻松接入腾讯云服务。SDK 基于 Tokio 提供异步能力，封装了 TC3-HMAC-SHA256 请求签名、统一请求处理，以及按服务划分的模块化接口（如 CVM、Billing、Tag 等）。

## Usage

### 添加依赖

```toml
[dependencies]
tencent-sdk = "0.1"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

### 配置凭证并创建客户端

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
        .no_system_proxy() // 可选：跳过系统代理
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

阻塞客户端与异步接口一致：

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

- **Async & Blocking Clients**：Tokio 驱动的异步客户端与 reqwest 阻塞客户端，共享配置与重试逻辑。
- **TC3 签名工具**：可复用的 TC3-HMAC-SHA256 头部生成。
- **强类型服务接口**：按服务模块提供类型化请求/响应与构建器。
- **错误分类**：按认证、限流、权限等分类的错误类型，便于恢复。
- **测试覆盖**：Wiremock 驱动的集成流与确定性签名快照，避免回归。

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
