extern crate image;

mod common {
    #[derive(Debug, Clone, Copy)]
    pub struct V3(pub [f32; 3]);
    #[derive(Debug, Clone, Copy)]
    pub struct Color(pub [f32; 3]);

    pub mod color {
        use super::Color;

        pub const Red: Color = Color([1.0, 0.0, 0.0]);
        pub const Green: Color = Color([0.0, 1.0, 0.0]);
        pub const Blue: Color = Color([0.0, 0.0, 1.0]);
    }

    #[derive(Debug, Clone)]
    pub struct Line(V3, V3);
    #[derive(Debug, Clone)]
    pub struct Plane(V3, V3);
    #[derive(Debug, Clone)]
    pub struct Trig(V3, V3, V3);

    #[derive(Debug, Clone)]
    pub enum Projection {
        Orthogonal,
        Perspective,
    }

    #[derive(Debug, Clone)]
    pub struct Ray {
        pub orig: V3,
        // direction, must be normalized
        pub dir: V3,
    }

    pub struct Hit {
        pub pos: V3,
        pub norm: V3,
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
        pub fn new(x: f32, y: f32, z: f32) -> Self {
            V3([x, y, z])
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
        }
    }

    impl Ray {
        pub fn new(orig: V3, dir: V3) -> Self {
            Self {
                orig,
                dir: dir.norm(),
            }
        }
    }

    impl Color {
        pub fn r(&self) -> f32 {
            self.0[0]
        }
        pub fn g(&self) -> f32 {
            self.0[1]
        }
        pub fn b(&self) -> f32 {
            self.0[2]
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
}

mod object {
    use common::*;
    use std::borrow::Cow;

    #[derive(Debug, Clone, Copy)]
    pub struct Material {
        pub surface_color: Color,
        pub emission_color: Color,
        pub refractive_index: f32,
        pub transparency: f32, // 0: opaque, 1: transparent
        pub reflexivity: f32,  // 0: black body, 1: perfect mirror
        pub roughness: f32,    // norm of random reflected shadow rays, 0: perfect smooth
    }

    pub trait Object {
        // returns hit and norm
        fn intersect(&self, ray: &Ray) -> Option<Hit>;
        fn material(&self, pos: V3) -> Cow<Material>;
    }

    pub struct Sphere {
        pub c: V3,
        pub r: f32,
        pub material: Material,
    }

    impl Object for Sphere {
        // See: http://www.scratchapixel.com/lessons/3d-basic-rendering/minimal-ray-tracer-rendering-simple-shapes/ray-sphere-intersection
        fn intersect(&self, ray: &Ray) -> Option<Hit> {
            let l = self.c - ray.orig;
            let tc = l.dot(ray.dir);

            if tc < 0.0 {
                return None;
            }

            let d2 = l.dot(l) - tc * tc;
            let r2 = self.r * self.r;

            if d2 > r2 {
                return None;
            }

            let t1c = (r2 - d2).sqrt();
            let mut t1 = tc - t1c;
            let mut t2 = tc + t1c;

            if t1 < 0.0 {
                t1 = t2;
            }
            if t2 < t1 {
                t1 = t2;
            }
            let t = if t1 > 0.0 { Some(t1) } else { None };

            if let Some(t) = t {
                let pos = ray.orig + ray.dir * t;
                let norm = (pos - self.c).norm();
                Some(Hit { pos, norm })
            } else {
                None
            }
        }

        fn material(&self, pos: V3) -> Cow<Material> {
            Cow::Borrowed(&self.material)
        }
    }
}

mod scene {
    use common::*;
    use object::Object;

    pub struct Scene {
        pub objs: Vec<Box<Object>>,
        // plane w/ center, width, height
        pub vp_plane: Plane,
        pub vp_width: f32,
        pub vp_height: f32,
        pub camera: V3,
        pub projection: Projection,
    }

    impl Scene {
        pub fn new(camera: V3, vp_plane: Plane, vp_width: f32, vp_height: f32) -> Self {
            Self {
                camera,
                vp_plane,
                vp_width,
                vp_height,
                objs: Vec::new(),
                projection: Projection::Perspective,
            }
        }

        pub fn add_object<T: 'static>(&mut self, obj: T)
        where
            T: Object + Sized,
        {
            self.objs.push(Box::new(obj))
        }

        pub fn projection(&mut self, p: Projection) {
            self.projection = p;
        }

        pub fn vp_from_pixel(&self, x: u32, y: u32, w: u32, h: u32) -> V3 {
            let dx = self.vp_width * 2.0 / (w as f32);
            let dy = self.vp_height * 2.0 / (h as f32);
            let plane = &self.vp_plane;
            let (x, y, w, h) = (x as i64, y as i64, w as i64, h as i64);

            let shift_x = plane.primary_axis() * dx * (x - w / 2) as f32;
            let shift_y = plane.secondary_axis() * dy * (y - h / 2) as f32;
            plane.r0() + shift_x + shift_y
        }

        pub fn generate_ray(&self, x: u32, y: u32, w: u32, h: u32) -> Ray {
            let orig = self.vp_from_pixel(x, y, w, h);
            let dir = match self.projection {
                Projection::Perspective => orig - self.camera,
                Projection::Orthogonal => self.vp_plane.n(),
            };
            Ray::new(orig, dir)
        }

        pub fn closet_hit<'a>(&'a self, ray: &Ray) -> Option<(&'a Box<Object>, Hit)> {
            use std::cmp::Ordering;
            use std::f32;
            let mut min_dist = f32::MAX;
            let mut result = None;

            for obj in self.objs.iter() {
                if let Some(hit) = obj.intersect(&ray) {
                    if dist2(hit.pos, ray.orig) >= min_dist {
                        continue;
                    }
                    result = Some((obj, hit));
                }
            }

            result
        }
    }
}

mod raytracing {
    use super::image::{GenericImage, GrayImage, ImageBuffer, Luma, Pixel, Rgb, RgbImage};
    use common::*;
    use scene::Scene;

    pub mod incidence {
        // simplest ray tracing algorithm,
        // only considering incidence
        use super::{dist, dist2, ImageBuffer, Rgb, RgbImage, Scene};

        pub fn trace(s: Scene, w: u32, h: u32) -> RgbImage {
            let mut film = ImageBuffer::new(w, h);

            for (x, y, pixel) in film.enumerate_pixels_mut() {
                let ray = s.generate_ray(x, y, w, h);

                if let Some((_obj, hit)) = s.closet_hit(&ray) {
                    let dist = dist(hit.pos, ray.orig);
                    let brit = 250 - ((dist - 4.0) * 60.0) as u8;
                    *pixel = Rgb([brit, brit, brit]);
                } else {
                    *pixel = Rgb([0, 0, 0]);
                }
            }
            film
        }
    }

    pub mod color {
        use super::*;
        pub fn trace_ray(s: &Scene, ray: Ray) -> Option<Color> {
            let hit = s.closet_hit(&ray);
            if hit.is_none() {
                return None;
            }

            let (obj, hit) = hit.unwrap();
            let color = obj.material(hit.pos).surface_color;
            Some(color)
        }

        pub fn trace(s: Scene, w: u32, h: u32) -> RgbImage {
            let mut film = ImageBuffer::new(w, h);
            let ambient_color = Color([0.2, 0.2, 0.2]);

            for (x, y, pixel) in film.enumerate_pixels_mut() {
                let ray = s.generate_ray(x, y, w, h);
                match trace_ray(&s, ray) {
                    Some(color) => {
                        *pixel = Rgb(color.into());
                    }
                    None => {
                        *pixel = Rgb(ambient_color.into());
                    }
                }
            }

            film
        }
    }
}

fn main() {
    use common::*;
    use object::{Material, Sphere};
    use scene::Scene;

    let mut scene1 = Scene::new(
        V3::zero(),
        Plane::new(
            V3([2.0, 0.0, 0.0]), // r0
            V3([1.0, 0.0, 0.0]), // n
        ),
        2.0,
        2.0,
    );

    let material = Material {
        surface_color: color::Green,
        emission_color: Color([0.1, 0.0, 0.0]),
        reflexivity: 0.0,
        refractive_index: 0.0,
        roughness: 0.0,
        transparency: 0.0,
    };

    for i in 0..5 {
        scene1.add_object(Sphere {
            c: V3([7.0, (i as f32 - 2.0) * 2.0, 0.0]),
            r: 0.5 + i as f32 * 0.1,
            material: Material {
                surface_color: Color([0.0, 0.2 * i as f32, 0.0]),
                ..material
            },
        });
    }
    // scene1.add_object(Sphere {
    //     c: V3([4.0, -2.0, 0.0]),
    //     r: 1.5,
    // });

    let img = raytracing::color::trace(scene1, 400, 400);
    img.save("./trace.png").ok();
}
