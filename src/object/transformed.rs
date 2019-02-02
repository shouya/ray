use common::*;
use object::Object;
use scene::Scene;
use shader::{Incidence, ShaderType};

pub struct Translated {
  pub object: Box<dyn Object>,
  pub offset: V3,
}

pub struct Scaled {
  pub object: Box<dyn Object>,
  pub factor: V3,
}

pub struct Rotated {
  pub object: Box<dyn Object>,
  // in radian, 3.14 rad = 180 deg
  pub angle: V3,
}

impl Object for Translated {
  fn intersect(&self, ray: &Ray) -> Option<Hit> {
    let ray = *ray + (-self.offset);
    self.object.intersect(&ray)
  }
  fn render(&self, s: &Scene, i: &Incidence) -> Option<Color> {
    let ray = *i.ray + self.offset;
    let hit = *i.hit + self.offset;
    let i = Incidence {
      ray: &ray,
      hit: &hit,
      ..*i
    };
    self.object.render(s, &i)
  }
}

/*
impl Object for Scaled {
  fn intersect(&self, ray: &Ray) -> Option<Hit> {
    // let ray = Ray {  };
    self.object.intersect(&ray)
  }
  fn render(&self, s: &Scene, i: &Incidence) -> Option<Color> {
    let ray = *i.ray + self.offset;
    let hit = *i.hit + self.offset;
    let i = Incidence {
      ray: &ray,
      hit: &hit,
      ..*i
    };
    self.object.render(s, &i)
  }
}
*/
