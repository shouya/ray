use super::*;

#[derive(Debug, Clone)]
pub struct Line(V3, V3);
#[derive(Debug, Clone, Copy)]
pub struct Plane(V3, V3);

impl Plane {
  pub fn r0(&self) -> V3 {
    self.0
  }
  pub fn n(&self) -> V3 {
    self.1
  }
  pub fn new(r0: V3, n: V3) -> Plane {
    Plane(r0, n.norm())
  }
  pub fn primary_axis(&self) -> V3 {
    let shift = V3([1.0, 0.0, 0.0]);
    let dist = shift.dot(self.n());
    (shift - self.n() * dist).norm()
  }
  pub fn secondary_axis(&self) -> V3 {
    self.primary_axis().cross(self.n())
  }

  pub fn intersect(&self, ray: &Ray) -> Option<V3> {
    let det = ray.dir.dot(self.n());
    if (det - 0.0).abs() < F32_EPSILON {
      // parallel to plane
      return None;
    }

    let d = (self.r0() - ray.orig).dot(self.n()) / det;
    if d <= 0.0 {
      // plane is behind
      return None;
    }
    let pos = ray.orig + ray.dir * d;

    Some(pos)
  }

  #[allow(unused)]
  pub fn from_points(ps: &[V3]) -> Option<Self> {
    if ps.len() < 3 {
      return None;
    }
    // test all trigs are coplane
    if !ps.windows(3)
          .map(|t| Trig(t[0], t[1], t[2]).n())
          .collect::<Vec<_>>()
          .windows(2)
          .map(|n| n[0] == n[1])
          .all(|t| t)
    {
      return None;
    }

    let n = Trig(ps[0], ps[1], ps[2]).n();
    let r0 = ps[0];

    Some(Plane(r0, n))
  }
}

impl Trig {
  pub fn a(&self) -> V3 {
    self.0
  }
  pub fn b(&self) -> V3 {
    self.1
  }
  pub fn c(&self) -> V3 {
    self.2
  }

  pub fn ab(&self) -> V3 {
    self.b() - self.a()
  }
  pub fn ac(&self) -> V3 {
    self.c() - self.a()
  }
  pub fn cb(&self) -> V3 {
    self.b() - self.c()
  }
  pub fn ca(&self) -> V3 {
    self.a() - self.c()
  }

  pub fn center(&self) -> V3 {
    (self.a() + self.b() + self.c()) / 3.0
  }

  // we use CCW system
  pub fn n(&self) -> V3 {
    self.ca().cross(self.cb()).norm()
  }

  pub fn to_plane(&self) -> Plane {
    Plane::new(self.a(), self.n())
  }

  // Möller–Trumbore intersection algorithm
  pub fn intersect(&self, ray: &Ray) -> Option<V3> {
    let e1 = self.ab();
    let e2 = self.ac();
    let h = ray.dir.cross(e2);
    let a = e1.dot(h);

    if f32_eq(a, 0.0) {
      return None;
    }

    let f = 1.0 / a;
    let s = ray.orig - self.a();
    let u = f * (s.dot(h));
    if u < 0.0 || u > 1.0 {
      return None;
    }
    let q = s.cross(e1);
    let v = f * (ray.dir.dot(q));
    if v < 0.0 || u + v > 1.0 {
      return None;
    }
    let t = f * e2.dot(q);
    if t <= 0.0 {
      // behind the ray
      return None;
    }
    Some(ray.orig + ray.dir * t)
  }

  #[allow(dead_code)]
  pub fn intersect_slow(&self, ray: &Ray) -> Option<V3> {
    self.to_plane().intersect(ray).filter(|p| self.contains(*p))
  }

  pub fn contains(&self, p: V3) -> bool {
    let test_edge = |v1: V3, v2: V3| {
      let c = (v2 - v1).cross(p - v1);
      self.n().dot(c) < 0.0
    };
    if test_edge(self.a(), self.b())
       || test_edge(self.b(), self.c())
       || test_edge(self.c(), self.a())
    {
      false
    } else {
      true
    }
  }

  #[allow(unused)]
  pub fn flip(&self) -> Trig {
    Trig(self.a(), self.c(), self.b())
  }

  pub fn area(&self) -> f32 {
    self.ab().cross(self.ac()).magn() / 2.0
  }

  pub fn at_uv(&self, uv: V2) -> V3 {
    self.a() * uv.w() + self.b() * uv.u() + self.c() * uv.v()
  }

  pub fn to_uv(&self, p: V3) -> V2 {
    let ap = p - self.a();
    let apc_area = ap.cross(self.ac()).magn() / 2.0;
    let u = apc_area / self.area();
    let apb_area = ap.cross(self.ab()).magn() / 2.0;
    let v = apb_area / self.area();

    V2([u, v])
  }
}
