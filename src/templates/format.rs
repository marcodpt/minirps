use minijinja::Value;
use serde_json::to_string_pretty;

pub fn format (value: &Value) -> Vec<u8> {
    if let Some(data) = value.as_bytes() {
        data.to_vec()
    } else if let Ok(data) = to_string_pretty(value.into()) {
        data.as_bytes().to_vec()
    } else {
        format!("{:#?}", value).as_bytes().to_vec()
    }
}
