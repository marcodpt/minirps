use std::fs;
use std::path::Path;
use minijinja::Value;

pub fn read (entry: &str) -> Option<Value> {
    let path = Path::new(entry);

    if path.try_exists().unwrap_or(false) {
        if path.is_dir() {
            let mut files: Vec<String> = Vec::new();
            match fs::read_dir(path) {
                Ok(entries) => {
                    for entry in entries {
                        if let Ok(entry) = entry {
                            if let Some(name) = entry.path().file_name() {
                                if let Some(name) = name.to_str() {
                                    files.push(name.to_string());
                                }
                            }
                        }
                    }
                    Some(files.into())
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
