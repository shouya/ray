use random_color::RandomColor;
use std::ops::{Add, Mul, Sub};

use super::f32_eq;

#[derive(Debug, Clone, Copy)]
pub struct Color(pub [f32; 3]);

#[allow(non_upper_case_globals)]
impl Color {
  #[allow(dead_code)]
  pub const Red: Color = Color([1.0, 0.0, 0.0]);
  #[allow(dead_code)]
  pub const Green: Color = Color([0.0, 1.0, 0.0]);
  #[allow(dead_code)]
  pub const Blue: Color = Color([0.0, 0.0, 1.0]);
  #[allow(dead_code)]
  pub const White: Color = Color([1.0, 1.0, 1.0]);
  #[allow(dead_code)]
  pub const Black: Color = Color([0.0, 0.0, 0.0]);
  pub const Zero: Color = Self::Black;
  pub const One: Color = Self::White;

  pub fn from_intensity(i: f32) -> Color {
    Color([i, i, i])
  }

  pub fn r(&self) -> f32 {
    self.0[0]
  }
  pub fn g(&self) -> f32 {
    self.0[1]
  }
  pub fn b(&self) -> f32 {
    self.0[2]
  }

  // ratio: 1: all self, 0: all rhs
  pub fn blend(&self, rhs: Color, ratio: f32) -> Color {
    let (t0, t1) = (ratio, 1.0 - ratio);
    self.mix_with(rhs, |l, r| l * t0 + r * t1)
  }

  #[allow(unused)]
  pub fn blend_all(colors: &[Color]) -> Color {
    if colors.is_empty() {
      return Color([0.0, 0.0, 0.0]);
    }

    let n = colors.len() as f32;
    let r: f32 = colors.iter().map(|x| x.r()).sum();
    let g: f32 = colors.iter().map(|x| x.g()).sum();
    let b: f32 = colors.iter().map(|x| x.b()).sum();
    Color([r / n, g / n, b / n])
  }

  pub fn channel_blend(&self, rhs: Color, frac: Color) -> Color {
    let c = frac;
    Color([self.r() * c.r() + rhs.r() * (1.0 - c.r()),
           self.g() * c.g() + rhs.g() * (1.0 - c.g()),
           self.b() * c.b() + rhs.b() * (1.0 - c.b())])
  }

  pub fn mix_with<F>(&self, rhs: Color, f: F) -> Color
    where F: Fn(f32, f32) -> f32
  {
    Color([f(self.r(), rhs.r()),
           f(self.g(), rhs.g()),
           f(self.b(), rhs.b())])
  }

  pub fn clamp(&self, min: f32, max: f32) -> Color {
    Color([self.r().max(min).min(max),
           self.g().max(min).min(max),
           self.b().max(min).min(max)])
  }
  pub fn regularize(&self) -> Color {
    self.clamp(0.0, 1.0)
  }

  #[allow(unused)]
  pub fn mult(self, brightness: Color) -> Color {
    self.mix_with(brightness, |a, b| a * b)
  }

  pub fn from_rgb(hex: [u32; 3]) -> Self {
    let [r, g, b] = hex;
    Self([r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0])
  }

  pub fn random() -> Self {
    Color::from_rgb(RandomColor::new().to_rgb_array())
  }

  pub fn average(colors: &[Color]) -> Color {
    if colors.is_empty() {
      return Color::Black;
    }

    let mut res = Color::Black;
    colors.iter().for_each(|c| res = res + *c);

    res * (1.0 / (colors.len() as f32))
  }
}

impl PartialEq for Color {
  fn eq(&self, other: &Color) -> bool {
    f32_eq(self.r(), other.r())
    && f32_eq(self.g(), other.g())
    && f32_eq(self.b(), other.b())
  }
}

impl Into<[u8; 3]> for Color {
  fn into(self) -> [u8; 3] {
    let normalize = |k: f32| {
      if k < 0.0 {
        0
      } else if k > 1.0 {
        1
      } else {
        (k * 255.0) as u8
      }
    };
    [normalize(self.r()),
     normalize(self.g()),
     normalize(self.b())]
  }
}

impl Add<Color> for Color {
  type Output = Color;
  fn add(self, rhs: Self) -> Color {
    Color([self.r() + rhs.r(), self.g() + rhs.g(), self.b() + rhs.b()])
  }
}

impl Sub<Color> for Color {
  type Output = Color;
  fn sub(self, rhs: Self) -> Color {
    Color([self.r() - rhs.r(), self.g() - rhs.g(), self.b() - rhs.b()])
  }
}

impl Add<f32> for Color {
  type Output = Color;
  fn add(self, rhs: f32) -> Color {
    Color([self.r() + rhs, self.g() + rhs, self.b() + rhs])
  }
}

impl Mul<Color> for Color {
  type Output = Color;
  fn mul(self, rhs: Color) -> Color {
    Color([self.r() * rhs.r(), self.g() * rhs.g(), self.b() * rhs.b()])
  }
}

impl Mul<f32> for Color {
  type Output = Color;
  fn mul(self, rhs: f32) -> Color {
    Color([self.r() * rhs, self.g() * rhs, self.b() * rhs])
  }
}
