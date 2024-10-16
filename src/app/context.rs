use std::error::Error;
use std::collections::HashMap;
use serde_derive::Serialize;
use axum::{extract::Request, body::to_bytes};

#[derive(Serialize)]
pub struct Context {
    pub method: String,
    url: String,
    route: String,
    path: String,
    query: String,
    params: HashMap<String, String>,
    vars: HashMap<String, String>,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>
}

impl Context {
    pub async fn new (
        route: &str,
        params: HashMap<String, String>,
        vars: HashMap<String, String>,
        request: Request
    ) -> Result<Context, Box<dyn Error>> {
        let method = request.method().as_str().to_string();
        let mut headers: HashMap<String, String> = HashMap::new();
        for key in request.headers().keys() {
            if let Some(value) = request.headers().get(key) {
                if let Ok(value) = value.to_str() {
                    headers.insert(key.to_string(), value.to_string());
                }
            }
        }
        let uri = request.uri().clone();
        let body = to_bytes(request.into_body(), usize::MAX).await?;
        Ok(Context {
            method: method.to_string(),
            url: uri.to_string(),
            route: route.to_string(),
            path: uri.path().to_string(),
            query: uri.query().unwrap_or("").to_string(),
            params,
            vars,
            headers,
            body: body.to_vec()
        })
    }
}
