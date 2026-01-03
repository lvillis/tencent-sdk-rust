use serde::Serialize;

/// General-purpose Tencent Cloud filter structure.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Filter {
    pub name: String,
    pub values: Vec<String>,
}

impl Filter {
    pub fn new<N, V, I>(name: N, values: I) -> Self
    where
        N: Into<String>,
        V: Into<String>,
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
#[serde(rename_all = "PascalCase")]
pub struct Tag {
    pub key: String,
    pub value: String,
}

impl Tag {
    pub fn new<K, V>(key: K, value: V) -> Self
    where
        K: Into<String>,
        V: Into<String>,
    {
        Self {
            key: key.into(),
            value: value.into(),
        }
    }
}
