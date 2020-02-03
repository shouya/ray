use crate::common::{dist2, Color, Ray};
use crate::scene::Scene;
use crate::shader::{Incidence, Shader};

pub struct Normal;

impl Shader for Normal {
    fn render(&self, _s: &Scene, i: &Incidence<'_, '_, '_>) -> Option<Color> {
        let n = i.hit.norm;
        let f = |v| 0.5 * (v + 1.0);
        let c = Color([f(n.x()), f(n.y()), f(n.z())]);
        Some(c)
    }
}
