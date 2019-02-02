use common::{Color, Ray};
use scene::Scene;
use shader::{DynValue, Incidence, Shader};

pub struct Diffuse {
  pub color: DynValue<Color>,
}

impl Diffuse {
  pub fn new(color: DynValue<Color>) -> Self {
    Self { color }
  }
}

const BIAS: f32 = 1e-4;

impl Shader for Diffuse {
  fn render(&self, s: &Scene, i: &Incidence) -> Option<Color> {
    let color = self.color.get(s, i);
    let mut intensity = s.background_light;
    let Incidence { hit, .. } = i;

    for light in s.lights.iter() {
      let shadowray_dir = light.pos - hit.pos;
      let shadowray = Ray::new(hit.pos, shadowray_dir).biased(BIAS);

      if !s.is_blocked(&shadowray) {
        let angle = shadowray_dir.norm().dot(hit.norm).max(0.0);
        intensity = intensity + light.color * light.brightness * angle;
      }
    }

    Some(color * intensity)
  }
}
