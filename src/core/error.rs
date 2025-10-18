use http::{Method, StatusCode};
use thiserror::Error;
use url::{ParseError, Url};

use crate::signing::SigningError;
use std::fmt;

pub type TencentCloudResult<T> = Result<T, TencentCloudError>;

#[derive(Debug)]
pub struct HttpFailure {
    pub code: StatusCode,
    pub method: Method,
    pub url: Url,
    pub body: String,
}

#[derive(Debug)]
pub struct TransportFailure {
    pub source: reqwest::Error,
    pub method: Method,
    pub url: Url,
}

#[derive(Debug)]
pub struct ServiceFailure {
    pub code: String,
    pub message: String,
    pub request_id: Option<String>,
}

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum TencentCloudError {
    #[error("HTTP {code} ({method} {url}): {body}", code = context.code, method = context.method, url = context.url, body = context.body)]
    Http { context: Box<HttpFailure> },

    #[error("Transport error during {method} {url}: {source}", method = context.method, url = context.url, source = context.source)]
    Transport { context: Box<TransportFailure> },

    #[error("Transport build error: {source}")]
    TransportBuild {
        #[source]
        source: reqwest::Error,
    },

    #[error(
        "Tencent Cloud service error {code}: {message}{request_id}",
        code = context.code,
        message = context.message,
        request_id = DisplayRequestId(&context.request_id)
    )]
    Service { context: Box<ServiceFailure> },

    #[error(transparent)]
    Url(#[from] ParseError),

    #[error(transparent)]
    Json(#[from] serde_json::Error),

    #[error(transparent)]
    Signing(#[from] SigningError),
}

impl TencentCloudError {
    pub fn http(code: StatusCode, method: Method, url: Url, body: String) -> Self {
        Self::Http {
            context: Box::new(HttpFailure {
                code,
                method,
                url,
                body,
            }),
        }
    }

    pub fn transport(source: reqwest::Error, method: Method, url: Url) -> Self {
        Self::Transport {
            context: Box::new(TransportFailure {
                source,
                method,
                url,
            }),
        }
    }

    pub fn transport_build(source: reqwest::Error) -> Self {
        Self::TransportBuild { source }
    }

    pub fn service(
        code: impl Into<String>,
        message: impl Into<String>,
        request_id: Option<String>,
    ) -> Self {
        Self::Service {
            context: Box::new(ServiceFailure {
                code: code.into(),
                message: message.into(),
                request_id,
            }),
        }
    }
}

struct DisplayRequestId<'a>(&'a Option<String>);

impl fmt::Display for DisplayRequestId<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(value) = self.0 {
            write!(f, " (request {value})")
        } else {
            Ok(())
        }
    }
}
