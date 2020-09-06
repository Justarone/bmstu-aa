use std::io;
use std::io::prelude::*;

pub fn read_data() -> (String, String) {
    let mut s1 = String::new();
    print!("Enter first line: ");
    io::stdout().flush().expect("Something went wrong during flushing stdout!");
    io::stdin().read_line(&mut s1).expect("Can't read string!");

    let mut s2 = String::new();
    print!("Enter second line: ");
    io::stdout().flush().expect("Something went wrong during flushing stdout!");
    io::stdin().read_line(&mut s2).expect("Can't read string!");

    if s1.len() != s1.chars().count() || s2.len() != s2.chars().count() {
        panic!("Can't work with non-ASCII symbols!");
    }

    (s1, s2)
}

// 4 local vars (usize x4 = 32 bytes)
// 2 string slices arguments (16 * 2 = 32 bytes)
// 1 usize argument (8 bytes)
// 1 address to return back (8 bytes)
// 2 usize values to return (16 bytes)
// total: 92 bytes
pub fn count_recursive_memory(depth: usize) -> usize {
    (depth + 1) * 92
}

// 4 local vars (usize x4 = 32 bytes)
// 2 string slices arguments (16 * 2 = 32 bytes)
// 1 usize argument (8 bytes)
// 1 matrix ref (8 bytes)
// 1 address to return back (8 bytes)
// 2 usize values to return (16 bytes)
// total: 100 bytes
pub fn count_recursive_with_mem_memory(depth: usize) -> usize {
    (depth + 1) * 100
}

pub fn print_matrix(matrix: &[Vec<usize>]) {
    let matrix_size = if matrix.len() == 0 { 0 } else { matrix.len() * matrix[0].len() };
    println!("Matrix size: {}", matrix_size);
    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            print!("{:4}", matrix[i][j]);
        }
        println!("");
    }
}
