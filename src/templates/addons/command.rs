use serde_derive::Serialize;
use std::process::Command;
use std::str::from_utf8;
use minijinja::{Error, ErrorKind::InvalidOperation, Value};

fn parser(data: &Vec<u8>) -> Result<String, Error> {
    match from_utf8(&data) {
        Ok(data) => Ok(data.to_string()),
        Err(err) => Err(Error::new(
            InvalidOperation,
            format!("Fail to parse command result!\n{:#}", err)
        ))
    }
}

#[derive(Serialize)]
struct Output {
    code: Option<i32>,
    stdout: String,
    stderr: String
}

pub fn command(command: String) -> Result<Value, Error> {
    let result = match Command::new("sh").arg("-c").arg(&command).output() {
        Ok(result) => result,
        Err(err) => {
            return Err(Error::new(
                InvalidOperation,
                format!("Fail to execute command!\n\n{:#}", err)
            ));
        }
    };

    Ok(Value::from_serialize(Output {
        code: result.status.code(),
        stdout: parser(&result.stdout)?,
        stderr: parser(&result.stderr)?
    }))
}
