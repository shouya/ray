use super::*;
use common::*;

// index type, if more than 65535 points are needed, use u32 here
type I = u16;

pub struct PolygonMesh {
  vs: Vec<V3>,
  // each face composes of a number of vertices
  fs: Vec<Vec<I>>,
  material: Material,
}

impl PolygonMesh {
  pub fn new(vs: Vec<V3>, fs: Vec<Vec<I>>, m: Material) -> PolygonMesh {
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

  pub fn trigs<'a>(&'a self) -> impl Iterator<Item = Trig> + 'a {
    let trig_idxs = self.fs.iter().flat_map(Self::face_to_trigs);

    trig_idxs.map(move |(ai, bi, ci)| {
      Trig(
        self.vs[ai as usize],
        self.vs[bi as usize],
        self.vs[ci as usize],
      )
    })
  }

  pub fn face_to_trigs<'a>(polygon: &'a Vec<I>) -> impl Iterator<Item = (I, I, I)> + 'a {
    let (pivot, rest) = polygon
      .split_first()
      .expect("polygon must contain >=3 vertices");

    rest.windows(2).map(move |u| (*pivot, u[0], u[1]))
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
