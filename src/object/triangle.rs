use super::*;
use common::*;

#[derive(Debug, Clone)]
pub struct Triangle<'a> {
    pub trig: Trig,
    pub material: Cow<'a, Material>,
    pub double_sided: bool,
}

impl<'a> Triangle<'a> {
    pub fn new(a: V3, b: V3, c: V3, m: Cow<'a, Material>) -> Self {
        Triangle {
            trig: Trig(a, b, c),
            material: m,
            double_sided: false,
        }
    }

    pub fn double_sided(mut self, b: bool) -> Self {
        self.double_sided = b;
        self
    }
}

impl<'a> Object for Triangle<'a> {
    fn intersect(&self, ray: &Ray) -> Option<Hit> {
        let pos = self.trig.to_plane().intersect(ray)?;

        // check if pos is inside the triangle
        if !self.trig.contains(pos) {
            return None;
        }

        let norm = self.trig.n();
        let cosi = ray.dir.dot(norm);
        if cosi > 0.0 && !self.double_sided {
            return None;
        }

        Some(Hit {
            pos,
            norm,
            inside: cosi > 0.0,
        })
    }

    fn material(&self, _pos: V3) -> Cow<Material> {
        Cow::Borrowed(&self.material)
    }

    fn const_normal(&self) -> Option<V3> {
        if self.double_sided {
            None
        } else {
            Some(self.trig.n())
        }
    }
}
