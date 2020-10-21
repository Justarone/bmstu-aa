use crossbeam::thread as cthread;
use std::sync::{ Arc, Mutex };
use super::*;

fn precompute_rows_elementary(matrix_slice: &[Vec<MatInner>], res_slice: Arc<Mutex<Vec<MatInner>>>,
    range: std::ops::Range<usize>) {
    let mut buf = vec![0; matrix_slice.len()];
    for (ind, row) in matrix_slice.iter().enumerate() {
        for i in (1..row.len()).step_by(2) {
            buf[ind] += row[i - 1] * row[i];
        }
    }

    let offset = range.start;
    let mut res = res_slice.lock().unwrap();
    for i in range {
        res[i] = buf[i - offset];
    }
}

pub fn precompute_rows_parallel(matrix: &[Vec<MatInner>], nofth: usize) -> Vec<MatInner> {
    if matrix.len() == 0 {
        return Vec::new();
    }

    cthread::scope(|s| {
        let res = Arc::new(Mutex::new(vec![0; matrix.len()]));
        let mut threads = Vec::with_capacity(nofth);
        let size = matrix.len() / (nofth + 1);

        for i in 0..nofth {
            let range = (i * size)..((i + 1) * size);
            let res_cpy = res.clone();
            threads.push(
                s.spawn(move |_| precompute_rows_elementary(&matrix[range.clone()], res_cpy, range)));
        }

        let range = (size * nofth)..matrix.len();
        let res_cpy = res.clone();
        precompute_rows_elementary(&matrix[range.clone()], res_cpy, range);

        for th in threads {
            th.join().unwrap();
        }

        Arc::try_unwrap(res).unwrap().into_inner().unwrap()
    }).unwrap()
}

fn precompute_cols_elementary(matrix_slice: &[Vec<MatInner>], res_slice: Arc<Mutex<Vec<MatInner>>>,
    range: std::ops::Range<usize>) {
    let mut buf = vec![0; range.end - range.start];

    for (ind, j) in range.clone().enumerate() {
        for i in (1..matrix_slice.len()).step_by(2) {
            buf[ind] += matrix_slice[i][j] * matrix_slice[i - 1][j];
        }
    }

    let mut res = res_slice.lock().unwrap();
    let offset = range.start;
    for i in range {
        res[i] = buf[i - offset];
    }
}

pub fn precompute_cols_parallel(matrix: &[Vec<MatInner>], nofth: usize) -> Vec<MatInner> {
    if matrix.len() == 0 {
        return Vec::new();
    }

    cthread::scope(|s| {
        let res = Arc::new(Mutex::new(vec![0; matrix[0].len()]));
        let mut threads = Vec::with_capacity(nofth);
        let size = matrix[0].len() / (nofth + 1);


        for i in 0..nofth {
            let range = (i * size)..((i + 1) * size);
            let res_cpy = res.clone();
            threads.push(s.spawn(move |_| precompute_cols_elementary(&matrix, res_cpy, range)));
        }

        let range = (size * nofth)..matrix[0].len();
        precompute_cols_elementary(&matrix, res.clone(), range);

        for th in threads {
            th.join().unwrap();
        }

        Arc::try_unwrap(res).unwrap().into_inner().unwrap()
    }).unwrap()
}

pub fn mult_parallel(matrix: Vec<Vec<MatInner>>, m1: &[Vec<MatInner>], m2: &[Vec<MatInner>], precomputed: &(Vec<MatInner>, Vec<MatInner>), nofth: usize) -> Vec<Vec<MatInner>> {
    cthread::scope(|s| {
        let mut threads = Vec::with_capacity(nofth);

        let size = matrix.len() / (nofth + 1);
        let matrix_guard = Arc::new(Mutex::new(matrix));

        for i in 0..nofth {
            let range = (i * size)..((i + 1) * size);
            let guard_copy = matrix_guard.clone();
            let precopy = &precomputed;
            threads.push(s.spawn(move |_| multiplication_in_range(&precopy, guard_copy, &m1, &m2, range)));
        }

        let range = (nofth * size)..m1.len();
        let guard_copy = matrix_guard.clone();
        multiplication_in_range(&precomputed, guard_copy, &m1, &m2, range);
        for th in threads {
            th.join().unwrap();
        }

        let matrix = Arc::try_unwrap(matrix_guard).unwrap().into_inner().unwrap();

        matrix
    }).unwrap()
}

pub fn multiplication_in_range(precomputed: &(Vec<MatInner>, Vec<MatInner>),
    matrix_guard: Arc<Mutex<Vec<Vec<MatInner>>>>, m1: &[Vec<MatInner>],
    m2: &[Vec<MatInner>], range: std::ops::Range<usize>) {
    let k_iterations = m2.len();
    let n = m2[0].len();

    for i in range {
        for j in 0..n {
            let mut buffer = -precomputed.0[i] - precomputed.1[j];
            for k in (0..(k_iterations - 1)).step_by(2) {
                buffer += (m1[i][k] + m2[k + 1][j]) * (m1[i][k + 1] + m2[k][j]);
            }
            let mut matrix = matrix_guard.lock().unwrap();
            matrix[i][j] = buffer;
        }
    }
}

pub fn precompute_values_parallel(m1: &[Vec<MatInner>], m2: &[Vec<MatInner>]) -> (Vec<MatInner>, Vec<MatInner>) {
    cthread::scope(move |s| {
        let nofth = NUMBER_OF_THREADS1 - 2;

        let t1 = s.spawn(move |_| precompute_rows_parallel(&m1, nofth / 2));
        let p2 = precompute_cols_parallel(&m2, nofth / 2 + nofth & 1);

        let p1 = t1.join().unwrap();
        (p1, p2)
    }).unwrap()
}

pub fn _odd_mult_parallel(matrix: Vec<Vec<MatInner>>, m1: &[Vec<MatInner>],
    m2: &[Vec<MatInner>], nofth: usize) -> Vec<Vec<MatInner>> {
    cthread::scope(move |s| {
        let mut threads = Vec::with_capacity(nofth);

        let size = matrix.len() / (nofth + 1);
        let matrix_guard = Arc::new(Mutex::new(matrix));

        for i in 0..nofth {
            let range = (i * size)..((i + 1) * size);
            let guard_copy = matrix_guard.clone();
            threads.push(s.spawn(move |_| _odd_mult_in_range(guard_copy, &m1, &m2, range)));
        }

        let range = (nofth * size)..m1.len();
        let guard_copy = matrix_guard.clone();
        _odd_mult_in_range(guard_copy, &m1, &m2, range);

        for th in threads {
            th.join().unwrap();
        }

        let matrix = Arc::try_unwrap(matrix_guard).unwrap().into_inner().unwrap();
        matrix
    }).unwrap()
}

fn _odd_mult_in_range(matrix_guard: Arc<Mutex<Vec<Vec<MatInner>>>>, m1: &[Vec<MatInner>],
    m2: &[Vec<MatInner>], range: std::ops::Range<usize>) {
    let max_k = m2.len() - 1;

    for i in range {
        for j in 0..m2[0].len() {
            let buf = m1[i][max_k] * m2[max_k][j];
            let mut matrix = matrix_guard.lock().unwrap();
            matrix[i][j] += buf;
        }
    }
}
