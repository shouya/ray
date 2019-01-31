use super::{DynValue, Incidence, Shader};
use common::Color;
use scene::Scene;

pub struct Mix {
  frac: DynValue<f32>,
  a: DynValue<Option<Color>>,
  b: DynValue<Option<Color>>
}

impl Mix {
  pub fn new(a: DynValue<Option<Color>>, b: DynValue<Option<Color>>, frac: DynValue<f32>) -> Mix {
    Mix { a, b, frac }
  }
}

impl Shader for Mix {
  fn render(&self, s: &Scene, i: &Incidence) -> Option<Color> {
    let f = self.frac.get(s, i);
    if f <= 0.0 {
      self.a.get(s, i)
    } else if f >= 1.0 {
      self.b.get(s, i)
    } else {
      let left = self.a.get(s, i)?;
      let right = self.b.get(s, i)?;
      Some(left.blend(right, f))
    }
  }
}
