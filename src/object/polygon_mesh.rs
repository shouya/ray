use super::*;
use obj_model::ObjModel;
use std::cell::RefCell;

// index type, if more than 65535 points are needed, use u32 here
type I = u16;

pub struct PolygonMesh {
  vs: Vec<V3>,
  // each face composes of a number of vertices
  fs: Vec<Vec<I>>,
  material: Material,
}

pub struct TrigMesh {
  vs: Vec<V3>,
  ts: Vec<[I; 3]>,
  ts_cache: RefCell<Vec<Trig>>,
  material: Material,
}

impl PolygonMesh {
  pub fn new(vs: Vec<V3>, fs: Vec<Vec<I>>, m: Material) -> Self {
    for f in fs.iter() {
      assert!(f.len() >= 3);

      let ps = f.iter().map(|i| vs[*i as usize]).collect::<Vec<_>>();
      assert!(Plane::from_points(ps.as_slice()).is_some());
    }

    PolygonMesh {
      vs,
      fs,
      material: m,
    }
  }

  pub fn from_model(model: &ObjModel, m: Material) -> Self {
    let mut mesh = PolygonMesh::new(vec![], vec![], m);

    for v in model.v.iter() {
      mesh.vs.push(*v);
    }
    for f in model.f.iter() {
      mesh.fs.push(f.iter().map(|v| *v as I).collect());
    }

    mesh
  }

  pub fn trigs<'a>(&'a self) -> impl Iterator<Item = Trig> + 'a {
    let trig_idxs = self.fs.iter().flat_map(Self::face_to_trigs);

    trig_idxs.map(move |[ai, bi, ci]| {
      Trig(
        self.vs[ai as usize],
        self.vs[bi as usize],
        self.vs[ci as usize],
      )
    })
  }

  pub fn face_to_trigs<'a>(polygon: &'a Vec<I>) -> impl Iterator<Item = [I; 3]> + 'a {
    let (pivot, rest) = polygon
      .split_first()
      .expect("polygon must contain >=3 vertices");

    rest.windows(2).map(move |u| [*pivot, u[0], u[1]])
  }
}

impl Object for PolygonMesh {
  fn intersect(&self, ray: &Ray) -> Option<Hit> {
    for trig in self.trigs() {
      if let Some(pos) = trig.intersect(ray) {
        let norm = trig.n();
        return Some(Hit {
          pos,
          norm,
          inside: ray.dir.dot(norm) > 0.0,
        });
      }
    }

    None
  }

  fn material(&self, _pos: V3) -> Cow<Material> {
    Cow::Borrowed(&self.material)
  }
}

impl TrigMesh {
  pub fn new(vs: Vec<V3>, ts: Vec<[I; 3]>, m: Material) -> Self {
    TrigMesh {
      vs,
      ts,
      ts_cache: RefCell::new(Vec::new()),
      material: m,
    }
  }

  pub fn from_model(model: &ObjModel, m: Material) -> Self {
    let mut mesh = TrigMesh::new(vec![], vec![], m);

    for v in model.v.iter() {
      mesh.vs.push(*v);
    }
    for f in model.f.iter() {
      if f.len() != 3 {
        panic!("Model has non-trig faces. Please import as PolygonMesh.");
      }
      let f: Vec<_> = f.iter().map(|v| *v as I).collect();
      mesh.ts.push([f[0], f[1], f[2]]);
    }

    mesh
  }

  pub fn trigs<'a>(&'a self) -> impl Iterator<Item = Trig> + 'a {
    let trig_idxs = self.ts.iter();

    trig_idxs.map(move |[ai, bi, ci]| {
      Trig(
        self.vs[*ai as usize],
        self.vs[*bi as usize],
        self.vs[*ci as usize],
      )
    })
  }

  pub fn save_cache(&self) {
    if self.ts_cache.borrow().len() > 0 {
      return;
    }
    *self.ts_cache.borrow_mut() = self.trigs().collect();
  }
  pub fn clear_cache(&self) {
    *self.ts_cache.borrow_mut() = Vec::new();
  }
}

impl Object for TrigMesh {
  fn intersect(&self, ray: &Ray) -> Option<Hit> {
    self.save_cache();
    for t in self.ts_cache.borrow().iter() {
      if let Some(pos) = t.intersect(ray) {
        let norm = t.n();
        return Some(Hit {
          pos,
          norm,
          inside: ray.dir.dot(norm) > 0.0,
        });
      }
    }

    None
  }

  fn material(&self, _pos: V3) -> Cow<Material> {
    Cow::Borrowed(&self.material)
  }
}

impl Transform for TrigMesh {
  fn translate(mut self, d: V3) -> Self {
    self.vs.iter_mut().for_each(|v| *v = *v + d);
    self.clear_cache();
    self
  }
}

