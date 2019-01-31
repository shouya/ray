use super::{DynValue, Incidence, Shader};
use common::{Color, Hit, Ray};
use scene::Scene;

pub struct Diffusion {
  pub color: DynValue<Color>,
  pub roughness: DynValue<f32>,
}

impl Diffusion {
  pub fn new(color: DynValue<Color>, roughness: DynValue<f32>) -> Self {
    Self { color, roughness }
  }
}

const BIAS: f32 = 1e-4;

// so a scene with no light can still has some color in it.
const BACKGROUND_INTENSITY: Color = Color([0.03; 3]);

impl Shader for Diffusion {
  fn render(&self, s: &Scene, i: &Incidence) -> Option<Color> {
    let color = self.color.get(s, i);
    let mut intensity = BACKGROUND_INTENSITY;
    let Incidence { hit, ray, obj, .. } = i;

    for light in s.lights.iter() {
      let shadowray_dir = light.pos - hit.pos;
      let shadowray = Ray::new(hit.pos, shadowray_dir).biased(BIAS);
      let angle = shadowray_dir.norm().dot(hit.norm).max(0.0);

      if let Some((obj, hit)) = s.nearest_hit(&shadowray) {
        // the light is blocked
      } else {
        // the surface is directly lighten
        intensity = intensity + angle * light.brightness;
      }
    }

    Some(color.mult(intensity))
  }
}
