use super::{DynValue, Incidence, Shader};
use common::Color;
use scene::Scene;

pub struct Mix {
  frac: DynValue<f32>,
  a: Box<dyn Shader>,
  b: Box<dyn Shader>,
}

impl Mix {
  pub fn new(a: Box<dyn Shader>, b: Box<dyn Shader>, frac: DynValue<f32>) -> Mix {
    Mix { a, b, frac }
  }
}

impl Shader for Mix {
  fn render_depth(&self, s: &Scene, i: &Incidence, depth: usize) -> Option<Color> {
    let f = self.frac.get(s, i);
    if f <= 0.0 {
      self.b.render_depth(s, i, depth)
    } else if f >= 1.0 {
      self.a.render_depth(s, i, depth)
    } else {
      let left = self.a.render_depth(s, i, depth).unwrap_or(Color::Black);
      let right = self.b.render_depth(s, i, depth).unwrap_or(Color::Black);
      Some(left.blend(right, f))
    }
  }

  fn is_transparent(&self) -> bool {
    self.a.is_transparent() || self.b.is_transparent()
  }
}
