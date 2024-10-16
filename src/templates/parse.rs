use serde_json;
use toml;
use std::str::from_utf8;
use minijinja::{Error, ErrorKind::InvalidOperation, Value};

pub fn parse (data: Vec<u8>) -> Result<Value, Error> {
    let text = match from_utf8(&data) {
        Ok(text) => text,
        Err(err) => {
            return Err(Error::new(
                InvalidOperation,
                format!("Unable to parse binary data into utf8!\n{:#}", err)
            ))
        }
    };

    Ok(match serde_json::from_str::<Value>(text) 
        .or(toml::from_str::<Value>(text)) {
            Ok(value) => value,
            Err(_) => Value::from(text)
        })
}
