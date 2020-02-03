use crate::common::{randn, Color};
use crate::scene::Scene;
use crate::shader::{Incidence, Shader};

pub struct ColorNoise {
    color: Color,
    roughness: f32, // std_dev
}

impl ColorNoise {
    #[allow(unused)]
    pub fn new(color: Color, roughness: f32) -> Self {
        Self { color, roughness }
    }
}

impl Shader for ColorNoise {
    fn render(&self, _s: &Scene, _i: &Incidence<'_, '_, '_>) -> Option<Color> {
        let dc = randn(0.0, self.roughness);
        let c = self.color + Color::from_intensity(dc);
        Some(c)
    }
}
