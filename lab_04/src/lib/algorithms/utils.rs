use super::*;
use rand::Rng;

const MODULAR: MatInner = 19;

pub fn get_result_matrix(m1: &[Vec<MatInner>], m2: &[Vec<MatInner>]) -> Vec<Vec<MatInner>> {
    if m1.len() == 0 || m2.len() == 0 {
        return Vec::new();
    } else if m1[0].len() != m2.len() {
        panic!("Bad matrices!");
    } else {
        vec![vec![0; m2[0].len()]; m1.len()]
    }
}

fn generate_row(size: usize) -> Vec<MatInner> {
    let mut rng = rand::thread_rng();
    (0..size).map(|_| rng.gen::<MatInner>() % MODULAR).collect()
}

pub fn generate_matrix(rows: usize, cols: usize) -> Vec<Vec<MatInner>> {
    (0..rows).map(|_| generate_row(cols)).collect()
}

pub fn odd_mult_sync(mut matrix: Vec<Vec<MatInner>>, m1: &[Vec<MatInner>],
    m2: &[Vec<MatInner>]) -> Vec<Vec<MatInner>> {
    let max_k = m2.len() - 1;

    for i in 0..m1.len() {
        for j in 0..m2[0].len() {
            matrix[i][j] += m1[i][max_k] * m2[max_k][j];
        }
    }

    matrix
}
