use log::{ debug };

pub type MatInner = i64;
type MultFnPtr = fn(&[Vec<MatInner>], &[Vec<MatInner>]) -> Vec<Vec<MatInner>>;

pub static MULTS_ARRAY: [MultFnPtr; 3] = [simple_mult, vinograd_mult, vinograd_improved];
pub static MULTS_DESCRIPTIONS: [&str; 3] = ["Простое умножение", "Алгоритм умножения Винограда", "Улучшенный алгоритм умножения Винограда"];

pub mod utils;

fn get_result_matrix(m1: &[Vec<MatInner>], m2: &[Vec<MatInner>]) -> Vec<Vec<MatInner>> {
    if m1.len() == 0 || m2.len() == 0 {
        return Vec::new();
    } else if m1[0].len() != m2.len() {
        panic!("Плохие размеры матриц!");
    } else {
        vec![vec![0; m2[0].len()]; m1.len()]
    }
}

pub fn simple_mult(m1: &[Vec<MatInner>], m2: &[Vec<MatInner>]) -> Vec<Vec<MatInner>> {
    let mut matrix = get_result_matrix(m1, m2);

    for i in 0..m1.len() {
        for j in 0..m2[0].len() {
            for k in 0..m2.len() {
                matrix[i][j] += m1[i][k] * m2[k][j];
            }
        }
    }

    matrix
}

fn precompute_rows(matrix: &[Vec<MatInner>]) -> Vec<MatInner> {
    let mut res = vec![0; matrix.len()];

    for i in 0..matrix.len() {
        for j in 0..((matrix[0].len() - 1) / 2) {
            res[i] = res[i] - matrix[i][j * 2] * matrix[i][j * 2 + 1];
        }
    }

    res
}

fn precompute_cols(matrix: &[Vec<MatInner>]) -> Vec<MatInner> {
    let mut res = vec![0; matrix[0].len()];

    for i in 0..((matrix.len() - 1) / 2) {
        for j in 0..matrix[0].len() {
            res[j] = res[j] - matrix[i * 2][j] * matrix[i * 2 + 1][j];
        }
    }

    res
}

fn precompute_rows_fast(matrix: &[Vec<MatInner>]) -> Vec<MatInner> {
    let mut res = vec![0; matrix.len()];

    for i in 0..matrix.len() {
        for j in (0..(matrix[0].len() - 1)).step_by(2) {
            res[i] -= matrix[i][j] * matrix[i][j + 1];
        }
    }

    res
}

fn precompute_cols_fast(matrix: &[Vec<MatInner>]) -> Vec<MatInner> {
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

fn precompute_values_fast(m1: &[Vec<MatInner>], m2: &[Vec<MatInner>]) -> (Vec<MatInner>, Vec<MatInner>) {
    (precompute_rows_fast(m1), precompute_cols_fast(m2))
}

pub fn vinograd_mult(m1: &[Vec<MatInner>], m2: &[Vec<MatInner>]) -> Vec<Vec<MatInner>> {
    let mut matrix = get_result_matrix(m1, m2);
    let precomputed = precompute_values(m1, m2);

    debug!("MH: {:?};\nMV: {:?}", precomputed.0, precomputed.1);

    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            matrix[i][j] = precomputed.0[i] + precomputed.1[j];

            for k in 0..(m2.len() / 2) {
                matrix[i][j] += (m1[i][k * 2] + m2[k * 2 + 1][j]) * (m1[i][k * 2 + 1] + m2[k * 2][j]);
            }
        }
    }

    if m2.len() % 2 != 0 {
        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                matrix[i][j] += m1[i][m2.len() - 1] * m2[m2.len() - 1][j];
            }
        }
    }


    matrix
}

pub fn vinograd_improved(m1: &[Vec<MatInner>], m2: &[Vec<MatInner>]) -> Vec<Vec<MatInner>> {
    let mut matrix = get_result_matrix(m1, m2);
    let precomputed = precompute_values_fast(m1, m2);

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
