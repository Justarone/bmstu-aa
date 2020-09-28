use super::*;
use rand::Rng;

const MODULAR: MatInner = 19;

fn generate_row(size: usize) -> Vec<MatInner> {
    let mut rng = rand::thread_rng();
    (0..size).map(|_| rng.gen::<MatInner>() % MODULAR).collect()
}

pub fn generate_matrix(rows: usize, cols: usize) -> Vec<Vec<MatInner>> {
    (0..rows).map(|_| generate_row(cols)).collect()
}
