use super::*;
use obj_model::ObjModel;
use std::cell::Ref;
use std::cell::RefCell;
use std::ops::Deref;

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
  material: Material,
  // we cache the vertices of trigs and the bounding box of the whole object
  cache: RefCell<Option<TrigMeshCache>>,
}

pub struct TrigMeshCache {
  cluster: Cluster,
}

enum ClusterKind {
  // trig -> index in ts
  Trig(Trig),
  Cluster(Cluster),
}

struct Cluster {
  // further optimizate computation of intersection
  clusters: Vec<ClusterKind>,
  bbox: BoundingBox,
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
      cache: RefCell::new(None),
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

  pub fn get_cache<'a>(&'a self) -> Ref<'a, TrigMeshCache> {
    if self.cache.borrow().is_some() {
      return Ref::map(self.cache.borrow(), |x| x.as_ref().unwrap());
    }

    let trigs = self.trigs().collect::<Vec<_>>();
    let cluster = Cluster::from_trigs(trigs.as_slice());

    *self.cache.borrow_mut() = Some(TrigMeshCache { cluster });

    Ref::map(self.cache.borrow(), |x| x.as_ref().unwrap())
  }
  pub fn clear_cache(&self) {
    *self.cache.borrow_mut() = None;
  }
}

impl Object for TrigMesh {
  fn intersect(&self, ray: &Ray) -> Option<Hit> {
    let cache = self.get_cache();
    cache.cluster.intersect(ray)
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

impl Cluster {
  const CLUSTER_LIMIT: usize = 5;

  pub fn from_trigs(ts: &[Trig]) -> Cluster {
    use std::iter::repeat;
    let bbox = Self::bbox_for_trigs(ts);

    if ts.len() <= Self::CLUSTER_LIMIT {
      return Cluster {
        bbox,
        clusters: ClusterKind::from_trigs(ts),
      };
    }

    // cluster center
    let sum_fn = |a: V3, t: &Trig| a + t.center();
    let cc: V3 = ts.iter().fold(V3::zero(), sum_fn) / (ts.len() as f32);
    let mut quadrants: Vec<Vec<Trig>> = repeat(Vec::new()).take(8).collect();

    // split trigs into 8 quadrant
    for t in ts.iter() {
      let tc = t.center();
      let x_part = (tc.x() < cc.x()) as usize;
      let y_part = (tc.y() < cc.y()) as usize;
      let z_part = (tc.z() < cc.z()) as usize;
      let idx = z_part * 1 + y_part * 2 + x_part * 4;
      quadrants[idx].push(*t);
    }

    let mut clusters = Vec::new();
    for p in quadrants.into_iter() {
      if p.len() == 0 {
        continue;
      }
      let part = Cluster::from_trigs(&p);
      clusters.push(ClusterKind::Cluster(part));
    }

    Cluster { bbox, clusters }
  }

  pub fn bbox_for_trigs(ts: &[Trig]) -> BoundingBox {
    let mut bbox = BoundingBox::new();
    for t in ts.iter() {
      bbox.extend(t.a());
      bbox.extend(t.b());
      bbox.extend(t.c());
    }
    bbox
  }

  fn intersect(&self, ray: &Ray) -> Option<Hit> {
    // Magic!
    if !self.bbox.intersect(ray) {
      return None;
    }

    let mut hits = Vec::new();

    for c in self.clusters.iter() {
      match c {
        ClusterKind::Trig(t) => {
          if let Some(pos) = t.intersect(ray) {
            let norm = t.n();
            hits.push(Hit {
              pos,
              norm,
              inside: ray.dir.dot(norm) < 0.0,
            });
          }
        }
        ClusterKind::Cluster(c) => {
          if let Some(hit) = c.intersect(ray) {
            hits.push(hit);
          }
        }
      }
    }

    hits.into_iter().min_by(|hit1, hit2| {
      use std::cmp::Ordering;
      dist2(hit1.pos, ray.orig)
        .partial_cmp(&dist2(hit2.pos, ray.orig))
        .unwrap_or(Ordering::Less)
    })
  }
}

impl ClusterKind {
  pub fn from_trigs(ts: &[Trig]) -> Vec<ClusterKind> {
    ts.iter().map(|t| ClusterKind::Trig(*t)).collect()
  }
}
