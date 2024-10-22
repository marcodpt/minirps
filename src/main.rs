mod templates;
mod assets;
mod config;
mod app;
mod debug;

use std::error::Error;
use std::path::PathBuf;
use std::collections::HashMap;
use std::net::SocketAddr;
use clap::{Parser};
use tower_http::cors::{Any, CorsLayer};
use axum::{
    extract::Path,
    routing::{get, on, Router},
    http::{Method, header::{HeaderValue}}
};
use axum_server::tls_openssl::OpenSSLConfig;
use crate::assets::Assets;
use crate::config::Config;
use crate::app::{AppState, handler};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// config file path. (Accept: .json, .toml)
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

fn init () -> Result<(Router, u16, Option<OpenSSLConfig>), Box<dyn Error>> {
    let cli = Cli::parse();
    let config = Config::new(cli.config.as_deref())?;
    let mut app = Router::new();

    if let Some(assets) = cli.assets.or(config.assets) {
        let mut ignore: Vec<String> = Vec::new();
        let has_home = assets.as_path().join("index.html").is_file();
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
        if has_home {
            let loader2 = loader.clone();
            app = app.route("/", get(|| async move {
                loader2.get("")
            }));
        }
        app = app.route("/*file", get(|
            Path(params): Path<HashMap<String, String>>,
        | async move {
            loader.get(params.get("file").map_or("", |v| v))
        }));
    }

    if let (Some(templates), Some(routes)) = (
        config.templates, config.routes
    ) {
        let env = templates::new(templates, config.data)?;
        for route in &routes {
            app = app.route(&route.path, on(
                Method::from_bytes(route.method.as_bytes())?.try_into()?,
                handler
            ).with_state(AppState::new(&env, &route)));
        }
    }


    let cors = if cli.allow_cors {Some(Vec::new())} else {config.cors};
    if let Some(origins) = cors {
        let mut layer = CorsLayer::new().allow_methods(Any);

        if origins.len() == 0 {
            layer = layer.allow_origin(Any);
        }
        for origin in origins {
            layer = layer.allow_origin(origin.parse::<HeaderValue>()?);
        }

        app = app.layer(layer);
    }

    let port = cli.port.unwrap_or(config.port.unwrap_or(3000));

    let mut ssl: Option<OpenSSLConfig> = None;
    if let (Some(cert), Some(key)) = (
        cli.cert.or(config.cert), cli.key.or(config.key)
    ) {
        ssl = Some(OpenSSLConfig::from_pem_file(cert, key)?);
    }

    Ok((app, port, ssl))
}

#[tokio::main]
async fn main() -> () {
    let (app, port, ssl) = match init() {
        Ok(server) => server,
        Err(err) => {
            println!("{}", err);
            return ();
        }
    };
    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    let server = match ssl {
        Some(ssl) => {
            println!("Server started at https://localhost:{}", port);
            axum_server::bind_openssl(addr, ssl)
                .serve(app.into_make_service()).await
        },
        None => {
            println!("Server started at http://localhost:{}", port);
            axum_server::bind(addr)
                .serve(app.into_make_service()).await
        }
    };

    match server {
        Ok(_) => (),
        Err(err) => {
            println!("Fail to start server!\n{}", err.to_string());
            ()
        }
    }
}
