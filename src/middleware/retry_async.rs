use crate::{core::TencentCloudError, transport::async_impl::AsyncTransport};
use async_trait::async_trait;
use http::{Method, StatusCode};
use std::{collections::HashMap, time::Duration};
use tokio::time::sleep;
use url::Url;

#[derive(Clone)]
pub struct RetryAsync<T> {
    inner: T,
    max: usize,
    base_delay: Duration,
}

impl<T> RetryAsync<T> {
    pub fn new(inner: T, max: usize, base_delay: Duration) -> Self {
        Self {
            inner,
            max,
            base_delay,
        }
    }

    fn delay_for(&self, attempt: usize) -> Duration {
        if attempt == 0 {
            Duration::from_secs(0)
        } else {
            self.base_delay.mul_f64(2f64.powi((attempt - 1) as i32))
        }
    }
}

#[async_trait]
impl<T: AsyncTransport> AsyncTransport for RetryAsync<T> {
    async fn send(
        &self,
        method: Method,
        url: Url,
        headers: HashMap<String, String>,
        body: Option<String>,
        timeout: Duration,
    ) -> Result<(StatusCode, String), TencentCloudError> {
        let mut attempt = 0usize;
        loop {
            let result = self
                .inner
                .send(
                    method.clone(),
                    url.clone(),
                    headers.clone(),
                    body.clone(),
                    timeout,
                )
                .await?;

            let (status, payload) = result;
            if status.is_server_error() && attempt < self.max {
                attempt += 1;
                let delay = self.delay_for(attempt);
                if !delay.is_zero() {
                    sleep(delay).await;
                }
                continue;
            }

            return Ok((status, payload));
        }
    }
}
