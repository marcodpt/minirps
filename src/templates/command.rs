use serde_derive::Serialize;
use std::process::Command;
use minijinja::{Error, ErrorKind::InvalidOperation, Value};

#[derive(Serialize)]
struct Output {
    code: Option<i32>,
    stdout: Vec<u8>,
    stderr: Vec<u8>
}

pub fn command(
    command: String,
    full: Option<bool>
) -> Result<Value, Error> {
    let result = match Command::new("sh").arg("-c").arg(&command).output() {
        Ok(result) => result,
        Err(err) => {
            return Err(Error::new(
                InvalidOperation,
                format!("Fail to execute command!\n\n{:#}", err)
            ));
        }
    };

    if full.unwrap_or(false) {
        Ok(Value::from_serialize(Output {
            code: result.status.code(),
            stdout: result.stdout,
            stderr: result.stderr
        }))
    } else {
        Ok(result.stdout.into())
    }
}
