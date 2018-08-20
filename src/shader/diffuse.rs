use super::{DynValue, Incidence, Shader};
use common::{Color, Hit, Ray};
use scene::Scene;

pub struct Diffuse {
  pub color: DynValue<Color>,
  pub roughness: DynValue<f32>,
}

impl Diffuse {
  pub fn new(color: DynValue<Color>, roughness: DynValue<f32>) -> Diffuse {
    Diffuse { color, roughness }
  }
}

const BIAS: f32 = 1e-4;
const BACKGROUND_BRIGHTNESS: Color = Color([0.05; 3]);

impl Shader for Diffuse {
  fn render(&self, s: &Scene, i: &Incidence) -> Color {
    let color = self.color.get(s, i);
    let mut brit = BACKGROUND_BRIGHTNESS;
    let Incidence { hit, ray, obj } = i;

    for light in s.lights.iter() {
      let shadowray_dir = light.pos - hit.pos;
      let shadowray = Ray::new(hit.pos, shadowray_dir).biased(BIAS);
      let angle = shadowray_dir.norm().dot(hit.norm);

      if let Some((obj, hit)) = s.nearest_hit(&shadowray) {
        if angle <= 0.0 {
          // indirect hit
          // brit = brit + angle * light.brightness;
        } else {
          // pixel is in shadow
          brit = brit + (Color::White + light.brightness * -1.0) * -1.0;
        }
      } else {
        // the surface is directly lighten
        brit = brit + angle * light.brightness;
      }
    }

    let brit = brit.clamp(0.0, 1.0);

    color.mult(brit)
  }
}
