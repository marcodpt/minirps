use std::error::Error;
use std::collections::HashMap;
use serde::Deserialize;
use serde_derive::Deserialize;
use minijinja::Value;
use axum::response::Response;
use reqwest::{Request, RequestBuilder, Client};

#[derive(Deserialize)]
pub struct Proxy {
    method: Option<String>,
    url: String,
    headers: Option<HashMap<String, String>>,
    body: Option<Vec<u8>>
}

impl Proxy {
    pub async fn new (
        method: &str,
        headers: &HashMap<String, String>,
        body: &Vec<u8>,
        proxy: &Value
    ) -> Result<Response, Box<dyn Error>> {
        let proxy = Proxy::deserialize(proxy)?;
        let method = proxy.method.unwrap_or(method.to_string());

        let mut r = RequestBuilder::from_parts(Client::new(),
            Request::new(method.parse()?, proxy.url.parse()?)
        );
        if let Some(headers) = proxy.headers {
            for (key, value) in headers.iter() {
                r = r.header(key, value);
            }
        }
        for (key, value) in headers.iter() {
            r = r.header(key.clone(), value.clone());
        }
        let result = r.body(proxy.body.unwrap_or(body.to_vec()))
            .send().await?;

        let mut response = Response::builder()
            .status(result.status());

        for (key, value) in result.headers().iter() {
            response = response.header(key, value);
        }

        Ok(response.body(result.bytes().await?.into())?)
    }
}
