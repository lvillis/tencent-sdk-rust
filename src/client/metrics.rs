use crate::Error;
use http::StatusCode;
use std::time::Duration;

pub(crate) fn record_success(
    service: &'static str,
    action: &'static str,
    status: StatusCode,
    retries: usize,
    elapsed: Duration,
) {
    let status = status.as_u16().to_string();

    metrics::counter!(
        "tencent_sdk_requests_total",
        "service" => service,
        "action" => action,
        "outcome" => "ok",
        "status" => status,
    )
    .increment(1);

    metrics::histogram!(
        "tencent_sdk_request_duration_seconds",
        "service" => service,
        "action" => action,
        "outcome" => "ok",
    )
    .record(elapsed.as_secs_f64());

    if retries > 0 {
        metrics::counter!(
            "tencent_sdk_retries_total",
            "service" => service,
            "action" => action,
        )
        .increment(retries as u64);
    }
}

pub(crate) fn record_error(
    service: &'static str,
    action: &'static str,
    error: &Error,
    retries: usize,
    elapsed: Duration,
) {
    let status = error
        .status()
        .map(|status| status.as_u16().to_string())
        .unwrap_or_else(|| "none".to_string());
    let kind = format!("{:?}", error.kind());

    metrics::counter!(
        "tencent_sdk_requests_total",
        "service" => service,
        "action" => action,
        "outcome" => "error",
        "status" => status,
        "error_kind" => kind,
    )
    .increment(1);

    metrics::histogram!(
        "tencent_sdk_request_duration_seconds",
        "service" => service,
        "action" => action,
        "outcome" => "error",
    )
    .record(elapsed.as_secs_f64());

    if retries > 0 {
        metrics::counter!(
            "tencent_sdk_retries_total",
            "service" => service,
            "action" => action,
        )
        .increment(retries as u64);
    }
}
