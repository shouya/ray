use common::*;
use object::Object;

#[derive(Builder)]
pub struct Scene {
    #[builder(setter(skip))]
    pub objs: Vec<Box<Object>>,
    // plane w/ center, width, height
    pub vp_plane: Plane,
    pub vp_width: f32,
    pub vp_height: f32,
    pub camera: V3,
    pub projection: Projection,
    pub ambient: Color,
    #[builder(setter(skip))]
    pub lights: Vec<PointLight>,
}

impl Scene {
    pub fn add_object<T: 'static>(&mut self, obj: T)
    where
        T: Object + Sized,
    {
        self.objs.push(Box::new(obj))
    }

    pub fn add_light(&mut self, pos: V3, brightness: f32) {
        self.lights.push(PointLight { pos, brightness })
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

    pub fn nearest_hit<'a>(&'a self, ray: &Ray) -> Option<(&'a Box<Object>, Hit)> {
        use std::f32;
        let mut min_dist = f32::INFINITY;
        let mut result = None;

        for obj in self.objs.iter() {
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
