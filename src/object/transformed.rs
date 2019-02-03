use common::*;
use object::Object;
use scene::Scene;
use shader::Incidence;

pub struct Transformed {
  obj: Box<dyn Object>,
  pub o2w: M4,
  pub w2o: M4,
  pub adj_t: M4,
}

impl Transformed {
  pub fn new(obj: impl Object + 'static) -> Self {
    Transformed {
      obj: Box::new(obj),
      o2w: M4::new_id(),
      w2o: M4::new_id(),
      adj_t: M4::new_id(),
    }
  }

  pub fn rotated(self, r: V3) -> Self {
    let o2w = M4::new_rotation(r) * self.o2w;
    Self { o2w, ..self }.fill_cache()
  }

  pub fn translated(self, v: V3) -> Self {
    let o2w = M4::new_translation(v) * self.o2w;
    Self { o2w, ..self }.fill_cache()
  }

  pub fn scaled(self, s: V3) -> Self {
    let o2w = M4::new_scaling(s) * self.o2w;
    Self { o2w, ..self }.fill_cache()
  }

  fn fill_cache(self) -> Self {
    let w2o = self.o2w.inv();
    let adj_t = self.o2w.transpose();
    Self { w2o, adj_t, ..self }
  }
}

impl Object for Transformed {
  fn intersect(&self, ray: &Ray) -> Option<Hit> {
    // ray: world to object
    let ray = self.w2o.transform_ray(ray);
    let hit = self.obj.intersect(&ray);

    // hit: object to world
    hit.map(|h| self.o2w.transform_hit(self.o2w.transpose(), &h))
  }

  fn render(&self, s: &Scene, i: &Incidence) -> Option<Color> {
    // let ray = &self.o2w.transform_ray(i.ray);
    // let hit = &self.w2o.transform_hit(self.o2w.transpose(), i.hit);
    // let mat = Some((self.o2w, self.w2o));
    // let mat = None;
    let i = Incidence {
      // ray,
      // hit,
      // mat,
      ..*i
    };
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
