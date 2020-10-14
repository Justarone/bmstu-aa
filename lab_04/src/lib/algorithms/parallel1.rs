use super::*;

pub fn vinograd_parallel1(m1: &[Vec<MatInner>], m2: &[Vec<MatInner>]) -> Vec<Vec<MatInner>> {
    let matrix = get_result_matrix(m1, m2);
    let precomputed = precompute_values_parallel(m1, m2);

    let mut matrix = mult_parallel(matrix, m1, m2, &precomputed, NUMBER_OF_THREADS1 - 1);

    if m2.len() & 1 != 0 {
        matrix = odd_mult_sync(matrix, m1, m2);
    }

    matrix
}
