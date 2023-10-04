use std::fs::{write, read_to_string};
use std::path::PathBuf;
use std::collections::HashMap;
use std::process::exit;
use serde_derive::Deserialize;
use clap::{Parser, Subcommand};
use toml::from_str;

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

fn main() {
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
                        println!("New config file generated: `{}`", p.display());
                    }
                    Err(_) => {
                        println!("Fail to write file: `{}`", p.display());
                    }
                }
            }
        }
        Commands::Start { path } => {
            println!("Start {}", path.display());
            let data = match read_to_string(path) {
                Ok(data) => data,
                Err(_) => {
                    eprintln!("Could not read file `{}`", path.display());
                    exit(1);
                }
            };

            let config: Config = match from_str(&data) {
                Ok(data) => data,
                Err(_) => {
                    eprintln!("Unable to load data from `{}`", path.display());
                    exit(1);
                }
            };

            println!("{:#?}", config);
        }
    }
}
