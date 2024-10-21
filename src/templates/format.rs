use minijinja::{Error, ErrorKind::InvalidOperation, Value};
use serde_json;
use serde_urlencoded;
use toml;

pub fn format (
    value: &Value,
    encoding: &str
) -> Result<String, Error> {
    match encoding {
        "form" => {
            match serde_urlencoded::to_string::<Value>(value.clone().into()) {
                Ok(data) => Ok(data),
                Err(err) => Err(Error::new(
                    InvalidOperation,
                    format!("Unable to format Form Data!\n{:#}", err)
                ))
            }
        },
        "json" => {
            match serde_json::to_string_pretty(value.into()) {
                Ok(data) => Ok(data),
                Err(err) => Err(Error::new(
                    InvalidOperation,
                    format!("Unable to format JSON!\n{:#}", err)
                ))
            }
        },
        "toml" => {
            match toml::to_string_pretty(value.into()) {
                Ok(data) => Ok(data),
                Err(err) => Err(Error::new(
                    InvalidOperation,
                    format!("Unable to format TOML!\n{:#}", err)
                ))
            }
        },
        "debug" => {
            Ok(format!("{:#?}", value))
        },
        encoding => {
            Err(Error::new(
                InvalidOperation,
                format!("Format {} not implemented!", encoding)
            ))
        }
    }
}

pub fn bytes (text: &str) -> Vec<u8> {
    text.as_bytes().to_vec()
}
