use super::{TransportConfig, TransportResponse, host_with_port};
use crate::Error;
use http::{HeaderMap, Method};
use std::time::Duration;
use url::Url;

#[derive(Clone)]
pub(crate) struct UreqBlockingTransport {
    agent: ureq::Agent,
}

impl UreqBlockingTransport {
    pub(crate) fn new(config: &TransportConfig) -> Result<Self, Error> {
        let mut builder = ureq::Agent::config_builder()
            .user_agent(config.user_agent.clone())
            .http_status_as_error(false)
            .timeout_connect(Some(config.connect_timeout));

        if let Some(read_timeout) = config.read_timeout {
            builder = builder
                .timeout_recv_response(Some(read_timeout))
                .timeout_recv_body(Some(read_timeout));
        }

        if config.no_proxy {
            builder = builder.proxy(None);
        }

        if config.accept_invalid_certs {
            #[cfg(any(feature = "blocking-rustls", feature = "blocking-native-tls"))]
            {
                let tls = ureq::tls::TlsConfig::builder()
                    .disable_verification(true)
                    .build();
                builder = builder.tls_config(tls);
            }

            #[cfg(not(any(feature = "blocking-rustls", feature = "blocking-native-tls")))]
            {
                let source = std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "TLS is disabled for the blocking transport",
                );
                return Err(Error::invalid_request_with_source(
                    "danger_accept_invalid_certs requires `blocking-rustls` or `blocking-native-tls`",
                    Box::new(source),
                ));
            }
        }

        let config = builder.build();
        let agent: ureq::Agent = config.into();

        Ok(Self { agent })
    }

    pub(crate) fn send(
        &self,
        method: Method,
        url: Url,
        headers: HeaderMap,
        body: Option<String>,
        timeout: Duration,
    ) -> Result<TransportResponse, Error> {
        let host = host_with_port(&url);
        let path = url.path().to_string();
        let mut builder = http::Request::builder()
            .method(method.clone())
            .uri(url.as_str());

        for (name, value) in headers.iter() {
            builder = builder.header(name, value);
        }

        if let Some(body) = body {
            let request = builder.body(body).map_err(|source| {
                Error::invalid_request_with_source("failed to build request", Box::new(source))
            })?;

            let request = self
                .agent
                .configure_request(request)
                .http_status_as_error(false)
                .timeout_per_call(Some(timeout))
                .build();

            let response = self.agent.run(request).map_err(|source| {
                Error::transport(method.clone(), host.clone(), path.clone(), source)
            })?;

            let (parts, mut body) = response.into_parts();
            let text = body
                .read_to_string()
                .map_err(|source| Error::transport(method, host, path, source))?;

            return Ok(TransportResponse {
                status: parts.status,
                headers: parts.headers,
                body: text,
            });
        }

        let request = builder.body(()).map_err(|source| {
            Error::invalid_request_with_source("failed to build request", Box::new(source))
        })?;

        let request = self
            .agent
            .configure_request(request)
            .http_status_as_error(false)
            .timeout_per_call(Some(timeout))
            .build();

        let response = self.agent.run(request).map_err(|source| {
            Error::transport(method.clone(), host.clone(), path.clone(), source)
        })?;

        let (parts, mut body) = response.into_parts();
        let text = body
            .read_to_string()
            .map_err(|source| Error::transport(method, host, path, source))?;

        Ok(TransportResponse {
            status: parts.status,
            headers: parts.headers,
            body: text,
        })
    }
}
