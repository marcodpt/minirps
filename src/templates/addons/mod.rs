mod cmd;
mod parse;

use minijinja::Environment;
use cmd::cmd;
use parse::parse;

pub fn addons (env: &mut Environment) {
    env.add_filter("parse", parse);
    env.add_function("cmd", cmd);
}
