use std::fs::File;
use std::io::BufWriter;
use std::time::Instant;

mod algorithms;
mod test_utils;
mod user_utils;

static SIZES_TO_CHECK: [usize; 6] = [10, 20, 30, 50, 100, 200];


pub fn run_user() {
    let (s1, s2) = user_utils::read_data();
    let time = Instant::now();
    let (score, depth) = algorithms::recursive(&s1, &s2);
    println!("Recursive algorithm call:\nResult: {}\nMemory: {}\nTime: {} nanos",
        score, user_utils::count_recursive_memory(depth), time.elapsed().as_nanos());

    let (score, depth, matrix) = algorithms::recursive_with_mem(&s1, &s2);
    let time = Instant::now();
    println!("\nRecursive algorithm with memoization call:\nResult: {}\nMemory: {}\nTime: {} nanos",
        score, user_utils::count_recursive_with_mem_memory(depth), time.elapsed().as_nanos());
    user_utils::print_matrix(&matrix);

    let time = Instant::now();
    let (score, matrix) = algorithms::iterative(&s1, &s2);
    println!("\nIterative algorithm call:\nResult: {}\nTime: {} nanos",
        score, time.elapsed().as_nanos());
    user_utils::print_matrix(&matrix);

    let time = Instant::now();
    let (score, matrix) = algorithms::iterative_dl(&s1, &s2);
    println!("\nIterative DL algorithm call:\nResult: {}\nTime: {} nanos",
        score, time.elapsed().as_nanos());
    user_utils::print_matrix(&matrix);
}


pub fn run_tests() {
    let z = test_utils::get_z();
    let f = File::create("res.txt").expect("Failed to create file!");
    let mut f = BufWriter::new(f);

    for &size in SIZES_TO_CHECK.iter() {
        println!("Running for size: {}", size);
        let s1 = test_utils::generate_string_of_size(size);
        let s2 = test_utils::generate_string_of_size(size);

        let times = test_utils::measure_time(&s1, &s2, z);
        test_utils::write_to_file(&times, &mut f);
    }
}
