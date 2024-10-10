mod parse;
mod command;
mod file;
mod fetch;

use minijinja::Environment;
use parse::parse;
use command::command;
use file::{read};
use fetch::{get, delete, head, options, post, put, patch};

pub fn addons (env: &mut Environment) {
    env.add_filter("parse", parse);
    env.add_function("command", command);
    env.add_function("read", read);
    env.add_function("get", get);
    env.add_function("delete", delete);
    env.add_function("head", head);
    env.add_function("options", options);
    env.add_function("post", post);
    env.add_function("put", put);
    env.add_function("patch", patch);
}
