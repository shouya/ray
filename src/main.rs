extern crate image;

mod common {
    #[derive(Debug, Clone, Copy)]
    pub struct V3(pub [f32; 3]);
    pub struct Color([f32; 3]);

    #[derive(Debug, Clone)]
    pub struct Line(V3, V3);
    #[derive(Debug, Clone)]
    pub struct Plane(V3, V3);
    #[derive(Debug, Clone)]
    pub struct Trig(V3, V3, V3);

    pub struct Ray {
        pub orig: V3,
        // direction, must be normalized
        pub dir: V3,
    }

    pub struct Camera {
        pub pos: V3,
        pub dir: V3,
    }

    pub fn dist2(a: V3, b: V3) -> f32 {
        let d = b - a;
        d.dot(d)
    }
    pub fn dist(a: V3, b: V3) -> f32 {
        dist(a, b).sqrt()
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
            self / self.magn()
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
            V3([self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.y()])
        }
    }

    impl Add<V3> for V3 {
        type Output = V3;

        fn add(self, rhs: V3) -> V3 {
            V3([self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.y()])
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
            let shift = V3([1.0, 1.0, 1.0]);
            (shift - shift.dot(self.n())).norm()
        }
        pub fn secondary_axis(&self) -> V3 {
            self.n().cross(self.primary_axis())
        }
    }

    impl Ray {
        pub fn new(orig: V3, dir: V3) -> Self {
            Self { orig, dir }
        }
    }
}

mod object {
    use common::*;

    pub trait Intersectable {
        // returns hit and norm
        fn intersect(&self, ray: &Ray) -> Option<(V3, V3)>;
    }

    pub struct Sphere {
        pub c: V3,
        pub r: f32,
    }

    impl Intersectable for Sphere {
        // See: http://www.scratchapixel.com/lessons/3d-basic-rendering/minimal-ray-tracer-rendering-simple-shapes/ray-sphere-intersection
        fn intersect(&self, ray: &Ray) -> Option<(V3, V3)> {
            let l = self.c - ray.orig;
            let tca = l.dot(ray.dir);

            if tca < 0.0 {
                return None;
            }

            let d2 = l.dot(l) - tca * tca;
            if d2 > self.r * self.r {
                return None;
            }

            let thc = (self.r * self.r - d2).sqrt();
            let t = if tca - thc > 0.0 {
                tca - thc
            } else {
                tca + thc
            };

            let hit = ray.orig + ray.dir * t;
            let norm = (hit - self.c).norm();

            Some((hit, norm))
        }
    }
}

mod scene {
    use common::*;
    use object::Intersectable;

    pub struct Scene {
        pub objs: Vec<Box<Intersectable>>,
        // plane w/ center, width, height
        pub vp_plane: Plane,
        pub vp_width: f32,
        pub vp_height: f32,
        pub camera: V3,
    }

    impl Scene {
        pub fn new(camera: V3, vp_plane: Plane, vp_width: f32, vp_height: f32) -> Self {
            Self {
                camera,
                vp_plane,
                vp_width,
                vp_height,
                objs: Vec::new(),
            }
        }

        pub fn add_object<T: 'static>(&mut self, obj: T)
        where
            T: Intersectable + Sized,
        {
            self.objs.push(Box::new(obj))
        }

        pub fn vp_from_pixel(&self, x: u32, y: u32, w: u32, h: u32) -> V3 {
            let dx = self.vp_width * 2.0 / (w as f32);
            let dy = self.vp_height * 2.0 / (h as f32);
            let plane = &self.vp_plane;

            let shift_x = plane.primary_axis() * dx * ((w / 2 - x) as f32);
            let shift_y = plane.secondary_axis() * dy * ((h / 2 - y) as f32);

            plane.r0() + shift_x + shift_y
        }
    }
}

mod raytracing {
    use super::image::{GenericImage, GrayImage, ImageBuffer, Luma, Pixel, Rgb, RgbImage};
    use common::*;
    use scene::Scene;

    // simplest ray tracing algorithm
    pub fn trace1(s: Scene, w: u32, h: u32) -> GrayImage {
        let mut film = ImageBuffer::new(w, h);
        for (x, y, pixel) in film.enumerate_pixels_mut() {
            let ray = {
                let orig = s.vp_from_pixel(x, y, w, h);
                let dir = orig - s.camera;
                Ray::new(orig, dir)
            };

            let mut hits = vec![];
            for obj in s.objs.iter() {
                if let Some((hit, _norm)) = obj.intersect(&ray) {
                    hits.push(hit);
                }
            }
            if let Some(_hit) = hits.into_iter().min_by(|hit1, hit2| {
                dist2(*hit1, ray.orig)
                    .partial_cmp(&dist(*hit2, ray.orig))
                    .unwrap()
            }) {
                *pixel = Luma([255]);
            } else {
                *pixel = Luma([0]);
            }
        }
        film
    }
}

fn main() {
    use common::*;
    use object::Sphere;
    use raytracing::trace1;
    use scene::Scene;

    let mut scene1 = Scene::new(
        V3::zero(),
        Plane::new(
            V3([2.0, 0.0, 0.0]), // r0
            V3([1.0, 0.0, 0.0]), // n
        ),
        3.0,
        3.0,
    );

    scene1.add_object(Sphere {
        c: V3([3.0, 1.0, 0.0]),
        r: 1.0
    });
    scene1.add_object(Sphere {
        c: V3([3.0, -2.0, 0.0]),
        r: 1.5
    });

    let img = trace1(scene1, 400, 400);
    img.save("./trace1.png").ok();
}
