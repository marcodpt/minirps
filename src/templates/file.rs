use std::fs;
use std::path::Path;
use minijinja::Value;
use serde_derive::Serialize;

#[derive(Serialize)]
struct File {
    is_dir: bool,
    is_file: bool,
    is_symlink: bool,
    name: String,
    len: u64
}

pub fn read (entry: &str) -> Option<Value> {
    let path = Path::new(entry);

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
                            if let Ok(meta) = p.metadata() {
                                len = meta.len();
                            }
                            files.push(File {
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

pub fn write (file: &str, data: &Vec<u8>) -> Option<String> {
    let path = Path::new(file);

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

pub fn remove (entry: &str) -> Option<String> {
    let path = Path::new(entry);

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
