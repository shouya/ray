use common::{Color, Ray, dist2};
use scene::Scene;
use shader::{DynValue, Incidence, Shader};

pub struct Normal;

impl Shader for Normal {
  fn render(&self, s: &Scene, i: &Incidence) -> Option<Color> {
    let n = i.hit.norm;
    let c = Color([n.x(), n.y(), n.z()]);
    Some(c)
  }
}
