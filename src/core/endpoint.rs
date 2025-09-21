use crate::core::TencentCloudError;
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::borrow::Cow;

pub trait Endpoint {
    type Output: DeserializeOwned + Send + Sync + 'static;

    fn service(&self) -> Cow<'static, str>;
    fn action(&self) -> Cow<'static, str>;
    fn version(&self) -> Cow<'static, str>;

    fn region(&self) -> Option<Cow<'_, str>> {
        None
    }

    fn scheme(&self) -> Cow<'static, str> {
        Cow::Borrowed("https")
    }

    fn host(&self) -> Cow<'_, str> {
        let service = self.service();
        Cow::Owned(format!("{}.tencentcloudapi.com", service))
    }

    fn path(&self) -> Cow<'_, str> {
        Cow::Borrowed("/")
    }

    fn payload(&self) -> Value {
        Value::Object(Default::default())
    }

    fn extra_headers(&self) -> Option<Vec<(Cow<'_, str>, Cow<'_, str>)>> {
        None
    }

    fn parse(&self, body: Value) -> Result<Self::Output, TencentCloudError> {
        Ok(serde_json::from_value(body)?)
    }
}
