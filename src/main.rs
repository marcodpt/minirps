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
    response::Response,
    extract::{Path, Query, State, Request},
    routing::{get, on, Router},
    http::{StatusCode, Method, header::{HeaderValue}}
};
use axum_server::tls_openssl::OpenSSLConfig;
use minijinja::{Environment, Value};
use crate::assets::Assets;
use crate::config::{Config, Route};
use crate::handler::{handler as run};

type Env = Environment<'static>;
#[derive(Clone)]
struct AppState {
    route: Route,
    env: Env
}

async fn handler (
    state: State<AppState>,
    Path(params): Path<Value>,
    Query(vars): Query<Value>,
    request: Request,
) -> Result<Response, (StatusCode, String)> {
    match run(
        &state.env,
        &state.route.template,
        state.route.path.clone(),
        params,
        vars,
        request
    ).await {
        Ok(response) => Ok(response),
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))
    }
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
    let mut app = Router::new();

    if let Some(assets) = cli.assets.or(config.assets) {
        let mut ignore: Vec<String> = Vec::new();
        if let Some(glob) = cli.ignore {
            ignore.push(glob);
            if let Some(globs) = config.ignore {
                ignore.append(&mut globs.clone());
            }
        }

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
            Path(params): Path<HashMap<String, String>>,
        | async move {
            loader.get(params.get("file").map_or("", |v| v))
        }));
    }

    if let (Some(templates), Some(routes)) = (
        config.templates, config.routes
    ) {
        let env = templates::new(templates);
        for route in &routes {
            app = app.route(&route.path, on(
                Method::from_bytes(route.method.as_bytes())?.try_into()?,
                handler
            ).with_state(AppState {
                route: route.clone(),
                env: env.clone()
            }));
        }
    }


    let cors = if cli.allow_cors {Some(Vec::new())} else {config.cors};
    if let Some(origins) = cors {
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

    let port = cli.port.unwrap_or(config.port.unwrap_or(3000));
    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    if let (Some(cert), Some(key)) = (
        cli.cert.or(config.cert), cli.key.or(config.key)
    ) {
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
