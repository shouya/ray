use common::{Color, Hit, Ray};
use scene::Scene;
use shader::diffuse::Diffuse;
use shader::{DynValue, Incidence, Shader};

pub struct Glossy {
  pub color: DynValue<Color>,
  pub roughness: DynValue<f32>,
}

impl Glossy {
  pub fn new(color: DynValue<Color>, roughness: DynValue<f32>) -> Glossy {
    Glossy { color, roughness }
  }
}

const BIAS: f32 = 1e-4;

impl Shader for Glossy {
  fn render(&self, s: &Scene, i: &Incidence) -> Color {
    let color = self.color.get(s, i);
    let diffuse = Diffuse::new(DynValue::Const(color), DynValue::Const(0.0));
    let diffuse_color = diffuse.render(s, i);
    diffuse_color
  }
}
