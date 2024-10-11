use toml;
use serde_json;
use serde_derive::Deserialize;
use std::error::Error;
use std::path::{Path, PathBuf};
use std::ffi::OsStr;
use std::fs::read_to_string;

#[derive(Deserialize, Clone, Debug)]
pub struct Route {
    pub method: String,
    pub path: String,
    pub template: String
}

#[derive(Deserialize, Clone, Debug, Default)]
pub struct Config {
    pub all: Option<bool>,
    pub ignore: Option<Vec<String>>,
    pub cors: Option<Vec<String>>,
    pub port: Option<u16>,
    pub cert: Option<PathBuf>,
    pub key: Option<PathBuf>,
    pub assets: Option<PathBuf>, 
    pub templates: Option<PathBuf>, 
    pub routes: Option<Vec<Route>>
}

impl Config {
    pub fn new(file: Option<&Path>) -> Result<Config, Box<dyn Error>> {
        match file {
            None => Ok(Default::default()),
            Some(file) => {
                let data = match read_to_string(file) {
                    Ok(data) => data,
                    Err(err) => {
                        return Err(format!(
                            "Unable to read <{}>\n{:#}",
                            file.display(), err
                        ).into());
                    }
                };

                let mut config: Config = match file.extension().unwrap_or(
                    OsStr::new("")
                ).to_str() {
                    Some("json") => {
                        match serde_json::from_str(&data) {
                            Ok(config) => config,
                            Err(err) => {
                                return Err(format!(
                                    "Unable to parse config file <{}>!\n{:#}",
                                    file.display(), err
                                ).into());
                            }
                        }
                    },
                    Some("toml") => {
                        match toml::from_str(&data) {
                            Ok(config) => config,
                            Err(err) => {
                                return Err(format!(
                                    "Unable to parse config file <{}>!\n{:#}",
                                    file.display(), err
                                ).into());
                            }
                        }
                    },
                    _ => {
                        return Err(format!(
                            "Configuration file <{}> must be .json or .toml",
                            file.display()
                        ).into());
                    }
                };

                if let Some(dir) = file.parent() {
                    if let Some(templates) = config.templates {
                        config.templates = Some(dir.join(templates));
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
                }

                Ok(config)
            }
        }
    }
}
