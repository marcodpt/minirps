use std::str::from_utf8;
use std::error::Error;
use std::collections::HashMap;
use serde::{Deserialize};
use serde_derive::{Serialize, Deserialize};
use axum::{
    response::Response,
    extract::Request,
    body::to_bytes
};
use reqwest::{Request as Req, RequestBuilder, Client};
use minijinja::{Environment, Value};

#[derive(Serialize)]
struct Context {
    method: String,
    url: String,
    route: String,
    path: String,
    query: String,
    params: Value,
    vars: Value,
    headers: HashMap<String, String>,
    body: String
}

#[derive(Deserialize)]
struct Redirect {
    method: Option<String>,
    url: String,
    headers: Option<HashMap<String, String>>,
    body: Option<String>
}

pub async fn handler (
    env: &Environment<'static>,
    template: &str,
    route: String,
    params: Value,
    vars: Value,
    request: Request,
) -> Result<Response, Box<dyn Error>> {
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
    let body = from_utf8(&body)?;
    let context = Context {
        method: method.clone(),
        url: uri.to_string(),
        route: route,
        path: uri.path().to_string(),
        query: uri.query().unwrap_or("").to_string(),
        params,
        vars,
        headers,
        body: body.to_string()
    };

    let tpl = env.get_template(&template)?;
    let (body, state) = tpl.render_and_return_state(&context)?;

    if let Some(redirect) = state.lookup("redirect") {
        let redirect = Redirect::deserialize(redirect)?;
        let method = redirect.method.unwrap_or(method);

        let mut r = RequestBuilder::from_parts(Client::new(),
            Req::new(method.parse()?, redirect.url.parse()?)
        );
        for (key, value) in context.headers.iter() {
            r = r.header(key, value);
        }
        if let Some(headers) = redirect.headers {
            for (key, value) in headers.iter() {
                r = r.header(key, value);
            }
        }
        let res = r.body(redirect.body.unwrap_or(body)).send().await?;

        let mut response = Response::builder()
            .status(res.status());

        for (key, value) in res.headers().iter() {
            response = response.header(key, value);
        }

        return Ok(response.body(res.bytes().await?.into())?);
    }

    let mut response = Response::builder()
        .status(200)
        .header("content-type", "text/html");

    if let Some(status) = state.lookup("status") {
        if let Ok(status) = u16::try_from(status) {
            response = response.status(status);
        }
    }

    if let Some(headers) = state.lookup("headers") {
        let headers = HashMap::<String, String>::deserialize(headers)?;
        for (key, value) in headers.iter() {
            response = response.header(key, value);
        }
    }

    Ok(response.body(body.into())?)
}
