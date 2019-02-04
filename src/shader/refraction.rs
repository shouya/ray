use common::Color;
use scene::Scene;
use shader::{DynValue, Incidence, Shader};

const BIAS: f32 = 1e-5;

#[derive(Clone)]
pub struct Refraction {
    pub ior: DynValue<f32>,
}

impl Shader for Refraction {
    fn render(&self, s: &Scene, i: &Incidence) -> Option<Color> {
        let ior = self.ior.get(s, i);
        // let ior = 1.0 / ior;
        // let ior = if i.hit.inside { ior } else { 1.0 / ior };
        let bias = if i.hit.inside { BIAS } else { -BIAS };
        let ray = i.ray.refract(&i.hit.biased(bias), ior);
        s.trace_ray(&ray, i.depth + 1)
    }
}
