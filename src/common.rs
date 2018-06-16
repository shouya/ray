use std::cmp::PartialEq;
use std::ops::{Add, Div, Mul, Neg, Sub};

pub const F32_EPSILON: f32 = 1e-10;

#[derive(Debug, Clone, Copy)]
pub struct V3(pub [f32; 3]);
#[derive(Debug, Clone, Copy)]
pub struct Color(pub [f32; 3]);

#[derive(Debug, Clone)]
pub struct Line(V3, V3);
#[derive(Debug, Clone, Copy)]
pub struct Plane(V3, V3);
#[derive(Debug, Clone, Copy)]
pub struct Trig(pub V3, pub V3, pub V3);

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
    pub brightness: f32, // 0.0 turned off
}

pub fn dist2(a: V3, b: V3) -> f32 {
    let d = b - a;
    d.dot(d)
}
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

impl V3 {
    #[inline]
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
    pub fn is_zero(&self) -> bool {
        self.x() == 0.0 && self.y() == 0.0 && self.z() == 0.0
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

impl PartialEq for V3 {
    fn eq(&self, other: &V3) -> bool {
        f32_eq(self.x(), other.x()) && f32_eq(self.y(), other.y()) && f32_eq(self.z(), other.z())
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

    pub fn from_points(ps: &[V3]) -> Option<Self> {
        if ps.len() < 3 {
            return None;
        }
        // test all trigs are coplane
        if !ps
            .windows(3)
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

    pub fn cb(&self) -> V3 {
        self.b() - self.c()
    }
    pub fn ca(&self) -> V3 {
        self.a() - self.c()
    }

    // we use CCW system
    pub fn n(&self) -> V3 {
        self.ca().cross(self.cb()).norm()
    }

    pub fn to_plane(&self) -> Plane {
        Plane::new(self.a(), self.n())
    }

    pub fn intersect(&self, ray: &Ray) -> Option<V3> {
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

    pub fn flip(&self) -> Trig {
        Trig(self.a(), self.c(), self.b())
    }
}

impl Ray {
    pub fn new(orig: V3, dir: V3) -> Self {
        Self {
            orig,
            dir: dir.norm(),
        }
    }

    pub fn reflect(&self, hit: &Hit) -> Ray {
        let proj_n_d = hit.norm * self.dir.dot(hit.norm);
        Ray::new(hit.pos, self.dir - proj_n_d * 2.0)
    }

    pub fn refract(&self, hit: &Hit, ior: f32) -> Ray {
        use std::mem;

        let mut cosi = hit.norm.dot(self.dir);
        let (mut etai, mut etat) = (1.0, ior);
        let mut n = hit.norm;
        if cosi < 0.0 {
            cosi = -cosi;
        } else {
            mem::swap(&mut etai, &mut etat);
            n = -n;
        }

        let eta = etai / etat;
        let sint2 = eta * eta * (1.0 - cosi * cosi);
        let trans = 1.0 - sint2;
        let dir = if trans < 0.0 {
            // full internal refl
            V3::zero()
        } else {
            self.dir * eta + n * (eta * cosi - trans.sqrt())
        };
        Ray::new(hit.pos, dir)
    }

    pub fn drift(&self, std_dev: f32) -> Ray {
        use rand::distributions::{Distribution, StandardNormal};
        use rand::thread_rng;

        if std_dev == 0.0 {
            return self.clone();
        }

        let n = StandardNormal;
        let dx = n.sample(&mut thread_rng()) as f32 * std_dev;
        let dy = n.sample(&mut thread_rng()) as f32 * std_dev;
        let dz = n.sample(&mut thread_rng()) as f32 * std_dev;
        Ray::new(self.orig, self.dir + V3([dx, dy, dz]))
    }

    pub fn biased(self, amount: f32) -> Ray {
        self + self.dir * amount
    }
}

impl Add<V3> for Ray {
    type Output = Ray;
    fn add(self, rhs: V3) -> Ray {
        Ray {
            orig: self.orig + rhs,
            dir: self.dir,
        }
    }
}

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

    pub fn r(&self) -> f32 {
        self.0[0]
    }
    pub fn g(&self) -> f32 {
        self.0[1]
    }
    pub fn b(&self) -> f32 {
        self.0[2]
    }

    // transparency: 0: all self, 1: all rhs
    pub fn blend(&self, rhs: Color, transparency: f32) -> Color {
        let (t0, t1) = (transparency, 1.0 - transparency);
        Color([
            rhs.r() * t0 + self.r() * t1,
            rhs.g() * t0 + self.g() * t1,
            rhs.b() * t0 + self.b() * t1,
        ])
    }

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
        [
            normalize(self.r()),
            normalize(self.g()),
            normalize(self.b()),
        ]
    }
}

impl Add<Color> for Color {
    type Output = Color;
    fn add(self, rhs: Self) -> Color {
        Color([self.r() + rhs.r(), self.g() + rhs.g(), self.b() + rhs.b()])
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
        Hit {
            pos: self.pos + self.pos * amount,
            ..self
        }
    }
}
