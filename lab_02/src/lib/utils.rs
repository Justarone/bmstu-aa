use std::io::prelude::*;
use super::algorithms::*;
use super::algorithms::utils::generate_matrix;
use std::time::Instant;
use std::env;
use std::vec::Vec;

const DEFAULT_ITERS: usize = 100;
const DEFAULT_SIZE: usize = 100;

fn input_matrix(s1: usize, s2: usize) -> Vec<Vec<MatInner>> {
    let mut matrix = Vec::new();

    println!("Введите через пробел элементы матрицы размеров {}x{}:", s1, s2);

    for i in 0..s1 {
        let mut row = String::new();
        print!("{}: ", i + 1);
        std::io::stdout().flush().expect("Невозможно напечатать номер строки!");
        std::io::stdin().read_line(&mut row).expect("Невозможно считать строку!");

        let row: Vec<_> = row.split_ascii_whitespace()
            .map(|elem| elem.parse::<MatInner>().expect("Невозможно преобразовать в число!"))
            .collect();

        if row.len() != s2 {
            panic!("Введено неверное кол-во элементов для столбца!");
        } else {
            matrix.push(row);
        }
    }

    matrix
}

pub fn read_data() -> (Vec<Vec<MatInner>>, Vec<Vec<MatInner>>) {
    print!("Введите n, k, m через пробел (матрицы в таком случае будут n x k, k x m): ");
    std::io::stdout().flush().expect("Невозможно напечатать приглашение!");
    let mut sizes = String::new();
    std::io::stdin().read_line(&mut sizes).expect("Невозможно считать строку!");

    let sizes: Vec<_> = sizes.split_ascii_whitespace()
        .map(|elem| elem.parse::<usize>().expect("Невозможно преобразовать в число!"))
        .collect();
    let sizes = if sizes.len() >= 3 {
        [sizes[0], sizes[1], sizes[2]]
    } else {
        panic!("Что-то не так с размерами матрицы!")
    };

    let m1 = input_matrix(sizes[0], sizes[1]);
    let m2 = input_matrix(sizes[1], sizes[2]);

    (m1, m2)
}

pub fn get_n() -> usize {
    match env::var("N") {
        Ok(value) => value.parse().unwrap(),
        Err(_) => DEFAULT_ITERS
    }
}

pub fn get_sizes() -> [usize; 3] {
    let mut sizes: [usize; 3] = [0, 0, 0];

    for i in 0..3 {
        sizes[i] = match env::var(String::from("SIZE") + &(i + 1).to_string()) {
            Ok(value) => value.parse().unwrap(),
            Err(_) => DEFAULT_SIZE
        }
    }

    sizes
}

pub fn measure(n: usize, sizes: [usize; 3]) {
    println!("Производится {0} замеров для матриц размера {1} x {2}, {2} x {3}", n, sizes[0], sizes[1], sizes[2]);

    let m1 = generate_matrix(sizes[0], sizes[1]);
    let m2 = generate_matrix(sizes[1], sizes[2]);

    for (algorithm, description) in MULTS_ARRAY.iter().zip(MULTS_DESCRIPTIONS.iter()) {

        let time = Instant::now();
        for i in 0..n {
            println!("Measurement №{}", i + 1);
            algorithm(&m1, &m2);
        }
        let time = time.elapsed().as_nanos() as usize / n;

        print!("{}: ", description);
        println!("\nВремя: {} нс", time);
    }
}

fn print_matrix<T: std::fmt::Display>(matrix: &[Vec<T>]) {
    print!("{{\n");

    for row in matrix.iter() {
        for elem in row.iter() {
            print!("{} ", elem);
        }
        println!("");
    }

    print!("}}");
}

pub fn run_tests(m1: &[Vec<MatInner>], m2: &[Vec<MatInner>]) {
    for (algorithm, description) in MULTS_ARRAY.iter().zip(MULTS_DESCRIPTIONS.iter()) {
        let time = Instant::now();
        let result = algorithm(m1, m2);
        let time = time.elapsed().as_nanos();

        print!("{}: ", description);
        print_matrix(&result);
        println!("\nВремя: {} нс", time);
    }
}
