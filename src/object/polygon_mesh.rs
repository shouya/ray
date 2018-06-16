use super::*;
use common::*;

// index type, if more than 65535 points are needed, use u32 here
type I = u16;

pub struct PolygonMesh {
  v: Vec<V3>,
  // each face composes of a number of vertices
  f: Vec<Vec<I>>,
  material: Material,
}

impl PolygonMesh {
  pub fn trigs<'a>(&'a self) -> impl Iterator<Item = Trig> + 'a {
    let trig_idxs = self.f.iter().flat_map(Self::face_to_trigs);

    trig_idxs.map(move |(ai, bi, ci)| {
      Trig(
        self.v[ai as usize],
        self.v[bi as usize],
        self.v[ci as usize],
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
