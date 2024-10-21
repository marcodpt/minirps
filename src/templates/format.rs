use minijinja::{Error, ErrorKind::InvalidOperation, Value};
use serde_json;
use serde_urlencoded;
use toml;

pub fn format (
    value: &Value,
    encoding: &str
) -> Result<Vec<u8>, Error> {
    match encoding {
        "form" => {
            match serde_urlencoded::to_string::<Value>(value.clone().into()) {
                Ok(data) => Ok(data.as_bytes().to_vec()),
                Err(err) => Err(Error::new(
                    InvalidOperation,
                    format!("Unable to format Form Data!\n{:#}", err)
                ))
            }
        },
        "json" => {
            match serde_json::to_string_pretty(value.into()) {
                Ok(data) => Ok(data.as_bytes().to_vec()),
                Err(err) => Err(Error::new(
                    InvalidOperation,
                    format!("Unable to format JSON!\n{:#}", err)
                ))
            }
        },
        "toml" => {
            match toml::to_string_pretty(value.into()) {
                Ok(data) => Ok(data.as_bytes().to_vec()),
                Err(err) => Err(Error::new(
                    InvalidOperation,
                    format!("Unable to format TOML!\n{:#}", err)
                ))
            }
        },
        "debug" => {
            Ok(format!("{:#?}", value).as_bytes().to_vec())
        },
        "raw" => {
            if let Some(data) = value.as_bytes() {
                Ok(data.to_vec())
            } else {
                Err(Error::new(
                    InvalidOperation,
                    format!("Data cannot be converted to bytes!")
                ))
            }
        },
        encoding => {
            Err(Error::new(
                InvalidOperation,
                format!("Format {} not implemented!", encoding)
            ))
        }
    }
}
