use crate::{
    core::{Credentials, Endpoint, TencentCloudError, TencentCloudResult},
    middleware::{RetryAsync, RetryBlocking},
    signing::{build_tc3_headers, SigningInput},
    transport::{
        async_impl::{AsyncTransport, DefaultAsyncTransport},
        blocking_impl::{BlockingTransport, DefaultBlockingTransport},
    },
};
use chrono::Utc;
use http::Method;
use serde_json::Value;
use std::time::Duration;
use url::Url;

const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);
const DEFAULT_USER_AGENT: &str = concat!("tencent-sdk-rust/", env!("CARGO_PKG_VERSION"));

pub struct TencentCloudAsyncBuilder<T = DefaultAsyncTransport> {
    credentials: Credentials,
    default_region: Option<String>,
    user_agent: String,
    insecure: bool,
    timeout: Duration,
    no_proxy: bool,
    transport: T,
}

impl TencentCloudAsyncBuilder<DefaultAsyncTransport> {
    fn default_builder(credentials: Credentials) -> TencentCloudResult<Self> {
        let transport =
            DefaultAsyncTransport::new(false, DEFAULT_USER_AGENT, DEFAULT_TIMEOUT, false)
                .map_err(TencentCloudError::transport_build)?;

        Ok(Self {
            credentials,
            default_region: None,
            user_agent: DEFAULT_USER_AGENT.to_string(),
            insecure: false,
            timeout: DEFAULT_TIMEOUT,
            no_proxy: false,
            transport,
        })
    }

    fn refresh_transport(&mut self) -> TencentCloudResult<()> {
        self.transport = DefaultAsyncTransport::new(
            self.insecure,
            &self.user_agent,
            self.timeout,
            self.no_proxy,
        )
        .map_err(TencentCloudError::transport_build)?;
        Ok(())
    }

    pub fn try_timeout(mut self, value: Duration) -> TencentCloudResult<Self> {
        self.timeout = value;
        self.refresh_transport()?;
        Ok(self)
    }

    pub fn timeout(self, value: Duration) -> Self {
        self.try_timeout(value)
            .expect("failed to apply timeout to TencentCloudAsyncBuilder")
    }

    pub fn try_user_agent(mut self, value: impl Into<String>) -> TencentCloudResult<Self> {
        self.user_agent = value.into();
        self.refresh_transport()?;
        Ok(self)
    }

    pub fn user_agent(self, value: impl Into<String>) -> Self {
        self.try_user_agent(value)
            .expect("failed to apply user agent to TencentCloudAsyncBuilder")
    }

    pub fn try_danger_accept_invalid_certs(mut self, yes: bool) -> TencentCloudResult<Self> {
        self.insecure = yes;
        self.refresh_transport()?;
        Ok(self)
    }

    pub fn danger_accept_invalid_certs(self, yes: bool) -> Self {
        self.try_danger_accept_invalid_certs(yes)
            .expect("failed to update TLS settings for TencentCloudAsyncBuilder")
    }

    pub fn try_no_system_proxy(mut self) -> TencentCloudResult<Self> {
        self.no_proxy = true;
        self.refresh_transport()?;
        Ok(self)
    }

    pub fn no_system_proxy(self) -> Self {
        self.try_no_system_proxy()
            .expect("failed to disable system proxy for TencentCloudAsyncBuilder")
    }
}

impl<T> TencentCloudAsyncBuilder<T> {
    pub fn with_default_region(mut self, region: impl Into<String>) -> Self {
        self.default_region = Some(region.into());
        self
    }

    pub fn clear_default_region(mut self) -> Self {
        self.default_region = None;
        self
    }

    pub fn with_token(mut self, token: impl Into<String>) -> Self {
        self.credentials.set_token(token);
        self
    }
}

impl<T: AsyncTransport> TencentCloudAsyncBuilder<T> {
    pub fn transport<NT: AsyncTransport>(self, transport: NT) -> TencentCloudAsyncBuilder<NT> {
        TencentCloudAsyncBuilder {
            credentials: self.credentials,
            default_region: self.default_region,
            user_agent: self.user_agent,
            insecure: self.insecure,
            timeout: self.timeout,
            no_proxy: self.no_proxy,
            transport,
        }
    }

    pub fn with_retry(
        self,
        max: usize,
        delay: Duration,
    ) -> TencentCloudAsyncBuilder<RetryAsync<T>> {
        let TencentCloudAsyncBuilder {
            credentials,
            default_region,
            user_agent,
            insecure,
            timeout,
            no_proxy,
            transport,
        } = self;

        TencentCloudAsyncBuilder {
            credentials,
            default_region,
            user_agent,
            insecure,
            timeout,
            no_proxy,
            transport: RetryAsync::new(transport, max, delay),
        }
    }

    pub fn build(self) -> TencentCloudResult<TencentCloudAsync<T>> {
        Ok(TencentCloudAsync {
            credentials: self.credentials,
            default_region: self.default_region,
            timeout: self.timeout,
            transport: self.transport,
        })
    }
}

pub struct TencentCloudAsync<T: AsyncTransport = DefaultAsyncTransport> {
    credentials: Credentials,
    default_region: Option<String>,
    timeout: Duration,
    transport: T,
}

impl TencentCloudAsync<DefaultAsyncTransport> {
    pub fn builder(
        secret_id: impl Into<String>,
        secret_key: impl Into<String>,
    ) -> TencentCloudResult<TencentCloudAsyncBuilder<DefaultAsyncTransport>> {
        let credentials = Credentials::new(secret_id, secret_key);
        TencentCloudAsyncBuilder::default_builder(credentials)
    }

    pub fn new(
        secret_id: impl Into<String>,
        secret_key: impl Into<String>,
    ) -> TencentCloudResult<Self> {
        Self::builder(secret_id, secret_key)?.build()
    }
}

impl<T: AsyncTransport> TencentCloudAsync<T> {
    pub async fn request<E: Endpoint>(&self, endpoint: &E) -> TencentCloudResult<E::Output> {
        let scheme = endpoint.scheme();
        let host = endpoint.host();
        let path = endpoint.path();
        let service = endpoint.service();
        let action = endpoint.action();
        let version = endpoint.version();
        let scheme_str = scheme.as_ref();
        let host_str = host.as_ref();
        let path_str = path.as_ref();
        let service_str = service.as_ref();
        let action_str = action.as_ref();
        let version_str = version.as_ref();
        let region = endpoint
            .region()
            .map(|value| value.into_owned())
            .or_else(|| self.default_region.clone());

        let payload_value = endpoint.payload();
        let payload = serde_json::to_string(&payload_value)?;
        let timestamp = Utc::now().timestamp();

        let url = Url::parse(&format!("{}://{}{}", scheme_str, host_str, path_str))?;

        let mut headers = build_tc3_headers(
            &self.credentials,
            &SigningInput {
                service: service_str,
                host: host_str,
                path: path_str,
                region: region.as_deref(),
                action: action_str,
                version: version_str,
                payload: &payload,
                timestamp,
            },
        )?;

        if let Some(extra) = endpoint.extra_headers() {
            for (key, value) in extra {
                headers.insert(key.into_owned(), value.into_owned());
            }
        }

        let body = Some(payload);
        let (status, text) = self
            .transport
            .send(Method::POST, url.clone(), headers, body, self.timeout)
            .await?;

        if !status.is_success() {
            return Err(TencentCloudError::http(status, Method::POST, url, text));
        }

        let json: Value = serde_json::from_str(&text)?;
        if let Some(err) = service_error_from_value(&json) {
            return Err(err);
        }
        endpoint.parse(json)
    }
}

pub struct TencentCloudBlockingBuilder<T = DefaultBlockingTransport> {
    credentials: Credentials,
    default_region: Option<String>,
    user_agent: String,
    insecure: bool,
    timeout: Duration,
    no_proxy: bool,
    transport: T,
}

impl TencentCloudBlockingBuilder<DefaultBlockingTransport> {
    fn default_builder(credentials: Credentials) -> TencentCloudResult<Self> {
        let transport =
            DefaultBlockingTransport::new(false, DEFAULT_USER_AGENT, DEFAULT_TIMEOUT, false)
                .map_err(TencentCloudError::transport_build)?;

        Ok(Self {
            credentials,
            default_region: None,
            user_agent: DEFAULT_USER_AGENT.to_string(),
            insecure: false,
            timeout: DEFAULT_TIMEOUT,
            no_proxy: false,
            transport,
        })
    }

    fn refresh_transport(&mut self) -> TencentCloudResult<()> {
        self.transport = DefaultBlockingTransport::new(
            self.insecure,
            &self.user_agent,
            self.timeout,
            self.no_proxy,
        )
        .map_err(TencentCloudError::transport_build)?;
        Ok(())
    }

    pub fn try_timeout(mut self, value: Duration) -> TencentCloudResult<Self> {
        self.timeout = value;
        self.refresh_transport()?;
        Ok(self)
    }

    pub fn timeout(self, value: Duration) -> Self {
        self.try_timeout(value)
            .expect("failed to apply timeout to TencentCloudBlockingBuilder")
    }

    pub fn try_user_agent(mut self, value: impl Into<String>) -> TencentCloudResult<Self> {
        self.user_agent = value.into();
        self.refresh_transport()?;
        Ok(self)
    }

    pub fn user_agent(self, value: impl Into<String>) -> Self {
        self.try_user_agent(value)
            .expect("failed to apply user agent to TencentCloudBlockingBuilder")
    }

    pub fn try_danger_accept_invalid_certs(mut self, yes: bool) -> TencentCloudResult<Self> {
        self.insecure = yes;
        self.refresh_transport()?;
        Ok(self)
    }

    pub fn danger_accept_invalid_certs(self, yes: bool) -> Self {
        self.try_danger_accept_invalid_certs(yes)
            .expect("failed to update TLS settings for TencentCloudBlockingBuilder")
    }

    pub fn try_no_system_proxy(mut self) -> TencentCloudResult<Self> {
        self.no_proxy = true;
        self.refresh_transport()?;
        Ok(self)
    }

    pub fn no_system_proxy(self) -> Self {
        self.try_no_system_proxy()
            .expect("failed to disable system proxy for TencentCloudBlockingBuilder")
    }
}

impl<T> TencentCloudBlockingBuilder<T> {
    pub fn with_default_region(mut self, region: impl Into<String>) -> Self {
        self.default_region = Some(region.into());
        self
    }

    pub fn clear_default_region(mut self) -> Self {
        self.default_region = None;
        self
    }

    pub fn with_token(mut self, token: impl Into<String>) -> Self {
        self.credentials.set_token(token);
        self
    }
}

impl<T: BlockingTransport> TencentCloudBlockingBuilder<T> {
    pub fn transport<NT: BlockingTransport>(
        self,
        transport: NT,
    ) -> TencentCloudBlockingBuilder<NT> {
        TencentCloudBlockingBuilder {
            credentials: self.credentials,
            default_region: self.default_region,
            user_agent: self.user_agent,
            insecure: self.insecure,
            timeout: self.timeout,
            no_proxy: self.no_proxy,
            transport,
        }
    }

    pub fn with_retry(
        self,
        max: usize,
        delay: Duration,
    ) -> TencentCloudBlockingBuilder<RetryBlocking<T>> {
        let TencentCloudBlockingBuilder {
            credentials,
            default_region,
            user_agent,
            insecure,
            timeout,
            no_proxy,
            transport,
        } = self;

        TencentCloudBlockingBuilder {
            credentials,
            default_region,
            user_agent,
            insecure,
            timeout,
            no_proxy,
            transport: RetryBlocking::new(transport, max, delay),
        }
    }

    pub fn build(self) -> TencentCloudResult<TencentCloudBlocking<T>> {
        Ok(TencentCloudBlocking {
            credentials: self.credentials,
            default_region: self.default_region,
            timeout: self.timeout,
            transport: self.transport,
        })
    }
}

pub struct TencentCloudBlocking<T: BlockingTransport = DefaultBlockingTransport> {
    credentials: Credentials,
    default_region: Option<String>,
    timeout: Duration,
    transport: T,
}

impl TencentCloudBlocking<DefaultBlockingTransport> {
    pub fn builder(
        secret_id: impl Into<String>,
        secret_key: impl Into<String>,
    ) -> TencentCloudResult<TencentCloudBlockingBuilder<DefaultBlockingTransport>> {
        let credentials = Credentials::new(secret_id, secret_key);
        TencentCloudBlockingBuilder::default_builder(credentials)
    }

    pub fn new(
        secret_id: impl Into<String>,
        secret_key: impl Into<String>,
    ) -> TencentCloudResult<Self> {
        Self::builder(secret_id, secret_key)?.build()
    }
}

impl<T: BlockingTransport> TencentCloudBlocking<T> {
    pub fn request<E: Endpoint>(&self, endpoint: &E) -> TencentCloudResult<E::Output> {
        let scheme = endpoint.scheme();
        let host = endpoint.host();
        let path = endpoint.path();
        let service = endpoint.service();
        let action = endpoint.action();
        let version = endpoint.version();
        let scheme_str = scheme.as_ref();
        let host_str = host.as_ref();
        let path_str = path.as_ref();
        let service_str = service.as_ref();
        let action_str = action.as_ref();
        let version_str = version.as_ref();
        let region = endpoint
            .region()
            .map(|value| value.into_owned())
            .or_else(|| self.default_region.clone());

        let payload_value = endpoint.payload();
        let payload = serde_json::to_string(&payload_value)?;
        let timestamp = Utc::now().timestamp();

        let url = Url::parse(&format!("{}://{}{}", scheme_str, host_str, path_str))?;

        let mut headers = build_tc3_headers(
            &self.credentials,
            &SigningInput {
                service: service_str,
                host: host_str,
                path: path_str,
                region: region.as_deref(),
                action: action_str,
                version: version_str,
                payload: &payload,
                timestamp,
            },
        )?;

        if let Some(extra) = endpoint.extra_headers() {
            for (key, value) in extra {
                headers.insert(key.into_owned(), value.into_owned());
            }
        }

        let body = Some(payload);
        let (status, text) =
            self.transport
                .send(Method::POST, url.clone(), headers, body, self.timeout)?;

        if !status.is_success() {
            return Err(TencentCloudError::http(status, Method::POST, url, text));
        }

        let json: Value = serde_json::from_str(&text)?;
        if let Some(err) = service_error_from_value(&json) {
            return Err(err);
        }
        endpoint.parse(json)
    }
}

fn service_error_from_value(value: &Value) -> Option<TencentCloudError> {
    let response = value.get("Response")?;
    let error = response.get("Error")?;
    let code = error.get("Code")?.as_str()?.to_string();
    let message = error.get("Message")?.as_str()?.to_string();
    let request_id = response
        .get("RequestId")
        .and_then(|id| id.as_str().map(|s| s.to_string()));
    Some(TencentCloudError::service(code, message, request_id))
}
