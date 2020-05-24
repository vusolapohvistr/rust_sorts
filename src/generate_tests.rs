use rand::{thread_rng, Rng};
use rand::distributions::{Standard, Uniform};
use rand::distributions::{Distribution};
use rand_distr::{Normal};

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

pub fn gen_random_vec_normal_dist(n: usize, mean: f64, dist: f64) -> Vec<f64>
{
    thread_rng()
        .sample_iter(Normal::new(mean, dist).unwrap())
        .take(n)
        .collect()
}