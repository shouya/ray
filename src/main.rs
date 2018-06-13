extern crate image;
extern crate rand;

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
        pub const White: Color = Color([1.0, 1.0, 1.0]);
        pub const Black: Color = Color([0.0, 0.0, 0.0]);
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

            let n = Normal::new(0.0, std_dev as f64);
            let dx = n.sample(&mut thread_rng());
            let dy = n.sample(&mut thread_rng());
            let dz = n.sample(&mut thread_rng());
            Ray::new(self.orig, self.dir + V3([dx as f32, dy as f32, dz as f32]))
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
}

mod object {
    use common::*;
    use std::borrow::Cow;

    #[derive(Debug, Clone, Copy)]
    pub struct Material {
        pub surface_color: Color,
        pub emission_color: Color,
        pub refractive_index: f32,
        pub transparency: f32,   // 0: opaque, 1: transparent
        pub reflexivity: f32,    // 0: black body, 1: perfect mirror
        pub specular_index: f32, // std dev of reflected shadow rays, 0: perfect smooth
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
            let mut inside = false;

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
            let t2 = tc + t1c;

            let t = if (t1 < 0.0 && t2 > 0.0) || (t1 > 0.0 && t2 < 0.0) {
                inside = true;
                Some(t1.max(t2))
            } else if t1 > 0.0 && t2 > 0.0 {
                Some(t1.min(t2))
            } else {
                None
            };

            if let Some(t) = t {
                let pos = ray.orig + ray.dir * t;
                let norm = (pos - self.c).norm();
                Some(Hit { pos, norm, inside })
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
            use std::f32;
            let mut min_dist = f32::MAX;
            let mut result = None;

            for obj in self.objs.iter() {
                if let Some(hit) = obj.intersect(&ray) {
                    if dist2(hit.pos, ray.orig) >= min_dist {
                        continue;
                    }
                    result = Some((obj, hit));
                    min_dist = dist2(hit.pos, ray.orig);
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

    pub mod reflection {
        use super::*;
        pub fn trace_ray(s: &Scene, ray: Ray, ambient: Color) -> Option<Color> {
            let hit = s.closet_hit(&ray);
            if hit.is_none() {
                return None;
            }

            let (obj, hit) = hit.unwrap();
            let dist = dist(hit.pos, ray.orig);

            let material = obj.material(hit.pos);
            let suf_color = material.surface_color;
            let refl_color = trace_ray(s, ray.reflect(&hit), ambient).unwrap_or(ambient);

            let color = suf_color.blend(refl_color, material.reflexivity);
            // fog
            let color = color.blend(ambient, 0.01 * dist);

            Some(color)
        }

        pub fn trace(s: Scene, w: u32, h: u32) -> RgbImage {
            let mut film = ImageBuffer::new(w, h);
            let ambient_color = Color([0.2, 0.2, 0.2]);

            for (x, y, pixel) in film.enumerate_pixels_mut() {
                let ray = s.generate_ray(x, y, w, h);
                match trace_ray(&s, ray, ambient_color) {
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

    pub mod scatter {
        use super::*;
        const MAX_DEPTH: u32 = 10;
        const SCATTER_AMOUNT: u32 = 20;

        pub fn trace_ray(s: &Scene, ray: Ray, ambient: Color, depth: u32) -> Option<Color> {
            if depth >= MAX_DEPTH {
                return None;
            }

            let hit = s.closet_hit(&ray);
            if hit.is_none() {
                return None;
            }

            let (obj, hit) = hit.unwrap();
            let dist = dist(hit.pos, ray.orig);

            let material = obj.material(hit.pos);
            let suf_color = material.surface_color;
            let shadowray = ray.reflect(&hit);
            let mut refl_colors = Vec::new();

            for _ in 0..(SCATTER_AMOUNT / depth) {
                let ray = shadowray.drift(material.specular_index);
                if ray.dir.dot(hit.norm) <= 0.0 {
                    continue;
                }
                if let Some(color) = trace_ray(s, ray, ambient, depth + 1) {
                    refl_colors.push(color);
                } else {
                    refl_colors.push(ambient);
                }
            }
            let refl_color = Color::blend_all(&refl_colors);

            let color = suf_color.blend(refl_color, material.reflexivity);
            // fog
            let color = color.blend(ambient, 0.05 * dist);

            Some(color)
        }

        pub fn trace(s: Scene, w: u32, h: u32) -> RgbImage {
            let mut film = ImageBuffer::new(w, h);
            let ambient_color = Color([0.2, 0.2, 0.2]);

            for (x, y, pixel) in film.enumerate_pixels_mut() {
                let ray = s.generate_ray(x, y, w, h);
                print!("Process: {}/{} ({}%)\r", y, h, y * 100 / h);
                match trace_ray(&s, ray, ambient_color, 1) {
                    Some(color) => {
                        *pixel = Rgb(color.into());
                    }
                    None => {
                        *pixel = Rgb(ambient_color.into());
                    }
                }
            }

            println!("");
            film
        }
    }

    pub mod transparency {
        use super::*;
        const MAX_DEPTH: u32 = 3;
        const SCATTER_AMOUNT: u32 = 30;

        pub fn trace_ray(s: &Scene, ray: Ray, ambient: Color, depth: u32) -> Option<Color> {
            if depth >= MAX_DEPTH {
                return None;
            }

            let hit = s.closet_hit(&ray);
            if hit.is_none() {
                return None;
            }

            let (obj, hit) = hit.unwrap();
            let material = obj.material(hit.pos);

            let suf_color = material.surface_color;

            let refl_ray = ray.reflect(&hit);
            let mut refl_color = color::Black;

            for _ in 0..(SCATTER_AMOUNT / depth) {
                let ray = refl_ray.drift(material.specular_index);
                if let Some(color) = trace_ray(s, ray, ambient, depth + 1) {
                    refl_color = refl_color + color;
                }
            }

            let refr_ray = ray.refract(&hit, material.refractive_index);
            let refr_color = trace_ray(s, refr_ray, ambient, depth + 1).unwrap_or(color::Black);

            let color = material.emission_color
                + suf_color * (1.0 - material.reflexivity - material.transparency)
                + refl_color * material.reflexivity
                + refr_color * material.transparency;

            // fog
            // let color = color.blend(ambient, 0.05 * dist);

            Some(color)
        }

        pub fn trace(s: Scene, w: u32, h: u32) -> RgbImage {
            let mut film = ImageBuffer::new(w, h);
            let ambient_color = Color([0.0, 0.0, 0.0]);

            for (x, y, pixel) in film.enumerate_pixels_mut() {
                let ray = s.generate_ray(x, y, w, h);
                if x == 0 {
                    print!("Process: {}/{} ({}%)\r\n", y, h, y * 100 / h);
                }
                match trace_ray(&s, ray, ambient_color, 1) {
                    Some(color) => {
                        *pixel = Rgb(color.into());
                    }
                    None => {
                        *pixel = Rgb(ambient_color.into());
                    }
                }
            }

            println!("");
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

    let colors = [
        color::Red,
        color::Green,
        color::Blue,
        color::White,
        color::Red,
    ];

    for i in 0..5 {
        scene1.add_object(Sphere {
            c: V3([7.0 + i as f32 * 2.0, i as f32 * 2.0, 0.0]),
            r: 1.5,
            material: Material {
                surface_color: colors[i],
                emission_color: Color([0.1, 0.1, 0.1]),
                reflexivity: 0.5,
                refractive_index: 0.9,
                specular_index: 0.00,
                transparency: 0.2,
            },
        });
    }
    // scene1.add_object(Sphere {
    //     c: V3([4.0, -2.0, 0.0]),
    //     r: 1.5,
    // });

    let img = raytracing::transparency::trace(scene1, 400, 400);
    img.save("./trace.png").ok();
}
