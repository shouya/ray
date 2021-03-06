use crate::common::Color;
use crate::scene::Scene;
use crate::shader::{Incidence, Shader};

const BIAS: f32 = 1e-5;

pub struct Reflection;

impl Shader for Reflection {
    fn render(&self, s: &Scene, i: &Incidence<'_, '_, '_>) -> Option<Color> {
        let bias = if i.hit.inside { -BIAS } else { BIAS };
        let ray = i.ray.reflect(&i.hit.biased(bias));
        s.trace_ray(&ray, i.depth + 1)
    }
}
