use super::*;
use common::*;

#[derive(Debug, Clone)]
pub struct Triangle {
    pub trig: Trig,
    pub double_sided: bool,
}

impl Triangle {
    pub fn new(a: V3, b: V3, c: V3) -> Self {
        Triangle {
            trig: Trig(a, b, c),
            double_sided: false,
        }
    }

    pub fn double_sided(mut self, b: bool) -> Self {
        self.double_sided = b;
        self
    }
}

impl Object for Triangle {
    fn intersect(&self, ray: &Ray) -> Option<Hit> {
        let pos = self.trig.intersect(ray)?;

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

    fn const_normal(&self) -> Option<V3> {
        if self.double_sided {
            None
        } else {
            Some(self.trig.n())
        }
    }
}

#[derive(Debug, Clone)]
pub struct Rectangle {
    pub t1: Triangle,
    pub t2: Triangle,
}

impl Rectangle {
    // follows CCW order, b is the right angle
    // c -> +--+ <- b
    //       \ |
    //      .  + <- a
    pub fn new(a: V3, b: V3, c: V3) -> Self {
        let ba = a - b;
        let bc = c - b;
        assert!(f32_eq(ba.dot(bc), 0.0));

        let d = c + ba;
        let t1 = Triangle::new(a, b, c);
        let t2 = Triangle::new(c, d, a);

        Rectangle {
            t1,
            t2,
        }
    }

    pub fn double_sided(mut self, b: bool) -> Self {
        self.t1 = self.t1.double_sided(b);
        self.t2 = self.t2.double_sided(b);
        self
    }
}

impl<'a> Object for Rectangle {
    fn intersect(&self, ray: &Ray) -> Option<Hit> {
        self.t1.intersect(ray).or_else(|| self.t2.intersect(ray))
    }

    fn const_normal(&self) -> Option<V3> {
        if self.t1.double_sided {
            None
        } else {
            Some(self.t1.trig.n())
        }
    }
}
