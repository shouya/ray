use crate::common::Color;
use crate::scene::Scene;
use crate::shader::{Incidence, Shader};

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
    fn render(&self, _s: &Scene, _i: &Incidence<'_, '_, '_>) -> Option<Color> {
        Some(self.color)
    }
}
