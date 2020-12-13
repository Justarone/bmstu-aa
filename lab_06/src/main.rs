mod lib;
use lib::{run_parametrization, run_time};
use std::env;

fn main() {
    if env::var("PARAMETRIZATION").is_ok() {
        run_parametrization();
    } else {
        run_time();
    }
}
