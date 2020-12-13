use super::constants;
use super::solvers::Config;
use std::fs::File;
use std::io::prelude::*;

pub fn read_data() -> Vec<Vec<f64>> {
    let mut f = File::open(constants::DATA_FILE).unwrap();
    let mut res = String::new();
    f.read_to_string(&mut res).unwrap();
    serde_json::from_str(&res).unwrap()
}

pub fn read_config() -> Config {
    let mut f = File::open(constants::CONFIG_FILE).unwrap();
    let mut res = String::new();
    f.read_to_string(&mut res).unwrap();
    serde_json::from_str(&res).unwrap()
}
