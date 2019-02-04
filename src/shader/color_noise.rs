use common::{randn, Color, Ray};
use scene::Scene;
use shader::{DynValue, Incidence, Shader};

pub struct ColorNoise {
    color: Color,
    roughness: f32, // std_dev
}

impl ColorNoise {
    pub fn new(color: Color, roughness: f32) -> Self {
        Self { color, roughness }
    }
}

impl Shader for ColorNoise {
    fn render(&self, _s: &Scene, i: &Incidence) -> Option<Color> {
        let dc = randn(0.0, self.roughness);
        let c = self.color + Color::from_intensity(dc);
        Some(c)
    }
}
