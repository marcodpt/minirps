mod cmd;

use minijinja::Environment;
use cmd::cmd;

pub fn addons (env: &mut Environment) {
    env.add_function("cmd", cmd);
}
