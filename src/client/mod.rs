#[cfg(feature = "async")]
mod async_client;
#[cfg(feature = "blocking")]
mod blocking_client;

mod common;
mod config;
pub(crate) mod endpoint;
#[cfg(feature = "metrics")]
pub(crate) mod metrics;

#[cfg(feature = "async")]
pub use async_client::{Client, ClientBuilder};
#[cfg(feature = "blocking")]
pub use blocking_client::{BlockingClient, BlockingClientBuilder};

pub use config::{EndpointMode, IdempotencyKey, RequestOptions};
