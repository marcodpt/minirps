use std::collections::HashMap;
use serde_derive::Serialize;
use axum::http::{Uri, HeaderMap, Method};
use axum::body::Bytes;
use axum::extract::MatchedPath;

#[derive(Serialize)]
pub struct Context {
    pub method: String,
    pub url: String,
    route: String,
    path: String,
    query: String,
    params: HashMap<String, String>,
    vars: HashMap<String, String>,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>
}

impl Context {
    pub fn new (
        route: MatchedPath,
        params: HashMap<String, String>,
        vars: HashMap<String, String>,
        method: Method,
        url: Uri,
        raw_headers: HeaderMap,
        body: Bytes 
    ) -> Context {
        let mut headers: HashMap<String, String> = HashMap::new();
        for (key, value) in raw_headers.iter() {
            if let Ok(value) = value.to_str() {
                headers.insert(key.to_string(), value.to_string());
            }
        }
        Context {
            method: method.as_str().to_string(),
            url: url.to_string(),
            route: route.as_str().to_string(),
            path: url.path().to_string(),
            query: url.query().unwrap_or("").to_string(),
            params,
            vars,
            headers,
            body: body.to_vec()
        }
    }
}
