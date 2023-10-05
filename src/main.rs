use std::error::Error;
use std::fs::{write, read, read_to_string, read_dir};
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use std::process::exit;
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
            let e = "Unexpected option none";
            let name = route.file_name().ok_or(e)?.to_str().ok_or(e)?;
            let root = route.parent().ok_or(e)?.to_str().ok_or(e)?;
            let route = route.to_str().ok_or(e)?;
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

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::New { path } => {
            let p = path.as_path();
            if p.exists() {
                eprintln!("File already exists `{}`", p.display());
                exit(1);
            } else if
                p.extension().is_none() ||
                p.extension().unwrap() != "toml"
            {
                eprintln!("File must have .toml extension `{}`", p.display());
                exit(1);
            } else {
                match write(p, include_str!("../tests/new.toml")) {
                    Ok(_) => {
                        println!("New config file generated: `{}`",
                            p.display()
                        );
                    }
                    Err(_) => {
                        println!("Fail to write file: `{}`", p.display());
                    }
                }
            }
        }
        Commands::Start { path } => {
            let p = path.as_path();
            let data = match read_to_string(p) {
                Ok(data) => data,
                Err(_) => {
                    eprintln!("Could not read file `{}`", p.display());
                    exit(1);
                }
            };

            let config: Config = match toml::from_str(&data) {
                Ok(data) => data,
                Err(_) => {
                    eprintln!("Unable to load data from `{}`", p.display());
                    exit(1);
                }
            };

            let dir = p.parent().unwrap();

            let mut app = Router::new();

            if config.assets.is_some() {
                let assets = dir.join(config.assets.unwrap());
                app = match build_assets(&assets, &assets, app) {
                    Ok(app) => app,
                    Err(_) => {
                        eprintln!("Unable to read assets dir: `{}`",
                            assets.display()
                        );
                        exit(1);
                    }
                };
            }

            println!("Server started at http://localhost:3000");
            Server::bind(&"0.0.0.0:3000".parse().unwrap())
                .serve(app.into_make_service())
                .await
                .unwrap();

            //println!("{:#?}", config);
        }
    }
}
