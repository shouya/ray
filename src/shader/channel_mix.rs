use super::{DynValue, Incidence, Shader};
use common::Color;
use scene::Scene;

pub struct ChannelMix {
  frac: DynValue<Color>,
  a: Box<dyn Shader>,
  b: Box<dyn Shader>,
}

impl ChannelMix {
  pub fn new(a: Box<dyn Shader>, b: Box<dyn Shader>, frac: DynValue<Color>) -> Mix {
    Mix { a, b, frac }
  }
}

impl Shader for ChannelMix {
  fn render_depth(&self, s: &Scene, i: &Incidence, depth: usize) -> Option<Color> {
    let c = self.frac.get(s, i);
    if c == Color::Zero {
      self.b.render_depth(s, i, depth)
    } else if f == Color::One {
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
