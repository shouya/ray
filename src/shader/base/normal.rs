use common::{dist2, Color, Ray};
use scene::Scene;
use shader::{DynValue, Incidence, Shader};

pub struct Normal;

impl Shader for Normal {
    fn render(&self, _s: &Scene, i: &Incidence) -> Option<Color> {
        let n = i.hit.norm;
        let f = |v| 0.5 * (v + 1.0);
        let c = Color([f(n.x()), f(n.y()), f(n.z())]);
        Some(c)
    }
}
