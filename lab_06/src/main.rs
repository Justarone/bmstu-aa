mod lib;
use lib::{run_interactive, run_parametrization, run_time};
use std::env;

fn main() {
    if env::var("INTERACTIVE").is_ok() {
        run_interactive();
    } else if env::var("PARAMETRIZATION").is_ok() {
        run_parametrization();
    } else {
        run_time();
    }
}
