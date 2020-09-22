use std::cmp::{min, max};

pub fn recursive(s1: &[char], s2: &[char]) -> (usize, usize) {
    _recursive(s1, s2, 0)
}

fn _recursive(s1: &[char], s2: &[char], depth: usize) -> (usize, usize) {
    if s1.len() == 0 {
        return (s2.len(), depth);
    } else if s2.len() == 0 {
        return (s1.len(), depth);
    }

    // insertion
    let (best_score, max_depth) = _recursive(&s1[..(s1.len() - 1)], s2, depth + 1);
    let best_score = best_score + 1;
    // deletion
    let (score, cur_depth) = _recursive(s1, &s2[..(s2.len() - 1)], depth + 1);
    let (best_score, max_depth) = (min(best_score, score + 1), max(cur_depth, max_depth));
    // match/replace
    let (score, cur_depth) = _recursive(&s1[..(s1.len() - 1)], &s2[..(s2.len() - 1)], depth + 1);
    let score = if s1[s1.len() - 1] == s2[s2.len() - 1] { score } else { score + 1 };
    let (best_score, max_depth) = (min(best_score, score), max(cur_depth, max_depth));
    (best_score, max_depth)
}

pub fn recursive_with_mem(s1: &[char], s2: &[char]) -> (usize, usize, Vec<Vec<usize>>) {
    let mut matrix = vec![vec![usize::MAX; s1.len() + 1]; s2.len() + 1];
    init_matrix(&mut matrix);
    let res = _recursive_with_mem(s1, s2, 0, &mut matrix);
    (res.0, res.1, matrix)
}

fn init_matrix(matrix: &mut [Vec<usize>]) {
    if matrix.len() == 0 {
        return;
    }

    for i in 0..matrix.len() {
        matrix[i][0] = i;
    }

    for j in 0..matrix[0].len() {
        matrix[0][j] = j;
    }
}

fn _recursive_with_mem(s1: &[char], s2: &[char], depth: usize, matrix: &mut [Vec<usize>]) -> (usize, usize) {
    if matrix[s2.len()][s1.len()] != usize::MAX {
        return (matrix[s2.len()][s1.len()], depth);
    }

    // insertion
    let (best_score, max_depth) = _recursive_with_mem(&s1[..(s1.len() - 1)], &s2, depth + 1, matrix);
    let best_score = best_score + 1;
    // deletion
    let (score, cur_depth) = _recursive_with_mem(&s1, &s2[..(s2.len() - 1)], depth + 1, matrix);
    let (best_score, max_depth) = (min(best_score, score + 1), max(cur_depth, max_depth));
    // match/replace
    let (score, cur_depth) = _recursive_with_mem(&s1[..(s1.len() - 1)], &s2[..(s2.len() - 1)], depth + 1, matrix);
    let score = if s1[s1.len() - 1] == s2[s2.len() - 1]  { score } else { score + 1 };
    let (best_score, max_depth) = (min(best_score, score), max(cur_depth, max_depth));
    matrix[s2.len()][s1.len()] = best_score;
    (best_score, max_depth)
}

pub fn iterative(s1: &[char], s2: &[char]) -> (usize, Vec<Vec<usize>>) {
    let mut matrix = vec![vec![usize::MAX; s1.len() + 1]; s2.len() + 1];
    init_matrix(&mut matrix);

    for i in 1..(s2.len() + 1) {
        for j in 1..(s1.len() + 1) {
            // insertion
            matrix[i][j] = min(matrix[i][j], matrix[i - 1][j] + 1);
            // deletion
            matrix[i][j] = min(matrix[i][j], matrix[i][j - 1] + 1);
            // match/replace
            let score = if s1[j - 1] == s2[i - 1] { matrix[i - 1][j - 1] } else { matrix[i - 1][j - 1] + 1 };
            matrix[i][j] = min(matrix[i][j], score);
        }
    }

    (matrix[s2.len()][s1.len()], matrix)
}

pub fn iterative_dl(s1: &[char], s2: &[char]) -> (usize, Vec<Vec<usize>>) {
    let mut matrix = vec![vec![usize::MAX; s1.len() + 1]; s2.len() + 1];
    init_matrix(&mut matrix);

    for i in 1..(s2.len() + 1) {
        for j in 1..(s1.len() + 1) {
            // insertion
            matrix[i][j] = min(matrix[i][j], matrix[i - 1][j] + 1);
            // deletion
            matrix[i][j] = min(matrix[i][j], matrix[i][j - 1] + 1);
            // match/replace
            let score = if s1[j - 1] == s2[i - 1] { matrix[i - 1][j - 1] } else { matrix[i - 1][j - 1] + 1 };
            matrix[i][j] = min(matrix[i][j], score);

            if i > 1 && j > 1 {
                let transition_condition = s1[j - 1] == s2[i - 2] && s1[j - 2] == s2[i - 1];
                if transition_condition {
                    matrix[i][j] = min(matrix[i][j], matrix[i - 2][j - 2] + 1);
                }
            }
        }
    }

    (matrix[s2.len()][s1.len()], matrix)
}

#[cfg(test)]
mod tests;
