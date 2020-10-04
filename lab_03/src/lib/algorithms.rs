type SortFnPtr<T> = fn(&mut [T]) -> ();
pub type VecInner = i64;

pub static SORTS_ARRAY: [SortFnPtr<VecInner>; 3] = [bubble_sort, insertion_sort, selection_sort];
pub static SORTS_DESCRIPTIONS: [&str; 3] = ["Bubble    sort", "Insertion sort", "Selection sort"];

pub fn bubble_sort<T: Ord>(arr: &mut [T]) {
    if arr.len() == 0 {
        return;
    }

    for i in 0..(arr.len() - 1) {
        for j in 0..(arr.len() - 1 - i) {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

pub fn selection_sort<T: Ord + Clone>(arr: &mut [T]) {
    if arr.len() == 0 {
        return;
    }

    for i in 0..(arr.len() - 1) {
        let mut min_elem = arr[i].clone();
        let mut min_index = i;

        for j in (i + 1)..arr.len() {
            if arr[j] < min_elem {
                min_elem = arr[j].clone();
                min_index = j;
            }
        }

        arr.swap(i, min_index);
    }
}

pub fn insertion_sort<T: Ord>(arr: &mut [T]) {
    for i in 1..arr.len() {
        for j in (1..(i + 1)).rev() {
            if arr[j] < arr[j - 1] {
                arr.swap(j, j - 1);
            } else {
                break;
            }
        }
    }
}


#[cfg(test)]
mod tests;
