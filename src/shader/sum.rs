use super::{DynValue, Incidence, Shader};
use common::Color;
use scene::Scene;

#[allow(unused)]
pub struct Sum {
  a: DynValue<Option<Color>>,
  b: DynValue<Option<Color>>,
}

impl Sum {
  #[allow(unused)]
  pub fn new(a: DynValue<Option<Color>>, b: DynValue<Option<Color>>) -> Sum {
    Sum { a, b }
  }
}

impl Shader for Sum {
  fn render(&self, s: &Scene, i: &Incidence) -> Option<Color> {
    let a = self.a.get(s, i)?;
    let b = self.b.get(s, i)?;
    Some(a + b)
  }
}
