use serde::Serialize;
use std::borrow::Cow;

/// General-purpose Tencent Cloud filter structure.
///
/// The platform expects each filter to carry a name and a list of values.
#[derive(Debug, Clone, Serialize)]
pub struct Filter<'a> {
    #[serde(rename = "Name")]
    pub name: Cow<'a, str>,
    #[serde(rename = "Values")]
    pub values: Vec<Cow<'a, str>>,
}

impl<'a> Filter<'a> {
    /// Construct a filter from a name and associated values.
    pub fn new<N, V, I>(name: N, values: I) -> Self
    where
        N: Into<Cow<'a, str>>,
        V: Into<Cow<'a, str>>,
        I: IntoIterator<Item = V>,
    {
        Self {
            name: name.into(),
            values: values.into_iter().map(Into::into).collect(),
        }
    }
}

/// Simple key/value tag structure reused across services.
#[derive(Debug, Clone, Serialize)]
pub struct Tag<'a> {
    #[serde(rename = "Key")]
    pub key: Cow<'a, str>,
    #[serde(rename = "Value")]
    pub value: Cow<'a, str>,
}

impl<'a> Tag<'a> {
    /// Construct a new tag.
    pub fn new<K, V>(key: K, value: V) -> Self
    where
        K: Into<Cow<'a, str>>,
        V: Into<Cow<'a, str>>,
    {
        Self {
            key: key.into(),
            value: value.into(),
        }
    }
}
