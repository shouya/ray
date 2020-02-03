use crate::common::{randn_v3, Color, Hit};
use crate::scene::Scene;
use crate::shader::{Incidence, Shader, ShaderType};

// varies the hit normal
pub struct Rough {
    shader: ShaderType,
    // best with value in (0.005, 0.1)
    roughness: f32, // std_dev
}

impl Rough {
    pub fn new(shader: ShaderType, roughness: f32) -> Self {
        Self { shader, roughness }
    }
}

impl Shader for Rough {
    fn render(&self, s: &Scene, i: &Incidence<'_, '_, '_>) -> Option<Color> {
        let dnorm = randn_v3(0.0, self.roughness);
        let hit = &Hit { norm: (i.hit.norm + dnorm).norm(),
                         ..*i.hit };
        let i = Incidence { hit, ..*i };
        self.shader.get(s, &i)
    }
}
