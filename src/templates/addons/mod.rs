mod command;
mod parse;

use minijinja::Environment;
use command::command;
use parse::parse;

pub fn addons (env: &mut Environment) {
    env.add_filter("parse", parse);
    env.add_function("command", command);
}
