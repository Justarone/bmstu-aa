use std::env;
use std::io::prelude::*;
use rand::random;
use std::time::{Instant};
use std::fs::File;
use std::io::BufWriter;

use super::algorithms;

static DEFAULT_Z: usize = 10;

static RECURSIVE_ADAPTER: fn(&str, &str) -> usize = |s1: &str, s2: &str| algorithms::recursive(s1, s2).0;
static RECURSIVE_WITH_MEM_ADAPTER: fn(&str, &str) -> usize = 
    |s1: &str, s2: &str| algorithms::recursive_with_mem(s1, s2).0;
static ITERATIVE_ADAPTER: fn(&str, &str) -> usize = |s1: &str, s2: &str| algorithms::iterative(s1, s2).0;
static ITERATIVE_DL_ADAPTER: fn(&str, &str) -> usize = |s1: &str, s2: &str| algorithms::iterative_dl(s1, s2).0;

static FUNCS: [fn(&str, &str) -> usize; 4] = [ITERATIVE_ADAPTER, RECURSIVE_ADAPTER, 
RECURSIVE_WITH_MEM_ADAPTER, ITERATIVE_DL_ADAPTER];


pub fn measure_time(s1: &str, s2: &str, z: usize) -> String {
    let mut times = Vec::new();

    for &func in FUNCS.iter() {
        let time = Instant::now();

        for _ in 0..z {
            func(&s1, &s2);
        }

        times.push(time.elapsed().as_nanos());
    }

    let times: Vec<String> = times.iter().map(|num| num.to_string()).collect();
    times.join(" ")
}

pub fn write_to_file(content: &str, f: &mut BufWriter<File>) {
    writeln!(f, "{}", content);
}

pub fn get_z() -> usize {
    match env::var("Z") {
        Ok(val) => val.trim().parse().expect("Failed to parse Z value!"),
        Err(_) => DEFAULT_Z,
    }
}

pub fn generate_string_of_size(size: usize) -> String {
    (0..size).map(|_| (0x61u8 + (random::<f32>() * 22.0) as u8) as char).collect()
}

