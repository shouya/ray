use common::*;
use std::borrow::Cow;
use object::{Object, Material};
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

  pub fn new_boxed(object: Box<dyn Object>, shader: Box<dyn Shader>) -> Shaded {
    Shaded { object, shader }
  }
}

impl Object for Shaded {
    fn intersect(&self, ray: &Ray) -> Option<Hit> { self.object.intersect(ray) }
    fn material(&self, pos: V3) -> Cow<Material> { self.object.material(pos) }
    fn const_normal(&self) -> Option<V3> { self.object.const_normal() }
    fn render_depth(&self, s: &Scene, i: &Incidence, d: usize) -> Option<Color> {
      self.shader.render_depth(s, i, d)
    }
}
