use crate::Error;
use std::{fmt, time::Duration};
use url::Url;

pub(crate) const DEFAULT_USER_AGENT: &str = concat!("tencent-sdk/", env!("CARGO_PKG_VERSION"));
pub(crate) const DEFAULT_CONNECT_TIMEOUT: Duration = Duration::from_secs(10);
pub(crate) const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);
pub(crate) const DEFAULT_BODY_SNIPPET_MAX_BYTES: usize = 4096;
pub(crate) const DEFAULT_RETRY_BASE_DELAY: Duration = Duration::from_millis(50);

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[non_exhaustive]
pub enum EndpointMode {
    /// Resolve `service` into `{service}.{base_host}`.
    ServiceSubdomain,
    /// Always use `base_host` directly (useful for testing against a mock server).
    FixedHost,
}

#[derive(Debug, Clone, Default)]
pub struct RequestOptions {
    pub(crate) timeout: Option<Duration>,
    pub(crate) capture_body_snippet: Option<bool>,
    pub(crate) idempotency_key: Option<IdempotencyKey>,
}

impl RequestOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    pub fn capture_body_snippet(mut self, enabled: bool) -> Self {
        self.capture_body_snippet = Some(enabled);
        self
    }

    pub fn idempotency_key(mut self, key: impl Into<IdempotencyKey>) -> Self {
        self.idempotency_key = Some(key.into());
        self
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct IdempotencyKey(String);

impl IdempotencyKey {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for IdempotencyKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("IdempotencyKey([redacted])")
    }
}

impl From<String> for IdempotencyKey {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for IdempotencyKey {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

#[derive(Debug, Clone)]
pub(crate) struct RetryConfig {
    pub(crate) max_retries: usize,
    pub(crate) base_delay: Duration,
}

#[derive(Debug, Clone)]
pub(crate) struct RequestDefaults {
    pub(crate) timeout: Duration,
    pub(crate) capture_body_snippet: bool,
    pub(crate) body_snippet_max_bytes: usize,
}

#[derive(Debug, Clone)]
pub(crate) struct EndpointConfig {
    pub(crate) scheme: String,
    pub(crate) host: String,
    pub(crate) port: Option<u16>,
    pub(crate) mode: EndpointMode,
}

impl EndpointConfig {
    pub(crate) fn from_base_url(base_url: &str, mode: EndpointMode) -> Result<Self, Error> {
        let url = Url::parse(base_url)
            .map_err(|source| Error::invalid_base_url(base_url.to_string(), Box::new(source)))?;

        let scheme = url.scheme();
        if scheme != "http" && scheme != "https" {
            let source = std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "base url scheme must be http or https",
            );
            return Err(Error::invalid_base_url(
                base_url.to_string(),
                Box::new(source),
            ));
        }

        if !url.username().is_empty() || url.password().is_some() {
            let source = std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "base url must not include credentials",
            );
            return Err(Error::invalid_base_url(
                base_url.to_string(),
                Box::new(source),
            ));
        }

        if url.fragment().is_some() {
            let source = std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "base url must not include a fragment",
            );
            return Err(Error::invalid_base_url(
                base_url.to_string(),
                Box::new(source),
            ));
        }

        let host = url.host_str().ok_or_else(|| {
            let source = std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "base url must include a host",
            );
            Error::invalid_base_url(base_url.to_string(), Box::new(source))
        })?;

        if !(url.path().is_empty() || url.path() == "/") || url.query().is_some() {
            let source = std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "base url must not include a path or query",
            );
            return Err(Error::invalid_base_url(
                base_url.to_string(),
                Box::new(source),
            ));
        }

        Ok(Self {
            scheme: scheme.to_string(),
            host: host.to_string(),
            port: url.port(),
            mode,
        })
    }

    pub(crate) fn authority_for_service(&self, service: &str) -> String {
        let base_host = match self.mode {
            EndpointMode::ServiceSubdomain => format!("{service}.{}", self.host),
            EndpointMode::FixedHost => self.host.clone(),
        };

        match self.port {
            Some(port) => format!("{base_host}:{port}"),
            None => base_host,
        }
    }
}
