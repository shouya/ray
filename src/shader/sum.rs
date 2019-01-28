use super::{DynValue, Incidence, Shader};
use common::Color;
use scene::Scene;

pub struct Sum {
  a: Box<dyn Shader>,
  b: Box<dyn Shader>,
}

impl Sum {
  pub fn new(a: Box<dyn Shader>, b: Box<dyn Shader>) -> Sum {
    Sum { a, b }
  }
}

impl Shader for Sum {
  fn render_depth(&self, s: &Scene, i: &Incidence, depth: usize) -> Option<Color> {
    self.a.render_depth(s, i, depth).and_then(|color_a| {
      self.b.render_depth(s, i, depth).and_then(|color_b| {
        Some(color_a + color_b)
      })
    })
  }

  fn is_transparent(&self) -> bool {
    self.a.is_transparent() || self.b.is_transparent()
  }
}
