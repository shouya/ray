use common::Color;
use shader::ShaderType;

use super::preset::{glass, mirror, solid};

pub fn simple_solid(color: Color) -> ShaderType {
  solid(color, 10.0)
}

pub fn simple_glass(color: Color, transparency: f32) -> ShaderType {
  glass(color, 25.0, transparency, 0.8, 1.5)
}

pub fn simple_mirror(color: Color) -> ShaderType {
  mirror(color, 45.0, 0.8)
}
