mod redirect;
mod context;

use std::error::Error;
use std::collections::HashMap;
use serde::Deserialize;
use minijinja::{Environment};
use axum::{
    response::Response,
    extract::{Path, Query, State, Request},
    http::StatusCode
};
use redirect::Redirect;
use context::Context;
use crate::config::Route;

type Env = Environment<'static>;
#[derive(Clone)]
pub struct AppState {
    env: Env,
    route: Route,
}

impl AppState {
    pub fn new (env: &Env, route: &Route) -> AppState {
        AppState {
            env: env.clone(),
            route: route.clone(),
        }
    }

    pub async fn run (&self,
        params: HashMap<String, String>,
        vars: HashMap<String, String>,
        request: Request
    ) -> Result<Response, Box<dyn Error>> {
        let context = Context::new(
            &self.route.path, params, vars, request
        ).await?;
        let tpl = self.env.get_template(&self.route.template)?;
        let (body, state) = match tpl.render_and_return_state(&context) {
            Ok(result) => result,
            Err(err) => {
                let mut info = format!("Fail to render template!\n{:#}", err);
                let mut err = &err as &dyn Error;
                while let Some(next_err) = err.source() {
                    info = format!("{}\n\n{:#}", info, next_err);
                    err = next_err;
                }
                return Err(info.into());
            }
        };

        if let Some(redirect) = state.lookup("redirect") {
            Redirect::new(
                &context.method,
                &context.headers,
                &context.body,
                &redirect
            ).await
        } else {
            let mut response = Response::builder()
                .status(200)
                .header("content-type", "text/html");

            if let Some(status) = state.lookup("status") {
                if let Ok(status) = u16::try_from(status) {
                    response = response.status(status);
                }
            }

            if let Some(headers) = state.lookup("set_headers") {
                let headers = HashMap::<String, String>::deserialize(headers)?;
                for (key, value) in headers.iter() {
                    response = response.header(key, value);
                }
            }

            Ok(response.body(body.into())?)
        }
    }
}

pub async fn handler (
    state: State<AppState>,
    Path(params): Path<HashMap<String, String>>,
    Query(vars): Query<HashMap<String, String>>,
    request: Request,
) -> Result<Response, (StatusCode, String)> {
    match state.run(params, vars, request).await {
        Ok(response) => Ok(response),
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))
    }
}
