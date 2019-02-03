use common::{Color, Ray, dist2};
use scene::Scene;
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
    let Incidence { hit, ray, .. } = i;
    let mut intensity = Color::Black;

    for light in s.lights.iter() {
      let light_pos = i.mat.map(|x| x.1 * light.pos).unwrap_or(light.pos);
      let shadowray_dir = light_pos - hit.pos;
      let shadowray = Ray::new(hit.pos, shadowray_dir).biased(BIAS);

      if !s.is_blocked(&shadowray, dist2(light_pos, hit.pos)) {
        let refl_ray = shadowray.reflect(hit);
        let angle = refl_ray.dir.dot(ray.dir);
        intensity = intensity + angle.max(0.0).powf(p);
      }
    }

    Some(intensity)
  }
}
