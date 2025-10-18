use serde_json::{json, Value};
use tencent_sdk::{
    client::{TencentCloudAsync, TencentCloudBlocking},
    core::{Endpoint, TencentCloudError},
};
use url::Url;
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

struct MockDescribeEndpoint {
    host: String,
    payload: Value,
}

impl MockDescribeEndpoint {
    fn new(server: &MockServer, payload: Value) -> Self {
        let url = Url::parse(&server.uri()).expect("mock server url");
        let host = match url.port() {
            Some(port) => format!("{}:{}", url.host_str().expect("host"), port),
            None => url.host_str().expect("host").to_string(),
        };
        Self { host, payload }
    }
}

impl Endpoint for MockDescribeEndpoint {
    type Output = Value;

    fn scheme(&self) -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed("http")
    }

    fn service(&self) -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed("test")
    }

    fn action(&self) -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed("DescribeMock")
    }

    fn version(&self) -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed("2020-01-01")
    }

    fn host(&self) -> std::borrow::Cow<'_, str> {
        std::borrow::Cow::Borrowed(&self.host)
    }

    fn payload(&self) -> Value {
        self.payload.clone()
    }

    fn parse(&self, body: Value) -> Result<Self::Output, TencentCloudError> {
        Ok(body)
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn async_client_handles_success_response() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/"))
        .and(header("X-TC-Action", "DescribeMock"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "Response": {
                "Message": "ok"
            }
        })))
        .mount(&server)
        .await;

    let client = TencentCloudAsync::builder("secret", "key")
        .expect("build async client builder")
        .no_system_proxy()
        .build()
        .expect("build async client");

    let endpoint = MockDescribeEndpoint::new(&server, json!({"Ping": "Pong"}));
    let response = client
        .request(&endpoint)
        .await
        .expect("async request succeeds");

    assert_eq!(response["Response"]["Message"], "ok");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn async_client_surfaces_error_status() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/"))
        .respond_with(ResponseTemplate::new(500).set_body_string("boom"))
        .mount(&server)
        .await;

    let client = TencentCloudAsync::builder("secret", "key")
        .expect("build async client builder")
        .no_system_proxy()
        .build()
        .expect("build async client");

    let endpoint = MockDescribeEndpoint::new(&server, json!({}));
    let err = client.request(&endpoint).await.expect_err("expect error");

    match err {
        TencentCloudError::Http { context } => {
            assert_eq!(context.code.as_u16(), 500);
            assert_eq!(context.body, "boom");
        }
        other => panic!("unexpected error: {other:?}"),
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn async_client_reports_service_error() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "Response": {
                "Error": {
                    "Code": "AuthFailure",
                    "Message": "signature mismatch"
                },
                "RequestId": "req-123"
            }
        })))
        .mount(&server)
        .await;

    let client = TencentCloudAsync::builder("secret", "key")
        .expect("build async client builder")
        .no_system_proxy()
        .build()
        .expect("build async client");

    let endpoint = MockDescribeEndpoint::new(&server, json!({}));
    let err = client
        .request(&endpoint)
        .await
        .expect_err("expect service error");

    match err {
        TencentCloudError::Service { context } => {
            assert_eq!(context.code, "AuthFailure");
            assert_eq!(context.message, "signature mismatch");
            assert_eq!(context.request_id.as_deref(), Some("req-123"));
        }
        other => panic!("unexpected error: {other:?}"),
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn blocking_client_handles_success_response() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/"))
        .and(header("X-TC-Action", "DescribeMock"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "Response": {
                "Message": "blocking"
            }
        })))
        .mount(&server)
        .await;

    let endpoint = MockDescribeEndpoint::new(&server, json!({"Hello": "World"}));

    let response = tokio::task::spawn_blocking(move || {
        let client = TencentCloudBlocking::builder("secret", "key")
            .expect("build blocking client builder")
            .no_system_proxy()
            .build()
            .expect("build blocking client");

        client.request(&endpoint)
    })
    .await
    .expect("join blocking thread")
    .expect("blocking request succeeds");

    assert_eq!(response["Response"]["Message"], "blocking");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn blocking_client_reports_service_error() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "Response": {
                "Error": {
                    "Code": "OperationDenied",
                    "Message": "capacity exceeded"
                },
                "RequestId": "req-456"
            }
        })))
        .mount(&server)
        .await;

    let endpoint = MockDescribeEndpoint::new(&server, json!({}));
    let err = tokio::task::spawn_blocking(move || {
        let client = TencentCloudBlocking::builder("secret", "key")
            .expect("build blocking client builder")
            .no_system_proxy()
            .build()
            .expect("build blocking client");
        client.request(&endpoint)
    })
    .await
    .expect("join blocking thread")
    .expect_err("expect blocking service error");

    match err {
        TencentCloudError::Service { context } => {
            assert_eq!(context.code, "OperationDenied");
            assert_eq!(context.message, "capacity exceeded");
            assert_eq!(context.request_id.as_deref(), Some("req-456"));
        }
        other => panic!("unexpected error: {other:?}"),
    }
}
