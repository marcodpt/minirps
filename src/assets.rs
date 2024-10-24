use std::error::Error;
use std::path::{PathBuf, Path};
use std::fs::read;
use axum::{
    response::Response,
    http::StatusCode,
    http::header::{HeaderValue, CONTENT_TYPE},
};
use glob_match::glob_match;
use mime_guess;
use crate::debug::debug;

#[derive(Clone)]
pub struct Assets {
    all: bool,
    ignore: Vec<String>,
    dir: PathBuf
}

impl Assets {
    pub fn new (
        dir: PathBuf,
        all: bool,
        ignore: Vec<String>
    ) -> Result<Assets, Box<dyn Error>> {
        let p = dir.as_path();
        if !p.is_dir() {
            Err(format!("assets is not a dir: {}", p.display()).into())
        } else {
            Ok(Assets {
                dir,
                all,
                ignore
            })
        }
    }

    fn getter (&self, path_str: &str) -> Result<Response, StatusCode> {
        let path = Path::new(path_str);
        let dir = self.dir.as_path();
        let mut file = dir.join(path);
        let mut response: Response;

        if file.is_dir() {
            file = file.join("index.html");
        }

        if !file.starts_with(dir) || !file.is_file() {
            return Err(StatusCode::NOT_FOUND);
        }

        let path_str = path.to_str().unwrap_or("");
        for glob in &self.ignore {
            if glob_match(&glob, path_str) {
                return Err(StatusCode::NOT_FOUND);
            }
        }

        if !self.all {
            for component in path.components() {
                let name = component.as_os_str().to_str().unwrap_or("");

                if name.len() == 0 || (
                    name.len() > 1 &&
                    name.as_bytes()[0] == b'.'
                ) {
                    return Err(StatusCode::NOT_FOUND);
                } 
            }
        }

        match read(&file) {
            Err(_) => {
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            },
            Ok(body) => {
                response = Response::new(body.into());
            }
        };

        let mime = mime_guess::from_path(&file).first_raw().unwrap_or("");

        if mime.len() > 0 {
            match HeaderValue::from_str(mime) {
                Ok(mime) => {
                    response.headers_mut().insert(CONTENT_TYPE, mime);
                },
                Err(_) => {}
            };
        }

        Ok(response)
    }

    pub fn get (&self, path_str: &str) -> Result<Response, StatusCode> {
        let path = format!("/{}", path_str);
        match self.getter(path_str) {
            Ok(response) => {
                debug("GET", &path, Some(200), "");
                Ok(response)
            },
            Err(status) => {
                debug("GET", &path, Some(status.as_u16()), "");
                Err(status)
            }
        }
    }
}
