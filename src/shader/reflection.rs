use common::Color;
use scene::Scene;
use shader::{DynValue, Incidence, Shader};

const BIAS: f32 = 1e-5;

pub struct Reflection {
  pub reflectivity: DynValue<f32>,
}

impl Shader for Reflection {
  fn render_depth(&self, s: &Scene, i: &Incidence, d: usize) -> Option<Color> {
    let ray = i.ray.reflect(&i.hit.biased(BIAS));
    s.trace_ray(&ray, d + 1)
  }
}
