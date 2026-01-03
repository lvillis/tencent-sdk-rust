use crate::Error;
use crate::types::Region;
use http::{HeaderMap, Method};
use serde::de::DeserializeOwned;
use serde_json::Value;

pub(crate) trait Endpoint {
    type Output: DeserializeOwned + Send + Sync + 'static;

    fn method(&self) -> Method {
        Method::POST
    }

    fn is_idempotent(&self) -> bool {
        match self.method() {
            Method::GET | Method::HEAD | Method::PUT | Method::DELETE => true,
            Method::POST => {
                let action = self.action();
                action.starts_with("Describe")
                    || action.starts_with("Get")
                    || action.starts_with("List")
                    || action.starts_with("Inquiry")
            }
            _ => false,
        }
    }

    fn service(&self) -> &'static str;
    fn action(&self) -> &'static str;
    fn version(&self) -> &'static str;

    fn region(&self) -> Option<&Region> {
        None
    }

    fn path_segments(&self) -> &'static [&'static str] {
        &[]
    }

    fn query(&self) -> Vec<(String, String)> {
        Vec::new()
    }

    fn payload(&self) -> Result<Option<Value>, Error> {
        Ok(Some(Value::Object(Default::default())))
    }

    fn extra_headers(&self) -> Result<HeaderMap, Error> {
        Ok(HeaderMap::new())
    }
}
