extern crate termion;

use std::env;

mod lib;

fn main() {
    env_logger::init();

    if env::var("TEST_MODE").is_ok() {
        lib::run_tests();
    } else {
        lib::run_interactive();
    }
}
