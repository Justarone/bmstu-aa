use std::env;
use std::io;
use std::io::prelude::*;
use rand::random;
use std::fs::File;
use std::io::BufWriter;
use std::time::{Duration, Instant};

mod algorithms;

static SIZES_TO_CHECK: [usize; 6] = [10, 20, 30, 50, 100, 200];
static DEFAULT_Z: usize = 10;

static RECURSIVE_ADAPTER: fn(&str, &str) -> usize = |s1: &str, s2: &str| algorithms::recursive(s1, s2).0;
static RECURSIVE_WITH_MEM_ADAPTER: fn(&str, &str) -> usize = 
    |s1: &str, s2: &str| algorithms::recursive_with_mem(s1, s2).0;

static FUNCS: [fn(&str, &str) -> usize; 4] = [algorithms::iterative, RECURSIVE_ADAPTER, 
RECURSIVE_WITH_MEM_ADAPTER, algorithms::iterative_dl];

fn generate_string_of_size(size: usize) -> String {
    (0..size).map(|_| (0x61u8 + (random::<f32>() * 22.0) as u8) as char).collect()
}

fn read_data() -> (String, String) {
    let mut s1 = String::new();
    print!("Enter first line: ");
    io::stdout().flush().expect("Something went wrong during flushing stdout!");
    io::stdin().read_line(&mut s1).expect("Can't read string!");

    let mut s2 = String::new();
    print!("Enter first line: ");
    io::stdout().flush().expect("Something went wrong during flushing stdout!");
    io::stdin().read_line(&mut s2).expect("Can't read string!");

    (s1, s2)
}

pub fn run_user() {
    let (s1, s2) = read_data();
}

fn measure_time(s1: &str, s2: &str, z: usize) -> String {
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

fn write_to_file(content: &str, f: &mut BufWriter<File>) {
    writeln!(f, "{}", content);
}

fn get_z() -> usize {
    match env::var("Z") {
        Ok(val) => val.trim().parse().expect("Failed to parse Z value!"),
        Err(_) => DEFAULT_Z,
    }
}

pub fn run_tests() {
    let z = get_z();
    let f = File::create("res.txt").expect("Failed to create file!");
    let mut f = BufWriter::new(f);

    for &size in SIZES_TO_CHECK.iter() {
        println!("Running for size: {}", size);
        let s1 = generate_string_of_size(size);
        let s2 = generate_string_of_size(size);

        let times = measure_time(&s1, &s2, z);
        write_to_file(&times, &mut f);
    }
}
