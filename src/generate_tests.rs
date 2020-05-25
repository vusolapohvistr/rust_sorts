use rand::{thread_rng, Rng};
use rand::distributions::{Standard, Uniform};
use rand::distributions::{Distribution};
use rand_distr::{Normal};
use std::time::{Instant, Duration};
use std::convert::{From};

pub fn gen_random_vec<T>(n: usize) -> Vec<T>
where Standard: Distribution<T>,
      T: Copy + Eq + Ord
{
    thread_rng().sample_iter(Standard).take(n).collect()
}

pub fn gen_random_vec_range<T>(n: usize, low: T, high: T) -> Vec<T>
    where Standard: Distribution<T>,
          T: Copy + Eq + Ord + rand::distributions::uniform::SampleUniform
{
    thread_rng().sample_iter(Uniform::new(low, high)).take(n).collect()
}

pub fn gen_random_vec_normal_dist(n: usize, mean: f64, dist: f64) -> Vec<usize>
{
    thread_rng()
        .sample_iter(Normal::new(mean, dist).unwrap()).map(|a| a as usize)
        .take(n)
        .collect()
}

pub fn test_sort<T, F>(sort: &mut F, arr: &mut [T]) -> Duration
    where F: FnMut(&mut [T])
{
    let now = Instant::now();
    sort(arr);
    now.elapsed()
}