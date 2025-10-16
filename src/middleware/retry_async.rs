use crate::{core::TencentCloudError, transport::async_impl::AsyncTransport};
use async_trait::async_trait;
use fastrand;
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
            let pow = 2f64.powi((attempt - 1) as i32);
            let base = self.base_delay.mul_f64(pow);
            let jitter = 0.5 + fastrand::f64();
            base.mul_f64(jitter)
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
            match self
                .inner
                .send(
                    method.clone(),
                    url.clone(),
                    headers.clone(),
                    body.clone(),
                    timeout,
                )
                .await
            {
                Ok((status, payload)) => {
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
                Err(err) => {
                    let should_retry =
                        attempt < self.max && matches!(err, TencentCloudError::Transport { .. });

                    if should_retry {
                        attempt += 1;
                        let delay = self.delay_for(attempt);
                        if !delay.is_zero() {
                            sleep(delay).await;
                        }
                        continue;
                    }

                    return Err(err);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use http::Method;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;
    use tokio::task;

    #[derive(Clone)]
    struct FlakyAsyncTransport {
        attempts: Arc<AtomicUsize>,
        fail_times: usize,
    }

    impl FlakyAsyncTransport {
        fn new(fail_times: usize) -> Self {
            Self {
                attempts: Arc::new(AtomicUsize::new(0)),
                fail_times,
            }
        }
    }

    #[async_trait]
    impl AsyncTransport for FlakyAsyncTransport {
        async fn send(
            &self,
            method: Method,
            url: Url,
            _headers: HashMap<String, String>,
            _body: Option<String>,
            _timeout: Duration,
        ) -> Result<(StatusCode, String), TencentCloudError> {
            let current = self.attempts.fetch_add(1, Ordering::SeqCst);
            if current < self.fail_times {
                let error = task::spawn_blocking(move || make_transport_error(method, url))
                    .await
                    .expect("spawn blocking for transport error");
                return Err(error);
            }

            Ok((StatusCode::OK, "{}".to_string()))
        }
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn retries_transport_errors_before_succeeding() {
        let transport = FlakyAsyncTransport::new(2);
        let retry = RetryAsync::new(transport.clone(), 3, Duration::from_millis(1));
        let result = retry
            .send(
                Method::POST,
                Url::parse("https://example.com").unwrap(),
                HashMap::new(),
                None,
                Duration::from_secs(1),
            )
            .await;

        assert!(
            result.is_ok(),
            "expected retry to eventually succeed: {result:?}"
        );
        assert_eq!(
            transport.attempts.load(Ordering::SeqCst),
            3,
            "expected two retries plus final success"
        );
    }

    fn make_transport_error(method: Method, url: Url) -> TencentCloudError {
        let client = reqwest::blocking::Client::builder()
            .build()
            .expect("build test reqwest client");

        let error = client
            .get("http://example.com")
            .header("\n", "value")
            .build()
            .expect_err("invalid header should fail before network IO");

        TencentCloudError::transport(error, method, url)
    }
}
