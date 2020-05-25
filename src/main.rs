use std::time::{Instant, Duration};
use crate::generate_tests::{gen_random_vec, test_sort, gen_random_vec_range, gen_random_vec_normal_dist};
use rand::distributions::{Distribution, Standard};
use crate::sorts::{heapsort, timsort, lsd_sort};
use std::fmt::{Debug, Error};
use std::fs;
use std::io::{Write, Result};

mod sorts;
mod generate_tests;

const BENCHMARK_FILE_PATH: &str = "benchmark_results.csv";

fn main() -> Result<()> {
    let mut test_results = Vec::new();
    let mut testing_sorts = [heapsort, timsort, sort_unstable, sort_stable];
    let mut testing_sorts_names = ["heapsort", "timsort", "sort_unstable", "sort_stable"];
    for (i, sort) in testing_sorts.iter_mut().enumerate(){
        let mut test_result = complex_test_sorts(sort, String::from(testing_sorts_names[i]));
        test_results.extend(test_result);
    }
    fs::remove_file(BENCHMARK_FILE_PATH).unwrap();
    let mut file = fs::File::create(BENCHMARK_FILE_PATH)?;
    writeln!(file, "{}", "sort_name, test_data_type, test_data_length, duration(ms)");
    write!(file, "{}", SortTestResult::lines(&test_results));

    Ok(())
}

fn sort_stable<T>(vec: &mut [T])
where T: Copy + Eq + Ord
{
    vec.sort()
}

fn sort_unstable<T>(vec: &mut [T])
where T: Copy + Eq + Ord
{
    vec.sort_unstable()
}

#[derive(Debug)]
struct SortTestResult {
    sort_name: String,
    test_data_length: usize,
    test_data_type: String,
    duration: Duration,
}

impl SortTestResult {
    fn to_line(&self) -> String {
        format!("{}, {}, {}, {}, \n", self.sort_name, self.test_data_type, self.test_data_length, self.duration.as_micros())
    }

    fn lines(arr: &[Self]) -> String {
        let mut result = String::new();
        for el in arr.iter() {
            result.push_str(&el.to_line());
        }
        result
    }
}

fn complex_test_sorts<F>(sort: &mut F, sort_name: String) -> Vec<SortTestResult>
where F: FnMut(&mut [usize]),
{
    let mut results = Vec::new();
    // let sizes: [usize; 4] = [30_000, 100_000, 300_000, 1_000_000];
    let sizes: [usize; 3] = [30000, 100000, 300000];
    for size in sizes.iter() {
        let mut test_data = gen_random_vec(*size);
        let duration = test_sort(sort, test_data.as_mut_slice());
        results.push(SortTestResult {
            sort_name: sort_name.clone(),
            test_data_length: *size,
            test_data_type: String::from("uniform 0 to 2^31"),
            duration,
        });
        let mut test_data = gen_random_vec_range(*size, 0, *size - 1);
        let duration = test_sort(sort, test_data.as_mut_slice());
        results.push(SortTestResult {
            sort_name: sort_name.clone(),
            test_data_length: *size,
            test_data_type: String::from("uniform 0 to N - 1"),
            duration,
        });
        let mut test_data = gen_random_vec(*size);
        let duration = test_sort(sort, test_data.as_mut_slice());
        results.push(SortTestResult {
            sort_name: sort_name.clone(),
            test_data_length: *size,
            test_data_type: String::from("uniform 0 to 2^15"),
            duration,
        });
        let mut test_data = gen_random_vec_normal_dist(*size, 2e32 / 2.0, 2e32 / 6.0);
        let duration = test_sort(sort, test_data.as_mut_slice());
        results.push(SortTestResult {
            sort_name: sort_name.clone(),
            test_data_length: *size,
            test_data_type: String::from("normal 0 to 2^31"),
            duration,
        });
    }
    results
}