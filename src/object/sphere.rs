use super::{Cow, Hit, Material, Object, Ray, V3};

#[derive(Debug, Clone)]
pub struct Sphere {
    pub c: V3,
    pub r: f32,
    pub material: Material,
}

impl Object for Sphere {
    // See: http://www.scratchapixel.com/lessons/3d-basic-rendering/minimal-ray-tracer-rendering-simple-shapes/ray-sphere-intersection
    fn intersect(&self, ray: &Ray) -> Option<Hit> {
        let l = self.c - ray.orig;
        let tc = l.dot(ray.dir);
        let mut inside = false;

        if tc < 0.0 {
            return None;
        }

        let d2 = l.dot(l) - tc * tc;
        let r2 = self.r * self.r;

        if d2 > r2 {
            return None;
        }

        let t1c = (r2 - d2).sqrt();
        let t1 = tc - t1c;
        let t2 = tc + t1c;

        let t = if (t1 < 0.0 && t2 > 0.0) || (t1 > 0.0 && t2 < 0.0) {
            inside = true;
            Some(t1.max(t2))
        } else if t1 > 0.0 && t2 > 0.0 {
            Some(t1.min(t2))
        } else {
            None
        };

        if let Some(t) = t {
            let pos = ray.orig + ray.dir * t;
            let norm = (pos - self.c).norm();
            Some(Hit { pos, norm, inside })
        } else {
            None
        }
    }

    fn material(&self, _pos: V3) -> Cow<Material> {
        Cow::Borrowed(&self.material)
    }
}
