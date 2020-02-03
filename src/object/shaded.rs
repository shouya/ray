use crate::common::*;
use crate::object::Object;
use crate::scene::Scene;
use crate::shader::{Incidence, ShaderType};

pub struct Shaded {
    pub object: Box<dyn Object>,
    pub shader: ShaderType,
}

impl Shaded {
    pub fn new(object: impl Object + 'static, shader: ShaderType) -> Shaded {
        Shaded {
            object: Box::new(object),
            shader: shader,
        }
    }
}

impl Object for Shaded {
    fn intersect(&self, ray: &Ray) -> Option<Hit> {
        self.object.intersect(ray)
    }
    fn const_normal(&self) -> Option<V3> {
        self.object.const_normal()
    }
    fn render(&self, s: &Scene, i: &Incidence<'_, '_, '_>) -> Option<Color> {
        self.shader.get(s, i)
    }
}
