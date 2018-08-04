use super::{DynValue, Incidence, Shader};
use common::Color;
use scene::Scene;

pub struct MixShader {
  frac: DynValue<f32>,
  a: Box<dyn Shader>,
  b: Box<dyn Shader>,
}

impl MixShader {
  pub fn new(a: Box<dyn Shader>, b: Box<dyn Shader>, frac: DynValue<f32>) -> MixShader {
    MixShader { a, b, frac }
  }
}

impl Shader for MixShader {
  fn shade(&self, s: &Scene, i: &Incidence) -> Color {
    let f = self.frac.get(s, i);
    let color_a = self.a.shade(s, i);
    let color_b = self.b.shade(s, i);
    color_a.blend(color_b, f)
  }
}
