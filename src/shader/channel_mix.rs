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
  fn render(&self, s: &Scene, i: &Incidence) -> Option<Color> {
    let c = self.frac.get(s, i);
    if c == Color::Zero {
      self.b.render(s, i)
    } else if f == Color::One {
      self.a.render(s, i)
    } else {
      let left = self.a.render(s, i)?;
      let right = self.b.render(s, i)?;
      Some(left.blend(right, f))
    }
  }

  fn is_transparent(&self) -> bool {
    self.a.is_transparent() || self.b.is_transparent()
  }
}
