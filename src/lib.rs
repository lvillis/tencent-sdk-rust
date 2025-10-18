//! Tencent Cloud SDK providing async and blocking TC3-signed HTTP clients,
//! strongly typed service bindings, and reusable signing helpers.
//!
//! The crate exposes:
//! - [`client`] builders with retryable async and blocking clients.
//! - [`services`] modules per Tencent Cloud product surface.
//! - [`signing`] utilities for TC3-HMAC-SHA256 header generation.
//!
//! See the `README.md` for detailed usage examples and supported services.

pub mod client;
pub mod core;
pub mod middleware;
pub mod services;
pub mod signing;
pub mod transport;
