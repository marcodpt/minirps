use std::fs;
use std::path::{Path};
use std::str::from_utf8;
use minijinja::{Error, ErrorKind::InvalidOperation, Value};

pub fn read (entry: &str) -> Result<Value, Error> {
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
                    Ok(files.into())
                },
                Err(err) => Err(Error::new(
                    InvalidOperation,
                    format!("Fail to open directory: {}\n{:#}", entry, err)
                ))
                
            }
        } else {
            match fs::read(path) {
                Ok(data) => {
                    match from_utf8(&data) {
                        Ok(data) => Ok(data.into()),
                        Err(err) => Err(Error::new(
                            InvalidOperation,
                            format!("Binary file: {}\n{:#}", entry, err)
                        ))
                    }
                },
                Err(err) => Err(Error::new(
                    InvalidOperation,
                    format!("Fail to read file: {}\n{:#}", entry, err)
                ))
            }
        }
    } else {
        Err(Error::new(
            InvalidOperation,
            format!("File <{}> does not exist!", entry)
        ))
    }
}

/*pub fn write (path: &str, data: &str) -> Result<(), Error> {

}

pub fn rm (path: &str) -> Result<(), Error> {

}*/
