use std::cmp::{min, max};

fn compare_last_chars(s1: &str, s2: &str) -> bool {
    s1.as_bytes().last().unwrap() == s2.as_bytes().last().unwrap()
    //s1.chars().last().unwrap() == s2.chars().last().unwrap()
}

fn compare_chars(s1: &str, s2: &str, i1: usize, i2: usize) -> bool {
    s1.as_bytes()[i1] == s2.as_bytes()[i2]
    //s1.chars().nth(i1).unwrap() == s2.chars().nth(i2).unwrap()
}

fn transition_condition_check(s1: &str, s2: &str, i1: usize, i2: usize) -> bool {
    s1.as_bytes()[i1] == s2.as_bytes()[i2 - 1] && s1.as_bytes()[i1 - 1] == s2.as_bytes()[i2]
    //s1.chars().nth(i1).unwrap() == s2.chars().nth(i2 - 1).unwrap() &&
        //s1.chars().nth(i1 - 1).unwrap() == s2.chars().nth(i2).unwrap()
}

pub fn recursive(s1: &str, s2: &str) -> (usize, usize) {
    _recursive(s1, s2, 0)
}

fn _recursive(s1: &str, s2: &str, depth: usize) -> (usize, usize) {
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
    let score = if compare_last_chars(s1, s2) { score } else { score + 1 };
    let (best_score, max_depth) = (min(best_score, score), max(cur_depth, max_depth));
    (best_score, max_depth)
}

pub fn recursive_with_mem(s1: &str, s2: &str) -> (usize, usize, Vec<Vec<usize>>) {
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

fn _recursive_with_mem(s1: &str, s2: &str, depth: usize, matrix: &mut [Vec<usize>]) -> (usize, usize) {
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
    let score = if compare_last_chars(s1, s2) { score } else { score + 1 };
    let (best_score, max_depth) = (min(best_score, score), max(cur_depth, max_depth));
    matrix[s2.len()][s1.len()] = best_score;
    (best_score, max_depth)
}

pub fn iterative(s1: &str, s2: &str) -> (usize, Vec<Vec<usize>>) {
    let mut matrix = vec![vec![usize::MAX; s1.len() + 1]; s2.len() + 1];
    init_matrix(&mut matrix);

    for i in 1..(s2.len() + 1) {
        for j in 1..(s1.len() + 1) {
            // insertion
            matrix[i][j] = min(matrix[i][j], matrix[i - 1][j] + 1);
            // deletion
            matrix[i][j] = min(matrix[i][j], matrix[i][j - 1] + 1);
            // match/replace
            let are_equal = compare_chars(s1, s2, j - 1, i - 1);
            let score = if are_equal { matrix[i - 1][j - 1] } else { matrix[i - 1][j - 1] + 1 };
            matrix[i][j] = min(matrix[i][j], score);
        }
    }

    (matrix[s2.len()][s1.len()], matrix)
}

pub fn iterative_dl(s1: &str, s2: &str) -> (usize, Vec<Vec<usize>>) {
    let mut matrix = vec![vec![usize::MAX; s1.len() + 1]; s2.len() + 1];
    init_matrix(&mut matrix);

    for i in 1..(s2.len() + 1) {
        for j in 1..(s1.len() + 1) {
            // insertion
            matrix[i][j] = min(matrix[i][j], matrix[i - 1][j] + 1);
            // deletion
            matrix[i][j] = min(matrix[i][j], matrix[i][j - 1] + 1);
            // match/replace
            let are_equal = compare_chars(s1, s2, j - 1, i - 1);
            let score = if are_equal { matrix[i - 1][j - 1] } else { matrix[i - 1][j - 1] + 1 };
            matrix[i][j] = min(matrix[i][j], score);

            if i > 1 && j > 1 {
                let transition_condition = transition_condition_check(s1, s2, j - 1, i - 1);
                if transition_condition {
                    matrix[i][j] = min(matrix[i][j], matrix[i - 2][j - 2] + 1);
                }
            }
        }
    }

    (matrix[s2.len()][s1.len()], matrix)
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_all_just_replaces() {
        let s1 = "super";
        let s2 = "hyper";
        let (score, depth) = recursive(&s1, &s2);
        assert_eq!(score, 2);
        assert_eq!(depth, 9);
        let (score, depth, _) = recursive_with_mem(&s1, &s2);
        assert_eq!(score, 2);
        assert_eq!(depth, 9);
        let (score, _) = iterative(&s1, &s2);
        assert_eq!(score, 2);
        let (score, _) = iterative_dl(&s1, &s2);
        assert_eq!(score, 2);
    }

    #[test]
    fn check_all_just_deletions() {
        let s1 = "per";
        let s2 = "hyper";
        let (score, depth) = recursive(&s1, &s2);
        assert_eq!(score, 2);
        assert_eq!(depth, 7);
        let (score, depth, _) = recursive_with_mem(&s1, &s2);
        assert_eq!(score, 2);
        assert_eq!(depth, 7);
        let (score, _) = iterative(&s1, &s2);
        assert_eq!(score, 2);
        let (score, _) = iterative_dl(&s1, &s2);
        assert_eq!(score, 2);
    }

    #[test]
    fn check_all_just_insertions() {
        let s1 = "superios";
        let s2 = "perio";
        let (score, depth) = recursive(&s1, &s2);
        assert_eq!(score, 3);
        assert_eq!(depth, 12);
        let (score, depth, _) = recursive_with_mem(&s1, &s2);
        assert_eq!(score, 3);
        assert_eq!(depth, 12);
        let (score, _) = iterative(&s1, &s2);
        assert_eq!(score, 3);
        let (score, _) = iterative_dl(&s1, &s2);
        assert_eq!(score, 3);
    }

    #[test]
    fn check_all_dl_replace() {
        let s1 = "superios";
        let s2 = "usperois";
        let (score, depth) = recursive(&s1, &s2);
        assert_eq!(score, 4);
        assert_eq!(depth, 15);
        let (score, depth, _) = recursive_with_mem(&s1, &s2);
        assert_eq!(score, 4);
        assert_eq!(depth, 15);
        let (score, _) = iterative(&s1, &s2);
        assert_eq!(score, 4);
        let (score, _) = iterative_dl(&s1, &s2);
        assert_eq!(score, 2);
    }

    #[test]
    fn check_all_deletions_and_insertions() {
        let s1 = "uperios";
        let s2 = "usperis";
        let (score, depth) = recursive(&s1, &s2);
        assert_eq!(score, 2);
        assert_eq!(depth, 13);
        let (score, depth, _) = recursive_with_mem(&s1, &s2);
        assert_eq!(score, 2);
        assert_eq!(depth, 13);
        let (score, _) = iterative(&s1, &s2);
        assert_eq!(score, 2);
        let (score, _) = iterative_dl(&s1, &s2);
        assert_eq!(score, 2);
    }

    #[test]
    fn check_all_mixed() {
        let s1 = "distance";
        let s2 = "editing";
        let (score, depth) = recursive(&s1, &s2);
        assert_eq!(score, 5);
        assert_eq!(depth, 14);
        let (score, depth, _) = recursive_with_mem(&s1, &s2);
        assert_eq!(score, 5);
        assert_eq!(depth, 14);
        let (score, _) = iterative(&s1, &s2);
        assert_eq!(score, 5);
        let (score, _) = iterative_dl(&s1, &s2);
        assert_eq!(score, 5);
    }
}
