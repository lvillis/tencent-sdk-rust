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

## Features

- **Asynchronous Support**: Built on Tokio for high concurrency.
- **Request Signing**: Implements Tencent Cloud's TC3-HMAC-SHA256 signature algorithm.
- **Detailed Documentation**: Each interface is documented with detailed input/output parameter tables.
- **Comprehensive Testing**: Each service interface includes test cases to ensure correct functionality.

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
