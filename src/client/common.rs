use serde_json::Value;

pub(crate) fn tencent_error_from_value(value: &Value) -> Option<(String, String, Option<String>)> {
    let response = value.get("Response")?;
    let error = response.get("Error")?;
    let code = error.get("Code")?.as_str()?.to_string();
    let message = error.get("Message")?.as_str()?.to_string();
    let request_id = response
        .get("RequestId")
        .and_then(|id| id.as_str().map(|s| s.to_string()));
    Some((code, message, request_id))
}

pub(crate) fn tencent_request_id_from_value(value: &Value) -> Option<String> {
    value
        .get("Response")?
        .get("RequestId")?
        .as_str()
        .map(|id| id.to_string())
}
