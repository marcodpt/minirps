use serde_json;
use toml;
use minijinja::{Error, ErrorKind::InvalidOperation, Value};

fn parse_json (value: &str) -> Result<Value, Error> {
    match serde_json::from_str::<Value>(value) {
        Ok(result) => Ok(result),
        Err(err) => Err(Error::new(
            InvalidOperation,
            format!("Fail to parse to JSON\n{:#}", err)
        ))
    }
}

fn parse_toml (value: &str) -> Result<Value, Error> {
    match toml::from_str::<Value>(value) {
        Ok(result) => Ok(result),
        Err(err) => Err(Error::new(
            InvalidOperation,
            format!("Fail to parse to TOML\n{:#}", err)
        ))
    }
}

pub fn parse (value: &str, encoding: Option<&str>) -> Result<Value, Error> {
    if let Some(encoding) = encoding {
        match encoding {
            "json" => parse_json(value),
            "toml" => parse_toml(value),
            encoding => Err(Error::new(
                InvalidOperation,
                format!("{} encoding not implemented!", encoding)
            ))
        }
    } else {
        parse_json(value)
            .or(parse_toml(value))
            .or(Ok(value.into()))
    }
}
