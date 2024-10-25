mod context;
mod proxy;
mod modify;

use std::error::Error;
use std::collections::HashMap;
use minijinja::{Environment};
use axum::{
    extract::{Path, Query, State, Request},
    http::{StatusCode, HeaderMap, HeaderName, HeaderValue}
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
    ) -> Result<(StatusCode, HeaderMap, Vec<u8>), Box<dyn Error>> {
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

        let mut status = StatusCode::OK;
        let mut headers = HeaderMap::new();
        let mut body = body.as_bytes().to_vec();

        if let Some(proxy) = state.lookup("proxy") {
            (status, headers, body) = Proxy::new(
                &context.method,
                &context.headers,
                &context.body,
                &proxy
            ).await?;
        }

        if let Some(modify) = state.lookup("modify") {
            if let Ok(modify) = Modify::new(&modify) {
                if let Some(modify_status) = modify.status {
                    if let Ok(
                        modify_status
                    ) = StatusCode::from_u16(modify_status) {
                        status = modify_status;
                    }
                }

                if let Some(modify_headers) = modify.headers {
                    for (name, value) in modify_headers.iter() {
                        if let (Ok(name), Ok(value)) = (
                            HeaderName::from_bytes(name.as_bytes()),
                            HeaderValue::from_str(value)
                        ) {
                            headers.insert(name, value);
                        }
                    }
                }
            }
        }

        Ok((status, headers, body))
    }
}

pub async fn handler (
    state: State<AppState>,
    Path(params): Path<HashMap<String, String>>,
    Query(vars): Query<HashMap<String, String>>,
    request: Request,
) -> Result<(StatusCode, HeaderMap, Vec<u8>), (StatusCode, String)> {
    let method = request.method().as_str().to_string();
    let path = request.uri().to_string();
    debug(&method, &path, None, "");
    match state.run(params, vars, request).await {
        Ok(response) => {
            debug(&method, &path, Some(response.0.as_u16()), "");
            Ok(response)
        },
        Err(err) => {
            let error = err.to_string();
            let status = StatusCode::INTERNAL_SERVER_ERROR;
            debug(&method, &path, Some(status.as_u16()), &error);
            Err((status, error))
        }
    }
}
