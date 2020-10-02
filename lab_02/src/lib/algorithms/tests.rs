use super::*;
use super::utils::generate_matrix;

extern crate test;
use test::Bencher;

const S1: usize = 11;
const S2: usize = 21;
const S3: usize = 31;
const S: usize = 21;

const T1: usize = 101;
const T2: usize = 201;
const T3: usize = 301;
const T4: usize = 401;


fn create_square_pair_matrices(s: usize, elem_by_indeces: fn(usize, usize) -> MatInner) -> Vec<Vec<Vec<MatInner>>> {
    (0..2).map(|_| (0..s)
        .map(|i| (0..s)
            .map(|j| elem_by_indeces(i, j))
            .collect())
        .collect())
        .collect()
}

fn run_check(matrices: &[Vec<Vec<MatInner>>]) {
    let mut results = Vec::new();
    for i in 0..MULTS_ARRAY.len() {
        results.push(MULTS_ARRAY[i](&matrices[0], &matrices[1]));
    }

    for i in 1..MULTS_ARRAY.len() {
        assert_eq!(results[i], results[0]);
    }
}

#[test]
fn check_random() {
    let matrices = [generate_matrix(S1, S2), generate_matrix(S2, S3)];
    run_check(&matrices);
}

#[test]
fn check_zeros() {
    let matrices = create_square_pair_matrices(S, |_, _| 0);
    run_check(&matrices);
}

#[test]
fn check_identity() {
    let matrices = create_square_pair_matrices(S, |i, j| if i == j { 1 } else { 0 });
    run_check(&matrices);
}


#[bench]
fn check_simple1(b: &mut Bencher) {
    let matrices = [generate_matrix(T1, T1), generate_matrix(T1, T1)];
    b.iter(|| simple_mult(&matrices[0], &matrices[1]));
}


#[bench]
fn check_vinograd1(b: &mut Bencher) {
    let matrices = [generate_matrix(T1, T1), generate_matrix(T1, T1)];
    b.iter(|| vinograd_mult(&matrices[0], &matrices[1]));
}


#[bench]
fn check_vino_improved1(b: &mut Bencher) {
    let matrices = [generate_matrix(T1, T1), generate_matrix(T1, T1)];
    b.iter(|| vinograd_improved(&matrices[0], &matrices[1]));
}


#[bench]
fn check_simple2(b: &mut Bencher) {
    let matrices = [generate_matrix(T2, T2), generate_matrix(T2, T2)];
    b.iter(|| simple_mult(&matrices[0], &matrices[1]));
}


#[bench]
fn check_vinograd2(b: &mut Bencher) {
    let matrices = [generate_matrix(T2, T2), generate_matrix(T2, T2)];
    b.iter(|| vinograd_mult(&matrices[0], &matrices[1]));
}


#[bench]
fn check_vino_improved2(b: &mut Bencher) {
    let matrices = [generate_matrix(T2, T2), generate_matrix(T2, T2)];
    b.iter(|| vinograd_improved(&matrices[0], &matrices[1]));
}


#[bench]
fn check_simple3(b: &mut Bencher) {
    let matrices = [generate_matrix(T3, T3), generate_matrix(T3, T3)];
    b.iter(|| simple_mult(&matrices[0], &matrices[1]));
}


#[bench]
fn check_vinograd3(b: &mut Bencher) {
    let matrices = [generate_matrix(T3, T3), generate_matrix(T3, T3)];
    b.iter(|| vinograd_mult(&matrices[0], &matrices[1]));
}


#[bench]
fn check_vino_improved3(b: &mut Bencher) {
    let matrices = [generate_matrix(T3, T3), generate_matrix(T3, T3)];
    b.iter(|| vinograd_improved(&matrices[0], &matrices[1]));
}


#[bench]
fn check_simple4(b: &mut Bencher) {
    let matrices = [generate_matrix(T4, T4), generate_matrix(T4, T4)];
    b.iter(|| simple_mult(&matrices[0], &matrices[1]));
}


#[bench]
fn check_vinograd4(b: &mut Bencher) {
    let matrices = [generate_matrix(T4, T4), generate_matrix(T4, T4)];
    b.iter(|| vinograd_mult(&matrices[0], &matrices[1]));
}


#[bench]
fn check_vino_improved4(b: &mut Bencher) {
    let matrices = [generate_matrix(T4, T4), generate_matrix(T4, T4)];
    b.iter(|| vinograd_improved(&matrices[0], &matrices[1]));
}


