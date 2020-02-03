use super::{Color, V3};

#[derive(Debug, Clone)]
pub struct PointLight {
  pub pos: V3,
  pub color: Color,
  pub brightness: f32, // 0: turned off
}
