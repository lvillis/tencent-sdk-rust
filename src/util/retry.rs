use http::HeaderMap;
use std::time::{Duration, SystemTime};

pub(crate) fn retry_delay(base: Duration, attempt: usize) -> Duration {
    if attempt == 0 {
        Duration::ZERO
    } else {
        let pow = 2f64.powi((attempt - 1) as i32);
        let jitter = 0.5 + fastrand::f64();
        base.mul_f64(pow).mul_f64(jitter)
    }
}

pub(crate) fn retry_after_delay(headers: &HeaderMap) -> Option<Duration> {
    let value = headers.get(http::header::RETRY_AFTER)?;
    let value = value.to_str().ok()?.trim();

    if let Ok(seconds) = value.parse::<u64>() {
        return Some(Duration::from_secs(seconds));
    }

    let time = httpdate::parse_http_date(value).ok()?;
    Some(
        time.duration_since(SystemTime::now())
            .unwrap_or(Duration::ZERO),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use http::{HeaderValue, header::RETRY_AFTER};

    #[test]
    fn retry_delay_is_zero_for_attempt_0() {
        assert_eq!(retry_delay(Duration::from_millis(100), 0), Duration::ZERO);
    }

    #[test]
    fn retry_delay_uses_exponential_backoff_with_jitter() {
        let base = Duration::from_millis(100);

        let delay_1 = retry_delay(base, 1);
        assert!(delay_1 >= Duration::from_millis(50));
        assert!(delay_1 < Duration::from_millis(150));

        let delay_2 = retry_delay(base, 2);
        assert!(delay_2 >= Duration::from_millis(100));
        assert!(delay_2 < Duration::from_millis(300));
    }

    #[test]
    fn retry_after_delay_parses_seconds() {
        let mut headers = HeaderMap::new();
        headers.insert(RETRY_AFTER, HeaderValue::from_static("10"));

        assert_eq!(retry_after_delay(&headers), Some(Duration::from_secs(10)));
    }

    #[test]
    fn retry_after_delay_parses_http_date() {
        let mut headers = HeaderMap::new();
        let past = httpdate::fmt_http_date(SystemTime::now() - Duration::from_secs(60));
        headers.insert(RETRY_AFTER, HeaderValue::from_str(&past).unwrap());

        assert_eq!(retry_after_delay(&headers), Some(Duration::ZERO));
    }
}
