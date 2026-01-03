use serde_json::Value;

const REDACTED: &str = "[redacted]";

pub(crate) fn body_snippet(body: &str, max_bytes: usize) -> String {
    let body = redact_body(body);
    truncate_to_bytes(&body, max_bytes)
}

fn redact_body(body: &str) -> String {
    let Ok(mut value) = serde_json::from_str::<Value>(body) else {
        return body.to_string();
    };

    redact_json_value(&mut value);
    serde_json::to_string(&value).unwrap_or_else(|_| body.to_string())
}

fn redact_json_value(value: &mut Value) {
    match value {
        Value::Object(map) => {
            for (key, value) in map.iter_mut() {
                if is_sensitive_key(key) {
                    *value = Value::String(REDACTED.to_string());
                } else {
                    redact_json_value(value);
                }
            }
        }
        Value::Array(values) => {
            for value in values {
                redact_json_value(value);
            }
        }
        _ => {}
    }
}

fn is_sensitive_key(key: &str) -> bool {
    let normalized: String = key
        .chars()
        .filter(|ch| ch.is_ascii_alphanumeric())
        .map(|ch| ch.to_ascii_lowercase())
        .collect();

    matches!(
        normalized.as_str(),
        "authorization"
            | "cookie"
            | "token"
            | "accesstoken"
            | "idtoken"
            | "refreshtoken"
            | "secret"
            | "secretid"
            | "secretkey"
            | "apikey"
            | "privatekey"
            | "password"
            | "passwd"
            | "signature"
    ) || normalized.contains("secret")
        || normalized.contains("password")
        || normalized.ends_with("token")
        || normalized.ends_with("privatekey")
}

fn truncate_to_bytes(value: &str, max_bytes: usize) -> String {
    let bytes = value.as_bytes();
    if bytes.len() <= max_bytes {
        return value.to_string();
    }

    let truncated = &bytes[..max_bytes];
    let mut snippet = String::from_utf8_lossy(truncated).to_string();
    snippet.push_str("...");
    snippet
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn body_snippet_redacts_sensitive_keys_in_json() {
        let input = r#"{"Authorization":"Bearer abc","nested":{"Token":"t"},"arr":[{"password":"p"}],"ok":"v","SecretKey":"s"}"#;
        let snippet = body_snippet(input, 4096);

        assert!(snippet.contains("\"Authorization\":\"[redacted]\""));
        assert!(snippet.contains("\"Token\":\"[redacted]\""));
        assert!(snippet.contains("\"password\":\"[redacted]\""));
        assert!(snippet.contains("\"SecretKey\":\"[redacted]\""));
        assert!(snippet.contains("\"ok\":\"v\""));
        assert!(!snippet.contains("Bearer abc"));
        assert!(!snippet.contains("\"t\""));
        assert!(!snippet.contains("\"p\""));
        assert!(!snippet.contains("\"s\""));
    }

    #[test]
    fn body_snippet_truncates_non_json_payloads() {
        let snippet = body_snippet("0123456789", 5);
        assert_eq!(snippet, "01234...");
    }
}
