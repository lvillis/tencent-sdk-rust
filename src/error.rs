use http::{HeaderMap, Method, StatusCode};
use std::{error::Error as StdError, fmt, time::Duration};

pub type Result<T> = std::result::Result<T, Error>;

pub(crate) type BoxError = Box<dyn StdError + Send + Sync + 'static>;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[non_exhaustive]
pub enum ErrorKind {
    InvalidConfig,
    Transport,
    Decode,
    Auth,
    NotFound,
    Conflict,
    RateLimited,
    Api,
}

#[non_exhaustive]
pub struct InvalidConfigError {
    message: String,
    base_url: Option<String>,
    source: Option<BoxError>,
}

impl InvalidConfigError {
    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn base_url(&self) -> Option<&str> {
        self.base_url.as_deref()
    }
}

impl fmt::Debug for InvalidConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("InvalidConfigError")
            .field("message", &self.message)
            .field("base_url", &self.base_url)
            .field("has_source", &self.source.is_some())
            .finish()
    }
}

#[non_exhaustive]
pub struct TransportError {
    method: Method,
    host: String,
    path: String,
    source: BoxError,
}

impl TransportError {
    pub fn method(&self) -> &Method {
        &self.method
    }

    pub fn host(&self) -> &str {
        &self.host
    }

    pub fn path(&self) -> &str {
        &self.path
    }
}

impl fmt::Debug for TransportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TransportError")
            .field("method", &self.method)
            .field("host", &self.host)
            .field("path", &self.path)
            .field("has_source", &true)
            .finish()
    }
}

#[non_exhaustive]
pub struct DecodeError {
    status: Option<StatusCode>,
    method: Method,
    host: String,
    path: String,
    request_id: Option<String>,
    body_snippet: Option<String>,
    source: BoxError,
}

impl DecodeError {
    pub fn status(&self) -> Option<StatusCode> {
        self.status
    }

    pub fn method(&self) -> &Method {
        &self.method
    }

    pub fn host(&self) -> &str {
        &self.host
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn request_id(&self) -> Option<&str> {
        self.request_id.as_deref()
    }

    pub fn body_snippet(&self) -> Option<&str> {
        self.body_snippet.as_deref()
    }
}

impl fmt::Debug for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DecodeError")
            .field("status", &self.status)
            .field("method", &self.method)
            .field("host", &self.host)
            .field("path", &self.path)
            .field("request_id", &self.request_id)
            .field("body_snippet", &self.body_snippet)
            .field("has_source", &true)
            .finish()
    }
}

#[non_exhaustive]
pub struct ApiError {
    status: Option<StatusCode>,
    method: Option<Method>,
    host: Option<String>,
    path: Option<String>,
    code: Option<String>,
    message: Option<String>,
    request_id: Option<String>,
    body_snippet: Option<String>,
}

impl ApiError {
    pub fn status(&self) -> Option<StatusCode> {
        self.status
    }

    pub fn method(&self) -> Option<&Method> {
        self.method.as_ref()
    }

    pub fn host(&self) -> Option<&str> {
        self.host.as_deref()
    }

    pub fn path(&self) -> Option<&str> {
        self.path.as_deref()
    }

    pub fn code(&self) -> Option<&str> {
        self.code.as_deref()
    }

    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }

    pub fn request_id(&self) -> Option<&str> {
        self.request_id.as_deref()
    }

    pub fn body_snippet(&self) -> Option<&str> {
        self.body_snippet.as_deref()
    }
}

impl fmt::Debug for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ApiError")
            .field("status", &self.status)
            .field("method", &self.method)
            .field("host", &self.host)
            .field("path", &self.path)
            .field("code", &self.code)
            .field("message", &self.message)
            .field("request_id", &self.request_id)
            .field("body_snippet", &self.body_snippet)
            .finish()
    }
}

#[non_exhaustive]
pub struct RateLimitedError {
    api: ApiError,
    retry_after: Option<Duration>,
}

impl RateLimitedError {
    pub fn api(&self) -> &ApiError {
        &self.api
    }

    pub fn retry_after(&self) -> Option<Duration> {
        self.retry_after
    }
}

impl fmt::Debug for RateLimitedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RateLimitedError")
            .field("api", &self.api)
            .field("retry_after", &self.retry_after)
            .finish()
    }
}

#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    InvalidConfig(Box<InvalidConfigError>),
    Transport(Box<TransportError>),
    Decode(Box<DecodeError>),
    Auth(Box<ApiError>),
    NotFound(Box<ApiError>),
    Conflict(Box<ApiError>),
    RateLimited(Box<RateLimitedError>),
    Api(Box<ApiError>),
}

impl Error {
    pub fn kind(&self) -> ErrorKind {
        match self {
            Error::InvalidConfig(_) => ErrorKind::InvalidConfig,
            Error::Transport(_) => ErrorKind::Transport,
            Error::Decode(_) => ErrorKind::Decode,
            Error::Auth(_) => ErrorKind::Auth,
            Error::NotFound(_) => ErrorKind::NotFound,
            Error::Conflict(_) => ErrorKind::Conflict,
            Error::RateLimited(_) => ErrorKind::RateLimited,
            Error::Api(_) => ErrorKind::Api,
        }
    }

    pub fn status(&self) -> Option<StatusCode> {
        match self {
            Error::InvalidConfig(_) => None,
            Error::Transport(_) => None,
            Error::Decode(err) => err.status,
            Error::Auth(err) => err.status,
            Error::NotFound(err) => err.status,
            Error::Conflict(err) => err.status,
            Error::RateLimited(err) => err.api.status,
            Error::Api(err) => err.status,
        }
    }

    pub fn method(&self) -> Option<&Method> {
        match self {
            Error::InvalidConfig(_) => None,
            Error::Transport(err) => Some(&err.method),
            Error::Decode(err) => Some(&err.method),
            Error::Auth(err) => err.method.as_ref(),
            Error::NotFound(err) => err.method.as_ref(),
            Error::Conflict(err) => err.method.as_ref(),
            Error::RateLimited(err) => err.api.method.as_ref(),
            Error::Api(err) => err.method.as_ref(),
        }
    }

    pub fn host(&self) -> Option<&str> {
        match self {
            Error::InvalidConfig(_) => None,
            Error::Transport(err) => Some(&err.host),
            Error::Decode(err) => Some(&err.host),
            Error::Auth(err) => err.host.as_deref(),
            Error::NotFound(err) => err.host.as_deref(),
            Error::Conflict(err) => err.host.as_deref(),
            Error::RateLimited(err) => err.api.host.as_deref(),
            Error::Api(err) => err.host.as_deref(),
        }
    }

    pub fn path(&self) -> Option<&str> {
        match self {
            Error::InvalidConfig(_) => None,
            Error::Transport(err) => Some(&err.path),
            Error::Decode(err) => Some(&err.path),
            Error::Auth(err) => err.path.as_deref(),
            Error::NotFound(err) => err.path.as_deref(),
            Error::Conflict(err) => err.path.as_deref(),
            Error::RateLimited(err) => err.api.path.as_deref(),
            Error::Api(err) => err.path.as_deref(),
        }
    }

    pub fn message(&self) -> Option<&str> {
        match self {
            Error::InvalidConfig(err) => Some(err.message.as_str()),
            Error::Transport(_) => None,
            Error::Decode(_) => None,
            Error::Auth(err) => err.message.as_deref(),
            Error::NotFound(err) => err.message.as_deref(),
            Error::Conflict(err) => err.message.as_deref(),
            Error::RateLimited(err) => err.api.message.as_deref(),
            Error::Api(err) => err.message.as_deref(),
        }
    }

    pub fn request_id(&self) -> Option<&str> {
        match self {
            Error::InvalidConfig(_) => None,
            Error::Transport(_) => None,
            Error::Decode(err) => err.request_id.as_deref(),
            Error::Auth(err) => err.request_id.as_deref(),
            Error::NotFound(err) => err.request_id.as_deref(),
            Error::Conflict(err) => err.request_id.as_deref(),
            Error::RateLimited(err) => err.api.request_id.as_deref(),
            Error::Api(err) => err.request_id.as_deref(),
        }
    }

    pub fn code(&self) -> Option<&str> {
        match self {
            Error::InvalidConfig(_) => None,
            Error::Transport(_) => None,
            Error::Decode(_) => None,
            Error::Auth(err) => err.code.as_deref(),
            Error::NotFound(err) => err.code.as_deref(),
            Error::Conflict(err) => err.code.as_deref(),
            Error::RateLimited(err) => err.api.code.as_deref(),
            Error::Api(err) => err.code.as_deref(),
        }
    }

    pub fn body_snippet(&self) -> Option<&str> {
        match self {
            Error::InvalidConfig(_) => None,
            Error::Transport(_) => None,
            Error::Decode(err) => err.body_snippet.as_deref(),
            Error::Auth(err) => err.body_snippet.as_deref(),
            Error::NotFound(err) => err.body_snippet.as_deref(),
            Error::Conflict(err) => err.body_snippet.as_deref(),
            Error::RateLimited(err) => err.api.body_snippet.as_deref(),
            Error::Api(err) => err.body_snippet.as_deref(),
        }
    }

    pub fn retry_after(&self) -> Option<Duration> {
        match self {
            Error::RateLimited(err) => err.retry_after,
            _ => None,
        }
    }

    pub fn is_retryable(&self) -> bool {
        match self {
            Error::RateLimited(_) => true,
            Error::Transport(err) => is_retryable_transport_source(err.source.as_ref()),
            Error::Api(err) => err.status.is_some_and(|status| {
                matches!(
                    status,
                    StatusCode::BAD_GATEWAY
                        | StatusCode::SERVICE_UNAVAILABLE
                        | StatusCode::GATEWAY_TIMEOUT
                )
            }),
            _ => false,
        }
    }

    pub(crate) fn invalid_base_url(
        base_url: impl Into<String>,
        source: impl Into<BoxError>,
    ) -> Self {
        Self::InvalidConfig(Box::new(InvalidConfigError {
            message: "invalid base url".to_string(),
            base_url: Some(base_url.into()),
            source: Some(source.into()),
        }))
    }

    pub(crate) fn invalid_request_with_source(
        message: impl Into<String>,
        source: impl Into<BoxError>,
    ) -> Self {
        Self::InvalidConfig(Box::new(InvalidConfigError {
            message: message.into(),
            base_url: None,
            source: Some(source.into()),
        }))
    }

    #[cfg(feature = "async")]
    pub(crate) fn transport_build(source: impl Into<BoxError>) -> Self {
        Self::InvalidConfig(Box::new(InvalidConfigError {
            message: "failed to build HTTP client".to_string(),
            base_url: None,
            source: Some(source.into()),
        }))
    }

    pub(crate) fn transport(
        method: Method,
        host: impl Into<String>,
        path: impl Into<String>,
        source: impl Into<BoxError>,
    ) -> Self {
        Self::Transport(Box::new(TransportError {
            method,
            host: host.into(),
            path: path.into(),
            source: source.into(),
        }))
    }

    pub(crate) fn decode(
        status: Option<StatusCode>,
        method: Method,
        host: impl Into<String>,
        path: impl Into<String>,
        request_id: Option<String>,
        body_snippet: Option<String>,
        source: impl Into<BoxError>,
    ) -> Self {
        Self::Decode(Box::new(DecodeError {
            status,
            method,
            host: host.into(),
            path: path.into(),
            request_id,
            body_snippet,
            source: source.into(),
        }))
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn api(
        status: Option<StatusCode>,
        method: Method,
        host: impl Into<String>,
        path: impl Into<String>,
        code: Option<String>,
        message: Option<String>,
        request_id: Option<String>,
        body_snippet: Option<String>,
        retry_after: Option<Duration>,
    ) -> Self {
        let api = ApiError {
            status,
            method: Some(method),
            host: Some(host.into()),
            path: Some(path.into()),
            code,
            message,
            request_id,
            body_snippet,
        };

        match classify_api_error(&api) {
            ErrorKind::Auth => Self::Auth(Box::new(api)),
            ErrorKind::NotFound => Self::NotFound(Box::new(api)),
            ErrorKind::Conflict => Self::Conflict(Box::new(api)),
            ErrorKind::RateLimited => {
                Self::RateLimited(Box::new(RateLimitedError { api, retry_after }))
            }
            _ => Self::Api(Box::new(api)),
        }
    }

    pub(crate) fn signing(source: impl Into<BoxError>) -> Self {
        Self::InvalidConfig(Box::new(InvalidConfigError {
            message: "signing error".to_string(),
            base_url: None,
            source: Some(source.into()),
        }))
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InvalidConfig(err) => {
                if let Some(base_url) = err.base_url.as_deref() {
                    write!(f, "invalid config (base url `{base_url}`): {}", err.message)
                } else {
                    write!(f, "invalid config: {}", err.message)
                }
            }
            Error::Transport(err) => write!(
                f,
                "transport error ({} {}{})",
                err.method, err.host, err.path
            ),
            Error::Decode(err) => {
                write!(f, "decode error ({} {}{})", err.method, err.host, err.path)?;
                if let Some(request_id) = err.request_id.as_deref() {
                    write!(f, " (request {request_id})")?;
                }
                Ok(())
            }
            Error::Auth(err) => write_api_error(f, "auth error", err),
            Error::NotFound(err) => write_api_error(f, "not found", err),
            Error::Conflict(err) => write_api_error(f, "conflict", err),
            Error::RateLimited(err) => {
                write_api_error(f, "rate limited", &err.api)?;
                if let Some(retry_after) = err.retry_after {
                    write!(f, " (retry after {retry_after:?})")?;
                }
                Ok(())
            }
            Error::Api(err) => write_api_error(f, "api error", err),
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Error::InvalidConfig(err) => err.source.as_deref().map(|source| source as _),
            Error::Transport(err) => Some(err.source.as_ref() as _),
            Error::Decode(err) => Some(err.source.as_ref() as _),
            _ => None,
        }
    }
}

fn write_api_error(f: &mut fmt::Formatter<'_>, label: &str, err: &ApiError) -> fmt::Result {
    let status = err
        .status
        .map_or("<unknown>".to_string(), |status| status.to_string());
    write!(f, "{label} (HTTP {status})")?;

    if let (Some(code), Some(message)) = (err.code.as_deref(), err.message.as_deref()) {
        write!(f, " {code}: {message}")?;
    } else if let Some(message) = err.message.as_deref() {
        write!(f, ": {message}")?;
    }

    if let Some(request_id) = err.request_id.as_deref() {
        write!(f, " (request {request_id})")?;
    }

    Ok(())
}

fn classify_api_error(err: &ApiError) -> ErrorKind {
    if err.status == Some(StatusCode::TOO_MANY_REQUESTS) {
        return ErrorKind::RateLimited;
    }

    if matches!(
        err.status,
        Some(StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN)
    ) {
        return ErrorKind::Auth;
    }

    if err.status == Some(StatusCode::NOT_FOUND) {
        return ErrorKind::NotFound;
    }

    if matches!(
        err.status,
        Some(StatusCode::CONFLICT | StatusCode::PRECONDITION_FAILED)
    ) {
        return ErrorKind::Conflict;
    }

    let Some(code) = err.code.as_deref() else {
        return ErrorKind::Api;
    };

    classify_tencent_service_code(code)
}

fn classify_tencent_service_code(code: &str) -> ErrorKind {
    if code.starts_with("AuthFailure")
        || code.starts_with("InvalidCredential")
        || code.starts_with("UnauthorizedOperation")
        || code.starts_with("OperationDenied")
        || code.starts_with("Forbidden")
    {
        ErrorKind::Auth
    } else if code.starts_with("LimitExceeded")
        || code.starts_with("RequestLimitExceeded")
        || code.starts_with("Throttling")
    {
        ErrorKind::RateLimited
    } else if code.starts_with("ResourceNotFound") {
        ErrorKind::NotFound
    } else if code.starts_with("ResourceInUse") || code.starts_with("ResourceUnavailable") {
        ErrorKind::Conflict
    } else {
        ErrorKind::Api
    }
}

pub(crate) fn request_id_from_headers(headers: &HeaderMap) -> Option<String> {
    for header_name in [
        "x-tc-requestid",
        "x-request-id",
        "x-requestid",
        "x-tc-traceid",
    ] {
        let Some(value) = headers.get(header_name) else {
            continue;
        };
        let Ok(value) = value.to_str() else {
            continue;
        };
        let trimmed = value.trim();
        if !trimmed.is_empty() {
            return Some(trimmed.to_string());
        }
    }
    None
}

fn is_retryable_transport_source(source: &(dyn StdError + 'static)) -> bool {
    #[cfg(feature = "async")]
    if let Some(reqwest_error) = source.downcast_ref::<reqwest::Error>() {
        return reqwest_error.is_timeout() || reqwest_error.is_connect();
    }

    #[cfg(feature = "blocking")]
    if let Some(ureq_error) = source.downcast_ref::<ureq::Error>() {
        return match ureq_error {
            ureq::Error::Timeout(_) => true,
            ureq::Error::HostNotFound => true,
            ureq::Error::ConnectionFailed => true,
            ureq::Error::Tls(_) => true,
            ureq::Error::Io(io) => matches!(
                io.kind(),
                std::io::ErrorKind::ConnectionReset
                    | std::io::ErrorKind::ConnectionAborted
                    | std::io::ErrorKind::ConnectionRefused
                    | std::io::ErrorKind::NotConnected
                    | std::io::ErrorKind::TimedOut
                    | std::io::ErrorKind::UnexpectedEof
            ),
            _ => false,
        };
    }

    false
}
