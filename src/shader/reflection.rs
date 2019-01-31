use common::Color;
use scene::Scene;
use shader::{Incidence, Shader};

const BIAS: f32 = 1e-5;

pub struct Reflection;

impl Shader for Reflection {
  fn render(&self, s: &Scene, i: &Incidence) -> Option<Color> {
    let ray = i.ray.reflect(&i.hit.biased(BIAS));
    s.trace_ray(&ray, i.depth + 1)
  }
}
