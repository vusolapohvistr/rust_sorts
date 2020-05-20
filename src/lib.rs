mod sorts {
    use rand::{thread_rng, Rng};

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

    pub fn patience_sort(arr: &mut [usize]) {
        let mut stacks: Vec<Vec<usize>> = Vec::new();
        for el in arr.iter() {
            if stacks.len() > 0 {
                let mut pushed = false;
                for stack in &mut stacks {
                    match stack.last() {
                        Some(val) => {
                            if *val > *el {
                                stack.push(*el);
                                pushed = true;
                                break;
                            }
                        }
                        _ => {}
                    }
                }
                if !pushed {
                    stacks.push(vec![*el]);
                }
            } else {
                stacks.push(vec![*el]);
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

    pub fn heapsort(arr: &mut [usize]) {
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

    fn sift_down(arr: &mut [usize], start: usize, end: usize) {
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

    pub fn gen_random_vec(n: usize) -> Vec<usize> {
        let mut rng = thread_rng();
        (0..n).map(|_| {
            rng.gen()
        }).collect()
    }

    pub fn introsort(arr: &mut [usize]) {
        let maxdepth = ((arr.len() as f64).log2() as usize) * 2;
        introsort_main(arr, maxdepth);
    }

    fn introsort_main(arr: &mut [usize], maxdepth: usize) {
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

    fn partition(arr: &mut [usize]) -> usize{
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

    pub fn insertion_sort(arr: &mut [usize]) {
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

    pub fn timsort(arr: &mut [usize]) {
        let minrun = getMinRun(arr.len());
        let mut runs = vec![vec![]];

        let mut current_index = 0;

        while current_index < arr.len() {
            let mut current_run = vec![];   
        }
    }

    pub fn getMinRun(mut n: usize) -> usize {
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
        let r = getMinRun(n);
        println!("{}", r);
        assert!(r > 0);
    }
}

#[cfg(test)]
mod test_sorts {
    use crate::sorts::*;

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
        let generated = gen_random_vec(n);
        assert_ne!(zero_vec, generated);
    }

    #[test]
    fn big_lsg_test() {
        let n = 10000;
        let mut random_vec = gen_random_vec(n);
        let mut copy = random_vec.clone();
        fat_sort(&mut copy);
        assert_ne!(copy, random_vec);
        lsd_sort(&mut random_vec);
        assert_eq!(copy, random_vec);
    }

    #[test]
    fn big_test_heapsort() {
        let n = 10000;
        let mut random_vec = gen_random_vec(n);
        let mut copy = random_vec.clone();
        fat_sort(&mut copy);
        assert_ne!(copy, random_vec);
        heapsort(&mut random_vec);
        assert_eq!(copy, random_vec);
    }

    #[test]
    fn big_test_introsort() {
        let n = 10000;
        let mut random_vec = gen_random_vec(n);
        let mut copy = random_vec.clone();
        fat_sort(&mut copy);
        assert_ne!(copy, random_vec);
        introsort(&mut random_vec);
        assert_eq!(copy, random_vec);
    }

    #[test]
    fn test_timsort() {
        let mut a = vec![6, 2, 4, 5];
        timsort(&mut a);
        assert_eq!(a, [2, 4, 5, 6]);
    }

    #[test]
    fn test_insertion_sort() {
        let mut a = vec![6, 2, 4, 5];
        insertion_sort(&mut a);
        assert_eq!(a, [2, 4, 5, 6]);
    }

    #[test]
    fn big_test_insertion_sort() {
        let n = 10000;
        let mut random_vec = gen_random_vec(n);
        let mut copy = random_vec.clone();
        fat_sort(&mut copy);
        assert_ne!(copy, random_vec);
        insertion_sort(&mut random_vec);
        assert_eq!(copy, random_vec);
    }
}