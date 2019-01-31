use shader::{Shader, Incidence};
use common::Color;
use scene::Scene;

pub struct Plain {
  color: Color
}

impl Plain {
  pub fn new(color: Color) -> Plain {
    Plain { color }
  }
}

impl Shader for Plain {
  fn render(&self, _s: &Scene, _i: &Incidence) -> Option<Color> {
    Some(self.color)
  }
}
