mod parallel_utils;
use parallel_utils::*;

pub mod parallel1;
pub use parallel1::vinograd_parallel1;

pub mod parallel2;
pub use parallel2::vinograd_parallel2;

pub type MatInner = i64;
type MultFnPtr = fn(&[Vec<MatInner>], &[Vec<MatInner>]) -> Vec<Vec<MatInner>>;

pub static MULTS_ARRAY: [MultFnPtr; 3] = [vinograd_simple, vinograd_parallel1, vinograd_parallel2];
pub static MULTS_DESCRIPTIONS: [&str; 3] = ["Простое умножение", "Распараллеленное умножение 1", "Распараллеленное умножение 2"];
//pub static MULTS_ARRAY: [MultFnPtr; 2] = [vinograd_parallel1, vinograd_parallel2];
//pub static MULTS_DESCRIPTIONS: [&str; 2] = ["Распараллеленное умножение 1", "Распараллеленное умножение 2"];

const NUMBER_OF_THREADS1: usize = 8;
const NUMBER_OF_THREADS2: usize = 8;

pub mod utils;
use utils::{ odd_mult_sync, get_result_matrix };

fn precompute_rows(matrix: &[Vec<MatInner>]) -> Vec<MatInner> {
    let mut res = vec![0; matrix.len()];

    for i in 0..matrix.len() {
        for j in (0..(matrix[0].len() - 1)).step_by(2) {
            res[i] -= matrix[i][j] * matrix[i][j + 1];
        }
    }

    res
}

fn precompute_cols(matrix: &[Vec<MatInner>]) -> Vec<MatInner> {
    let mut res = vec![0; matrix[0].len()];

    for i in (0..(matrix.len() - 1)).step_by(2) {
        for j in 0..matrix[0].len() {
            res[j] -= matrix[i][j] * matrix[i + 1][j];
        }
    }

    res
}

fn precompute_values(m1: &[Vec<MatInner>], m2: &[Vec<MatInner>]) -> (Vec<MatInner>, Vec<MatInner>) {
    (precompute_rows(m1), precompute_cols(m2))
}


pub fn vinograd_simple(m1: &[Vec<MatInner>], m2: &[Vec<MatInner>]) -> Vec<Vec<MatInner>> {
    let mut matrix = get_result_matrix(m1, m2);
    let precomputed = precompute_values(m1, m2);

    let m = matrix.len();
    let n = matrix[0].len();
    let k_iteration = m2.len();

    for i in 0..m {
        for j in 0..n {

            matrix[i][j] = precomputed.0[i] + precomputed.1[j];
            for k in (0..(k_iteration - 1)).step_by(2) {
                matrix[i][j] += (m1[i][k] + m2[k + 1][j]) * (m1[i][k + 1] + m2[k][j]);
            }

        }
    }

    if m2.len() & 1 != 0 {
        let max_k = m2.len() - 1;

        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                matrix[i][j] += m1[i][max_k] * m2[max_k][j];
            }
        }
    }
    
    matrix
}


#[cfg(test)]
mod tests;
