mod parse;
mod command;
mod file;
mod fetch;

use minijinja::{Environment, path_loader};
use parse::parse;
use command::command;
use file::{read, write, remove};
use fetch::{get, delete, head, options, post, put, patch};
use std::path::{PathBuf};

pub fn new (dir: Option<PathBuf>) -> Environment<'static> {
    let mut env = Environment::new();

    if let Some(dir) = dir {
        env.set_loader(path_loader(dir));

        env.add_filter("parse", parse);
        env.add_function("command", command);
        env.add_function("read", read);
        env.add_function("write", write);
        env.add_function("remove", remove);
        env.add_function("get", get);
        env.add_function("delete", delete);
        env.add_function("head", head);
        env.add_function("options", options);
        env.add_function("post", post);
        env.add_function("put", put);
        env.add_function("patch", patch);
    }

    env
}
