use serde_json;
use toml;
use std::str::from_utf8;
use minijinja::{Value, value::ValueKind};

pub fn parse (value: &Value) -> Value {
    let mut value = value.clone();

    if value.kind() == ValueKind::Bytes {
        if let Some(data) = value.as_bytes() {
            value = match from_utf8(data) {
                Ok(data) => Value::from_serialize(data),
                Err(_) => value
            };
        }
    }

    if value.kind() == ValueKind::String {
        if let Some(data) = value.as_str() {
            value = match serde_json::from_str::<Value>(data) 
                .or(toml::from_str::<Value>(data)) {
                    Ok(data) => data,
                    Err(_) => value
                };
        }
    }

    value
}
