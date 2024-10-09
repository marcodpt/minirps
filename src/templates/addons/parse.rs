use serde_json;
use toml;
use minijinja::{Error, ErrorKind::InvalidOperation, Value};

pub fn parse (value: &str, encoding: &str) -> Result<Value, Error> {
    if encoding == "json" {
        match serde_json::from_str::<Value>(value) {
            Ok(result) => Ok(result),
            Err(err) => Err(Error::new(
                InvalidOperation,
                format!("Fail to parse to {}\n{:#}", encoding, err)
            ))
        }
    } else if encoding == "toml" {
        match toml::from_str::<Value>(value) {
            Ok(result) => Ok(result),
            Err(err) => Err(Error::new(
                InvalidOperation,
                format!("Fail to parse to {}\n{:#}", encoding, err)
            ))
        }
    } else {
        Err(Error::new(
            InvalidOperation,
            format!("{} encoding not implemented!", encoding)
        ))
    }
}
