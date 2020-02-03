use super::f32_eq;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Clone, Copy)]
pub struct V3(pub [f32; 3]);

// V3 with normal
#[derive(Debug, Clone, Copy)]
pub struct V3N {
  pub v: V3,
  pub n: V3,
}

// ray uses right hand side coordinate system:
// positive directions:
//  X - right, Y - up, Z - out of screen
// CCW rotations are positive
impl V3 {
  pub fn x(&self) -> f32 {
    self.0[0]
  }
  pub fn y(&self) -> f32 {
    self.0[1]
  }
  pub fn z(&self) -> f32 {
    self.0[2]
  }

  pub fn dot(self, rhs: Self) -> f32 {
    self.x() * rhs.x() + self.y() * rhs.y() + self.z() * rhs.z()
  }
  pub fn magn(self) -> f32 {
    self.dot(self).sqrt()
  }
  pub fn norm(self) -> Self {
    if self.magn() == 0.0 {
      V3::zero()
    } else {
      self / self.magn()
    }
  }
  pub fn cross(self, rhs: V3) -> V3 {
    let (u1, u2, u3) = (self.x(), self.y(), self.z());
    let (v1, v2, v3) = (rhs.x(), rhs.y(), rhs.z());
    V3([u2 * v3 - u3 * v2, u3 * v1 - u1 * v3, u1 * v2 - u2 * v1])
  }
  pub fn zero() -> V3 {
    V3([0.0, 0.0, 0.0])
  }

  #[allow(unused)]
  pub fn is_zero(&self) -> bool {
    f32_eq(self.0[0], 0.0) && f32_eq(self.0[1], 0.0) && f32_eq(self.0[2], 0.0)
  }
}

impl Sub<f32> for V3 {
  type Output = V3;
  fn sub(self, rhs: f32) -> V3 {
    V3([self.x() - rhs, self.y() - rhs, self.z() - rhs])
  }
}
impl Add<f32> for V3 {
  type Output = V3;
  fn add(self, rhs: f32) -> V3 {
    V3([self.x() + rhs, self.y() + rhs, self.z() + rhs])
  }
}
impl Neg for V3 {
  type Output = V3;
  fn neg(self) -> V3 {
    V3([-self.x(), -self.y(), -self.z()])
  }
}
impl Mul<f32> for V3 {
  type Output = V3;
  fn mul(self, rhs: f32) -> V3 {
    V3([self.x() * rhs, self.y() * rhs, self.z() * rhs])
  }
}
impl Div<f32> for V3 {
  type Output = V3;
  fn div(self, rhs: f32) -> V3 {
    V3([self.x() / rhs, self.y() / rhs, self.z() / rhs])
  }
}

impl Sub<V3> for V3 {
  type Output = V3;

  fn sub(self, rhs: V3) -> V3 {
    V3([self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z()])
  }
}

impl Add<V3> for V3 {
  type Output = V3;

  fn add(self, rhs: V3) -> V3 {
    V3([self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z()])
  }
}

impl Default for V3 {
  fn default() -> Self {
    V3([0., 0., 0.])
  }
}

impl PartialEq for V3 {
  fn eq(&self, other: &V3) -> bool {
    f32_eq(self.x(), other.x())
    && f32_eq(self.y(), other.y())
    && f32_eq(self.z(), other.z())
  }
}
