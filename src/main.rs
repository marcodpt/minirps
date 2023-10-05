use std::error::Error;
use std::fs::{write, read, read_to_string, read_dir};
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use std::net::SocketAddr;
use serde_derive::Deserialize;
use clap::{Parser, Subcommand};
use toml;
use mime_guess;
use axum::{
    routing::get,
    Router,
    Server,
    http::header::{HeaderMap, CONTENT_TYPE}
};
use axum_server::tls_rustls::RustlsConfig;

#[derive(Deserialize, Debug)]
struct Request {
    method: Option<String>,
    headers: Option<HashMap<String, String>>,
    url: Option<String>,
    body: Option<String>
}

#[derive(Deserialize, Debug)]
struct Response {
    status: Option<String>,
    headers: Option<HashMap<String, String>>,
    body: Option<String>
}

#[derive(Deserialize, Debug)]
struct Route {
    method: String,
    path: String,
    request: Option<Request>,
    response: Option<Response>
}

#[derive(Deserialize, Debug)]
struct Config {
    cors: Option<Vec<String>>,
    port: Option<u16>,
    cert: Option<String>,
    key: Option<String>,
    assets: Option<String>, 
    templates: Option<String>, 
    routes: Option<Vec<Route>>
}

fn ok<T> (data: Option<T>) -> Result<T, Box<dyn Error>> {
    data.ok_or("Unexpected option none".into())
}

fn assert (test: bool, msg: &str) -> Result<(), Box<dyn Error>> {
    if test {Ok(())} else {Err(msg.into())}
}

fn gen_config (path: &PathBuf) -> Result<(), Box<dyn Error>> {
    let p = path.as_path();
    assert(!p.exists(), &format!("File already exists `{}`", p.display()))?;
    let e = format!("File must have .toml extension `{}`", p.display());
    let ext = p.extension().ok_or(e.clone())?;
    assert(ext == "toml", &e)?;
    write(p, include_str!("../tests/new.toml"))?;
    println!("New config file generated: `{}`", p.display());
    Ok(())
}

fn build_assets (
    base: &Path, dir: &Path, mut app: Router
) -> Result<Router, Box<dyn Error>> {
    for entry in read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            app = build_assets(base, &path, app)?;
        } else {
            let route = match path.strip_prefix(&base) {
                Ok(route) => Path::new("/").join(route),
                Err(_) => path.clone()
            };
            let name = ok(ok(route.file_name())?.to_str())?;
            let root = ok(ok(route.parent())?.to_str())?;
            let route = ok(route.to_str())?;
            let mut headers = HeaderMap::new();
            match mime_guess::from_path(&path).first_raw() {
                Some(value) => {
                    headers.insert(CONTENT_TYPE, value.parse()?);
                },
                None => {}
            };
            let body = read(&path)?;

            if name == "index.html" {
                app = app.route(root,
                    get((headers.clone(), body.clone()))
                );
            }

            app = app.route(route, get((headers, body)));
        }
    }
    Ok(app)
}

async fn start_server (path: &PathBuf) -> Result<(), Box<dyn Error>> {
    let p = path.as_path();
    let data = read_to_string(p)?;
    let config: Config = toml::from_str(&data)?;
    let dir = ok(p.parent())?;

    let mut app = Router::new();

    if config.assets.is_some() {
        let assets = dir.join(ok(config.assets)?);
        app = build_assets(&assets, &assets, app)?;
    }

    let port = config.port.unwrap_or(3000);
    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    if config.cert.is_some() && config.key.is_some() {
        let cert = dir.join(ok(config.cert)?);
        let key = dir.join(ok(config.key)?);
        let tls = RustlsConfig::from_pem(read(cert)?, read(key)?).await?;

        println!("Server started at https://localhost:{}", port);
        axum_server::bind_rustls(addr, tls)
            .serve(app.into_make_service())
            .await?;
    } else {
        println!("Server started at http://localhost:{}", port);
        Server::bind(&addr)
            .serve(app.into_make_service())
            .await?;
    }

    Ok(())
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate a config file sample
    New {
        /// Path for the generated config file
        #[arg(value_name = "FILE", required = true)]
        path: PathBuf,
    },
    /// Starts the server based on a given config file
    Start {
        /// Path for the config file
        #[arg(value_name = "FILE", required = true)]
        path: PathBuf,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::New { path } => gen_config(path),
        Commands::Start { path } => start_server(path).await
    }
}
