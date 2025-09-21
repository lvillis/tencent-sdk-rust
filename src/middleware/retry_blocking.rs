use crate::{core::TencentCloudError, transport::blocking_impl::BlockingTransport};
use http::{Method, StatusCode};
use std::{collections::HashMap, thread::sleep, time::Duration};
use url::Url;

#[derive(Clone)]
pub struct RetryBlocking<T> {
    inner: T,
    max: usize,
    base_delay: Duration,
}

impl<T> RetryBlocking<T> {
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

impl<T: BlockingTransport> BlockingTransport for RetryBlocking<T> {
    fn send(
        &self,
        method: Method,
        url: Url,
        headers: HashMap<String, String>,
        body: Option<String>,
        timeout: Duration,
    ) -> Result<(StatusCode, String), TencentCloudError> {
        let mut attempt = 0usize;
        loop {
            let result = self.inner.send(
                method.clone(),
                url.clone(),
                headers.clone(),
                body.clone(),
                timeout,
            )?;

            let (status, payload) = result;
            if status.is_server_error() && attempt < self.max {
                attempt += 1;
                let delay = self.delay_for(attempt);
                if !delay.is_zero() {
                    sleep(delay);
                }
                continue;
            }

            return Ok((status, payload));
        }
    }
}
