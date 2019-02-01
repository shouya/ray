use common::*;
use object::Object;
use shader::{Shader, Incidence};
use scene::Scene;

pub struct Shaded {
  pub object: Box<dyn Object>,
  pub shader: Box<dyn Shader>,
}

impl Shaded {
  pub fn new(object: impl Object + 'static, shader: impl Shader + 'static) -> Shaded {
    Shaded {
      object: Box::new(object),
      shader: Box::new(shader),
    }
  }
}

impl Object for Shaded {
    fn intersect(&self, ray: &Ray) -> Option<Hit> { self.object.intersect(ray) }
    fn const_normal(&self) -> Option<V3> { self.object.const_normal() }
    fn render(&self, s: &Scene, i: &Incidence) -> Option<Color> {
      self.shader.render(s, i)
    }
}
