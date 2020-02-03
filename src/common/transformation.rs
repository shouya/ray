use super::*;
use std::ops::Mul;

// Transformation matrix (excluding translation)
#[derive(Debug, Clone)]
pub struct M33(pub V3, pub V3, pub V3);

// Transformation matrix
#[derive(Clone, Copy)]
pub struct M4(pub [[f32; 4]; 4]);

// transformation matrix with inverse cache
#[derive(Clone, Copy, Debug)]
pub struct TransMat {
  pub o2w: M4,
  pub w2o: M4,
}

impl M4 {
  pub fn new_id() -> M4 {
    M4([[1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0]])
  }

  pub fn new_translation(v: V3) -> M4 {
    M4([[1.0, 0.0, 0.0, v.x()],
        [0.0, 1.0, 0.0, v.y()],
        [0.0, 0.0, 1.0, v.z()],
        [0.0, 0.0, 0.0, 1.0]])
  }

  pub fn new_rotation_x(t: f32) -> M4 {
    M4([[1.0, 0.0, 0.0, 0.0],
        [0.0, t.cos(), -t.sin(), 0.0],
        [0.0, t.sin(), t.cos(), 0.0],
        [0.0, 0.0, 0.0, 1.0]])
  }

  pub fn new_rotation_y(t: f32) -> M4 {
    M4([[t.cos(), 0.0, t.sin(), 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [-t.sin(), 0.0, t.cos(), 0.0],
        [0.0, 0.0, 0.0, 1.0]])
  }

  pub fn new_rotation_z(t: f32) -> M4 {
    M4([[t.cos(), -t.sin(), 0.0, 0.0],
        [t.sin(), t.cos(), 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0]])
  }

  pub fn new_rotation(r: V3) -> M4 {
    Self::new_rotation_x(r.x())
    * Self::new_rotation_y(r.y())
    * Self::new_rotation_z(r.z())
  }

  pub fn new_scaling(s: V3) -> M4 {
    M4([[s.x(), 0.0, 0.0, 0.0],
        [0.0, s.y(), 0.0, 0.0],
        [0.0, 0.0, s.z(), 0.0],
        [0.0, 0.0, 0.0, 1.0]])
  }

  // shearing implementation is skipped

  // panic if not invertible, be careful
  pub fn inv(self) -> M4 {
    // copied from https://stackoverflow.com/a/7596981
    let mut res = [[0.0; 4]; 4];
    let i = self.0;

    let s0 = i[0][0] * i[1][1] - i[1][0] * i[0][1];
    let s1 = i[0][0] * i[1][2] - i[1][0] * i[0][2];
    let s2 = i[0][0] * i[1][3] - i[1][0] * i[0][3];
    let s3 = i[0][1] * i[1][2] - i[1][1] * i[0][2];
    let s4 = i[0][1] * i[1][3] - i[1][1] * i[0][3];
    let s5 = i[0][2] * i[1][3] - i[1][2] * i[0][3];

    let c5 = i[2][2] * i[3][3] - i[3][2] * i[2][3];
    let c4 = i[2][1] * i[3][3] - i[3][1] * i[2][3];
    let c3 = i[2][1] * i[3][2] - i[3][1] * i[2][2];
    let c2 = i[2][0] * i[3][3] - i[3][0] * i[2][3];
    let c1 = i[2][0] * i[3][2] - i[3][0] * i[2][2];
    let c0 = i[2][0] * i[3][1] - i[3][0] * i[2][1];

    // Should check for 0 determinant
    let det = s0 * c5 - s1 * c4 + s2 * c3 + s3 * c2 - s4 * c1 + s5 * c0;
    if f32_eq(det, 0.0) {
      panic!("inverting a non-invertible matrix")
    }
    let det = 1.0 / det;

    res[0][0] = (i[1][1] * c5 - i[1][2] * c4 + i[1][3] * c3) * det;
    res[0][1] = (-i[0][1] * c5 + i[0][2] * c4 - i[0][3] * c3) * det;
    res[0][2] = (i[3][1] * s5 - i[3][2] * s4 + i[3][3] * s3) * det;
    res[0][3] = (-i[2][1] * s5 + i[2][2] * s4 - i[2][3] * s3) * det;

    res[1][0] = (-i[1][0] * c5 + i[1][2] * c2 - i[1][3] * c1) * det;
    res[1][1] = (i[0][0] * c5 - i[0][2] * c2 + i[0][3] * c1) * det;
    res[1][2] = (-i[3][0] * s5 + i[3][2] * s2 - i[3][3] * s1) * det;
    res[1][3] = (i[2][0] * s5 - i[2][2] * s2 + i[2][3] * s1) * det;

    res[2][0] = (i[1][0] * c4 - i[1][1] * c2 + i[1][3] * c0) * det;
    res[2][1] = (-i[0][0] * c4 + i[0][1] * c2 - i[0][3] * c0) * det;
    res[2][2] = (i[3][0] * s4 - i[3][1] * s2 + i[3][3] * s0) * det;
    res[2][3] = (-i[2][0] * s4 + i[2][1] * s2 - i[2][3] * s0) * det;

    res[3][0] = (-i[1][0] * c3 + i[1][1] * c1 - i[1][2] * c0) * det;
    res[3][1] = (i[0][0] * c3 - i[0][1] * c1 + i[0][2] * c0) * det;
    res[3][2] = (-i[3][0] * s3 + i[3][1] * s1 - i[3][2] * s0) * det;
    res[3][3] = (i[2][0] * s3 - i[2][1] * s1 + i[2][2] * s0) * det;

    M4(res)
  }

  #[allow(unused)]
  pub fn adjoint(self) -> Self {
    // A^-1 = 1/det(A) adj(A)
    let mut res = [[0.0; 4]; 4];
    let i = self.0;

    let s0 = i[0][0] * i[1][1] - i[1][0] * i[0][1];
    let s1 = i[0][0] * i[1][2] - i[1][0] * i[0][2];
    let s2 = i[0][0] * i[1][3] - i[1][0] * i[0][3];
    let s3 = i[0][1] * i[1][2] - i[1][1] * i[0][2];
    let s4 = i[0][1] * i[1][3] - i[1][1] * i[0][3];
    let s5 = i[0][2] * i[1][3] - i[1][2] * i[0][3];

    let c5 = i[2][2] * i[3][3] - i[3][2] * i[2][3];
    let c4 = i[2][1] * i[3][3] - i[3][1] * i[2][3];
    let c3 = i[2][1] * i[3][2] - i[3][1] * i[2][2];
    let c2 = i[2][0] * i[3][3] - i[3][0] * i[2][3];
    let c1 = i[2][0] * i[3][2] - i[3][0] * i[2][2];
    let c0 = i[2][0] * i[3][1] - i[3][0] * i[2][1];

    res[0][0] = i[1][1] * c5 - i[1][2] * c4 + i[1][3] * c3;
    res[0][1] = -i[0][1] * c5 + i[0][2] * c4 - i[0][3] * c3;
    res[0][2] = i[3][1] * s5 - i[3][2] * s4 + i[3][3] * s3;
    res[0][3] = -i[2][1] * s5 + i[2][2] * s4 - i[2][3] * s3;

    res[1][0] = -i[1][0] * c5 + i[1][2] * c2 - i[1][3] * c1;
    res[1][1] = i[0][0] * c5 - i[0][2] * c2 + i[0][3] * c1;
    res[1][2] = -i[3][0] * s5 + i[3][2] * s2 - i[3][3] * s1;
    res[1][3] = i[2][0] * s5 - i[2][2] * s2 + i[2][3] * s1;

    res[2][0] = i[1][0] * c4 - i[1][1] * c2 + i[1][3] * c0;
    res[2][1] = -i[0][0] * c4 + i[0][1] * c2 - i[0][3] * c0;
    res[2][2] = i[3][0] * s4 - i[3][1] * s2 + i[3][3] * s0;
    res[2][3] = -i[2][0] * s4 + i[2][1] * s2 - i[2][3] * s0;

    res[3][0] = -i[1][0] * c3 + i[1][1] * c1 - i[1][2] * c0;
    res[3][1] = i[0][0] * c3 - i[0][1] * c1 + i[0][2] * c0;
    res[3][2] = -i[3][0] * s3 + i[3][1] * s1 - i[3][2] * s0;
    res[3][3] = i[2][0] * s3 - i[2][1] * s1 + i[2][2] * s0;

    M4(res)
  }

  pub fn transpose(self) -> Self {
    let mut m = self.0;
    for r in 0..4 {
      for c in (r + 1)..4 {
        let mcr = m[c][r];
        m[c][r] = m[r][c];
        m[r][c] = mcr;
      }
    }
    Self(m)
  }

  pub fn transform_ray(self, r: &Ray) -> Ray {
    let new_orig = self.transform_point(r.orig);
    let new_dir = self.transform_vector(r.dir).norm();
    Ray::new(new_orig, new_dir)
  }

  pub fn transform_hit(self, trans_norm: Self, h: &Hit) -> Hit {
    let new_pos = self.transform_point(h.pos);
    let new_norm = trans_norm.transform_vector(h.norm).norm();

    Hit { pos: new_pos,
          norm: new_norm,
          ..*h }
  }

  pub fn transform_v4(self, v4: [f32; 4]) -> [f32; 4] {
    let a = self.0;
    let b = v4;
    let mut r = [0.0; 4];

    for i in 0..4 {
      for k in 0..4 {
        r[i] += a[i][k] * b[k]
      }
    }

    r
  }

  pub fn transform_point(self, p: V3) -> V3 {
    let p = p.0;
    let r = self.transform_v4([p[0], p[1], p[2], 1.0]);
    V3([r[0], r[1], r[2]])
  }

  pub fn transform_vector(self, p: V3) -> V3 {
    let p = p.0;
    let r = self.transform_v4([p[0], p[1], p[2], 0.0]);
    V3([r[0], r[1], r[2]])
  }
}

impl std::fmt::Debug for M4 {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut fmt_row = |i, r: [f32; 4]| {
      writeln!(f,
               "[{:1}] {:<+.5} {:<+.5} {:+.5} {:+.5}",
               i, r[0], r[1], r[2], r[3])
    };
    let m = self.0;
    fmt_row(0, m[0])?;
    fmt_row(1, m[1])?;
    fmt_row(2, m[2])?;
    fmt_row(3, m[3])
  }
}

impl Mul<M4> for M4 {
  type Output = M4;
  fn mul(self, rhs: M4) -> Self::Output {
    let (a, b) = (self.0, rhs.0);
    let mut res = [[0.0; 4]; 4];

    for i in 0..4 {
      for j in 0..4 {
        for k in 0..4 {
          res[i][j] += a[i][k] * b[k][j];
        }
      }
    }

    M4(res)
  }
}

impl TransMat {
  pub fn new() -> Self {
    Self { o2w: M4::new_id(),
           w2o: M4::new_id() }
  }

  pub fn append(&mut self, m: M4) {
    self.o2w = m * self.o2w;
    self.w2o = self.w2o * m.inv();
  }

  #[allow(unused)]
  pub fn prepend(&mut self, m: M4) {
    self.o2w = self.o2w * m;
    self.w2o = m.inv() * self.w2o;
  }

  #[allow(unused)]
  pub fn mult_opt(&self, t: Option<TransMat>) -> TransMat {
    match t {
      None => *self,
      Some(t) => Self { o2w: self.o2w * t.o2w,
                        w2o: t.w2o * self.w2o },
    }
  }
}
