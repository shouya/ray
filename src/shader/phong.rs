use common::{Color, Hit, Ray};
use scene::Scene;
use shader::diffusion::Diffusion;
use shader::{DynValue, Incidence, Shader};

pub struct Phong {
  pub specular_index: DynValue<f32>,
}

impl Phong {
  pub fn new(specular_index: DynValue<f32>) -> Phong {
    Phong { specular_index }
  }
}

const BIAS: f32 = 1e-4;

impl Shader for Phong {
  fn render(&self, s: &Scene, i: &Incidence) -> Option<Color> {
    let p = self.specular_index.get(s, i);
    let Incidence { hit, ray, obj, .. } = i;
    let sight = -ray.dir;
    let mut intensity = Color::Black;

    for light in s.lights.iter() {
      let shadowray_dir = light.pos - hit.pos;
      let shadowray = Ray::new(hit.pos, shadowray_dir).biased(BIAS);

      if !s.is_blocked(&shadowray) {
        let angle = shadowray_dir.norm().dot(hit.norm).max(0.0);
        let h = (sight + shadowray_dir).norm();
        intensity = intensity + h.dot(hit.norm).max(0.0).powf(p);
      }
    }

    Some(intensity)
  }
}
