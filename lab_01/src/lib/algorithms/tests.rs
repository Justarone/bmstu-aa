use super::*;

#[test]
fn check_all_just_replaces() {
    let s1 = "super";
    let s2 = "hyper";
    let (s1, s2): (Vec<char>, Vec<char>) = (s1.chars().collect(), s2.chars().collect());
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
    let (s1, s2): (Vec<char>, Vec<char>) = (s1.chars().collect(), s2.chars().collect());
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
    let (s1, s2): (Vec<char>, Vec<char>) = (s1.chars().collect(), s2.chars().collect());
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
    let (s1, s2): (Vec<char>, Vec<char>) = (s1.chars().collect(), s2.chars().collect());
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
    let (s1, s2): (Vec<char>, Vec<char>) = (s1.chars().collect(), s2.chars().collect());
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
    let (s1, s2): (Vec<char>, Vec<char>) = (s1.chars().collect(), s2.chars().collect());
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
