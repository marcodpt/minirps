use std::path::{PathBuf};
use minijinja::{Error, Environment, path_loader, value::Value};
use std::str::from_utf8;
use std::process::Command;

type Env = Environment<'static>;
pub struct Engine {
    env: Env
}

impl Engine {
    pub fn new (dir: PathBuf) -> Result<Engine, Error> {
        let mut env = Environment::new();
        env.set_loader(path_loader(dir));

        fn cmd(cmd: String) -> String {
            let stdout = Command::new("sh")
                .arg("-c")
                .arg(&cmd)
                .output()
                .expect("failed to execute process")
                .stdout;
            let stdout = from_utf8(&stdout).expect("failed to parse output");
            stdout.to_string()
        }
        env.add_function("cmd", cmd);

        Ok(Engine {
            env
        })
    }

    pub fn render (
        &self,
        template: &str,
        context: &Value
    ) -> Result<String, Error> {
        self.env.get_template(template)?.render(context)
    }
}
