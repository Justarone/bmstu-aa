use std::env;
use std::io::prelude::*;
use rand::random;
use std::time::{Instant};
use std::fs::File;
use std::io::BufWriter;

extern crate test;
use test::Bencher;

use super::algorithms;

static DEFAULT_Z: usize = 10;

static RECURSIVE_ADAPTER: fn(&[char], &[char]) -> usize = |s1: &[char], s2: &[char]| algorithms::recursive(s1, s2).0;
static RECURSIVE_WITH_MEM_ADAPTER: fn(&[char], &[char]) -> usize = 
    |s1: &[char], s2: &[char]| algorithms::recursive_with_mem(s1, s2).0;
static ITERATIVE_ADAPTER: fn(&[char], &[char]) -> usize = |s1: &[char], s2: &[char]| algorithms::iterative(s1, s2).0;
static ITERATIVE_DL_ADAPTER: fn(&[char], &[char]) -> usize = |s1: &[char], s2: &[char]| algorithms::iterative_dl(s1, s2).0;

static FUNCS: [fn(&[char], &[char]) -> usize; 4] = [ITERATIVE_ADAPTER, RECURSIVE_ADAPTER, 
RECURSIVE_WITH_MEM_ADAPTER, ITERATIVE_DL_ADAPTER];
//static FUNCS: [fn(&str, &str) -> usize; 1] = [RECURSIVE_ADAPTER]; // just recursion


pub fn measure_time(s1: &[char], s2: &[char], z: usize) -> String {
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
    writeln!(f, "{}", content).expect("Can't write to file!");
}

pub fn get_z() -> usize {
    match env::var("Z") {
        Ok(val) => val.trim().parse().expect("Failed to parse Z value!"),
        Err(_) => DEFAULT_Z,
    }
}

pub fn generate_string_of_size(size: usize) -> Vec<char> {
    (0..size).map(|_| (0x61u8 + (random::<f32>() * 22.0) as u8) as char).collect()
}

#[cfg(test)]
mod benchs {
    use super::*;

    #[bench]
    fn iterative10(b: &mut Bencher) {
        let s1 = generate_string_of_size(10);
        let s2 = generate_string_of_size(10);
        b.iter(|| algorithms::iterative(&s1, &s2));
    }

    #[bench]
    fn iterative20(b: &mut Bencher) {
        let s1 = generate_string_of_size(20);
        let s2 = generate_string_of_size(20);
        b.iter(|| algorithms::iterative(&s1, &s2));
    }

    #[bench]
    fn iterative30(b: &mut Bencher) {
        let s1 = generate_string_of_size(30);
        let s2 = generate_string_of_size(30);
        b.iter(|| algorithms::iterative(&s1, &s2));
    }

    #[bench]
    fn iterative50(b: &mut Bencher) {
        let s1 = generate_string_of_size(50);
        let s2 = generate_string_of_size(50);
        b.iter(|| algorithms::iterative(&s1, &s2));
    }

    #[bench]
    fn iterative100(b: &mut Bencher) {
        let s1 = generate_string_of_size(100);
        let s2 = generate_string_of_size(100);
        b.iter(|| algorithms::iterative(&s1, &s2));
    }

    #[bench]
    fn iterative200(b: &mut Bencher) {
        let s1 = generate_string_of_size(200);
        let s2 = generate_string_of_size(200);
        b.iter(|| algorithms::iterative(&s1, &s2));
    }

    #[bench]
    fn recursive5(b: &mut Bencher) {
        let s1 = generate_string_of_size(5);
        let s2 = generate_string_of_size(5);
        b.iter(|| algorithms::recursive(&s1, &s2));
    }

    #[bench]
    fn recursive10(b: &mut Bencher) {
        let s1 = generate_string_of_size(10);
        let s2 = generate_string_of_size(10);
        b.iter(|| algorithms::recursive(&s1, &s2));
    }

    //#[bench]
    //fn recursive15(b: &mut Bencher) {
        //let s1 = generate_string_of_size(15);
        //let s2 = generate_string_of_size(15);
        //b.iter(|| algorithms::recursive(&s1, &s2));
    //}

    #[bench]
    fn recursive_with_mem10(b: &mut Bencher) {
        let s1 = generate_string_of_size(10);
        let s2 = generate_string_of_size(10);
        b.iter(|| algorithms::recursive_with_mem(&s1, &s2));
    }

    #[bench]
    fn recursive_with_mem20(b: &mut Bencher) {
        let s1 = generate_string_of_size(20);
        let s2 = generate_string_of_size(20);
        b.iter(|| algorithms::recursive_with_mem(&s1, &s2));
    }

    #[bench]
    fn recursive_with_mem30(b: &mut Bencher) {
        let s1 = generate_string_of_size(30);
        let s2 = generate_string_of_size(30);
        b.iter(|| algorithms::recursive_with_mem(&s1, &s2));
    }

    #[bench]
    fn recursive_with_mem50(b: &mut Bencher) {
        let s1 = generate_string_of_size(50);
        let s2 = generate_string_of_size(50);
        b.iter(|| algorithms::recursive_with_mem(&s1, &s2));
    }

    #[bench]
    fn recursive_with_mem100(b: &mut Bencher) {
        let s1 = generate_string_of_size(100);
        let s2 = generate_string_of_size(100);
        b.iter(|| algorithms::recursive_with_mem(&s1, &s2));
    }

    #[bench]
    fn recursive_with_mem200(b: &mut Bencher) {
        let s1 = generate_string_of_size(200);
        let s2 = generate_string_of_size(200);
        b.iter(|| algorithms::recursive_with_mem(&s1, &s2));
    }

    #[bench]
    fn iterative_dl10(b: &mut Bencher) {
        let s1 = generate_string_of_size(10);
        let s2 = generate_string_of_size(10);
        b.iter(|| algorithms::iterative_dl(&s1, &s2));
    }

    #[bench]
    fn iterative_dl20(b: &mut Bencher) {
        let s1 = generate_string_of_size(20);
        let s2 = generate_string_of_size(20);
        b.iter(|| algorithms::iterative_dl(&s1, &s2));
    }

    #[bench]
    fn iterative_dl30(b: &mut Bencher) {
        let s1 = generate_string_of_size(30);
        let s2 = generate_string_of_size(30);
        b.iter(|| algorithms::iterative_dl(&s1, &s2));
    }

    #[bench]
    fn iterative_dl50(b: &mut Bencher) {
        let s1 = generate_string_of_size(50);
        let s2 = generate_string_of_size(50);
        b.iter(|| algorithms::iterative_dl(&s1, &s2));
    }

    #[bench]
    fn iterative_dl100(b: &mut Bencher) {
        let s1 = generate_string_of_size(100);
        let s2 = generate_string_of_size(100);
        b.iter(|| algorithms::iterative_dl(&s1, &s2));
    }

    #[bench]
    fn iterative_dl200(b: &mut Bencher) {
        let s1 = generate_string_of_size(200);
        let s2 = generate_string_of_size(200);
        b.iter(|| algorithms::recursive_with_mem(&s1, &s2));
    }
}
