use super::{DynValue, Incidence, Shader};
use common::{Ray, Color, Hit};
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

impl Shader for Diffuse {
  fn render(&self, s: &Scene, i: &Incidence) -> Color {
    let mut brightness = 0.0;
    let color = self.color.get(s, i);

    let Incidence { hit, ray, obj } = i;

    for light in s.lights.iter() {
      let shadowray_dir = light.pos - hit.pos;
      let shadowray = Ray::new(hit.pos, shadowray_dir).biased(BIAS);
      let angle = (light.pos - hit.pos).norm().dot(hit.norm);

      if let Some((obj, hit)) = s.nearest_hit(&shadowray) {
        if angle <= 0.0 {
          // indirect hit
          brightness += angle * light.brightness;
        } else {
          // pixel is in shadow
          let shadowobj_m = obj.material(hit.pos);
          let mut opaqueness = 1.0 - shadowobj_m.transparency;
          if opaqueness < 1.0 {
            // more rough -> more opaque
            opaqueness += shadowobj_m.roughness;
          }
          brightness -= opaqueness.min(1.0) * light.brightness;
        }
      } else {
        brightness += angle * light.brightness;
      }
    }

    let apparence_color = color;
    let brightness = brightness.min(1.0).max(-1.0);

    if brightness >= 0.0 {
      apparence_color.blend(Color::White, brightness)
    } else {
      apparence_color.blend(Color::Black, -brightness)
    }
  }
}
