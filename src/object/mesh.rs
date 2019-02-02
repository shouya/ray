use super::*;
use obj_model::ObjModel;
use std::cell::Ref;
use std::cell::RefCell;

// index type, if more than 65535 points are needed, use u32 here
type I = u16;

pub struct TrigMesh {
  // vertices
  vs: Vec<V3>,
  // vertex normals
  vns: Option<Vec<V3>>,
  // triangles
  ts: Vec<([I; 3], [I; 3])>,
  // we cache the vertices of trigs and the bounding box of the whole object
  cache: RefCell<Option<TrigMeshCache>>,
}

pub struct TrigMeshCache {
  cluster: Cluster,
}

enum ClusterKind {
  // trig -> index in ts
  Trig(TrigGen),
  Cluster(Cluster),
}

struct Cluster {
  // further optimizate computation of intersection
  clusters: Vec<ClusterKind>,
  bbox: BoundingBox,
}

impl TrigMesh {
  pub fn new(vs: Vec<V3>, ts: Vec<([I; 3], [I; 3])>) -> Self {
    TrigMesh {
      vs,
      ts,
      vns: None,
      cache: RefCell::new(None),
    }
  }

  pub fn from_model(model: &ObjModel) -> Self {
    let mut mesh = TrigMesh::new(vec![], vec![]);
    let mut vns = Vec::new();

    for v in model.v.iter() {
      mesh.vs.push(*v);
    }
    for vn in model.vn.iter() {
      vns.push(*vn);
    }

    for f in model.f.iter() {
      let (tv, tvn): (Vec<_>, Vec<_>) = f.iter().cloned().unzip();
      if tv.len() != 3 {
        panic!("Model has non-trig faces. Please import as PolygonMesh.");
      }
      let tv = [tv[0] as I, tv[1] as I, tv[2] as I];
      let tvn = [tvn[0] as I, tvn[1] as I, tvn[2] as I];
      mesh.ts.push((tv, tvn));
    }

    if vns.len() > 0 {
      mesh.vns = Some(vns);
    }

    mesh
  }

  fn trigs<'a>(&'a self) -> impl Iterator<Item = TrigGen> + 'a {
    let trig_idxs = self.ts.iter();
    let vns = self.vns.as_ref();

    trig_idxs.map(move |([ai, bi, ci], [ani, bni, cni])| {
      let t = Trig(
        self.vs[*ai as usize],
        self.vs[*bi as usize],
        self.vs[*ci as usize],
      );
      let n = vns.map(|vns| Trig(vns[*ani as usize], vns[*bni as usize], vns[*cni as usize]));
      TrigGen::new(t, n)
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
}

impl Transform for TrigMesh {
  fn translate(mut self, d: V3) -> Self {
    self.vs.iter_mut().for_each(|v| *v = *v + d);
    self.clear_cache();
    self
  }
}

impl Cluster {
  const CLUSTER_LIMIT: usize = 4;

  pub fn from_trigs(ts: &[TrigGen]) -> Cluster {
    use std::iter::repeat;
    let trigs: Vec<Trig> = ts.iter().map(|x| *x.trig()).collect();
    let bbox = Self::bbox_for_trigs(&trigs);

    if ts.len() <= Self::CLUSTER_LIMIT {
      return Cluster {
        bbox,
        clusters: ClusterKind::from_trigs(ts),
      };
    }

    // cluster center
    let sum_fn = |a: V3, t: &Trig| a + t.center();
    let cc: V3 = trigs.iter().fold(V3::zero(), sum_fn) / (ts.len() as f32);
    let mut quadrants: Vec<Vec<TrigGen>> = repeat(Vec::new()).take(8).collect();

    // split trigs into 8 quadrant
    for t in ts.into_iter() {
      let tc = t.trig().center();
      let x_part = (tc.x() < cc.x()) as usize;
      let y_part = (tc.y() < cc.y()) as usize;
      let z_part = (tc.z() < cc.z()) as usize;
      let idx = z_part * 1 + y_part * 2 + x_part * 4;
      quadrants[idx].push(t.clone());
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
          let trig = t.trig();
          if let Some(pos) = trig.intersect(ray) {
            let norm = t.trig_n().map(|n| n.norm_at(pos)).unwrap();
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
  pub fn from_trigs(ts: &[TrigGen]) -> Vec<ClusterKind> {
    ts.iter().map(|t| ClusterKind::Trig(t.clone())).collect()
  }
}
