use crate::common::*;
use crate::object::Object;

#[derive(Builder)]
pub struct Scene {
    #[builder(setter(skip))]
    pub objs: Vec<Box<dyn Object>>,
    // plane w/ center, width, height
    pub vp_plane: Plane,
    pub vp_width: f32,
    pub vp_height: f32,
    pub camera: V3,
    pub projection: Projection,
    pub ambient: Color,
    #[builder(setter(skip))]
    pub lights: Vec<PointLight>,
    #[builder(default = "Color([0.2;3])")]
    pub background_light: Color,
    #[builder(default = "5")]
    pub max_depth: usize,
}

impl Scene {
    pub fn add_object<T: 'static>(&mut self, obj: T)
    where
        T: Object + Sized,
    {
        self.objs.push(Box::new(obj))
    }

    pub fn add_white_light(&mut self, pos: V3, brightness: f32) {
        self.add_light(pos, Color::White, brightness)
    }

    pub fn add_light(&mut self, pos: V3, color: Color, brightness: f32) {
        self.lights.push(PointLight {
            pos,
            color,
            brightness,
        })
    }

    pub fn vp_from_pixel(&self, x: f32, y: f32, w: f32, h: f32) -> V3 {
        let dx = self.vp_width * 2.0 / w;
        // y on screen coordinate system is inverted, down is positive
        let dy = -self.vp_height * 2.0 / h;
        let plane = &self.vp_plane;

        let shift_x = plane.primary_axis() * dx * (x - w / 2.0);
        let shift_y = plane.secondary_axis() * dy * (y - h / 2.0);
        plane.r0() + shift_x + shift_y
    }

    pub fn generate_ray(&self, x: f32, y: f32, w: f32, h: f32) -> Ray {
        let orig = self.vp_from_pixel(x, y, w, h);
        let dir = match self.projection {
            Projection::Perspective => orig - self.camera,
            Projection::Orthogonal => self.vp_plane.n(),
        };
        Ray::new(orig, dir)
    }

    pub fn trace_ray(&self, ray: &Ray, d: usize) -> Option<Color> {
        use crate::shader::Incidence;
        if d >= self.max_depth {
            return None;
        }

        match self.nearest_hit(ray) {
            None => Some(self.ambient),
            Some((obj, hit)) => {
                let inci = Incidence {
                    ray: &ray,
                    obj: obj.as_ref(),
                    hit: &hit,
                    trans: None,
                    depth: d,
                };
                obj.render(self, &inci)
            }
        }
    }

    pub fn is_blocked(&self, ray: &Ray, light_dist2: f32) -> bool {
        match self.nearest_hit(ray) {
            None => false,
            Some((_, h)) => dist2(h.pos, ray.orig) < light_dist2,
        }
    }

    pub fn nearest_hit<'a>(
        &'a self,
        ray: &Ray,
    ) -> Option<(&'a Box<dyn Object>, Hit)> {
        use std::f32;
        let mut min_dist = f32::INFINITY;
        let mut result = None;

        for obj in self.objs.iter() {
            // back-face bulling for optimizing rendering speed
            if let Some(n) = obj.const_normal() {
                if ray.dir.dot(n) > 0.0 {
                    continue;
                }
            }

            if let Some(bound) = obj.bound() {
                if !bound.intersect(&ray) {
                    continue;
                }
            }
            if let Some(hit) = obj.intersect(&ray) {
                if dist2(hit.pos, ray.orig) > min_dist {
                    continue;
                }
                result = Some((obj, hit));
                min_dist = dist2(hit.pos, ray.orig);
            }
        }

        result
    }
}
