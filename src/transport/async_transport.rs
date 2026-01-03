use super::{TransportConfig, TransportResponse, host_with_port};
use crate::Error;
use http::{HeaderMap, Method};
use std::time::Duration;
use url::Url;

#[derive(Clone)]
pub(crate) struct ReqwestAsyncTransport {
    client: reqwest::Client,
}

impl ReqwestAsyncTransport {
    pub(crate) fn new(config: &TransportConfig) -> Result<Self, Error> {
        #[cfg(feature = "rustls")]
        {
            if rustls::crypto::CryptoProvider::get_default().is_none() {
                let _ = rustls::crypto::ring::default_provider().install_default();
            }
        }

        let mut builder = reqwest::Client::builder()
            .danger_accept_invalid_certs(config.accept_invalid_certs)
            .user_agent(&config.user_agent)
            .connect_timeout(config.connect_timeout);

        if let Some(read_timeout) = config.read_timeout {
            builder = builder.read_timeout(read_timeout);
        }

        if config.no_proxy {
            builder = builder.no_proxy();
        }

        let client = builder.build().map_err(Error::transport_build)?;

        Ok(Self { client })
    }

    pub(crate) async fn send(
        &self,
        method: Method,
        url: Url,
        headers: HeaderMap,
        body: Option<String>,
        timeout: Duration,
    ) -> Result<TransportResponse, Error> {
        let host = host_with_port(&url);
        let path = url.path().to_string();

        let mut request = self
            .client
            .request(method.clone(), url.clone())
            .timeout(timeout);
        request = request.headers(headers);
        if let Some(body) = body {
            request = request.body(body);
        }

        let response = request.send().await.map_err(|source| {
            Error::transport(method.clone(), host.clone(), path.clone(), source)
        })?;

        let status = response.status();
        let headers = response.headers().clone();
        let body = response
            .text()
            .await
            .map_err(|source| Error::transport(method, host, path, source))?;

        Ok(TransportResponse {
            status,
            headers,
            body,
        })
    }
}
