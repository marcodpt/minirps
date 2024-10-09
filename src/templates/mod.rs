mod addons;

use std::path::{PathBuf};
use minijinja::{Error, Environment, path_loader, value::Value};
use addons::addons;

type Env = Environment<'static>;
pub struct Engine {
    env: Env
}

impl Engine {
    pub fn new (dir: PathBuf) -> Result<Engine, Error> {
        let mut env = Environment::new();
        env.set_loader(path_loader(dir));

        addons(&mut env);

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
