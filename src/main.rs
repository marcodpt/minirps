mod templates;
mod assets;
mod config;

use std::str::from_utf8;
use std::error::Error;
use std::path::PathBuf;
use std::collections::HashMap;
use std::net::SocketAddr;
use serde::{Deserialize};
use serde_derive::{Serialize, Deserialize};
use clap::{Parser};
use tower_http::cors::{Any, CorsLayer};
use axum::{
    response::{IntoResponse, Response},
    extract::{Path as Params, Query, State, MatchedPath, Request as Req},
    routing::{get, on, Router},
    http::{StatusCode, Method, header::{HeaderValue}},
    body::{to_bytes, Body}
};
use axum_server::tls_openssl::OpenSSLConfig;
use minijinja::{Environment, Value};
use reqwest::{Request, RequestBuilder, Client};
use crate::assets::Assets;
use crate::config::{Config, Route};

struct AppError(Box<dyn Error>);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, self.0.to_string()).into_response()
    }
}

impl<E> From<E> for AppError where  E: Into<Box<dyn Error>> {
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

type Env = Environment<'static>;
#[derive(Clone)]
struct AppState {
    routes: Vec<Route>,
    env: Env,
    loader: Option<Assets>
}

async fn file_loader (
    state: State<AppState>,
    Params(params): Params<HashMap<String, String>>,
) -> Result<Response<Body>, StatusCode> {
    state.loader.as_ref().unwrap().get(params.get("file").map_or("", |v| v))
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

#[derive(Deserialize)]
struct Redirect {
    method: Option<String>,
    url: String,
    headers: Option<HashMap<String, String>>,
    body: Option<String>
}

async fn handler (
    state: State<AppState>,
    route: MatchedPath,
    Params(params): Params<Value>,
    Query(vars): Query<Value>,
    request: Req,
) -> Result<Response, AppError> {
    let routes = &state.routes;
    let env = &state.env;
    let route = route.as_str();
    let method = request.method().as_str().to_string();

    let mut matched: Option<Route> = None;
    for test in routes {
        if
            matched.is_none() &&
            &test.method == &method &&
            &test.path == route
        {
            matched = Some(test.clone());
        }
    }
    let matched = matched.ok_or("no route matched!")?;

    let mut headers: HashMap<String, String> = HashMap::new();
    let header_map = request.headers().clone();
    for key in header_map.keys() {
        if let Some(value) = header_map.get(key) {
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
        route: route.to_string(),
        path: uri.path().to_string(),
        query: uri.query().unwrap_or("").to_string(),
        params,
        vars,
        headers,
        body: body.to_string()
    };

    let tpl = env.get_template(&matched.template)?;
    let (body, state) = tpl.render_and_return_state(&context)?;

    if let Some(redirect) = state.lookup("redirect") {
        let redirect = Redirect::deserialize(redirect)?;
        let method = redirect.method.unwrap_or(method);

        let mut r = RequestBuilder::from_parts(Client::new(),
            Request::new(method.parse()?, redirect.url.parse()?)
        );
        for (key, value) in header_map.iter() {
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

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// config.toml file path.
    #[clap(short='f', long)]
    config: Option<PathBuf>,

    /// static files folder path.
    #[clap()]
    assets: Option<PathBuf>, 

    /// port number to run the server on.
    #[clap(short, long)]
    port: Option<u16>,

    /// public key file path.
    #[clap(short, long)]
    cert: Option<PathBuf>,
    
    /// private key file path.
    #[clap(short, long)]
    key: Option<PathBuf>,

    /// allow CORS from all origins.
    #[clap(short='o', long)]
    allow_cors: bool,

    /// all files, include hidden files
    #[clap(short, long)]
    all: bool,

    /// ignore files based on glob match
    #[clap(short, long)]
    ignore: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    let config = Config::new(cli.config.as_deref())?;

    let cors: Option<Vec<String>> = match cli.allow_cors {
        true => Some(Vec::new()),
        false => None
    };
    let mut ignore: Vec<String> = Vec::new();
    if let Some(glob) = cli.ignore {
        ignore.push(glob);
        if let Some(globs) = config.ignore {
            ignore.append(&mut globs.clone());
        }
    }

    let mut app = Router::new();

    let mut loader: Option<Assets> = None;
    if let Some(assets) = cli.assets.or(config.assets) {
        loader = Some(Assets::new(
            assets,
            cli.all || config.all.unwrap_or(false),
            ignore
        )?);
        app = app.route("/", get(file_loader));
        app = app.route("/*file", get(file_loader));
    }

    let routes: Vec<Route> = config.routes.unwrap_or(Vec::new());
    for route in &routes {
        app = app.route(&route.path, on(
            Method::from_bytes(route.method.as_bytes())?.try_into()?,
            handler
        ));
    }

    if let Some(origins) = cors.or(config.cors) {
        let mut layer = CorsLayer::new()
            .allow_methods(Any);

        if origins.len() == 0 {
            layer = layer.allow_origin(Any);
        }
        for origin in origins {
            layer = layer.allow_origin(origin.parse::<HeaderValue>()?);
        }

        app = app.layer(layer);
    }

    let state = AppState {
        routes,
        env: templates::new(config.templates),
        loader
    };
    let app = app.with_state(state);

    let port = cli.port.unwrap_or(config.port.unwrap_or(3000));
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let cert = cli.cert.or(config.cert);
    let key = cli.key.or(config.key);

    if cert.is_some() && key.is_some() {
        let cert = cert.ok_or("unreachable")?;
        let key = key.ok_or("unreachable")?;
        let config = OpenSSLConfig::from_pem_file(cert, key)?;

        println!("Server started at https://localhost:{}", port);
        axum_server::bind_openssl(addr, config)
            .serve(app.into_make_service())
            .await?;
    } else {
        println!("Server started at http://localhost:{}", port);
        axum_server::bind(addr)
            .serve(app.into_make_service())
            .await?;
    }

    Ok(())
}
