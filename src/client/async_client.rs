use crate::{
    Error, Result,
    auth::Auth,
    client::{
        common::{tencent_error_from_value, tencent_request_id_from_value},
        config::{
            DEFAULT_BODY_SNIPPET_MAX_BYTES, DEFAULT_CONNECT_TIMEOUT, DEFAULT_RETRY_BASE_DELAY,
            DEFAULT_TIMEOUT, DEFAULT_USER_AGENT, EndpointConfig, EndpointMode, RequestDefaults,
            RequestOptions, RetryConfig,
        },
        endpoint::Endpoint,
    },
    error::request_id_from_headers,
    signing::{SigningInput, build_tc3_headers},
    transport::{TransportConfig, async_transport::ReqwestAsyncTransport},
    types::Region,
    util::{body_snippet, build_url, canonical_query_string, retry_after_delay, retry_delay},
};
use chrono::Utc;
use http::{HeaderValue, Method, StatusCode};
use serde_json::Value;
use std::{sync::Arc, time::Duration};

#[derive(Clone)]
pub struct Client {
    inner: Arc<Inner>,
}

struct Inner {
    auth: Auth,
    endpoint: EndpointConfig,
    default_region: Option<Region>,
    transport: ReqwestAsyncTransport,
    defaults: RequestDefaults,
    retry: RetryConfig,
}

pub struct ClientBuilder {
    auth: Auth,
    endpoint: EndpointConfig,
    default_region: Option<Region>,
    transport: TransportConfig,
    defaults: RequestDefaults,
    retry: RetryConfig,
}

impl Client {
    pub fn builder(base_url: impl AsRef<str>) -> Result<ClientBuilder> {
        ClientBuilder::new(base_url.as_ref())
    }

    pub fn builder_tencent_cloud() -> Result<ClientBuilder> {
        ClientBuilder::new("https://tencentcloudapi.com")
    }

    pub fn billing(&self) -> crate::api::billing::BillingService {
        crate::api::billing::BillingService::new(self.clone())
    }

    pub fn tag(&self) -> crate::api::tag::TagService {
        crate::api::tag::TagService::new(self.clone())
    }

    pub fn cvm(&self) -> crate::api::cvm::CvmService {
        crate::api::cvm::CvmService::new(self.clone())
    }

    pub fn vpc(&self) -> crate::api::vpc::VpcService {
        crate::api::vpc::VpcService::new(self.clone())
    }

    pub fn cdn(&self) -> crate::api::cdn::CdnService {
        crate::api::cdn::CdnService::new(self.clone())
    }

    pub fn dns(&self) -> crate::api::dns::DnsService {
        crate::api::dns::DnsService::new(self.clone())
    }

    pub fn ssl(&self) -> crate::api::ssl::SslService {
        crate::api::ssl::SslService::new(self.clone())
    }

    pub(crate) async fn execute<E: Endpoint>(
        &self,
        endpoint: &E,
        options: Option<&RequestOptions>,
    ) -> Result<E::Output> {
        let method = endpoint.method();
        let service = endpoint.service();
        let action = endpoint.action();
        let version = endpoint.version();
        let path_segments = endpoint.path_segments();

        let region_owned = endpoint
            .region()
            .cloned()
            .or_else(|| self.inner.default_region.clone());
        let region = region_owned.as_ref().map(Region::as_str);

        let query_params = endpoint.query();
        let canonical_query = canonical_query_string(&query_params);
        let host = self.inner.endpoint.authority_for_service(service);
        let url = build_url(
            &self.inner.endpoint.scheme,
            &host,
            path_segments,
            &canonical_query,
        )?;
        let path = url.path().to_string();

        let payload_value = endpoint.payload()?;
        let payload_string = payload_value
            .as_ref()
            .map(serde_json::to_string)
            .transpose()
            .map_err(|source| {
                Error::invalid_request_with_source(
                    "failed to serialize request payload",
                    Box::new(source),
                )
            })?;

        let body = body_for_method(&method, payload_string.clone());
        let signing_payload = body.as_deref().unwrap_or("");

        let timeout = options
            .and_then(|o| o.timeout)
            .unwrap_or(self.inner.defaults.timeout);
        let capture_body_snippet = options
            .and_then(|o| o.capture_body_snippet)
            .unwrap_or(self.inner.defaults.capture_body_snippet);
        let idempotency_key = options.and_then(|o| o.idempotency_key.as_ref());
        let retryable_request = endpoint.is_idempotent() || idempotency_key.is_some();

        #[cfg(any(feature = "tracing", feature = "metrics"))]
        let started = std::time::Instant::now();

        #[cfg(feature = "tracing")]
        let span = tracing::info_span!(
            "tencent_sdk.request",
            service,
            action,
            version,
            method = %method,
            host = %host,
            path = %path,
            region = region.unwrap_or(""),
        );

        let mut attempt = 0usize;
        loop {
            let timestamp = Utc::now().timestamp();

            #[cfg(feature = "tracing")]
            tracing::debug!(parent: &span, attempt = attempt + 1, "sending request");
            let mut headers = build_tc3_headers(
                &self.inner.auth,
                &SigningInput {
                    method: &method,
                    service,
                    host: &host,
                    path: &path,
                    canonical_query: &canonical_query,
                    region,
                    action,
                    version,
                    payload: signing_payload,
                    timestamp,
                },
            )?;

            headers.extend(endpoint.extra_headers()?);

            if let Some(key) = idempotency_key {
                headers.insert(
                    "Idempotency-Key",
                    HeaderValue::from_str(key.as_str()).map_err(|source| {
                        Error::invalid_request_with_source("invalid idempotency key", source)
                    })?,
                );
            }

            let response = self
                .inner
                .transport
                .send(method.clone(), url.clone(), headers, body.clone(), timeout)
                .await;

            let response = match response {
                Ok(value) => value,
                Err(err) => {
                    if attempt < self.inner.retry.max_retries
                        && retryable_request
                        && err.is_retryable()
                    {
                        attempt += 1;
                        let delay = retry_delay(self.inner.retry.base_delay, attempt);

                        #[cfg(feature = "tracing")]
                        tracing::warn!(
                            parent: &span,
                            attempt,
                            delay = ?delay,
                            kind = ?err.kind(),
                            "retrying after transport error"
                        );

                        if !delay.is_zero() {
                            tokio::time::sleep(delay).await;
                        }
                        continue;
                    }

                    #[cfg(feature = "tracing")]
                    tracing::error!(
                        parent: &span,
                        kind = ?err.kind(),
                        "request failed with transport error"
                    );

                    #[cfg(feature = "metrics")]
                    super::metrics::record_error(service, action, &err, attempt, started.elapsed());
                    return Err(err);
                }
            };

            if !response.status.is_success() {
                if attempt < self.inner.retry.max_retries
                    && retryable_request
                    && is_retryable_status(response.status)
                {
                    attempt += 1;
                    let delay = retry_after_delay(&response.headers)
                        .unwrap_or_else(|| retry_delay(self.inner.retry.base_delay, attempt));

                    #[cfg(feature = "tracing")]
                    tracing::warn!(
                        parent: &span,
                        attempt,
                        status = %response.status,
                        delay = ?delay,
                        "retrying after retryable HTTP status"
                    );

                    if !delay.is_zero() {
                        tokio::time::sleep(delay).await;
                    }
                    continue;
                }

                let request_id_header = request_id_from_headers(&response.headers);
                let retry_after = retry_after_delay(&response.headers);
                let (code, message, request_id_body) =
                    serde_json::from_str::<Value>(&response.body)
                        .ok()
                        .and_then(|json| tencent_error_from_value(&json))
                        .map(|(code, message, request_id)| (Some(code), Some(message), request_id))
                        .unwrap_or((None, None, None));

                let request_id = request_id_body.or(request_id_header);
                let snippet = capture_body_snippet.then(|| {
                    body_snippet(&response.body, self.inner.defaults.body_snippet_max_bytes)
                });

                let err = Error::api(
                    Some(response.status),
                    method,
                    host,
                    path,
                    code,
                    message,
                    request_id,
                    snippet,
                    retry_after,
                );

                #[cfg(feature = "tracing")]
                tracing::error!(
                    parent: &span,
                    kind = ?err.kind(),
                    status = ?err.status(),
                    request_id = ?err.request_id(),
                    "request failed with HTTP status"
                );

                #[cfg(feature = "metrics")]
                super::metrics::record_error(service, action, &err, attempt, started.elapsed());

                return Err(err);
            }

            let json: Value = serde_json::from_str(&response.body).map_err(|source| {
                let request_id = request_id_from_headers(&response.headers);
                let snippet = capture_body_snippet.then(|| {
                    body_snippet(&response.body, self.inner.defaults.body_snippet_max_bytes)
                });
                let err = Error::decode(
                    Some(response.status),
                    method.clone(),
                    host.clone(),
                    path.clone(),
                    request_id,
                    snippet,
                    Box::new(source),
                );

                #[cfg(feature = "tracing")]
                tracing::error!(
                    parent: &span,
                    kind = ?err.kind(),
                    status = ?err.status(),
                    request_id = ?err.request_id(),
                    "request failed to decode response JSON"
                );

                #[cfg(feature = "metrics")]
                super::metrics::record_error(service, action, &err, attempt, started.elapsed());

                err
            })?;

            if let Some((code, message, request_id_body)) = tencent_error_from_value(&json) {
                let request_id =
                    request_id_body.or_else(|| request_id_from_headers(&response.headers));
                let snippet = capture_body_snippet.then(|| {
                    body_snippet(&response.body, self.inner.defaults.body_snippet_max_bytes)
                });
                let err = Error::api(
                    Some(response.status),
                    method.clone(),
                    host.clone(),
                    path.clone(),
                    Some(code),
                    Some(message),
                    request_id,
                    snippet,
                    retry_after_delay(&response.headers),
                );

                if attempt < self.inner.retry.max_retries && retryable_request && err.is_retryable()
                {
                    attempt += 1;
                    let delay = err
                        .retry_after()
                        .unwrap_or_else(|| retry_delay(self.inner.retry.base_delay, attempt));

                    #[cfg(feature = "tracing")]
                    tracing::warn!(
                        parent: &span,
                        attempt,
                        delay = ?delay,
                        kind = ?err.kind(),
                        code = ?err.code(),
                        "retrying after API error"
                    );

                    if !delay.is_zero() {
                        tokio::time::sleep(delay).await;
                    }
                    continue;
                }

                #[cfg(feature = "tracing")]
                tracing::error!(
                    parent: &span,
                    kind = ?err.kind(),
                    status = ?err.status(),
                    request_id = ?err.request_id(),
                    code = ?err.code(),
                    "request failed with API error"
                );

                #[cfg(feature = "metrics")]
                super::metrics::record_error(service, action, &err, attempt, started.elapsed());

                return Err(err);
            }

            let request_id = tencent_request_id_from_value(&json)
                .or_else(|| request_id_from_headers(&response.headers));
            let snippet = capture_body_snippet
                .then(|| body_snippet(&response.body, self.inner.defaults.body_snippet_max_bytes));

            let output = serde_json::from_value(json).map_err(|source| {
                let err = Error::decode(
                    Some(response.status),
                    method.clone(),
                    host.clone(),
                    path.clone(),
                    request_id.clone(),
                    snippet.clone(),
                    Box::new(source),
                );

                #[cfg(feature = "tracing")]
                tracing::error!(
                    parent: &span,
                    kind = ?err.kind(),
                    status = ?err.status(),
                    request_id = ?err.request_id(),
                    "request failed to decode response body"
                );

                #[cfg(feature = "metrics")]
                super::metrics::record_error(service, action, &err, attempt, started.elapsed());

                err
            })?;

            #[cfg(feature = "tracing")]
            tracing::info!(
                parent: &span,
                status = %response.status,
                retries = attempt,
                elapsed = ?started.elapsed(),
                request_id = ?request_id.as_deref(),
                "request succeeded"
            );

            #[cfg(feature = "metrics")]
            super::metrics::record_success(
                service,
                action,
                response.status,
                attempt,
                started.elapsed(),
            );

            return Ok(output);
        }
    }
}

impl ClientBuilder {
    fn new(base_url: &str) -> Result<Self> {
        let endpoint = EndpointConfig::from_base_url(base_url, EndpointMode::ServiceSubdomain)?;

        Ok(Self {
            auth: Auth::none(),
            endpoint,
            default_region: None,
            transport: TransportConfig {
                user_agent: DEFAULT_USER_AGENT.to_string(),
                accept_invalid_certs: false,
                no_proxy: false,
                connect_timeout: DEFAULT_CONNECT_TIMEOUT,
                read_timeout: None,
            },
            defaults: RequestDefaults {
                timeout: DEFAULT_TIMEOUT,
                capture_body_snippet: true,
                body_snippet_max_bytes: DEFAULT_BODY_SNIPPET_MAX_BYTES,
            },
            retry: RetryConfig {
                max_retries: 0,
                base_delay: DEFAULT_RETRY_BASE_DELAY,
            },
        })
    }

    pub fn auth(mut self, auth: Auth) -> Self {
        self.auth = auth;
        self
    }

    pub fn endpoint_mode(mut self, mode: EndpointMode) -> Self {
        self.endpoint.mode = mode;
        self
    }

    pub fn default_region(mut self, region: impl Into<Region>) -> Self {
        self.default_region = Some(region.into());
        self
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.defaults.timeout = timeout;
        self
    }

    pub fn connect_timeout(mut self, timeout: Duration) -> Self {
        self.transport.connect_timeout = timeout;
        self
    }

    pub fn read_timeout(mut self, timeout: Duration) -> Self {
        self.transport.read_timeout = Some(timeout);
        self
    }

    pub fn capture_body_snippet(mut self, enabled: bool) -> Self {
        self.defaults.capture_body_snippet = enabled;
        self
    }

    pub fn body_snippet_max_bytes(mut self, max_bytes: usize) -> Self {
        self.defaults.body_snippet_max_bytes = max_bytes;
        self
    }

    pub fn user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.transport.user_agent = user_agent.into();
        self
    }

    pub fn danger_accept_invalid_certs(mut self, enabled: bool) -> Self {
        self.transport.accept_invalid_certs = enabled;
        self
    }

    pub fn no_system_proxy(mut self, enabled: bool) -> Self {
        self.transport.no_proxy = enabled;
        self
    }

    pub fn retry(mut self, max_retries: usize, base_delay: Duration) -> Self {
        self.retry.max_retries = max_retries;
        self.retry.base_delay = base_delay;
        self
    }

    pub fn build(self) -> Result<Client> {
        let transport = ReqwestAsyncTransport::new(&self.transport)?;

        Ok(Client {
            inner: Arc::new(Inner {
                auth: self.auth,
                endpoint: self.endpoint,
                default_region: self.default_region,
                transport,
                defaults: self.defaults,
                retry: self.retry,
            }),
        })
    }
}

fn body_for_method(method: &Method, payload: Option<String>) -> Option<String> {
    if *method == Method::GET || *method == Method::HEAD || *method == Method::OPTIONS {
        None
    } else {
        payload
    }
}

fn is_retryable_status(status: StatusCode) -> bool {
    matches!(
        status,
        StatusCode::TOO_MANY_REQUESTS
            | StatusCode::BAD_GATEWAY
            | StatusCode::SERVICE_UNAVAILABLE
            | StatusCode::GATEWAY_TIMEOUT
    )
}
