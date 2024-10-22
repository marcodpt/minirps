use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::error::Error;
use minijinja::Value;
use serde_derive::Serialize;
use chrono::{DateTime, Local};

#[derive(Serialize)]
struct File {
    accessed: String,
    created: String,
    modified: String,
    is_dir: bool,
    is_file: bool,
    is_symlink: bool,
    name: String,
    len: u64
}

#[derive(Clone)]
pub struct IO {
    dir: PathBuf
}

impl IO {
    pub fn new (dir: PathBuf) -> Result<IO, Box<dyn Error>> {
        let p = dir.as_path();
        if p.is_dir() {
            Ok(IO {dir})
        } else {
            Err(format!("Data must be a directory: {}", p.display()).into())
        }
    }

    fn get_path (&self, path: &str) -> Option<PathBuf> {
        let dir = self.dir.as_path();
        let mut path = Path::new(path);
        if let Ok(p) = path.strip_prefix("/") {
            path = p
        }
        let path = dir.join(path);

        if path.starts_with(dir) {
            Some(path)
        } else {
            None
        }
    }

    pub fn read (&self, entry: &str) -> Option<Value> {
        let path = match self.get_path(entry) {
            Some(path) => path,
            None => {
                return None
            }
        };
        let path = path.as_path();

        if path.try_exists().unwrap_or(false) {
            if path.is_dir() {
                let mut files: Vec<File> = Vec::new();
                match fs::read_dir(path) {
                    Ok(entries) => {
                        for entry in entries {
                            if let Ok(entry) = entry {
                                let p = entry.path();
                                let mut fname = String::new();
                                if let Some(name) = p.file_name() {
                                    if let Some(name) = name.to_str() {
                                        fname = name.to_string();
                                    }
                                }
                                let mut len: u64 = 0;
                                let mut accessed = String::new();
                                let mut created = String::new();
                                let mut modified = String::new();
                                if let Ok(meta) = p.metadata() {
                                    len = meta.len();
                                    if let Ok(time) = meta.accessed() {
                                        let time: DateTime<Local> =
                                            time.into();
                                        accessed = time
                                            .format("%Y-%m-%d %H:%M")
                                            .to_string();
                                    }
                                    if let Ok(time) = meta.created() {
                                        let time: DateTime<Local> =
                                            time.into();
                                        created = time
                                            .format("%Y-%m-%d %H:%M")
                                            .to_string();
                                    }
                                    if let Ok(time) = meta.modified() {
                                        let time: DateTime<Local> =
                                            time.into();
                                        modified = time
                                            .format("%Y-%m-%d %H:%M")
                                            .to_string();
                                    }
                                }
                                files.push(File {
                                    accessed,
                                    created,
                                    modified,
                                    is_dir: p.is_dir(),
                                    is_file: p.is_file(),
                                    is_symlink: p.is_symlink(),
                                    name: fname,
                                    len
                                });
                            }
                        }
                        Some(Value::from_serialize(files))
                    },
                    Err(_) => None
                }
            } else {
                match fs::read(path) {
                    Ok(data) => Some(data.into()),
                    Err(_) => None
                }
            }
        } else {
            None
        }
    }

    pub fn write (&self, file: &str, data: &Vec<u8>) -> Option<String> {
        let path = match self.get_path(file) {
            Some(path) => path,
            None => {
                return Some(format!("Unable to resolve file: {}", file))
            }
        };
        let path = path.as_path();

        if let Some(dir) = path.parent() {
            if !dir.exists() {
                if let Err(err) = fs::create_dir_all(dir) {
                    return Some(format!("Unable to create dir: {}\n{:#}",
                        dir.display(), err
                    ));
                }
            }
        }

        match fs::write(path, data) {
            Ok(_) => None,
            Err(err) => Some(format!("Unable to write file: {}\n{:#}",
                file, err
            ))
        }
    }

    pub fn remove (&self, entry: &str) -> Option<String> {
        let path = match self.get_path(entry) {
            Some(path) => path,
            None => {
                return Some(format!("Unable to resolve path: {}", entry))
            }
        };
        let path = path.as_path();

        if path.is_dir() {
            match fs::remove_dir_all(path) {
                Ok(_) => None,
                Err(err) => Some(format!(
                    "Unable to remove dir: {}\n{:#}", entry, err
                ))
            }
        } else {
            match fs::remove_file(path) {
                Ok(_) => None,
                Err(err) => Some(format!(
                    "Unable to remove file: {}\n{:#}", entry, err
                ))
            }
        }
    }
} 
