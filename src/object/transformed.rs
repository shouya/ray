use common::*;
use object::Object;
use scene::Scene;
use shader::Incidence;

pub struct Transformed {
  obj: Box<dyn Object>,
  pub mat: M4,
  pub inv_mat: M4,
}

impl Transformed {
  pub fn new(obj: impl Object + 'static) -> Self {
    Transformed {
      obj: Box::new(obj),
      mat: M4::new_id(),
      inv_mat: M4::new_id(),
    }
  }

  pub fn rotated(self, r: V3) -> Self {
    let mat = M4::new_rotation(r) * self.mat;
    let inv_mat = mat.inv();
    Self {
      mat,
      inv_mat,
      ..self
    }
  }

  pub fn translated(self, v: V3) -> Self {
    let mat = M4::new_translation(v) * self.mat;
    let inv_mat = mat.inv();
    Self {
      mat,
      inv_mat,
      ..self
    }
  }

  pub fn scaled(self, s: V3) -> Self {
    let mat = M4::new_scaling(s) * self.mat;
    let inv_mat = mat.inv();
    Self {
      mat,
      inv_mat,
      ..self
    }
  }

  pub fn transform_ray(&self, r: &Ray, inv: bool) -> Ray {
    let mat = if inv { self.inv_mat } else { self.mat };
    let new_orig = mat * r.orig;
    Ray {
      orig: new_orig,
      dir: mat * (r.orig + r.dir) - new_orig,
      ..*r
    }
  }

  pub fn transform_hit(&self, h: &Hit, inv: bool) -> Hit {
    let mat = if inv { self.inv_mat } else { self.mat };
    let new_pos = mat * h.pos;

    Hit {
      pos: new_pos,
      norm: mat * (h.pos + h.norm) - new_pos,
      ..*h
    }
  }
}

impl Object for Transformed {
  fn intersect(&self, ray: &Ray) -> Option<Hit> {
    // world to object
    //dbg!(ray);
    let ray = self.transform_ray(ray, true);
    //dbg!(ray);
    self.obj.intersect(&ray)
  }

  fn render(&self, s: &Scene, i: &Incidence) -> Option<Color> {
    let ray = &self.transform_ray(i.ray, false);
    let hit = &self.transform_hit(i.hit, false);
    let i = Incidence { ray, hit, ..*i };
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
