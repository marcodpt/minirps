use serde_derive::Serialize;
use std::process::Command;
use minijinja::Value;

#[derive(Serialize)]
struct Output {
    code: i32,
    stdout: Vec<u8>,
    stderr: Vec<u8>
}

pub fn command(command: String) -> Value {
    let result = match Command::new("sh").arg("-c").arg(&command).output() {
        Ok(result) => result,
        Err(err) => {
            return Value::from_serialize(Output {
                code: 999999,
                stdout: format!(
                    "Fail to execute command!"
                ).as_bytes().to_vec(),
                stderr: err.to_string().as_bytes().to_vec()
            });
        }
    };

    Value::from_serialize(Output {
        code: result.status.code().unwrap_or(0),
        stdout: result.stdout,
        stderr: result.stderr
    })
}
