mod redirect;

use std::error::Error;
use std::str::from_utf8;
use std::collections::HashMap;
use serde_derive::Serialize;
use serde::Deserialize;
use minijinja::{Environment, Value};
use axum::{
    response::Response,
    extract::{Path, Query, State, Request},
    http::StatusCode,
    body::to_bytes
};
use redirect::Redirect;
use crate::config::Route;

type Env = Environment<'static>;
#[derive(Clone)]
pub struct AppState {
    env: Env,
    route: Route,
}

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

impl AppState {
    pub fn new (env: &Env, route: &Route) -> AppState {
        AppState {
            env: env.clone(),
            route: route.clone(),
        }
    }

    pub async fn run (&self,
        params: Value,
        vars: Value,
        request: Request
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
            route: self.route.path.clone(),
            path: uri.path().to_string(),
            query: uri.query().unwrap_or("").to_string(),
            params,
            vars,
            headers,
            body: body.to_string()
        };

        let tpl = self.env.get_template(&self.route.template)?;
        let (body, state) = tpl.render_and_return_state(&context)?;

        if let Some(redirect) = state.lookup("redirect") {
            return Redirect::new(
                method, &context.headers, body, &redirect
            ).await;
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
}

pub async fn handler (
    state: State<AppState>,
    Path(params): Path<Value>,
    Query(vars): Query<Value>,
    request: Request,
) -> Result<Response, (StatusCode, String)> {
    match state.run(params, vars, request).await {
        Ok(response) => Ok(response),
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))
    }
}
