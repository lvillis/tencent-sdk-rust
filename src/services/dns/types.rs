use serde::Serialize;

/// Supported DNSPod record types (defaults to TXT when used in builders).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RecordType<'a> {
    Txt,
    A,
    Cname,
    Custom(&'a str),
}

impl<'a> RecordType<'a> {
    pub fn as_str(&self) -> &str {
        match self {
            RecordType::Txt => "TXT",
            RecordType::A => "A",
            RecordType::Cname => "CNAME",
            RecordType::Custom(value) => value,
        }
    }
}

impl<'a> From<&'a str> for RecordType<'a> {
    fn from(value: &'a str) -> Self {
        match value.to_uppercase().as_str() {
            "TXT" => RecordType::Txt,
            "A" => RecordType::A,
            "CNAME" => RecordType::Cname,
            _ => RecordType::Custom(value),
        }
    }
}

impl<'a> Serialize for RecordType<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

/// DNSPod record line values.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RecordLine<'a> {
    Default,
    Custom(&'a str),
}

impl<'a> RecordLine<'a> {
    pub fn as_str(&self) -> &str {
        match self {
            RecordLine::Default => "默认",
            RecordLine::Custom(value) => value,
        }
    }
}

impl<'a> From<&'a str> for RecordLine<'a> {
    fn from(value: &'a str) -> Self {
        match value {
            "默认" => RecordLine::Default,
            other if other.eq_ignore_ascii_case("default") => RecordLine::Default,
            other => RecordLine::Custom(other),
        }
    }
}

impl<'a> Serialize for RecordLine<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
