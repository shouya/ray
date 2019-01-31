use super::{Cow, Material, Object};
use scene::Scene;
use shader::preset::simple_solid;
use shader::{DynValue, Incidence};

use common::*;

#[derive(Clone)]
pub struct ChessBoard {
    pub plane: Plane,
    pub material: (DynValue<Option<Color>>, DynValue<Option<Color>>),
    pub cell_size: f32,
}

impl Default for ChessBoard {
    fn default() -> ChessBoard {
        let base = Material {
            surface_color: Color::Black,
            emission_color: Color::Black,
            ior: 0.0,
            diffusion: 0.9,
            transparency: 0.0,   // 0: opaque, 1: transparent
            reflexivity: 0.0,    // 0: black body, 1: perfect mirror
            specular_index: 0.1, // std dev of reflected shadow rays, 0: perfect smooth
            roughness: 0.0,
        };

        ChessBoard {
            plane: Plane::new(V3::zero(), V3([0.0, 0.0, 1.0])),
            material: (
                simple_solid(Color([0.3; 3])).into(),
                simple_solid(Color([0.7; 3])).into(),
            ),
            cell_size: 1.0,
        }
    }
}

impl Object for ChessBoard {
    // https://en.wikipedia.org/wiki/Line%E2%80%93plane_intersection
    fn intersect(&self, ray: &Ray) -> Option<Hit> {
        self.plane.intersect(ray).map(|pos| Hit {
            pos,
            inside: false,
            norm: self.plane.n(),
        })
    }

    fn render(&self, s: &Scene, i: &Incidence) -> Option<Color> {
        let p = self.map_to_2d(i.hit.pos);
        let is_even = |v: f32| (v / self.cell_size) as i32 % 2 == 0;
        let a = is_even(p.x()) ^ (p.x() < 0.0);
        let b = is_even(p.y()) ^ (p.y() < 0.0);

        if a ^ b {
            self.material.0.get(s, i)
        } else {
            self.material.1.get(s, i)
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
