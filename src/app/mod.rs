mod context;
mod proxy;
mod modify;

use std::error::Error;
use std::collections::HashMap;
use minijinja::{Environment};
use axum::{
    response::Response,
    extract::{Path, Query, State, Request},
    http::StatusCode
};
use context::Context;
use proxy::Proxy;
use modify::Modify;
use crate::config::Route;
use crate::debug::debug;

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

        if let Some(proxy) = state.lookup("proxy") {
            Proxy::new(
                &context.method,
                &context.headers,
                &context.body,
                &proxy
            ).await
        } else {
            let mut response = Response::builder()
                .status(200)
                .header("content-type", "text/html");

            if let Some(modify) = state.lookup("modify") {
                if let Ok(modify) = Modify::new(&modify) {
                    if let Some(status) = modify.status {
                        response = response.status(status);
                    }

                    if let Some(headers) = modify.headers {
                        for (key, value) in headers.iter() {
                            response = response.header(key, value);
                        }
                    }
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
    let method = request.method().as_str().to_string();
    let path = request.uri().to_string();
    debug(&method, &path, None, "");
    match state.run(params, vars, request).await {
        Ok(response) => {
            debug(&method, &path, Some(200), "");
            Ok(response)
        },
        Err(err) => {
            let error = err.to_string();
            debug(&method, &path, Some(500), &error);
            Err((StatusCode::INTERNAL_SERVER_ERROR, error))
        }
    }
}
