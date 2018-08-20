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
    self.a.render_depth(s, i, depth).and_then(|color_a| {
      self.b.render_depth(s, i, depth).and_then(|color_b| {
        let c = Some(color_a.blend(color_b, f));
        c
      })
    })
  }

  fn is_transparent(&self) -> bool {
    self.a.is_transparent() || self.b.is_transparent()
  }
}
