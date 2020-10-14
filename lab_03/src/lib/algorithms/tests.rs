use super::*;
use rand::Rng;

extern crate test;
use test::Bencher;

const T1: usize = 100;
const T2: usize = 200;
const T3: usize = 300;

fn generate_arr(from: usize, to: usize) -> Vec<VecInner> {
    let mut rng = rand::thread_rng();
    (from..to).map(|_| rng.gen()).collect()
    //((from as i64)..(to as i64)).rev().collect()
}

#[test]
fn check_sorted() { 
    let mut arr: Vec<_> = (0..100).collect();
    let mut arrays: Vec<_> = (0..SORTS_ARRAY.len()).map(|_| arr.clone()).collect();
    arr.sort();
    for i in 0..SORTS_ARRAY.len() {
        SORTS_ARRAY[i](&mut arrays[i]);
        assert_eq!(arrays[i], arr, "{}", SORTS_DESCRIPTIONS[i]);
    }
}


#[test]
fn check_reversed() { 
    let mut arr: Vec<_> = (0..100).rev().collect();
    let mut arrays: Vec<_> = (0..SORTS_ARRAY.len()).map(|_| arr.clone()).collect();
    arr.sort();
    for i in 0..SORTS_ARRAY.len() {
        SORTS_ARRAY[i](&mut arrays[i]);
        assert_eq!(arrays[i], arr, "{}", SORTS_DESCRIPTIONS[i]);
    }
}


#[test]
fn check_random() { 
    let mut rng = rand::thread_rng();
    let mut arr: Vec<_> = (0..100).map(|_| rng.gen()).collect();
    let mut arrays: Vec<_> = (0..SORTS_ARRAY.len()).map(|_| arr.clone()).collect();
    arr.sort();
    for i in 0..SORTS_ARRAY.len() {
        SORTS_ARRAY[i](&mut arrays[i]);
        assert_eq!(arrays[i], arr, "{}", &SORTS_DESCRIPTIONS[i]);
    }
}


#[test]
fn check_empty() { 
    let mut arr = Vec::new();
    let mut arrays: Vec<_> = (0..SORTS_ARRAY.len()).map(|_| arr.clone()).collect();
    arr.sort();
    for i in 0..SORTS_ARRAY.len() {
        SORTS_ARRAY[i](&mut arrays[i]);
        assert_eq!(arrays[i], arr, "{}", &SORTS_DESCRIPTIONS[i]);
    }
}


#[bench]
fn check_bubble1(b: &mut Bencher) {
    let arr = generate_arr(0, T1);
    b.iter(|| bubble_sort(&mut arr.clone()));
}


#[bench]
fn check_bubble2(b: &mut Bencher) {
    let arr = generate_arr(0, T2);
    b.iter(|| bubble_sort(&mut arr.clone()));
}


#[bench]
fn check_bubble3(b: &mut Bencher) {
    let arr = generate_arr(0, T3);
    b.iter(|| bubble_sort(&mut arr.clone()));
}


#[bench]
fn check_selection1(b: &mut Bencher) {
    let arr = generate_arr(0, T1);
    b.iter(|| selection_sort(&mut arr.clone()));
}


#[bench]
fn check_selection2(b: &mut Bencher) {
    let arr = generate_arr(0, T2);
    b.iter(|| selection_sort(&mut arr.clone()));
}


#[bench]
fn check_selection3(b: &mut Bencher) {
    let arr = generate_arr(0, T3);
    b.iter(|| selection_sort(&mut arr.clone()));
}


#[bench]
fn check_insertion1(b: &mut Bencher) {
    let arr = generate_arr(0, T1);
    b.iter(|| insertion_sort(&mut arr.clone()));
}


#[bench]
fn check_insertion2(b: &mut Bencher) {
    let arr = generate_arr(0, T2);
    b.iter(|| insertion_sort(&mut arr.clone()));
}


#[bench]
fn check_insertion3(b: &mut Bencher) {
    let arr = generate_arr(0, T3);
    b.iter(|| insertion_sort(&mut arr.clone()));
}


