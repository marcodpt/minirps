//mod templates;
mod assets;

use std::default::Default;
use std::error::Error;
use std::str::from_utf8;
use std::fs::read_to_string;
use std::path::PathBuf;
use std::collections::HashMap;
use std::net::SocketAddr;
use serde_derive::Deserialize;
use serde_json::{Value, json};
use clap::{Parser};
use toml;
use tower_http::cors::{Any, CorsLayer};
use axum::{
    response::{IntoResponse, Response},
    extract::{OriginalUri, Path as Params, Query, State, MatchedPath},
    routing::{get, on, Router},
    http::{StatusCode, Method, header::{
        HeaderMap, HeaderName, HeaderValue
    }},
    body::Body
};
use axum_server::tls_openssl::OpenSSLConfig;
use minijinja::{Environment, path_loader};
use reqwest::{Request, RequestBuilder, Client};
use std::process::Command;
use crate::assets::Assets;

#[derive(Deserialize, Clone, Debug)]
struct Req {
    name: Option<String>,
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

#[derive(Deserialize, Clone, Debug, Default)]
struct Config {
    all: Option<bool>,
    ignore: Option<Vec<String>>,
    cors: Option<Vec<String>>,
    port: Option<u16>,
    cert: Option<PathBuf>,
    key: Option<PathBuf>,
    assets: Option<PathBuf>, 
    templates: Option<PathBuf>, 
    routes: Option<Vec<Route>>
}

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

async fn handler (
    state: State<AppState>,
    OriginalUri(url): OriginalUri,
    Params(params): Params<HashMap<String, String>>,
    Query(vars): Query<Value>,
    path: MatchedPath,
    headers: HeaderMap,
    method: Method,
    body: String,
) -> Result<(StatusCode, HeaderMap, String), AppError> {
    let routes = &state.routes;
    let env = &state.env;
    let mut context = json!({}); 
    let x = context.as_object_mut().ok_or("unreachable")?;
    x.insert(String::from("path"), json!(url.path()));
    x.insert(String::from("query"), json!(url.query()));
    x.insert(String::from("headers"), json!({}));
    let h = x.get_mut("headers").ok_or("unreachable")?
        .as_object_mut().ok_or("unreachable")?;
    for key in headers.keys() {
        let v = headers.get(key).ok_or("unreachable")?.to_str()?;
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
    for test in routes {
        if
            route.is_none() &&
            &test.method == method.as_str() &&
            &test.path == path.as_str()
        {
            route = Some(test.clone());
        }
    }
    let route = route.ok_or("no route matched!")?;

    for req in route.requests.ok_or("unreachable")? {
        let ctx = context.clone();
        let mut r = RequestBuilder::from_parts(Client::new(), Request::new(
            env.get_template(&req.method)?.render(&ctx)?.parse()?,
            env.get_template(&req.url)?.render(&ctx)?.parse()?
        ));
        if let Some(headers) = req.headers {
            for (key, value) in headers {
                r = r.header(key.clone(),
                    env.get_template(&value)?.render(&ctx)?
                );
            }
        }
        if let Some(body) = req.body {
            r = r.body(env.get_template(&body)?.render(&ctx)?);
        }
        let res = r.send().await?;
        let status = res.status();
        let headers = res.headers().clone();
        let body = res.text().await?;
        let d = context.get_mut("data").ok_or("unreachable")?
            .as_object_mut().ok_or("unreachable")?;
        let json = match serde_json::from_str(&body) {
            Ok(data) => data,
            Err(_) => json!(body)
        };

        if let Some(name) = req.name {
            d.insert(name.clone(), json!({
                "status": status.as_u16(),
                "headers": {},
                "body": body,
                "json": json
            }));

            let h = d.get_mut(&name).ok_or("unreachable")?
                .get_mut("headers").ok_or("unreachable")?
                .as_object_mut().ok_or("unreachable")?;
            for (key, value) in &headers {
                h.insert(key.to_string(), json!(value.to_str()?));
            }
        }
        response = (status, headers, body)
    }

    if let Some(res) = route.response {
        let ctx = context.clone();
        if let Some(status) = res.status {
            response.0 = env.get_template(&status)?.render(&ctx)?.parse()?;
        }
        if let Some(headers) = res.headers {
            let mut r = HeaderMap::new();
            for (key, value) in headers {
                r.insert(
                    HeaderName::from_bytes(key.as_bytes())?,
                    env.get_template(&value)?.render(&ctx)?.parse()?
                );
            }
            response.1 = r;
        }
        if let Some(body) = res.body {
            response.2 = env.get_template(&body)?.render(&ctx)?.parse()?;
        }
    }

    Ok(response)
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

    let mut env = Environment::new();
    fn cmd(cmd: String) -> String {
        let stdout = Command::new("sh")
            .arg("-c")
            .arg(&cmd)
            .output()
            .expect("failed to execute process")
            .stdout;
        let stdout = from_utf8(&stdout).expect("failed to parse output");
        stdout.to_string()
    }
    env.add_function("cmd", cmd);
    let mut config: Config = Default::default();
    let cors: Option<Vec<String>> = match cli.allow_cors {
        true => Some(Vec::new()),
        false => None
    };
    let mut ignore: Vec<String> = Vec::new();
    if let Some(glob) = cli.ignore {
        ignore.push(glob);
    }
    if let Some(p) = cli.config {
        let p = p.as_path();
        let data = read_to_string(p)?;
        config = toml::from_str(&data)?;
        let dir = p.parent().ok_or("unreachable")?;
        if let Some(templates) = config.templates {
            let templates = dir.join(templates);
            env.set_loader(path_loader(templates));
        }
        if let Some(assets) = config.assets {
            config.assets = Some(dir.join(assets));
        }
        if let Some(cert) = config.cert {
            config.cert = Some(dir.join(cert));
        }
        if let Some(key) = config.key {
            config.key = Some(dir.join(key));
        }
        if let Some(mut globs) = config.ignore {
            ignore.append(&mut globs);
        }
    }

    let mut app = Router::new();

    let mut r = 0;
    let mut routes: Vec<Route> = Vec::new();
    for route in config.routes.unwrap_or(Vec::new()) {
        let mut i = 0;
        let mut requests: Vec<Req> = Vec::new();
        for mut req in route.requests.unwrap_or(Vec::new()) {
            let name = format!("__r{}_{}_method__", r, i);
            env.add_template_owned(name.clone(), req.method.clone())?;
            req.method = name;

            let name = format!("__r{}_{}_url__", r, i);
            env.add_template_owned(name.clone(), req.url.clone())?;
            req.url = name;

            if let Some(body) = req.body {
                let name = format!("__r{}_{}_body__", r, i);
                req.body = Some(name.clone());
                env.add_template_owned(name, body.clone())?;
            }

            let mut headers = HashMap::new();
            for (key, value) in req.headers.unwrap_or(HashMap::new()) {
                let v = format!("__r{}_{}_header_{}__", r, i, key);
                env.add_template_owned(v.clone(), value.clone())?;

                headers.insert(key, v);
            }
            req.headers = Some(headers);
            requests.push(req);
            i = i+1;
        }

        let mut response: Option<Res> = None;
        if let Some(mut res) = route.response {
            if let Some(status) = res.status {
                let name = format!("__r{}_status__", r);
                env.add_template_owned(name.clone(), status)?;
                res.status = Some(name);
            }

            if let Some(body) = res.body {
                let name = format!("__r{}_body__", r);
                env.add_template_owned(name.clone(), body)?;
                res.body = Some(name);
            }

            let mut headers = HashMap::new();
            for (key, value) in res.headers.unwrap_or(HashMap::new()) {
                let v = format!("__r{}_header_{}__", r, key);
                env.add_template_owned(v.clone(), value.clone())?;

                headers.insert(key, v);
            }
            res.headers = Some(headers);
            response = Some(res);
        }
        routes.push(Route {
            method: route.method.clone(),
            path: route.path.clone(),
            requests: Some(requests),
            response: response
        });
        r = r+1;
    }
    //println!("{:#?}", routes);

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
        env,
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
