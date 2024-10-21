use std::error::Error;
use std::collections::HashMap;
use minijinja::Value;
use serde_derive::Deserialize;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Modify {
    pub status: Option<u16>,
    pub headers: Option<HashMap<String, String>>
}

impl Modify {
    pub fn new (modify: &Value) -> Result<Modify, Box<dyn Error>> {
        Ok(Modify::deserialize(modify)?)
    }
}
