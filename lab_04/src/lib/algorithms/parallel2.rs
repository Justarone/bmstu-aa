use super::*;

pub fn vinograd_parallel2(m1: &[Vec<MatInner>], m2: &[Vec<MatInner>]) -> Vec<Vec<MatInner>> {
    let matrix = get_result_matrix(m1, m2);
    let precomputed = (precompute_rows_parallel(&m1, NUMBER_OF_THREADS2 - 1),
        precompute_cols_parallel(&m2, NUMBER_OF_THREADS2 - 1));

    let mut matrix = mult_parallel(matrix, m1, m2, &precomputed, NUMBER_OF_THREADS2 - 1);

    if m2.len() & 1 != 0 {
        matrix = odd_mult_sync(matrix, m1, m2);
    }
        
    matrix
}
