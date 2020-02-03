use super::V3;

pub const F32_EPSILON: f32 = 1e-10;

pub fn dist2(a: V3, b: V3) -> f32 {
  let d = b - a;
  d.dot(d)
}
#[allow(unused)]
pub fn dist(a: V3, b: V3) -> f32 {
  dist2(a, b).sqrt()
}

pub fn f32_eq(a: f32, b: f32) -> bool {
  (a - b).abs() < F32_EPSILON
}

#[allow(unused)]
pub fn f32_ge(a: f32, b: f32) -> bool {
  a > b || f32_eq(a, b)
}

pub fn randn_v3(mean: f32, std_dev: f32) -> V3 {
  V3([randn(mean, std_dev),
      randn(mean, std_dev),
      randn(mean, std_dev)])
}

pub fn randn(mean: f32, std_dev: f32) -> f32 {
  use rand::thread_rng;
  use rand_distr::{Distribution, Normal};

  let n = Normal::new(mean, std_dev).unwrap();
  n.sample(&mut thread_rng())
}

#[allow(unused)]
pub fn randn0() -> f32 {
  randn(0.0, 1.0)
}
