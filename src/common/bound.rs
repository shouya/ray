use super::*;

#[derive(Debug, Clone)]
pub struct BoundingBox {
  pub min: V3,
  pub max: V3,
}

#[derive(Debug, Clone)]
pub struct BoundingSphere {
  pub c: V3,
  pub r: f32,
}

#[allow(dead_code)]
pub enum Bound {
  BoundingBox(BoundingBox),
  BoundingSphere(BoundingSphere),
}

impl BoundingBox {
  pub fn new() -> Self {
    Self { min: V3([f32::MAX, f32::MAX, f32::MAX]),
           max: V3([f32::MIN, f32::MIN, f32::MIN]) }
  }
  pub fn extend(&mut self, p: V3) {
    macro_rules! check {
      ($x:ident, $i:expr) => {
        if p.$x() < self.min.$x() {
          self.min.0[$i] = p.$x();
        }
        if p.$x() > self.max.$x() {
          self.max.0[$i] = p.$x();
        }
      };
    }

    check!(x, 0);
    check!(y, 1);
    check!(z, 2);
  }

  pub fn intersect(&self, ray: &Ray) -> bool {
    let mut tmin = (self.min.x() - ray.orig.x()) / ray.dir.x();
    let mut tmax = (self.max.x() - ray.orig.x()) / ray.dir.x();
    if tmin > tmax {
      mem::swap(&mut tmin, &mut tmax);
    }
    let mut tymin = (self.min.y() - ray.orig.y()) / ray.dir.y();
    let mut tymax = (self.max.y() - ray.orig.y()) / ray.dir.y();
    if tymin > tymax {
      mem::swap(&mut tymin, &mut tymax);
    }
    if tmin > tymax || tymin > tmax {
      return false;
    }
    if tymin > tmin {
      tmin = tymin;
    }
    if tymax < tmax {
      tmax = tymax;
    }
    let mut tzmin = (self.min.z() - ray.orig.z()) / ray.dir.z();
    let mut tzmax = (self.max.z() - ray.orig.z()) / ray.dir.z();
    if tzmin > tzmax {
      mem::swap(&mut tzmin, &mut tzmax);
    }

    if tmin > tzmax || tzmin > tmax {
      return false;
    }
    if tzmin > tmin {
      tmin = tzmin;
    }
    if tmin < 0.0 {
      return false;
    }
    true
  }
}

impl BoundingSphere {
  pub fn intersect(&self, ray: &Ray) -> bool {
    let l = self.c - ray.orig;
    let tc = l.dot(ray.dir);

    if tc < 0.0 {
      return false;
    }

    let d2 = l.dot(l) - tc * tc;
    let r2 = self.r * self.r;

    if d2 > r2 {
      return false;
    }

    let t1c = (r2 - d2).sqrt();
    let t1 = tc - t1c;
    let t2 = tc + t1c;

    if (t1 < 0.0) && (t2 > 0.0) || (t1 > 0.0 && t2 < 0.0) {
      true
    } else if t1 > 0.0 && t2 > 0.0 {
      true
    } else {
      false
    }
  }
}

impl Bound {
  pub fn intersect(&self, ray: &Ray) -> bool {
    use self::Bound::*;
    match self {
      BoundingBox(x) => x.intersect(ray),
      BoundingSphere(x) => x.intersect(ray),
    }
  }
}
