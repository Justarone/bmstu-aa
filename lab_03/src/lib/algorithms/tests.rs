use super::*;
use rand::Rng;

extern crate test;
use test::Bencher;


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
fn check_bubble100(b: &mut Bencher) {
    let mut rng = rand::thread_rng();
    let arr: Vec<VecInner> = (0..100).map(|_| rng.gen()).collect();
    b.iter(|| bubble_sort(&mut arr.clone()));
}


#[bench]
fn check_bubble1000(b: &mut Bencher) {
    let mut rng = rand::thread_rng();
    let arr: Vec<VecInner> = (0..1_000).map(|_| rng.gen()).collect();
    b.iter(|| bubble_sort(&mut arr.clone()));
}


#[bench]
fn check_bubble10_000(b: &mut Bencher) {
    let mut rng = rand::thread_rng();
    let arr: Vec<VecInner> = (0..10_000).map(|_| rng.gen()).collect();
    b.iter(|| bubble_sort(&mut arr.clone()));
}


#[bench]
fn check_selection100(b: &mut Bencher) {
    let mut rng = rand::thread_rng();
    let arr: Vec<VecInner> = (0..100).map(|_| rng.gen()).collect();
    b.iter(|| selection_sort(&mut arr.clone()));
}


#[bench]
fn check_selection1000(b: &mut Bencher) {
    let mut rng = rand::thread_rng();
    let arr: Vec<VecInner> = (0..1_000).map(|_| rng.gen()).collect();
    b.iter(|| selection_sort(&mut arr.clone()));
}


#[bench]
fn check_selection10_000(b: &mut Bencher) {
    let mut rng = rand::thread_rng();
    let arr: Vec<VecInner> = (0..10_000).map(|_| rng.gen()).collect();
    b.iter(|| selection_sort(&mut arr.clone()));
}


#[bench]
fn check_insertion100(b: &mut Bencher) {
    let mut rng = rand::thread_rng();
    let arr: Vec<VecInner> = (0..100).map(|_| rng.gen()).collect();
    b.iter(|| insertion_sort(&mut arr.clone()));
}


#[bench]
fn check_insertion1000(b: &mut Bencher) {
    let mut rng = rand::thread_rng();
    let arr: Vec<VecInner> = (0..1_000).map(|_| rng.gen()).collect();
    b.iter(|| insertion_sort(&mut arr.clone()));
}


#[bench]
fn check_insertion10_000(b: &mut Bencher) {
    let mut rng = rand::thread_rng();
    let arr: Vec<VecInner> = (0..10_000).map(|_| rng.gen()).collect();
    b.iter(|| insertion_sort(&mut arr.clone()));
}


