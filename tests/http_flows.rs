use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use wiremock::{Request, Respond, ResponseTemplate};

#[derive(Clone)]
struct TwoStepResponder {
    attempts: Arc<AtomicUsize>,
    first: ResponseTemplate,
    second: ResponseTemplate,
}

impl Respond for TwoStepResponder {
    fn respond(&self, _request: &Request) -> ResponseTemplate {
        let attempt = self.attempts.fetch_add(1, Ordering::SeqCst);
        if attempt == 0 {
            self.first.clone()
        } else {
            self.second.clone()
        }
    }
}

#[cfg(feature = "async")]
mod async_client {
    use httpdate::fmt_http_date;
    use serde_json::json;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::time::{Duration, SystemTime};
    use tencent_sdk::client::{EndpointMode, RequestOptions};
    use tencent_sdk::error::ErrorKind;
    use tencent_sdk::types::cvm::{DescribeInstancesRequest, RunInstancesRequest};
    use tencent_sdk::{Auth, Client};
    use wiremock::matchers::{body_json, header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    fn build_client(server: &MockServer) -> Client {
        Client::builder(server.uri())
            .expect("build client builder")
            .endpoint_mode(EndpointMode::FixedHost)
            .auth(Auth::tc3("secret_id", "secret_key"))
            .default_region("ap-guangzhou")
            .no_system_proxy(true)
            .build()
            .expect("build client")
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn async_client_parses_success_response() {
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/"))
            .and(header("X-TC-Action", "DescribeInstances"))
            .and(body_json(json!({"Limit": 1})))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "Response": {
                    "TotalCount": 0,
                    "InstanceSet": [],
                    "RequestId": "req-1"
                }
            })))
            .mount(&server)
            .await;

        let client = build_client(&server);
        let request = DescribeInstancesRequest::new().limit(1);

        let resp = client
            .cvm()
            .describe_instances(&request)
            .await
            .expect("request succeeds");

        assert_eq!(resp.response.request_id.as_str(), "req-1");
        assert_eq!(resp.response.total_count, Some(0));
        assert!(resp.response.instance_set.is_empty());
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn async_client_surfaces_http_status_error() {
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/"))
            .respond_with(ResponseTemplate::new(500).set_body_string("boom"))
            .mount(&server)
            .await;

        let client = build_client(&server);
        let request = DescribeInstancesRequest::new().limit(1);

        let err = client
            .cvm()
            .describe_instances(&request)
            .await
            .expect_err("status error surfaced");

        assert_eq!(err.kind(), ErrorKind::Api);
        assert_eq!(err.status().map(|status| status.as_u16()), Some(500));
        assert_eq!(err.body_snippet(), Some("boom"));
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn async_client_surfaces_service_error() {
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "Response": {
                    "Error": {
                        "Code": "AuthFailure.SignatureFailure",
                        "Message": "signature mismatch"
                    },
                    "RequestId": "req-123"
                }
            })))
            .mount(&server)
            .await;

        let client = build_client(&server);
        let request = DescribeInstancesRequest::new().limit(1);

        let err = client
            .cvm()
            .describe_instances(&request)
            .await
            .expect_err("service error surfaced");

        assert_eq!(err.kind(), ErrorKind::Auth);
        assert_eq!(err.code(), Some("AuthFailure.SignatureFailure"));
        assert_eq!(err.message(), Some("signature mismatch"));
        assert_eq!(err.request_id(), Some("req-123"));
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn async_client_retries_retryable_service_errors() {
        let server = MockServer::start().await;
        let attempts = Arc::new(AtomicUsize::new(0));

        Mock::given(method("POST"))
            .and(path("/"))
            .respond_with(super::TwoStepResponder {
                attempts: attempts.clone(),
                first: ResponseTemplate::new(200).set_body_json(json!({
                    "Response": {
                        "Error": {
                            "Code": "Throttling",
                            "Message": "slow down"
                        },
                        "RequestId": "req-retry"
                    }
                })),
                second: ResponseTemplate::new(200).set_body_json(json!({
                    "Response": {
                        "TotalCount": 0,
                        "InstanceSet": [],
                        "RequestId": "req-ok"
                    }
                })),
            })
            .mount(&server)
            .await;

        let client = Client::builder(server.uri())
            .expect("build client builder")
            .endpoint_mode(EndpointMode::FixedHost)
            .auth(Auth::tc3("secret_id", "secret_key"))
            .default_region("ap-guangzhou")
            .no_system_proxy(true)
            .retry(1, Duration::from_millis(0))
            .build()
            .expect("build client");

        let request = DescribeInstancesRequest::new().limit(1);

        let resp = client
            .cvm()
            .describe_instances(&request)
            .await
            .expect("eventual success");

        assert_eq!(attempts.load(Ordering::SeqCst), 2);
        assert_eq!(resp.response.request_id.as_str(), "req-ok");
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn async_client_respects_retry_after_on_http_429() {
        let server = MockServer::start().await;
        let attempts = Arc::new(AtomicUsize::new(0));

        Mock::given(method("POST"))
            .and(path("/"))
            .respond_with(super::TwoStepResponder {
                attempts: attempts.clone(),
                first: ResponseTemplate::new(429)
                    .insert_header("Retry-After", "0")
                    .set_body_string("rate limited"),
                second: ResponseTemplate::new(200).set_body_json(json!({
                    "Response": {
                        "TotalCount": 0,
                        "InstanceSet": [],
                        "RequestId": "req-429-ok"
                    }
                })),
            })
            .mount(&server)
            .await;

        let client = Client::builder(server.uri())
            .expect("build client builder")
            .endpoint_mode(EndpointMode::FixedHost)
            .auth(Auth::tc3("secret_id", "secret_key"))
            .default_region("ap-guangzhou")
            .no_system_proxy(true)
            .retry(1, Duration::from_millis(0))
            .build()
            .expect("build client");

        let request = DescribeInstancesRequest::new().limit(1);
        let resp = client
            .cvm()
            .describe_instances(&request)
            .await
            .expect("eventual success");

        assert_eq!(attempts.load(Ordering::SeqCst), 2);
        assert_eq!(resp.response.request_id.as_str(), "req-429-ok");
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn async_client_respects_retry_after_http_date_on_http_429() {
        let server = MockServer::start().await;
        let attempts = Arc::new(AtomicUsize::new(0));
        let retry_at = fmt_http_date(SystemTime::now());

        Mock::given(method("POST"))
            .and(path("/"))
            .respond_with(super::TwoStepResponder {
                attempts: attempts.clone(),
                first: ResponseTemplate::new(429)
                    .insert_header("Retry-After", retry_at)
                    .set_body_string("rate limited"),
                second: ResponseTemplate::new(200).set_body_json(json!({
                    "Response": {
                        "TotalCount": 0,
                        "InstanceSet": [],
                        "RequestId": "req-429-date-ok"
                    }
                })),
            })
            .mount(&server)
            .await;

        let client = Client::builder(server.uri())
            .expect("build client builder")
            .endpoint_mode(EndpointMode::FixedHost)
            .auth(Auth::tc3("secret_id", "secret_key"))
            .default_region("ap-guangzhou")
            .no_system_proxy(true)
            .retry(1, Duration::from_millis(0))
            .build()
            .expect("build client");

        let request = DescribeInstancesRequest::new().limit(1);
        let resp = client
            .cvm()
            .describe_instances(&request)
            .await
            .expect("eventual success");

        assert_eq!(attempts.load(Ordering::SeqCst), 2);
        assert_eq!(resp.response.request_id.as_str(), "req-429-date-ok");
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn async_client_does_not_retry_non_idempotent_post_without_idempotency_key() {
        let server = MockServer::start().await;
        let attempts = Arc::new(AtomicUsize::new(0));

        Mock::given(method("POST"))
            .and(path("/"))
            .and(header("X-TC-Action", "RunInstances"))
            .respond_with(super::TwoStepResponder {
                attempts: attempts.clone(),
                first: ResponseTemplate::new(429)
                    .insert_header("Retry-After", "0")
                    .set_body_string("rate limited"),
                second: ResponseTemplate::new(200).set_body_json(json!({
                    "Response": {
                        "InstanceIdSet": [],
                        "RequestId": "req-run-ok"
                    }
                })),
            })
            .mount(&server)
            .await;

        let client = Client::builder(server.uri())
            .expect("build client builder")
            .endpoint_mode(EndpointMode::FixedHost)
            .auth(Auth::tc3("secret_id", "secret_key"))
            .default_region("ap-guangzhou")
            .no_system_proxy(true)
            .retry(1, Duration::from_millis(0))
            .build()
            .expect("build client");

        let request = RunInstancesRequest::new("ap-guangzhou", "img-123", "S4.SMALL1");

        let err = client
            .cvm()
            .run_instances(&request)
            .await
            .expect_err("non-idempotent POST should not retry");

        assert_eq!(attempts.load(Ordering::SeqCst), 1);
        assert_eq!(err.kind(), ErrorKind::RateLimited);
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn async_client_retries_non_idempotent_post_with_idempotency_key() {
        let server = MockServer::start().await;
        let attempts = Arc::new(AtomicUsize::new(0));

        Mock::given(method("POST"))
            .and(path("/"))
            .and(header("X-TC-Action", "RunInstances"))
            .and(header("Idempotency-Key", "key-1"))
            .respond_with(super::TwoStepResponder {
                attempts: attempts.clone(),
                first: ResponseTemplate::new(429)
                    .insert_header("Retry-After", "0")
                    .set_body_string("rate limited"),
                second: ResponseTemplate::new(200).set_body_json(json!({
                    "Response": {
                        "InstanceIdSet": [],
                        "RequestId": "req-run-ok"
                    }
                })),
            })
            .mount(&server)
            .await;

        let client = Client::builder(server.uri())
            .expect("build client builder")
            .endpoint_mode(EndpointMode::FixedHost)
            .auth(Auth::tc3("secret_id", "secret_key"))
            .default_region("ap-guangzhou")
            .no_system_proxy(true)
            .retry(1, Duration::from_millis(0))
            .build()
            .expect("build client");

        let request = RunInstancesRequest::new("ap-guangzhou", "img-123", "S4.SMALL1");
        let options = RequestOptions::new().idempotency_key("key-1");

        let resp = client
            .cvm()
            .run_instances_with_options(&request, &options)
            .await
            .expect("eventual success");

        assert_eq!(attempts.load(Ordering::SeqCst), 2);
        assert_eq!(resp.response.request_id.as_str(), "req-run-ok");
    }
}

#[cfg(feature = "blocking")]
mod blocking_client {
    use httpdate::fmt_http_date;
    use serde_json::json;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::time::{Duration, SystemTime};
    use tencent_sdk::client::{EndpointMode, RequestOptions};
    use tencent_sdk::error::ErrorKind;
    use tencent_sdk::types::cvm::{DescribeInstancesRequest, RunInstancesRequest};
    use tencent_sdk::{Auth, BlockingClient};
    use wiremock::matchers::{body_json, header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    fn build_client(server: &MockServer) -> BlockingClient {
        BlockingClient::builder(server.uri())
            .expect("build client builder")
            .endpoint_mode(EndpointMode::FixedHost)
            .auth(Auth::tc3("secret_id", "secret_key"))
            .default_region("ap-guangzhou")
            .no_system_proxy(true)
            .build()
            .expect("build client")
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn blocking_client_parses_success_response() {
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/"))
            .and(header("X-TC-Action", "DescribeInstances"))
            .and(body_json(json!({"Limit": 1})))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "Response": {
                    "TotalCount": 0,
                    "InstanceSet": [],
                    "RequestId": "req-1"
                }
            })))
            .mount(&server)
            .await;

        let client = build_client(&server);
        let request = DescribeInstancesRequest::new().limit(1);

        let resp = tokio::task::spawn_blocking(move || client.cvm().describe_instances(&request))
            .await
            .expect("join blocking task")
            .expect("request succeeds");

        assert_eq!(resp.response.request_id.as_str(), "req-1");
        assert_eq!(resp.response.total_count, Some(0));
        assert!(resp.response.instance_set.is_empty());
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn blocking_client_surfaces_http_status_error() {
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/"))
            .respond_with(ResponseTemplate::new(500).set_body_string("boom"))
            .mount(&server)
            .await;

        let client = build_client(&server);
        let request = DescribeInstancesRequest::new().limit(1);

        let err = tokio::task::spawn_blocking(move || client.cvm().describe_instances(&request))
            .await
            .expect("join blocking task")
            .expect_err("status error surfaced");

        assert_eq!(err.kind(), ErrorKind::Api);
        assert_eq!(err.status().map(|status| status.as_u16()), Some(500));
        assert_eq!(err.body_snippet(), Some("boom"));
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn blocking_client_surfaces_service_error() {
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "Response": {
                    "Error": {
                        "Code": "AuthFailure.SignatureFailure",
                        "Message": "signature mismatch"
                    },
                    "RequestId": "req-123"
                }
            })))
            .mount(&server)
            .await;

        let client = build_client(&server);
        let request = DescribeInstancesRequest::new().limit(1);

        let err = tokio::task::spawn_blocking(move || client.cvm().describe_instances(&request))
            .await
            .expect("join blocking task")
            .expect_err("service error surfaced");

        assert_eq!(err.kind(), ErrorKind::Auth);
        assert_eq!(err.code(), Some("AuthFailure.SignatureFailure"));
        assert_eq!(err.message(), Some("signature mismatch"));
        assert_eq!(err.request_id(), Some("req-123"));
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn blocking_client_retries_retryable_service_errors() {
        let server = MockServer::start().await;
        let attempts = Arc::new(AtomicUsize::new(0));

        Mock::given(method("POST"))
            .and(path("/"))
            .respond_with(super::TwoStepResponder {
                attempts: attempts.clone(),
                first: ResponseTemplate::new(200).set_body_json(json!({
                    "Response": {
                        "Error": {
                            "Code": "RequestLimitExceeded",
                            "Message": "throttled"
                        },
                        "RequestId": "req-retry"
                    }
                })),
                second: ResponseTemplate::new(200).set_body_json(json!({
                    "Response": {
                        "TotalCount": 0,
                        "InstanceSet": [],
                        "RequestId": "req-ok"
                    }
                })),
            })
            .mount(&server)
            .await;

        let client = BlockingClient::builder(server.uri())
            .expect("build client builder")
            .endpoint_mode(EndpointMode::FixedHost)
            .auth(Auth::tc3("secret_id", "secret_key"))
            .default_region("ap-guangzhou")
            .no_system_proxy(true)
            .retry(1, Duration::from_millis(0))
            .build()
            .expect("build client");

        let request = DescribeInstancesRequest::new().limit(1);

        let resp = tokio::task::spawn_blocking(move || client.cvm().describe_instances(&request))
            .await
            .expect("join blocking task")
            .expect("eventual success");

        assert_eq!(attempts.load(Ordering::SeqCst), 2);
        assert_eq!(resp.response.request_id.as_str(), "req-ok");
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn blocking_client_respects_retry_after_on_http_429() {
        let server = MockServer::start().await;
        let attempts = Arc::new(AtomicUsize::new(0));

        Mock::given(method("POST"))
            .and(path("/"))
            .respond_with(super::TwoStepResponder {
                attempts: attempts.clone(),
                first: ResponseTemplate::new(429)
                    .insert_header("Retry-After", "0")
                    .set_body_string("rate limited"),
                second: ResponseTemplate::new(200).set_body_json(json!({
                    "Response": {
                        "TotalCount": 0,
                        "InstanceSet": [],
                        "RequestId": "req-429-ok"
                    }
                })),
            })
            .mount(&server)
            .await;

        let client = BlockingClient::builder(server.uri())
            .expect("build client builder")
            .endpoint_mode(EndpointMode::FixedHost)
            .auth(Auth::tc3("secret_id", "secret_key"))
            .default_region("ap-guangzhou")
            .no_system_proxy(true)
            .retry(1, Duration::from_millis(0))
            .build()
            .expect("build client");

        let request = DescribeInstancesRequest::new().limit(1);
        let resp = tokio::task::spawn_blocking(move || client.cvm().describe_instances(&request))
            .await
            .expect("join blocking task")
            .expect("eventual success");

        assert_eq!(attempts.load(Ordering::SeqCst), 2);
        assert_eq!(resp.response.request_id.as_str(), "req-429-ok");
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn blocking_client_respects_retry_after_http_date_on_http_429() {
        let server = MockServer::start().await;
        let attempts = Arc::new(AtomicUsize::new(0));
        let retry_at = fmt_http_date(SystemTime::now());

        Mock::given(method("POST"))
            .and(path("/"))
            .respond_with(super::TwoStepResponder {
                attempts: attempts.clone(),
                first: ResponseTemplate::new(429)
                    .insert_header("Retry-After", retry_at)
                    .set_body_string("rate limited"),
                second: ResponseTemplate::new(200).set_body_json(json!({
                    "Response": {
                        "TotalCount": 0,
                        "InstanceSet": [],
                        "RequestId": "req-429-date-ok"
                    }
                })),
            })
            .mount(&server)
            .await;

        let client = BlockingClient::builder(server.uri())
            .expect("build client builder")
            .endpoint_mode(EndpointMode::FixedHost)
            .auth(Auth::tc3("secret_id", "secret_key"))
            .default_region("ap-guangzhou")
            .no_system_proxy(true)
            .retry(1, Duration::from_millis(0))
            .build()
            .expect("build client");

        let request = DescribeInstancesRequest::new().limit(1);
        let resp = tokio::task::spawn_blocking(move || client.cvm().describe_instances(&request))
            .await
            .expect("join blocking task")
            .expect("eventual success");

        assert_eq!(attempts.load(Ordering::SeqCst), 2);
        assert_eq!(resp.response.request_id.as_str(), "req-429-date-ok");
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn blocking_client_does_not_retry_non_idempotent_post_without_idempotency_key() {
        let server = MockServer::start().await;
        let attempts = Arc::new(AtomicUsize::new(0));

        Mock::given(method("POST"))
            .and(path("/"))
            .and(header("X-TC-Action", "RunInstances"))
            .respond_with(super::TwoStepResponder {
                attempts: attempts.clone(),
                first: ResponseTemplate::new(429)
                    .insert_header("Retry-After", "0")
                    .set_body_string("rate limited"),
                second: ResponseTemplate::new(200).set_body_json(json!({
                    "Response": {
                        "InstanceIdSet": [],
                        "RequestId": "req-run-ok"
                    }
                })),
            })
            .mount(&server)
            .await;

        let client = BlockingClient::builder(server.uri())
            .expect("build client builder")
            .endpoint_mode(EndpointMode::FixedHost)
            .auth(Auth::tc3("secret_id", "secret_key"))
            .default_region("ap-guangzhou")
            .no_system_proxy(true)
            .retry(1, Duration::from_millis(0))
            .build()
            .expect("build client");

        let request = RunInstancesRequest::new("ap-guangzhou", "img-123", "S4.SMALL1");
        let err = tokio::task::spawn_blocking(move || client.cvm().run_instances(&request))
            .await
            .expect("join blocking task")
            .expect_err("non-idempotent POST should not retry");

        assert_eq!(attempts.load(Ordering::SeqCst), 1);
        assert_eq!(err.kind(), ErrorKind::RateLimited);
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn blocking_client_retries_non_idempotent_post_with_idempotency_key() {
        let server = MockServer::start().await;
        let attempts = Arc::new(AtomicUsize::new(0));

        Mock::given(method("POST"))
            .and(path("/"))
            .and(header("X-TC-Action", "RunInstances"))
            .and(header("Idempotency-Key", "key-1"))
            .respond_with(super::TwoStepResponder {
                attempts: attempts.clone(),
                first: ResponseTemplate::new(429)
                    .insert_header("Retry-After", "0")
                    .set_body_string("rate limited"),
                second: ResponseTemplate::new(200).set_body_json(json!({
                    "Response": {
                        "InstanceIdSet": [],
                        "RequestId": "req-run-ok"
                    }
                })),
            })
            .mount(&server)
            .await;

        let client = BlockingClient::builder(server.uri())
            .expect("build client builder")
            .endpoint_mode(EndpointMode::FixedHost)
            .auth(Auth::tc3("secret_id", "secret_key"))
            .default_region("ap-guangzhou")
            .no_system_proxy(true)
            .retry(1, Duration::from_millis(0))
            .build()
            .expect("build client");

        let request = RunInstancesRequest::new("ap-guangzhou", "img-123", "S4.SMALL1");
        let options = RequestOptions::new().idempotency_key("key-1");

        let resp = tokio::task::spawn_blocking(move || {
            client.cvm().run_instances_with_options(&request, &options)
        })
        .await
        .expect("join blocking task")
        .expect("eventual success");

        assert_eq!(attempts.load(Ordering::SeqCst), 2);
        assert_eq!(resp.response.request_id.as_str(), "req-run-ok");
    }
}
