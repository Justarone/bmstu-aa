use std::io::prelude::*;
use super::algorithms::*;
use std::time::Instant;
use std::env;
use rand::Rng;

const DEFAULT_ITERS: usize = 100;
const DEFAULT_SIZE: usize = 100;
//const SIZE_OF_ARR: usize = 3;
//const ARR_WITH_ELEMS: [VecInner; SIZE_OF_ARR] = [1, 100, -5];

pub fn read_data() -> Vec<VecInner> {
    let mut array = String::new();
    print!("Введите элементы через пробел для тестирования: ");
    std::io::stdout().flush().expect("Невозможно напечатать приглашение!");
    std::io::stdin().read_line(&mut array).expect("Невозможно считать строку!");

    array.split_ascii_whitespace()
        .map(|elem| elem.parse::<VecInner>().expect("Невозможно преобразовать в число!"))
        .collect()
}

pub fn get_n() -> usize {
    match env::var("N") {
        Ok(value) => value.parse().unwrap(),
        Err(_) => DEFAULT_ITERS
    }
}

pub fn get_size() -> usize {
    match env::var("SIZE") {
        Ok(value) => value.parse().unwrap(),
        Err(_) => DEFAULT_SIZE
    }
}

pub fn measure(n: usize, size: usize) {
    println!("Производится {} замеров для массивов размера {}", n, size);
    let mut rng = rand::thread_rng();

    //let arr: Vec<_> = (0..size).map(|_| ARR_WITH_ELEMS[rng.gen::<usize>() % SIZE_OF_ARR]).collect();
    //println!("{:?}", arr);
    let arr: Vec<_> = (0..size).map(|_| rng.gen()).collect();

    for (algorithm, description) in SORTS_ARRAY.iter().zip(SORTS_DESCRIPTIONS.iter()) {

        let time = Instant::now();
        for _ in 0..n {
            algorithm(&mut arr.clone());
        }
        let time = time.elapsed().as_nanos() as usize / n;

        print!("{}: ", description);
        println!("\nВремя: {} нс", time);
    }
}

fn print_array<T: std::fmt::Display>(arr: &Vec<T>) {
    print!("{{ ");

    for elem in arr.iter() {
        print!("{} ", elem);
    }

    print!("}}");
}

pub fn run_tests(arr: &mut Vec<VecInner>) {
    for (algorithm, description) in SORTS_ARRAY.iter().zip(SORTS_DESCRIPTIONS.iter()) {
        let mut cloned = arr.clone();
        let time = Instant::now();
        algorithm(&mut cloned);
        let time = time.elapsed().as_nanos();

        print!("{}: ", description);
        print_array(&cloned);
        println!("\nВремя: {} нс", time);
    }
}
