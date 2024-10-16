use std::error::Error;
use std::collections::HashMap;
use serde::Deserialize;
use serde_derive::Deserialize;
use minijinja::Value;
use axum::response::Response;
use reqwest::{Request, RequestBuilder, Client};

#[derive(Deserialize)]
pub struct Redirect {
    method: Option<String>,
    url: String,
    headers: Option<HashMap<String, String>>,
    body: Option<Vec<u8>>
}

impl Redirect {
    pub async fn new (
        method: &str,
        headers: &HashMap<String, String>,
        body: &Vec<u8>,
        redirect: &Value
    ) -> Result<Response, Box<dyn Error>> {
        let redirect = Redirect::deserialize(redirect)?;
        let method = redirect.method.unwrap_or(method.to_string());

        let mut r = RequestBuilder::from_parts(Client::new(),
            Request::new(method.parse()?, redirect.url.parse()?)
        );
        for (key, value) in headers.iter() {
            r = r.header(key.clone(), value.clone());
        }
        if let Some(headers) = redirect.headers {
            for (key, value) in headers.iter() {
                r = r.header(key, value);
            }
        }
        let result = r.body(redirect.body.unwrap_or(body.to_vec()))
            .send().await?;

        let mut response = Response::builder()
            .status(result.status());

        for (key, value) in result.headers().iter() {
            response = response.header(key, value);
        }

        Ok(response.body(result.bytes().await?.into())?)
    }
}
