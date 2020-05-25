use super::generate_tests::gen_random_vec;

use rand::{thread_rng, Rng};
use std::collections::LinkedList;

pub fn fat_sort<T: Copy + PartialOrd>(arr: &mut [T]) {
    for i in 0..arr.len() {
        let mut min_index = i;
        for index in i..arr.len() {
            if arr[index] < arr[min_index] {
                min_index = index;
            }
        }
        let t = arr[i];
        arr[i] = arr[min_index];
        arr[min_index] = t;
    }
}

pub fn lsd_sort(arr: &mut [usize]) {
    let mut last_zero_length = 0;
    let mut right_pos: usize = 0;
    while last_zero_length != arr.len() {
        let mut digits: Vec<Vec<usize>> = vec![vec![]; 10];
        for n in arr.iter() {
            digits[get_right_number(*n, right_pos)].push(*n);
        }

        let mut pos_in_arr = 0;
        for digit in &digits {
            for val in digit {
                arr[pos_in_arr] = *val;
                pos_in_arr += 1;
            }
        }

        last_zero_length = digits[0].len();
        right_pos += 1;
    }
}

pub fn patience_sort<T: Copy + Eq + Ord>(arr: &mut [T]) {
    let mut stacks: Vec<LinkedList<T>> = Vec::new();
    for el in arr.iter() {
        if stacks.len() > 0 {
            let mut pushed = false;
            for stack in stacks.iter_mut() {
                match stack.back() {
                    Some(val) => {
                        if *val > *el {
                            stack.push_back(*el);
                            pushed = true;
                            break;
                        }
                    }
                    None => {}
                }
            }
            if !pushed {
                let mut new_stack = LinkedList::new();
                new_stack.push_back(*el);
                stacks.push(new_stack);
            }
        } else {
            let mut new_stack = LinkedList::new();
            new_stack.push_back(*el);
            stacks.push(new_stack);
        }
    }
}

pub fn get_right_number(n: usize, pos: usize) -> usize {
    let mut copy = n;
    for _ in 0..pos {
        copy /= 10;
    }
    copy % 10
}

pub fn heapsort<T: Copy + Eq + Ord>(arr: &mut [T]) {
    let end = arr.len();
    for start in (0..end / 2).rev() {
        // Skip leaf nodes (end / 2).
        sift_down(arr, start, end - 1);
    }

    for end in (1..arr.len()).rev() {
        arr.swap(end, 0);
        sift_down(arr, 0, end - 1);
    }
}

fn sift_down<T: Copy + Eq + Ord>(arr: &mut [T], start: usize, end: usize) {
    let mut root = start;
    loop {
        let mut child = root * 2 + 1;
        if child > end {
            break;
        }
        if child < end && arr[child] < arr[child + 1] {
            child += 1;
        }

        if arr[root] < arr[child] {
            arr.swap(root, child);
            root = child;
        } else {
            break;
        }
    }
}

pub fn introsort<T: Copy + Eq + Ord>(arr: &mut [T]) {
    let maxdepth = ((arr.len() as f64).log2() as usize) * 2;
    introsort_main(arr, maxdepth);
}

fn introsort_main<T: Copy + Eq + Ord>(arr: &mut [T], maxdepth: usize) {
    let n = arr.len();
    if n <= 1 {
        return;
    } else {
        if maxdepth == 0 {
            heapsort(arr);
        } else {
            let mut p = partition(arr);
            introsort_main(&mut arr[0..p], maxdepth - 1);
            introsort_main(&mut arr[p+1..], maxdepth - 1);
        }
    }
}

fn partition<T: Copy + Eq + Ord>(arr: &mut [T]) -> usize{
    let mut pivot = arr[arr.len() / 2];
    let mut i = 0;
    let mut j = arr.len() - 1;
    loop {
        while arr[i] < pivot {
            i += 1;
        }
        while arr[j] > pivot {
            j -= 1;
        }
        if i >= j {
            return j;
        }
        arr.swap(i ,j);
    }
}

pub fn insertion_sort<T: Copy + Eq + Ord>(arr: &mut [T]) {
    for i in 1..arr.len() {
        let x = arr[i];
        let mut j = i;
        while j >= 1 && arr[j - 1] > x {
            arr[j] = arr[j - 1];
            j -= 1;
        }
        arr[j] = x;
    }
}


pub fn timsort<T: Copy + Eq + Ord>(arr: &mut [T]) {
    let minrun = get_min_run(arr.len());
    let mut runs = vec![vec![]];


    // Creating runs
    let mut current_index = 0;
    while current_index < arr.len() {
        if current_index + minrun < arr.len() {
            let mut run = arr[current_index..current_index + minrun].to_vec();
            current_index += minrun;
            loop {
                match arr.get(current_index) {
                    Some(val) => {
                        if *val > *run.last().unwrap() {
                            run.push(*val);
                            current_index += 1;
                        } else {
                            break;
                        }
                    }
                    None => {
                        break;
                    }
                }
            }
            insertion_sort(run.as_mut_slice());
            runs.push(run);
        } else {
            let mut run = arr[current_index..].to_vec();
            insertion_sort(run.as_mut_slice());
            runs.push(run);
            break;
        }
    }

    // Merging

    if runs.len() > 1 {
        let mut merged = vec![];
        for run in runs {
            merged = merge(merged.as_slice(), run.as_slice());
        }
        for i in 0..merged.len() {
            arr[i] = merged[i];
        }
    } else {
        for i in 0..runs[0].len() {
            arr[i] = runs[0][i];
        }
    }
}

fn merge<T: Copy + Eq + Ord>(arr1: &[T], arr2: &[T]) -> Vec<T> {
    if arr1.len() == 0 {
        return arr2.to_vec();
    }
    if arr2.len() == 0 {
        return arr1.to_vec();
    }

    let n = arr1.len() + arr2.len();
    let mut merged = Vec::with_capacity(n);
    let mut arr1_index = 0;
    let mut arr2_index = 0;
    for _ in 0..n {
        if let Some(&val1) = arr1.get(arr1_index) {
            if let Some(&val2) = arr2.get(arr2_index) {
                if val1 < val2 {
                    merged.push(val1);
                    arr1_index += 1;
                } else {
                    merged.push(val2);
                    arr2_index += 1;
                }
            } else {
                merged.push(val1);
                arr1_index += 1;
            }
        } else {
            merged.push(arr2[arr2_index]);
            arr2_index += 1;
        }
    }
    merged
}


pub fn get_min_run(mut n: usize) -> usize {
    let mut r = 0;

    while n >= 64 {
        r |= n & 1;
        n >>= 1;
    }

    r + n
}

#[test]
fn test_min_run() {
    let n = 10000;
    let r = get_min_run(n);
    println!("{}", r);
    assert!(r > 0);
}

#[test]
fn test_merge() {
    let a = vec![1, 3, 5];
    let b = vec![2, 4, 6];
    let merged = merge(a.as_slice(), b.as_slice());
    assert_eq!(merged, vec![1, 2, 3, 4, 5, 6]);
}

#[cfg(test)]
mod test_sorts {
    use super::*;

    fn big_sort_test<F>(sort: &mut F, n: usize)
    where F: FnMut(&mut [usize])
    {
        let mut random_vec = gen_random_vec::<usize>(n);
        let mut copy = random_vec.clone();
        fat_sort(&mut copy);
        assert_ne!(copy, random_vec);
        sort(random_vec.as_mut_slice());
        assert_eq!(copy, random_vec);
    }

    #[test]
    fn test_fat_sort() {
        let mut a = vec![6, 2, 4, 5];
        fat_sort(&mut a);
        assert_eq!(a, [2, 4, 5, 6]);
    }

    #[test]
    fn test_get_right_num() {
        let test_number = 1234;
        assert_eq!(4, get_right_number(test_number, 0));
        assert_eq!(1, get_right_number(test_number, 3));
        assert_eq!(0, get_right_number(test_number, 25));
    }

    #[test]
    fn test_lsd_sort() {
        let mut a = vec![6, 2, 4, 5];
        lsd_sort(&mut a);
        assert_eq!(a, [2, 4, 5, 6]);
    }

    #[test]
    fn test_introsort() {
        let mut a = vec![6, 2, 4, 5];
        introsort(&mut a);
        assert_eq!(a, [2, 4, 5, 6]);
    }

    #[test]
    fn test_random_vector_generator() {
        let n = 5;
        let zero_vec = vec![n; 0];
        let generated = gen_random_vec::<usize>(n);
        assert_ne!(zero_vec, generated);
    }

    #[test]
    fn test_timsort() {
        let mut a = vec![6, 2, 4, 5];
        timsort(&mut a);
        assert_eq!(a, [2, 4, 5, 6]);
    }

    #[test]
    fn big_lsg_test() {
        big_sort_test(&mut lsd_sort, 10000);
    }

    #[test]
    fn big_test_heapsort() {
        big_sort_test(&mut heapsort, 10000);
    }

    #[test]
    fn big_test_introsort() {
        big_sort_test(&mut introsort, 10000);
    }


    #[test]
    fn test_insertion_sort() {
        let mut a = vec![6, 2, 4, 5];
        insertion_sort(&mut a);
        assert_eq!(a, [2, 4, 5, 6]);
    }

    #[test]
    fn big_test_insertion_sort() {
        big_sort_test(&mut insertion_sort, 10000);
    }

    #[test]
    fn big_test_timsort() {
        big_sort_test(&mut timsort, 10000);
    }
}