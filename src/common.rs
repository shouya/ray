#[derive(Debug, Clone, Copy)]
pub struct V3(pub [f32; 3]);
#[derive(Debug, Clone, Copy)]
pub struct Color(pub [f32; 3]);

#[derive(Debug, Clone)]
pub struct Line(V3, V3);
#[derive(Debug, Clone)]
pub struct Plane(V3, V3);
#[derive(Debug, Clone)]
pub struct Trig(V3, V3, V3);

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

pub fn dist2(a: V3, b: V3) -> f32 {
    let d = b - a;
    d.dot(d)
}
pub fn dist(a: V3, b: V3) -> f32 {
    dist2(a, b).sqrt()
}

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
}

use std::ops::{Add, Div, Mul, Sub};

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
        let shift = V3([0.0, 1.0, 0.0]);
        let dist = shift.dot(self.n());
        (shift - self.n() * dist).norm()
    }
    pub fn secondary_axis(&self) -> V3 {
        self.primary_axis().cross(self.n())
        // self.n().cross(self.primary_axis())
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

    pub fn refract(&self, hit: &Hit, index: f32) -> Ray {
        let eta = if hit.inside { 1.0 / index } else { index };
        let cosi = -hit.norm.dot(self.dir);
        let k = 1.0 - eta * eta * (1.0 - cosi * cosi);
        let dir = self.dir * eta + hit.norm * (eta * cosi - k.sqrt());
        Ray::new(hit.pos, dir)
    }

    pub fn drift(&self, std_dev: f32) -> Ray {
        use rand::distributions::Distribution;
        use rand::distributions::Normal;
        use rand::thread_rng;

        if std_dev == 0.0 {
            return self.clone();
        }

        let n = Normal::new(0.0, std_dev as f64);
        let dx = n.sample(&mut thread_rng());
        let dy = n.sample(&mut thread_rng());
        let dz = n.sample(&mut thread_rng());
        Ray::new(self.orig, self.dir + V3([dx as f32, dy as f32, dz as f32]))
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
