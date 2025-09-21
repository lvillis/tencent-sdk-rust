use crate::core::TencentCloudError;
use http::{Method, StatusCode};
use std::{collections::HashMap, time::Duration};
use url::Url;

pub mod async_impl {
    use super::*;
    use async_trait::async_trait;
    use reqwest::Client;

    #[async_trait]
    pub trait AsyncTransport: Clone + Send + Sync + 'static {
        async fn send(
            &self,
            method: Method,
            url: Url,
            headers: HashMap<String, String>,
            body: Option<String>,
            timeout: Duration,
        ) -> Result<(StatusCode, String), TencentCloudError>;
    }

    #[derive(Clone)]
    pub struct ReqwestAsync {
        client: Client,
    }

    impl ReqwestAsync {
        pub fn new(insecure: bool, ua: &str, timeout: Duration, no_proxy: bool) -> Self {
            let mut builder = Client::builder()
                .danger_accept_invalid_certs(insecure)
                .user_agent(ua)
                .timeout(timeout);

            if no_proxy {
                builder = builder.no_proxy();
            }

            let client = builder.build().expect("build async reqwest client");
            Self { client }
        }
    }

    #[async_trait]
    impl AsyncTransport for ReqwestAsync {
        async fn send(
            &self,
            method: Method,
            url: Url,
            headers: HashMap<String, String>,
            body: Option<String>,
            timeout: Duration,
        ) -> Result<(StatusCode, String), TencentCloudError> {
            let mut req = self
                .client
                .request(method.clone(), url.clone())
                .timeout(timeout);

            for (key, value) in &headers {
                req = req.header(key, value);
            }
            if let Some(body) = &body {
                req = req.body(body.clone());
            }

            let response = req.send().await.map_err(|source| {
                TencentCloudError::transport(source, method.clone(), url.clone())
            })?;

            let status = response.status();
            let body = response.text().await.map_err(|source| {
                TencentCloudError::transport(source, method.clone(), url.clone())
            })?;
            Ok((status, body))
        }
    }

    pub type DefaultAsyncTransport = ReqwestAsync;
}

pub mod blocking_impl {
    use super::*;
    use reqwest::blocking::Client;

    pub trait BlockingTransport: Clone + Send + Sync + 'static {
        fn send(
            &self,
            method: Method,
            url: Url,
            headers: HashMap<String, String>,
            body: Option<String>,
            timeout: Duration,
        ) -> Result<(StatusCode, String), TencentCloudError>;
    }

    #[derive(Clone)]
    pub struct ReqwestBlocking {
        client: Client,
    }

    impl ReqwestBlocking {
        pub fn new(insecure: bool, ua: &str, timeout: Duration, no_proxy: bool) -> Self {
            let mut builder = Client::builder()
                .danger_accept_invalid_certs(insecure)
                .user_agent(ua)
                .timeout(timeout);

            if no_proxy {
                builder = builder.no_proxy();
            }

            let client = builder.build().expect("build blocking reqwest client");
            Self { client }
        }
    }

    impl BlockingTransport for ReqwestBlocking {
        fn send(
            &self,
            method: Method,
            url: Url,
            headers: HashMap<String, String>,
            body: Option<String>,
            timeout: Duration,
        ) -> Result<(StatusCode, String), TencentCloudError> {
            let mut req = self
                .client
                .request(method.clone(), url.clone())
                .timeout(timeout);

            for (key, value) in &headers {
                req = req.header(key, value);
            }
            if let Some(body) = &body {
                req = req.body(body.clone());
            }

            let response = req.send().map_err(|source| {
                TencentCloudError::transport(source, method.clone(), url.clone())
            })?;

            let status = response.status();
            let body = response
                .text()
                .map_err(|source| TencentCloudError::transport(source, method, url))?;
            Ok((status, body))
        }
    }

    pub type DefaultBlockingTransport = ReqwestBlocking;
}
