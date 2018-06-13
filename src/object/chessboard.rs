use super::{Cow, Material, Object};

use common::*;

#[derive(Debug, Clone)]
pub struct ChessBoard {
    pub plane: Plane,
    pub material: (Material, Material),
    pub cell_size: f32,
}

impl Default for ChessBoard {
    fn default() -> ChessBoard {
        let material = Material {
            surface_color: Color::Black,
            emission_color: Color::Black,
            refractive_index: 0.0,
            transparency: 0.0,   // 0: opaque, 1: transparent
            reflexivity: 0.3,    // 0: black body, 1: perfect mirror
            specular_index: 0.0, // std dev of reflected shadow rays, 0: perfect smooth
        };
        let black = Material {
            surface_color: Color::White * 0.3,
            ..material
        };
        let white = Material {
            surface_color: Color::White * 0.7,
            ..material
        };

        ChessBoard {
            plane: Plane::new(V3::zero(), V3([0.0, 0.0, 1.0])),
            material: (black, white),
            cell_size: 1.0,
        }
    }
}

impl Object for ChessBoard {
    // https://en.wikipedia.org/wiki/Line%E2%80%93plane_intersection
    fn intersect(&self, ray: &Ray) -> Option<Hit> {
        // parallel to plane
        let det = ray.dir.dot(self.plane.n());
        if det <= 0.0 {
            return None;
        }

        let d = (self.plane.r0() - ray.orig).dot(self.plane.n()) / det;
        if d < 0.0 {
            return None;
        }
        let pos = ray.orig + ray.dir * d;
        let norm = V3::zero() - self.plane.n();

        Some(Hit {
            pos,
            norm,
            inside: false,
        })
    }

    fn material(&self, p: V3) -> Cow<Material> {
        let p = self.map_to_2d(p);
        let a = (p.x() / self.cell_size) as i32 % 2 == 0;
        let b = (p.y() / self.cell_size) as i32 % 2 == 0;

        if a && !b || !a && b {
            Cow::Borrowed(&self.material.0)
        } else {
            Cow::Borrowed(&self.material.1)
        }
    }
}

impl ChessBoard {
    // returning V3 must have .z() == 0.0
    fn map_to_2d(&self, p: V3) -> V3 {
        let rp = p - self.plane.r0();
        let b1 = self.plane.primary_axis();
        let b2 = self.plane.secondary_axis();
        V3([rp.dot(b1), rp.dot(b2), 0.0])
    }
}
