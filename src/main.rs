use std::error::Error;
use std::fs::{write, read, read_to_string, read_dir};
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use std::net::SocketAddr;
use serde_derive::Deserialize;
use serde_json::{Value, json};
use clap::{Parser, Subcommand};
use toml;
use mime_guess;
use tower_http::cors::{Any, CorsLayer};
use axum::{
    extract::{OriginalUri, Path as Params, Query, Extension, MatchedPath},
    routing::{get, on, MethodFilter},
    Router,
    Server,
    http::{StatusCode, Method, header::{
        HeaderMap, HeaderName, HeaderValue, CONTENT_TYPE
    }}
};
use axum_server::tls_rustls::RustlsConfig;
use minijinja::{Environment, path_loader};
use reqwest::{Request, RequestBuilder, Client};

#[derive(Deserialize, Clone, Debug)]
struct Req {
    name: String,
    method: String,
    headers: Option<HashMap<String, String>>,
    url: String,
    body: Option<String>
}

#[derive(Deserialize, Clone, Debug)]
struct Res {
    status: Option<String>,
    headers: Option<HashMap<String, String>>,
    body: Option<String>
}

#[derive(Deserialize, Clone, Debug)]
struct Route {
    method: String,
    path: String,
    requests: Option<Vec<Req>>,
    response: Option<Res>
}

#[derive(Deserialize, Clone, Debug)]
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

fn resolve_method (method: &str) -> Result<MethodFilter, Box<dyn Error>> {
    let res = match method.to_ascii_uppercase().as_str() {
        "GET" => MethodFilter::GET,
        "POST" => MethodFilter::POST,
        "DELETE" => MethodFilter::DELETE,
        "PUT" => MethodFilter::PUT,
        "PATCH" => MethodFilter::PATCH,
        "HEAD" => MethodFilter::HEAD,
        "OPTIONS" => MethodFilter::OPTIONS,
        "TRACE" => MethodFilter::TRACE,
        _ => {
            return Err(format!("Unknown method: {}", method).into());
        }
    };
    Ok(res)
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
                app = app.route(root, get((headers.clone(), body.clone())));
            }

            app = app.route(route, get((headers, body)));
        }
    }
    Ok(app)
}

async fn handler (
    OriginalUri(url): OriginalUri,
    Params(params): Params<HashMap<String, String>>,
    Query(vars): Query<Value>,
    Extension(env): Extension<Environment<'static>>,
    Extension(config): Extension<Config>,
    path: MatchedPath,
    headers: HeaderMap,
    method: Method,
    body: String,
) -> Result<(StatusCode, HeaderMap, String), StatusCode> {
    let mut context = json!({}); 
    let x = context.as_object_mut().unwrap();
    x.insert(String::from("url"), json!(url.to_string()));
    x.insert(String::from("schema"), json!(url.scheme_str()));
    x.insert(String::from("host"), json!(url.host()));
    x.insert(String::from("port"), json!(url.port_u16()));
    x.insert(String::from("path"), json!(url.path()));
    x.insert(String::from("query"), json!(url.query()));
    x.insert(String::from("headers"), json!({}));
    let h = x["headers"].as_object_mut().unwrap();
    for key in headers.keys() {
        let v = headers.get(key).unwrap().to_str().unwrap();
        h.insert(key.to_string(), json!(v));
    }
    x.insert(String::from("params"), json!(params));
    x.insert(String::from("vars"), vars);
    x.insert(String::from("body"), json!(body));
    x.insert(String::from("json"), match serde_json::from_str(&body) {
        Ok(data) => data,
        Err(_) => json!(body)
    });
    x.insert(String::from("data"), json!({}));

    let mut response = (StatusCode::OK, HeaderMap::new(), String::new());

    let mut route: Option<Route> = None;
    for test in config.routes.unwrap_or(Vec::new()) {
        if
            route.is_none() &&
            &test.method == method.as_str() &&
            &test.path == path.as_str()
        {
            route = Some(test);
        }
    }
    let route = match route {
        Some(route) => route,
        None => {
            return Err(StatusCode::NOT_FOUND);
        }
    };
    for req in route.requests.unwrap_or(Vec::new()) {
        let ctx = context.clone();
        let mut r = RequestBuilder::from_parts(Client::new(), Request::new(
            env.render_str(&req.method, &ctx).unwrap().parse().unwrap(),
            env.render_str(&req.url, &ctx).unwrap().parse().unwrap()
        ));
        if req.headers.is_some() {
            let headers = req.headers.unwrap();
            for (key, value) in headers {
                r = r.header(
                    env.render_str(&key, &ctx).unwrap(),
                    env.render_str(&value, &ctx).unwrap()
                );
            }
        }
        if req.body.is_some() {
            let body = req.body.unwrap().to_string();
            r = r.body(env.render_str(&body, &ctx).unwrap());
        }
        let res = r.send().await.unwrap();
        let status = res.status();
        let headers = res.headers().clone();
        let body = res.text().await.unwrap();
        let d = context["data"].as_object_mut().unwrap();
        d.insert(req.name.clone(), json!({
            "status": status.as_u16(),
            "body": body,
            "headers": {}
        }));

        let h = d["headers"].as_object_mut().unwrap();
        for (key, value) in &headers {
            h.insert(key.to_string(), json!(value.to_str().unwrap()));
        }
        response = (status, headers, body)
    }

    if route.response.is_some() {
        let res = route.response.unwrap();
        let ctx = context.clone();
        if res.status.is_some() {
            let status = res.status.unwrap();
            response.0 = env.render_str(&status, &ctx).unwrap().parse().unwrap();
        }
        if res.headers.is_some() {
            let mut r = HeaderMap::new();
            let headers = res.headers.unwrap();
            for (key, value) in headers {
                r.insert(
                    HeaderName::from_bytes(env.render_str(&key, &ctx).unwrap().as_bytes()).unwrap(),
                    env.render_str(&value, &ctx).unwrap().parse().unwrap()
                );
            }
            response.1 = r;
        }
        if res.body.is_some() {
            let body = res.body.unwrap();
            response.2 = env.render_str(&body, &ctx).unwrap().parse().unwrap();
        }
    }

    Ok(response)
}

async fn start_server (path: &PathBuf) -> Result<(), Box<dyn Error>> {
    let p = path.as_path();
    let data = read_to_string(p)?;
    let config: Config = toml::from_str(&data)?;
    let dir = ok(p.parent())?;
    let mut env = Environment::new();

    let mut app = Router::new();

    app = app.layer(Extension(config.clone()));

    if config.templates.is_some() {
        let templates = dir.join(ok(config.templates)?);
        env.set_loader(path_loader(templates));
    }
    app = app.layer(Extension(env));

    if config.assets.is_some() {
        let assets = dir.join(ok(config.assets)?);
        app = build_assets(&assets, &assets, app)?;
    }

    if config.routes.is_some() {
        let routes = ok(config.routes)?;

        for route in routes {
            app = app.route(&route.path,
                on(resolve_method(&route.method)?, handler)
            );
        }
    }

    if config.cors.is_some() {
        let origins = ok(config.cors)?;

        let mut layer = CorsLayer::new()
            .allow_methods(Any);

        if origins.len() == 0 {
            layer = layer.allow_origin(Any)
        }
        for origin in origins {
            layer = layer.allow_origin(origin.parse::<HeaderValue>()?);
        }

        app = app.layer(layer);
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
