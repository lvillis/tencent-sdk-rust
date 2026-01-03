use crate::Error;
use percent_encoding::{AsciiSet, NON_ALPHANUMERIC, percent_encode};
use std::cmp::Ordering;
use url::Url;

const QUERY_ENCODE_SET: &AsciiSet = &NON_ALPHANUMERIC
    .remove(b'-')
    .remove(b'_')
    .remove(b'.')
    .remove(b'~');

pub(crate) fn canonical_query_string(pairs: &[(String, String)]) -> String {
    let mut items: Vec<(String, String)> = pairs.to_vec();

    items.sort_by(|a, b| match a.0.cmp(&b.0) {
        Ordering::Equal => a.1.cmp(&b.1),
        other => other,
    });

    let mut buf = String::new();
    for (idx, (key, value)) in items.iter().enumerate() {
        if idx > 0 {
            buf.push('&');
        }
        buf.push_str(&encode_query_component(key));
        buf.push('=');
        buf.push_str(&encode_query_component(value));
    }
    buf
}

fn encode_query_component(component: &str) -> String {
    percent_encode(component.as_bytes(), QUERY_ENCODE_SET).to_string()
}

pub(crate) fn build_url(
    scheme: &str,
    host: &str,
    path_segments: &[&str],
    canonical_query: &str,
) -> Result<Url, Error> {
    let base = format!("{scheme}://{host}");
    let mut url = Url::parse(&base)
        .map_err(|source| Error::invalid_request_with_source("invalid request url", source))?;

    if path_segments.is_empty() {
        url.set_path("/");
    } else {
        let mut segments = url.path_segments_mut().map_err(|_| {
            let source = std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "request url must be hierarchical",
            );
            Error::invalid_request_with_source("invalid request url", source)
        })?;
        segments.clear();
        for segment in path_segments {
            segments.push(segment);
        }
    }

    if canonical_query.is_empty() {
        url.set_query(None);
    } else {
        url.set_query(Some(canonical_query));
    }

    Ok(url)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_query_string_sorts_and_encodes() {
        let query = canonical_query_string(&[
            ("b".to_string(), "1".to_string()),
            ("a".to_string(), "z".to_string()),
            ("a".to_string(), "a b".to_string()),
        ]);

        assert_eq!(query, "a=a%20b&a=z&b=1");
    }

    #[test]
    fn build_url_encodes_path_segments_and_sets_query() {
        let query = canonical_query_string(&[("k".to_string(), "v".to_string())]);
        let url = build_url("https", "example.com", &["a/b", "c"], &query).unwrap();

        assert_eq!(url.path(), "/a%2Fb/c");
        assert_eq!(url.query(), Some("k=v"));

        let root = build_url("https", "example.com", &[], "").unwrap();
        assert_eq!(root.as_str(), "https://example.com/");
    }
}
