#![feature(assoc_char_funcs)]

extern crate termion;

use std::env;

mod lib;

fn main() {
    env_logger::init();

    if env::var("INTERACTIVE_MODE").is_ok() {
        lib::run_interactive();
    } else {
        lib::run_tests();
    }
}
