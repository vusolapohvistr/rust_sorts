use rand::{thread_rng, Rng};
use rand::distributions::Standard;
use rand::distributions::{Distribution};
use rand_distr::{Normal, Float, StandardNormal};

pub fn gen_random_vec<T>(n: usize) -> Vec<T>
where Standard: Distribution<T>,
      T: Copy + Eq + Ord
{
    let mut rng = thread_rng();
    (0..n).map(|_| {
        rng.gen()
    }).collect()
}

pub fn gen_random_vec_range<T>(n: usize, low: T, high: T) -> Vec<T>
    where Standard: Distribution<T>,
          T: Copy + Eq + Ord + rand::distributions::uniform::SampleUniform
{
    let mut rng = thread_rng();
    (0..n).map(|_| {
        rng.gen_range(low, high)
    }).collect()
}

pub fn gen_random_vec_normal_dist(n: usize, mean: f64, dist: f64) -> Vec<usize>
{
    let normal = Normal::new(mean, dist).unwrap();
    let mut rng = thread_rng().sample_iter(normal);
    (0..n).map(|_| {
        rng.next().unwrap() as usize
    }).collect()
}