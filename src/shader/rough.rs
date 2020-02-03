use common::{Color, Hit, V3};
use scene::Scene;
use shader::{Incidence, Shader, ShaderType};

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

impl Rough {
    fn pseudo_rand_v3(std_dev: f32) -> V3 {
        use rand::prelude::thread_rng;
        use rand_distr::Normal;
        use rand_distr::Distribution;

        let rng = thread_rng();
        let distribution = Normal::new(0.0, std_dev.into()).unwrap();
        let v = distribution.sample_iter(rng)
                            .take(3)
                            .collect::<Vec<f32>>();

        V3([v[0], v[1], v[2]])
    }
}

impl Shader for Rough {
    fn render(&self, s: &Scene, i: &Incidence) -> Option<Color> {
        let dnorm = Self::pseudo_rand_v3(self.roughness);
        let hit = &Hit { norm: (i.hit.norm + dnorm).norm(),
                         ..*i.hit };
        let i = Incidence { hit, ..*i };
        self.shader.get(s, &i)
    }
}
