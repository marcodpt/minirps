mod templates;
mod assets;
mod config;
mod handler;

use std::error::Error;
use std::path::PathBuf;
use std::collections::HashMap;
use std::net::SocketAddr;
use clap::{Parser};
use tower_http::cors::{Any, CorsLayer};
use axum::{
    response::{IntoResponse, Response},
    extract::{Path as Params, Query, State, MatchedPath, Request as Req},
    routing::{get, on, Router},
    http::{StatusCode, Method, header::{HeaderValue}}
};
use axum_server::tls_openssl::OpenSSLConfig;
use minijinja::{Environment, Value};
use crate::assets::Assets;
use crate::config::{Config, Route};
use crate::handler::{handler as run};

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
    env: Env
}

/*async fn file_loader (
    state: State<AppState>,
    Params(params): Params<HashMap<String, String>>,
) -> Result<Response<Body>, StatusCode> {
    state.loader.as_ref().unwrap().get(params.get("file").map_or("", |v| v))
}*/

async fn handler (
    state: State<AppState>,
    route: MatchedPath,
    Params(params): Params<Value>,
    Query(vars): Query<Value>,
    request: Req,
) -> Result<Response, AppError> {
    let routes = &state.routes;
    let route = route.as_str().to_string();
    let method = request.method().as_str().to_string();

    let mut matched: Option<Route> = None;
    for test in routes {
        if
            matched.is_none() &&
            &test.method == &method &&
            &test.path == &route
        {
            matched = Some(test.clone());
        }
    }
    let matched = matched.ok_or("no route matched!")?;
    Ok(run(
        &state.env,
        &matched.template,
        route,
        params,
        vars,
        request
    ).await?)
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

    if let Some(assets) = cli.assets.or(config.assets) {
        let loader = Assets::new(
            assets,
            cli.all || config.all.unwrap_or(false),
            ignore
        )?;
        let loader2 = loader.clone();
        app = app.route("/", get(|| async move {
            loader2.get("")
        }));
        app = app.route("/*file", get(|
            Params(params): Params<HashMap<String, String>>,
        | async move {
            loader.get(params.get("file").map_or("", |v| v))
        }));
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
        env: templates::new(config.templates)
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
