//! Tencent Cloud API SDK for Rust.
//!
//! ## Quick start (async)
//! ```no_run
//! # #[cfg(feature = "async")]
//! use tencent_sdk::{Auth, Client};
//!
//! # #[cfg(feature = "async")]
//! # async fn demo() -> Result<(), tencent_sdk::Error> {
//! let client = Client::builder_tencent_cloud()?
//!     .auth(Auth::tc3("SECRET_ID", "SECRET_KEY"))
//!     .default_region("ap-guangzhou")
//!     .build()?;
//!
//! // Example:
//! // let resp = client.cvm().describe_instances(...).await?;
//! # Ok(())
//! # }
//! # #[cfg(not(feature = "async"))]
//! # fn demo() {}
//! ```
//!
//! ## Quick start (blocking)
//! ```no_run
//! # #[cfg(feature = "blocking")]
//! use tencent_sdk::{Auth, BlockingClient};
//!
//! # #[cfg(feature = "blocking")]
//! # fn demo() -> Result<(), tencent_sdk::Error> {
//! let client = BlockingClient::builder_tencent_cloud()?
//!     .auth(Auth::tc3("SECRET_ID", "SECRET_KEY"))
//!     .build()?;
//!
//! // Example:
//! // let resp = client.billing().describe_account_balance()?;
//! # Ok(())
//! # }
//! # #[cfg(not(feature = "blocking"))]
//! # fn demo() {}
//! ```

#[cfg(not(any(feature = "async", feature = "blocking")))]
compile_error!("Enable at least one of the crate features: `async` or `blocking`.");

#[cfg(all(feature = "rustls", feature = "native-tls"))]
compile_error!("Enable only one of: rustls, native-tls.");

#[cfg(all(feature = "blocking-rustls", feature = "blocking-native-tls"))]
compile_error!("Enable only one of: blocking-rustls, blocking-native-tls.");

mod signing;
mod transport;
mod util;

pub mod api;
pub mod auth;
pub mod client;
pub mod error;
pub mod types;

pub use auth::Auth;
#[cfg(feature = "blocking")]
pub use client::BlockingClient;
#[cfg(feature = "async")]
pub use client::Client;
pub use error::Error;

pub type Result<T> = std::result::Result<T, Error>;
