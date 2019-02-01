use super::Object;
use scene::Scene;
use shader::simple_solid;
use shader::{ShaderType, Incidence};

use common::*;

#[derive(Clone)]
pub struct ChessBoard {
    pub plane: Plane,
    pub material: (ShaderType, ShaderType),
    pub cell_size: f32,
}

impl Default for ChessBoard {
    fn default() -> ChessBoard {
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
