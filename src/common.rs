use std::cmp::PartialEq;
use std::f32;
use std::mem;
use std::ops::{Add, Div, Mul, Neg, Sub};

pub const F32_EPSILON: f32 = 1e-10;

#[derive(Debug, Clone, Copy)]
pub struct V3(pub [f32; 3]);

// Represents barycentric coordinates or planar coordinates
#[derive(Debug, Clone, Copy)]
pub struct V2(pub [f32; 2]);

// Transformation matrix
#[derive(Clone, Copy)]
pub struct M4(pub [[f32; 4]; 4]);

// V3 with normal
#[derive(Debug, Clone, Copy)]
pub struct V3N {
    pub v: V3,
    pub n: V3,
}

#[derive(Debug, Clone, Copy)]
pub struct Color(pub [f32; 3]);

#[derive(Debug, Clone)]
pub struct Line(V3, V3);
#[derive(Debug, Clone, Copy)]
pub struct Plane(V3, V3);
#[derive(Debug, Clone, Copy)]
pub struct Trig(pub V3, pub V3, pub V3);

// trig with vertex normals
#[derive(Debug, Clone, Copy)]
pub struct TrigN {
    pub v: Trig,
    pub n: Trig,
}

// general trig
#[derive(Debug, Clone)]
pub enum TrigGen {
    TrigN(TrigN),
    Trig(Trig),
}

#[derive(Debug, Clone)]
pub struct M33(pub V3, pub V3, pub V3);

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Projection {
    Orthogonal,
    Perspective,
}

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub orig: V3,
    // direction, must be normalized
    pub dir: V3,
}

#[derive(Debug, Clone, Copy)]
pub struct Hit {
    pub pos: V3,
    pub norm: V3,
    pub inside: bool,
}

#[derive(Debug, Clone)]
pub struct PointLight {
    pub pos: V3,
    pub color: Color,
    pub brightness: f32, // 0: turned off
}

#[derive(Debug, Clone)]
pub struct BoundingBox {
    pub min: V3,
    pub max: V3,
}

#[derive(Debug, Clone)]
pub struct BoundingSphere {
    pub c: V3,
    pub r: f32,
}

#[allow(dead_code)]
pub enum Bound {
    BoundingBox(BoundingBox),
    BoundingSphere(BoundingSphere),
}

pub fn dist2(a: V3, b: V3) -> f32 {
    let d = b - a;
    d.dot(d)
}
#[allow(unused)]
pub fn dist(a: V3, b: V3) -> f32 {
    dist2(a, b).sqrt()
}

pub fn f32_eq(a: f32, b: f32) -> bool {
    (a - b).abs() < F32_EPSILON
}

#[allow(unused)]
pub fn f32_ge(a: f32, b: f32) -> bool {
    a > b || f32_eq(a, b)
}

pub fn randn_v3(mean: f32, std_dev: f32) -> V3 {
    V3([randn(mean, std_dev),
        randn(mean, std_dev),
        randn(mean, std_dev)])
}

pub fn randn(mean: f32, std_dev: f32) -> f32 {
    use rand::thread_rng;
    use rand_distr::{Distribution, Normal};

    let n = Normal::new(mean, std_dev).unwrap();
    n.sample(&mut thread_rng())
}

#[allow(unused)]
pub fn randn0() -> f32 {
    randn(0.0, 1.0)
}

impl V2 {
    pub fn u(&self) -> f32 {
        self.0[0]
    }
    pub fn v(&self) -> f32 {
        self.0[1]
    }
    pub fn w(&self) -> f32 {
        1.0 - self.u() - self.v()
    }
}

// ray uses right hand side coordinate system:
// positive directions:
//  X - right, Y - up, Z - out of screen
// CCW rotations are positive
impl V3 {
    pub fn x(&self) -> f32 {
        self.0[0]
    }
    pub fn y(&self) -> f32 {
        self.0[1]
    }
    pub fn z(&self) -> f32 {
        self.0[2]
    }

    pub fn dot(self, rhs: Self) -> f32 {
        self.x() * rhs.x() + self.y() * rhs.y() + self.z() * rhs.z()
    }
    pub fn magn(self) -> f32 {
        self.dot(self).sqrt()
    }
    pub fn norm(self) -> Self {
        if self.magn() == 0.0 {
            V3::zero()
        } else {
            self / self.magn()
        }
    }
    pub fn cross(self, rhs: V3) -> V3 {
        let (u1, u2, u3) = (self.x(), self.y(), self.z());
        let (v1, v2, v3) = (rhs.x(), rhs.y(), rhs.z());
        V3([u2 * v3 - u3 * v2, u3 * v1 - u1 * v3, u1 * v2 - u2 * v1])
    }
    pub fn zero() -> V3 {
        V3([0.0, 0.0, 0.0])
    }

    #[allow(unused)]
    pub fn is_zero(&self) -> bool {
        f32_eq(self.0[0], 0.0)
        && f32_eq(self.0[1], 0.0)
        && f32_eq(self.0[2], 0.0)
    }
}

impl Sub<f32> for V3 {
    type Output = V3;
    fn sub(self, rhs: f32) -> V3 {
        V3([self.x() - rhs, self.y() - rhs, self.z() - rhs])
    }
}
impl Add<f32> for V3 {
    type Output = V3;
    fn add(self, rhs: f32) -> V3 {
        V3([self.x() + rhs, self.y() + rhs, self.z() + rhs])
    }
}
impl Neg for V3 {
    type Output = V3;
    fn neg(self) -> V3 {
        V3([-self.x(), -self.y(), -self.z()])
    }
}
impl Mul<f32> for V3 {
    type Output = V3;
    fn mul(self, rhs: f32) -> V3 {
        V3([self.x() * rhs, self.y() * rhs, self.z() * rhs])
    }
}
impl Div<f32> for V3 {
    type Output = V3;
    fn div(self, rhs: f32) -> V3 {
        V3([self.x() / rhs, self.y() / rhs, self.z() / rhs])
    }
}

impl Sub<V3> for V3 {
    type Output = V3;

    fn sub(self, rhs: V3) -> V3 {
        V3([self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z()])
    }
}

impl Add<V3> for V3 {
    type Output = V3;

    fn add(self, rhs: V3) -> V3 {
        V3([self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z()])
    }
}

impl Default for V3 {
    fn default() -> Self {
        V3([0., 0., 0.])
    }
}

impl PartialEq for V3 {
    fn eq(&self, other: &V3) -> bool {
        f32_eq(self.x(), other.x())
        && f32_eq(self.y(), other.y())
        && f32_eq(self.z(), other.z())
    }
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

// transformation matrix with inverse cache
#[derive(Clone, Copy, Debug)]
pub struct TransMat {
    pub o2w: M4,
    pub w2o: M4,
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

impl Ray {
    pub fn new(orig: V3, dir: V3) -> Self {
        Self { orig,
               dir: dir.norm() }
    }

    pub fn reflect(&self, hit: &Hit) -> Ray {
        let proj_n_d = hit.norm * self.dir.dot(hit.norm);
        Self { orig: hit.pos,
               dir: self.dir - proj_n_d * 2.0 }
        // Ray::new(hit.pos, self.dir - proj_n_d * 2.0)
    }

    pub fn refract(&self, hit: &Hit, ior: f32) -> Ray {
        let i = self.dir;
        let mut n = hit.norm;
        let mut cosi = n.dot(i).max(-1.0).min(1.0);
        let (mut etai, mut etat) = (1.0, ior);
        if cosi < 0.0 {
            cosi = -cosi;
        } else {
            mem::swap(&mut etai, &mut etat);
            n = -n;
        }

        let eta = etai / etat;
        let k = 1.0 - eta * eta * (1.0 - cosi * cosi);
        let dir = if k < 0.0 {
            // full internal refl
            V3([1.0, 0.0, 0.0])
        } else {
            i * eta + n * (eta * cosi - k.sqrt())
        };

        Ray::new(hit.pos, dir)
    }

    #[allow(unused)]
    pub fn drift(&self, std_dev: f32) -> Ray {
        if std_dev == 0.0 {
            return self.clone();
        }

        let dx = randn(0.0, std_dev);
        let dy = randn(0.0, std_dev);
        let dz = randn(0.0, std_dev);
        Ray::new(self.orig, self.dir + V3([dx, dy, dz]))
    }

    // amount = 0: no drift
    // amount = 1: scattered everywhere
    #[allow(unused)]
    pub fn drift_array(&self,
                       amount: f32,
                       count: usize,
                       bias: f32)
                       -> Vec<Ray> {
        let mut v = Vec::new();
        if amount == 0.0 {
            return vec![self.biased(bias)];
        }

        for _ in 1..count {
            v.push(self.drift(amount * f32::consts::PI).biased(bias))
        }

        v
    }

    pub fn biased(self, amount: f32) -> Ray {
        self + self.dir * amount
    }
}

impl Add<V3> for Ray {
    type Output = Ray;
    fn add(self, rhs: V3) -> Ray {
        Ray { orig: self.orig + rhs,
              dir: self.dir }
    }
}

impl Neg for Ray {
    type Output = Ray;
    fn neg(self) -> Ray {
        Ray { orig: self.orig,
              dir: -self.dir }
    }
}

use self::random_color::RandomColor;
use random_color;

#[allow(non_upper_case_globals)]
impl Color {
    #[allow(dead_code)]
    pub const Red: Color = Color([1.0, 0.0, 0.0]);
    #[allow(dead_code)]
    pub const Green: Color = Color([0.0, 1.0, 0.0]);
    #[allow(dead_code)]
    pub const Blue: Color = Color([0.0, 0.0, 1.0]);
    #[allow(dead_code)]
    pub const White: Color = Color([1.0, 1.0, 1.0]);
    #[allow(dead_code)]
    pub const Black: Color = Color([0.0, 0.0, 0.0]);
    pub const Zero: Color = Self::Black;
    pub const One: Color = Self::White;

    pub fn from_intensity(i: f32) -> Color {
        Color([i, i, i])
    }

    pub fn r(&self) -> f32 {
        self.0[0]
    }
    pub fn g(&self) -> f32 {
        self.0[1]
    }
    pub fn b(&self) -> f32 {
        self.0[2]
    }

    // ratio: 1: all self, 0: all rhs
    pub fn blend(&self, rhs: Color, ratio: f32) -> Color {
        let (t0, t1) = (ratio, 1.0 - ratio);
        self.mix_with(rhs, |l, r| l * t0 + r * t1)
    }

    #[allow(unused)]
    pub fn blend_all(colors: &[Color]) -> Color {
        if colors.is_empty() {
            return Color([0.0, 0.0, 0.0]);
        }

        let n = colors.len() as f32;
        let r: f32 = colors.iter().map(|x| x.r()).sum();
        let g: f32 = colors.iter().map(|x| x.g()).sum();
        let b: f32 = colors.iter().map(|x| x.b()).sum();
        Color([r / n, g / n, b / n])
    }

    pub fn channel_blend(&self, rhs: Color, frac: Color) -> Color {
        let c = frac;
        Color([self.r() * c.r() + rhs.r() * (1.0 - c.r()),
               self.g() * c.g() + rhs.g() * (1.0 - c.g()),
               self.b() * c.b() + rhs.b() * (1.0 - c.b())])
    }

    pub fn mix_with<F>(&self, rhs: Color, f: F) -> Color
        where F: Fn(f32, f32) -> f32
    {
        Color([f(self.r(), rhs.r()),
               f(self.g(), rhs.g()),
               f(self.b(), rhs.b())])
    }

    pub fn clamp(&self, min: f32, max: f32) -> Color {
        Color([self.r().max(min).min(max),
               self.g().max(min).min(max),
               self.b().max(min).min(max)])
    }
    pub fn regularize(&self) -> Color {
        self.clamp(0.0, 1.0)
    }

    #[allow(unused)]
    pub fn mult(self, brightness: Color) -> Color {
        self.mix_with(brightness, |a, b| a * b)
    }

    pub fn from_rgb(hex: [u32; 3]) -> Self {
        let [r, g, b] = hex;
        Self([r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0])
    }

    pub fn random() -> Self {
        Color::from_rgb(RandomColor::new().to_rgb_array())
    }

    pub fn average(colors: &[Color]) -> Color {
        if colors.is_empty() {
            return Color::Black;
        }

        let mut res = Color::Black;
        colors.iter().for_each(|c| res = res + *c);

        res * (1.0 / (colors.len() as f32))
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Color) -> bool {
        f32_eq(self.r(), other.r())
        && f32_eq(self.g(), other.g())
        && f32_eq(self.b(), other.b())
    }
}

impl Into<[u8; 3]> for Color {
    fn into(self) -> [u8; 3] {
        let normalize = |k: f32| {
            if k < 0.0 {
                0
            } else if k > 1.0 {
                1
            } else {
                (k * 255.0) as u8
            }
        };
        [normalize(self.r()),
         normalize(self.g()),
         normalize(self.b())]
    }
}

impl Add<Color> for Color {
    type Output = Color;
    fn add(self, rhs: Self) -> Color {
        Color([self.r() + rhs.r(), self.g() + rhs.g(), self.b() + rhs.b()])
    }
}

impl Sub<Color> for Color {
    type Output = Color;
    fn sub(self, rhs: Self) -> Color {
        Color([self.r() - rhs.r(), self.g() - rhs.g(), self.b() - rhs.b()])
    }
}

impl Add<f32> for Color {
    type Output = Color;
    fn add(self, rhs: f32) -> Color {
        Color([self.r() + rhs, self.g() + rhs, self.b() + rhs])
    }
}

impl Mul<Color> for Color {
    type Output = Color;
    fn mul(self, rhs: Color) -> Color {
        Color([self.r() * rhs.r(), self.g() * rhs.g(), self.b() * rhs.b()])
    }
}

impl Mul<f32> for Color {
    type Output = Color;
    fn mul(self, rhs: f32) -> Color {
        Color([self.r() * rhs, self.g() * rhs, self.b() * rhs])
    }
}

impl Hit {
    pub fn biased(self, amount: f32) -> Hit {
        self + self.norm * amount
    }
}

impl Add<V3> for Hit {
    type Output = Hit;

    fn add(self, rhs: V3) -> Hit {
        Hit { pos: self.pos + rhs,
              ..self }
    }
}

impl BoundingBox {
    pub fn new() -> Self {
        Self { min: V3([f32::MAX, f32::MAX, f32::MAX]),
               max: V3([f32::MIN, f32::MIN, f32::MIN]) }
    }
    pub fn extend(&mut self, p: V3) {
        macro_rules! check {
            ($x:ident, $i:expr) => {
                if p.$x() < self.min.$x() {
                    self.min.0[$i] = p.$x();
                }
                if p.$x() > self.max.$x() {
                    self.max.0[$i] = p.$x();
                }
            };
        }

        check!(x, 0);
        check!(y, 1);
        check!(z, 2);
    }

    pub fn intersect(&self, ray: &Ray) -> bool {
        let mut tmin = (self.min.x() - ray.orig.x()) / ray.dir.x();
        let mut tmax = (self.max.x() - ray.orig.x()) / ray.dir.x();
        if tmin > tmax {
            mem::swap(&mut tmin, &mut tmax);
        }
        let mut tymin = (self.min.y() - ray.orig.y()) / ray.dir.y();
        let mut tymax = (self.max.y() - ray.orig.y()) / ray.dir.y();
        if tymin > tymax {
            mem::swap(&mut tymin, &mut tymax);
        }
        if tmin > tymax || tymin > tmax {
            return false;
        }
        if tymin > tmin {
            tmin = tymin;
        }
        if tymax < tmax {
            tmax = tymax;
        }
        let mut tzmin = (self.min.z() - ray.orig.z()) / ray.dir.z();
        let mut tzmax = (self.max.z() - ray.orig.z()) / ray.dir.z();
        if tzmin > tzmax {
            mem::swap(&mut tzmin, &mut tzmax);
        }

        if tmin > tzmax || tzmin > tmax {
            return false;
        }
        if tzmin > tmin {
            tmin = tzmin;
        }
        if tmin < 0.0 {
            return false;
        }
        true
    }
}

impl BoundingSphere {
    pub fn intersect(&self, ray: &Ray) -> bool {
        let l = self.c - ray.orig;
        let tc = l.dot(ray.dir);

        if tc < 0.0 {
            return false;
        }

        let d2 = l.dot(l) - tc * tc;
        let r2 = self.r * self.r;

        if d2 > r2 {
            return false;
        }

        let t1c = (r2 - d2).sqrt();
        let t1 = tc - t1c;
        let t2 = tc + t1c;

        if (t1 < 0.0) && (t2 > 0.0) || (t1 > 0.0 && t2 < 0.0) {
            true
        } else if t1 > 0.0 && t2 > 0.0 {
            true
        } else {
            false
        }
    }
}

impl Bound {
    pub fn intersect(&self, ray: &Ray) -> bool {
        use self::Bound::*;
        match self {
            BoundingBox(x) => x.intersect(ray),
            BoundingSphere(x) => x.intersect(ray),
        }
    }
}

impl TrigN {
    pub fn norm_at(&self, p: V3) -> V3 {
        let uv = self.v.to_uv(p);
        self.n.at_uv(uv)
    }
}

impl TrigGen {
    pub fn new(v: Trig, n: Option<Trig>) -> TrigGen {
        if let Some(n) = n {
            TrigGen::TrigN(TrigN { v, n })
        } else {
            TrigGen::Trig(v)
        }
    }

    pub fn trig(&self) -> &Trig {
        match self {
            TrigGen::Trig(t) => t,
            TrigGen::TrigN(TrigN { v, n: _ }) => v,
        }
    }

    pub fn trig_n(&self) -> Option<&TrigN> {
        match self {
            TrigGen::Trig(_) => None,
            TrigGen::TrigN(x) => Some(x),
        }
    }
}
