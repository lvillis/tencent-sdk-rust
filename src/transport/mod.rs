use http::{HeaderMap, StatusCode};
use std::time::Duration;
use url::Url;

#[derive(Debug, Clone)]
pub(crate) struct TransportResponse {
    pub(crate) status: StatusCode,
    pub(crate) headers: HeaderMap,
    pub(crate) body: String,
}

#[cfg(feature = "async")]
pub(crate) mod async_transport;
#[cfg(feature = "blocking")]
pub(crate) mod blocking_transport;

#[derive(Debug, Clone)]
pub(crate) struct TransportConfig {
    pub(crate) user_agent: String,
    pub(crate) accept_invalid_certs: bool,
    pub(crate) no_proxy: bool,
    pub(crate) connect_timeout: Duration,
    pub(crate) read_timeout: Option<Duration>,
}

fn host_with_port(url: &Url) -> String {
    match (url.host_str(), url.port()) {
        (Some(host), Some(port)) => format!("{host}:{port}"),
        (Some(host), None) => host.to_string(),
        (None, _) => "<unknown-host>".to_string(),
    }
}
