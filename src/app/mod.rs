mod context;
mod proxy;
mod modify;

use std::error::Error;
use std::collections::HashMap;
use minijinja::{Environment};
use axum::{
    extract::{Path, Query, State, OriginalUri, MatchedPath},
    body::{Bytes, Body},
    http::{Method, StatusCode, HeaderMap, HeaderName, HeaderValue, header},
};
use context::Context;
use proxy::Proxy;
use modify::Modify;
use crate::debug::debug;
use mime_guess;

type Env = Environment<'static>;
#[derive(Clone)]
pub struct AppState {
    env: Env,
    template: String,
    mime: Option<HeaderValue>
}

impl AppState {
    pub fn new (env: &Env, template: &str) -> AppState {
        AppState {
            env: env.clone(),
            template: template.to_string(),
            mime: match mime_guess::from_path(template).first_raw() {
                Some(mime) => match HeaderValue::from_str(mime) {
                    Ok(mime) => Some(mime),
                    Err(_) => None
                },
                None => None
            }
        }
    }

    pub async fn run (&self,
        ctx: &Context
    ) -> Result<(StatusCode, HeaderMap, Body), Box<dyn Error>> {
        let tpl = self.env.get_template(&self.template)?;
        let (tpl, state) = match tpl.render_and_return_state(ctx) {
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
        let mut body: Body = tpl.into();

        if let Some(proxy) = state.lookup("proxy") {
            (status, headers, body) = Proxy::new(
                &ctx.method,
                &ctx.headers,
                &ctx.body,
                &proxy
            ).await?;
            headers.remove(header::TRANSFER_ENCODING);
        } else if let Some(mime) = &self.mime {
            headers.insert(header::CONTENT_TYPE, mime.clone());
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

        if let Ok(server) = HeaderValue::from_str("minirps") {
            headers.insert(header::SERVER, server);
        }

        Ok((status, headers, body))
    }
}

pub async fn handler (
    state: State<AppState>,
    OriginalUri(url): OriginalUri,
    Path(params): Path<HashMap<String, String>>,
    Query(vars): Query<HashMap<String, String>>,
    route: MatchedPath,
    headers: HeaderMap,
    method: Method,
    body: Bytes,
) -> (StatusCode, HeaderMap, Body) {
    let ctx = Context::new(route, params, vars, method, url, headers, body);
    debug(&ctx.method, &ctx.url, None, "");
    match state.run(&ctx).await {
        Ok(response) => {
            debug(&ctx.method, &ctx.url, Some(response.0.as_u16()), "");
            response
        },
        Err(err) => {
            let error = err.to_string();
            let status = StatusCode::INTERNAL_SERVER_ERROR;
            debug(&ctx.method, &ctx.url, Some(status.as_u16()), &error);
            (status, HeaderMap::new(), error.into())
        }
    }
}
