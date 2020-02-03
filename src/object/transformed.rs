use crate::common::*;
use crate::object::Object;
use crate::scene::Scene;
use crate::shader::Incidence;

pub struct Transformed {
    obj: Box<dyn Object>,
    // transformation matrix
    trans: TransMat,
}

impl Transformed {
    pub fn new(obj: impl Object + 'static) -> Self {
        Transformed {
            obj: Box::new(obj),
            trans: TransMat::new(),
        }
    }

    pub fn rotated(mut self, r: V3) -> Self {
        self.trans.append(M4::new_rotation(r));
        self
    }

    pub fn translated(mut self, v: V3) -> Self {
        self.trans.append(M4::new_translation(v));
        self
    }

    pub fn scaled(mut self, s: V3) -> Self {
        self.trans.append(M4::new_scaling(s));
        self
    }
}

impl Object for Transformed {
    fn intersect(&self, ray: &Ray) -> Option<Hit> {
        // ray: world to object
        let ray = self.trans.w2o.transform_ray(ray);
        let hit = self.obj.intersect(&ray);

        // hit: object to world
        hit.map(|h| self.trans.o2w.transform_hit(self.trans.w2o.transpose(), &h))
    }

    fn render(&self, s: &Scene, i: &Incidence<'_, '_, '_>) -> Option<Color> {
        self.obj.render(s, &i)
    }

    // implement this method to allow back-face bulling
    fn const_normal(&self) -> Option<V3> {
        self.obj.const_normal()
    }

    fn bound(&self) -> Option<Bound> {
        // TODO: transform bound as well
        None
    }
}
