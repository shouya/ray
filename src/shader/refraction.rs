use common::{Color};
use scene::Scene;
use shader::{DynValue, Incidence, Shader};

const BIAS: f32 = 1e-5;

pub struct Refraction {
  pub ior: DynValue<f32>,
}

impl Shader for Refraction {
  fn render_depth(&self, s: &Scene, i: &Incidence, d: usize) -> Option<Color> {
    let ior = if i.hit.inside {
      1.0 / (self.ior.get(s, i) + 1e-10)
    } else {
      self.ior.get(s, i)
    };
    let ray = i.ray.refract(&i.hit.biased(BIAS), ior);

    match s.nearest_hit(&ray) {
      None => Some(s.ambient),
      Some((obj, hit)) => {
        let inci = Incidence {
          ray: &ray,
          obj: obj.as_ref(),
          hit: &hit,
        };
        obj.render_depth(s, &inci, d + 1)
      }
    }
  }
}
