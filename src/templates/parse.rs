use serde_json;
use serde_urlencoded;
use toml;
use std::str::from_utf8;
use minijinja::{Error, ErrorKind::InvalidOperation, Value};

pub fn parse (
    data: Vec<u8>,
    encoding: Option<&str>
) -> Result<Value, Error> {
    let text = match from_utf8(&data) {
        Ok(text) => text,
        Err(err) => {
            return Err(Error::new(
                InvalidOperation,
                format!("Unable to parse binary data into utf8!\n{:#}", err)
            ))
        }
    };

    match encoding {
        Some("form") => {
            match serde_urlencoded::from_str::<Value>(text) {
                Ok(value) => Ok(value),
                Err(err) => Err(Error::new(
                    InvalidOperation,
                    format!("Failed to parse from Form Data!\n{}",
                        err.to_string()
                    )
                ))
            }
        },
        Some("json") => {
            match serde_json::from_str::<Value>(text) {
                Ok(value) => Ok(value),
                Err(err) => Err(Error::new(
                    InvalidOperation,
                    format!("Failed to parse from JSON!\n{}", err.to_string())
                ))
            }
        },
        Some("toml") => {
            match toml::from_str::<Value>(text) {
                Ok(value) => Ok(value),
                Err(err) => Err(Error::new(
                    InvalidOperation,
                    format!("Failed to parse from TOML!\n{}", err.to_string())
                ))
            }
        },
        Some("text") => {
            Ok(Value::from(text))
        },
        Some(encoding) => Err(Error::new(
            InvalidOperation,
            format!("{} encoding not implemented!", encoding)
        )),
        None => match serde_urlencoded::from_str::<Value>(text)
            .or(serde_json::from_str::<Value>(text))
            .or(toml::from_str::<Value>(text))
        {
            Ok(value) => Ok(value),
            Err(_) => Ok(Value::from(text))
        }
    }
}
