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
    pub kind: ServiceErrorKind,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ServiceErrorKind {
    Auth,
    Throttled,
    Forbidden,
    NotFound,
    Validation,
    Internal,
    Unknown,
}

impl fmt::Display for ServiceErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ServiceErrorKind::Auth => write!(f, "auth"),
            ServiceErrorKind::Throttled => write!(f, "throttled"),
            ServiceErrorKind::Forbidden => write!(f, "forbidden"),
            ServiceErrorKind::NotFound => write!(f, "not_found"),
            ServiceErrorKind::Validation => write!(f, "validation"),
            ServiceErrorKind::Internal => write!(f, "internal"),
            ServiceErrorKind::Unknown => write!(f, "unknown"),
        }
    }
}

impl ServiceErrorKind {
    /// Returns true if the error usually merits retrying the request.
    pub fn is_retryable(self) -> bool {
        matches!(
            self,
            ServiceErrorKind::Throttled | ServiceErrorKind::Internal | ServiceErrorKind::Unknown
        )
    }
}

impl ServiceFailure {
    pub fn kind(&self) -> ServiceErrorKind {
        self.kind
    }
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
        "Tencent Cloud service error ({kind}) {code}: {message}{request_id}",
        kind = context.kind,
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
        let code_owned = code.into();
        Self::Service {
            context: Box::new(ServiceFailure {
                kind: classify_service_error(&code_owned),
                code: code_owned,
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

fn classify_service_error(code: &str) -> ServiceErrorKind {
    if code.starts_with("AuthFailure")
        || code.starts_with("InvalidCredential")
        || code.starts_with("UnauthorizedOperation")
    {
        ServiceErrorKind::Auth
    } else if code.starts_with("LimitExceeded")
        || code.starts_with("RequestLimitExceeded")
        || code.starts_with("Throttling")
    {
        ServiceErrorKind::Throttled
    } else if code.starts_with("OperationDenied") || code.starts_with("Forbidden") {
        ServiceErrorKind::Forbidden
    } else if code.starts_with("ResourceNotFound") {
        ServiceErrorKind::NotFound
    } else if code.starts_with("InvalidParameter") || code.starts_with("MissingParameter") {
        ServiceErrorKind::Validation
    } else if code.starts_with("InternalError") {
        ServiceErrorKind::Internal
    } else {
        ServiceErrorKind::Unknown
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classifies_auth_errors() {
        let err = TencentCloudError::service("AuthFailure.SecretIdNotFound", "missing id", None);
        match err {
            TencentCloudError::Service { context } => {
                assert_eq!(context.kind(), ServiceErrorKind::Auth);
                assert_eq!(context.code, "AuthFailure.SecretIdNotFound");
                assert!(!context.kind().is_retryable());
            }
            _ => panic!("expected service error"),
        }
    }

    #[test]
    fn classifies_throttled_errors() {
        let err = TencentCloudError::service(
            "RequestLimitExceeded",
            "too many calls",
            Some("req".into()),
        );
        match err {
            TencentCloudError::Service { context } => {
                assert_eq!(context.kind(), ServiceErrorKind::Throttled);
                assert!(context.kind().is_retryable());
                assert_eq!(context.request_id.as_deref(), Some("req"));
            }
            _ => panic!("expected service error"),
        }
    }

    #[test]
    fn classifies_unknown_errors() {
        let err = TencentCloudError::service("FooBar", "???", None);
        match err {
            TencentCloudError::Service { context } => {
                assert_eq!(context.kind(), ServiceErrorKind::Unknown);
                assert!(context.kind().is_retryable());
            }
            _ => panic!("expected service error"),
        }
    }
}
