#![feature(test)]

use std::env;

mod lib;

fn main() {
    if env::var("USER_MODE").is_ok() {
        lib::run_user();
    } else {
        lib::run_tests();
    }
    
}
