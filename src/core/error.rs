use http::{Method, StatusCode};
use thiserror::Error;
use url::{ParseError, Url};

use crate::signing::SigningError;

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

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum TencentCloudError {
    #[error("HTTP {code} ({method} {url}): {body}", code = context.code, method = context.method, url = context.url, body = context.body)]
    Http { context: Box<HttpFailure> },

    #[error("Transport error during {method} {url}: {source}", method = context.method, url = context.url, source = context.source)]
    Transport { context: Box<TransportFailure> },

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
}
