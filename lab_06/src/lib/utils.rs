use super::ant_solver::Config;
use super::Cost;
use rand::prelude::*;
use std::fs::File;
use std::io::prelude::*;

#[allow(dead_code)]
pub fn read_data(filename: &str) -> Vec<Vec<Cost>> {
    let mut f = File::open(filename).unwrap();
    let mut res = String::new();
    f.read_to_string(&mut res).unwrap();
    serde_json::from_str(&res).unwrap()
}

pub fn generate_data(size: usize, from: Cost, to: Cost) -> Vec<Vec<Cost>> {
    let mut rng = thread_rng();
    let mut data = vec![vec![0 as Cost; size]; size];
    for i in 0..size {
        for j in (i + 1)..size {
            let val = rng.gen_range(from, to);
            data[i][j] = val;
            data[j][i] = val;
        }
    }
    data
}

pub fn read_config(filename: &str) -> Config {
    let mut f = File::open(filename).unwrap();
    let mut res = String::new();
    f.read_to_string(&mut res).unwrap();
    serde_json::from_str(&res).unwrap()
}
