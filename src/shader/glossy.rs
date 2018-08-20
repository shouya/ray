use common::{Color, Hit, Ray};
use scene::Scene;
use shader::diffuse::Diffuse;
use shader::{DynValue, Incidence, Shader};

pub struct Glossy {
  pub color: DynValue<Color>,
  pub roughness: DynValue<f32>,
}

const REFL_COUNT: usize = 5;
const BIAS: f32 = 1e-4;
const MAX_DEPTH: usize = 5;

impl Glossy {
  pub fn new(color: DynValue<Color>, roughness: DynValue<f32>) -> Glossy {
    Glossy { color, roughness }
  }

  pub fn pure_glossy(&self, s: &Scene, i: &Incidence, d: usize) -> Color {
    let roughness = self.roughness.get(s, i);
    let color = self.color.get(s, i);
    let refl_rays = i.ray.drift_array(roughness, REFL_COUNT, BIAS);
    let mut brit = color;

    for ray in refl_rays.into_iter() {
      if let Some((obj, hit)) = s.nearest_hit(&ray) {
        let inci = Incidence {
          obj: obj.as_ref(),
          hit: &hit,
          ray: &ray,
        };

        if let Some(obj_color) = obj.render_depth(s, &inci, d+1) {
          brit = brit.mult(obj_color);
        }
      }
    }

    brit
  }

  pub fn pure_diffusive(&self, s: &Scene, i: &Incidence) -> Color {
    let color = self.color.get(s, i);
    let diffuse = Diffuse::new(color.into(), 0.0.into());
    let diffuse_color = diffuse.render(s, i);
    diffuse_color
  }
}

impl Shader for Glossy {
  fn render_depth(&self, s: &Scene, i: &Incidence, d: usize) -> Option<Color> {
    if d > MAX_DEPTH {
      return Some(self.color.get(s, i))
    }
    let roughness = self.roughness.get(s, i);
    let glossy_color = self.pure_glossy(s, i, d);
    let diffusive_color = self.pure_diffusive(s, i);
    let color = glossy_color.mix_with(diffusive_color, |a, b| {
      a * (1.0-roughness) + b * roughness
    });

    Some(color)
  }
}
