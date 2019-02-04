use common::Color;
use scene::Scene;
use shader::{Incidence, Shader};

#[allow(unused)]
pub struct Plain {
    color: Color,
}

impl Plain {
    #[allow(unused)]
    pub fn new(color: Color) -> Plain {
        Plain { color }
    }
}

impl Shader for Plain {
    fn render(&self, _s: &Scene, _i: &Incidence) -> Option<Color> {
        Some(self.color)
    }
}
