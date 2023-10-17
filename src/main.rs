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
    response::{IntoResponse, Response},
    extract::{OriginalUri, Path as Params, Query, State, MatchedPath},
    routing::{get, on},
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

fn gen_config (path: &PathBuf) -> Result<(), Box<dyn Error>> {
    let p = path.as_path();
    if p.exists() {
        return Err(format!("File already exists `{}`", p.display()).into());
    }
    let e = format!("File must have .toml extension `{}`", p.display());
    let ext = p.extension().ok_or(e.clone())?;
    if ext != "toml" {
        return Err(e.into());
    }
    write(p, include_str!("../tests/default.toml"))?;
    println!("New config file generated: `{}`", p.display());
    Ok(())
}

fn build_assets (
    base: &Path, dir: &Path, mut app: Router<AppState>
) -> Result<Router<AppState>, Box<dyn Error>> {
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
            let name = route.file_name().ok_or("unreachable")?
                .to_str().ok_or("unreachable")?;
            let root = route.parent().ok_or("unreachable")?
                .to_str().ok_or("unreachable")?;
            let route = route.to_str().ok_or("unreachable")?;
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

async fn start_server (path: &PathBuf) -> Result<(), Box<dyn Error>> {
    let p = path.as_path();
    let data = read_to_string(p)?;
    let config: Config = toml::from_str(&data)?;
    let dir = p.parent().ok_or("unreachable")?;
    let mut env = Environment::new();
    //println!("{:#?}", config);

    let mut app = Router::new();

    if let Some(templates) = config.templates {
        let templates = dir.join(templates);
        env.set_loader(path_loader(templates));
    }

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

    if let Some(assets) = config.assets {
        let assets = dir.join(assets);
        app = build_assets(&assets, &assets, app)?;
    }

    for route in &routes {
        app = app.route(&route.path, on(
            Method::from_bytes(route.method.as_bytes())?.try_into()?,
            handler
        ));
    }

    if let Some(origins) = config.cors {
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

    let state = AppState {
        routes: routes,
        env: env,
    };
    let app = app.with_state(state);

    let port = config.port.unwrap_or(3000);
    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    if config.cert.is_some() && config.key.is_some() {
        let cert = dir.join(config.cert.ok_or("unreachable")?);
        let key = dir.join(config.key.ok_or("unreachable")?);
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
