#![feature(test)]

use std::env;

mod lib;

fn main() {
    if env::var("TEST_MODE").is_ok() {
        lib::run_tests();
    } else {
        lib::run_interactive();
    }
}
