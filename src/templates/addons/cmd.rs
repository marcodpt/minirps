use std::process::Command;
use std::str::from_utf8;
use minijinja::{Error, ErrorKind::InvalidOperation};

pub fn cmd(cmd: String) -> Result<String, Error> {
    let result = match Command::new("sh").arg("-c").arg(&cmd).output() {
        Ok(result) => result,
        Err(err) => {
            return Err(Error::new(
                InvalidOperation,
                format!("Fail to execute command:\n{}\n\n{:#}",
                    &cmd, err
                ))
            );
        }
    };

    let stdout = match from_utf8(&result.stdout) {
        Ok(stdout) => stdout,
        Err(err) => {
            return Err(Error::new(
                InvalidOperation,
                format!("Fail to parse command output:\n{}\n\n{:#}",
                    &cmd, err
                )
            ));
        }
    };

    Ok(stdout.to_string())
}
