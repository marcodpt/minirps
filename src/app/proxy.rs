use std::error::Error;
use std::collections::HashMap;
use serde::Deserialize;
use serde_derive::Deserialize;
use minijinja::Value;
use axum::http::{StatusCode, HeaderMap};
use axum::body::Body;
use reqwest::{Request, RequestBuilder, Client};
use crate::debug::debug;

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
    ) -> Result<(StatusCode, HeaderMap, Body), Box<dyn Error>> {
        let proxy = Proxy::deserialize(proxy)?;
        let method = proxy.method.unwrap_or(method.to_string());

        debug(&method, &proxy.url, None, "");
        let mut r = RequestBuilder::from_parts(Client::new(),
            Request::new(method.parse()?, proxy.url.parse()?)
        );
        if let Some(headers) = proxy.headers {
            for (name, value) in headers.iter() {
                r = r.header(name, value);
            }
        }
        for (name, value) in headers.iter() {
            r = r.header(name.clone(), value.clone());
        }
        let response = match r.body(
            proxy.body.unwrap_or(body.to_vec())
        ).send().await {
            Ok(response) => {
                debug(
                    &method,
                    &proxy.url,
                    Some(response.status().as_u16()),
                    ""
                );
                response
            },
            Err(err) => {
                debug(&method, &proxy.url, Some(500), &err.to_string());
                return Err(err.into());
            }
        };

        Ok((
            response.status(),
            response.headers().clone(),
            response.bytes().await?.into()
        ))
    }
}
