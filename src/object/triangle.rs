use super::*;
use common::*;

#[derive(Debug, Clone)]
pub struct Triangle<'a> {
    pub trig: Trig,
    pub material: Cow<'a, Material>,
}

impl<'a> Object for Triangle<'a> {
    fn intersect(&self, ray: &Ray) -> Option<Hit> {
        let pos = self.trig.to_plane().intersect(ray)?;

        // check if pos is inside the triangle
        if !self.trig.contains(pos) {
            return None;
        }

        let norm = self.trig.n();
        Some(Hit {
            pos,
            norm,
            inside: ray.dir.dot(norm) > 0.0,
        })
    }

    fn material(&self, _pos: V3) -> Cow<Material> {
        Cow::Borrowed(&self.material)
    }
}
