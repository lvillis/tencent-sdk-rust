use serde_json::{json, Value};
use std::thread;
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

#[cfg_attr(
    target_os = "windows",
    ignore = "tokio runtime drop restriction on Windows"
)]
#[test]
fn blocking_client_handles_success_response() {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("build test runtime");

    rt.block_on(async {
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

        let client = TencentCloudBlocking::builder("secret", "key")
            .no_system_proxy()
            .build()
            .expect("build blocking client");
        let endpoint = MockDescribeEndpoint::new(&server, json!({"Hello": "World"}));

        let response = thread::spawn(move || client.request(&endpoint))
            .join()
            .expect("join blocking thread")
            .expect("blocking request succeeds");

        assert_eq!(response["Response"]["Message"], "blocking");
    });
}
