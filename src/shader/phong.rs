use common::{Color, Hit, Ray};
use scene::Scene;
use shader::diffusion::Diffusion;
use shader::{DynValue, Incidence, Shader};

pub struct Phong {
  pub specular_color: DynValue<Color>,
  pub specular_index: DynValue<f32>,
}

impl Phong {
  pub fn new(specular_color: DynValue<Color>, specular_index: DynValue<f32>) -> Phong {
    Phong { specular_color, specular_index }
  }
}

const BIAS: f32 = 1e-4;
const BACKGROUND_INTENSITY: Color = Color([0.0; 3]);

impl Shader for Phong {
  fn render(&self, s: &Scene, i: &Incidence) -> Option<Color> {
    let color = self.specular_color.get(s, i);
    let p = self.specular_index.get(s, i);
    let Incidence { hit, ray, obj, .. } = i;
    let v = -ray.dir;
    let mut i = BACKGROUND_INTENSITY;

    for light in s.lights.iter() {
      let l = light.pos - hit.pos;
      let l_ray = Ray::new(hit.pos, l).biased(BIAS);

      if let Some((obj, hit)) = s.nearest_hit(&l_ray) {
        // the light is blocked
      } else {
        // the surface is directly lighten
        let h = (v + l).norm();
        i = i + h.dot(hit.norm).max(0.0).powf(p);
      }
    }

    Some(color.mult(i))
  }
}
