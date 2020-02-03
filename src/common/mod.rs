use std::f32;
use std::mem;
use std::ops::{Add, Neg};

mod bound;
mod color;
mod light;
mod math;
mod ray;
mod shape;
mod transformation;
mod trig;
mod v2;
mod v3;

pub use bound::{Bound, BoundingBox, BoundingSphere};
pub use color::Color;
pub use light::PointLight;
pub use math::*;
pub use ray::{Hit, Ray};
pub use shape::{Line, Plane};
pub use transformation::{TransMat, M33, M4};
pub use trig::{Trig, TrigGen, TrigN};
pub use v2::V2;
pub use v3::{V3, V3N};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Projection {
  Orthogonal,
  Perspective,
}
