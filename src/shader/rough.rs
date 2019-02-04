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
    fn hash_hit(h: &Hit) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::Hasher;

        let mut hasher = DefaultHasher::new();
        // this might be better since it wipes out some inaccuracies
        let f2i = |f| (f * 10000000.0) as i32;
        let pos = h.pos.0;

        hasher.write_i32(f2i(pos[0]));
        hasher.write_i32(f2i(pos[1]));
        hasher.write_i32(f2i(pos[2]));

        hasher.finish()
    }

    fn pseudo_rand_v3(seed: u64, std_dev: f32) -> V3 {
        use rand::distributions::{Distribution, StandardNormal};
        use rand::{SeedableRng, StdRng};

        let mut rng = StdRng::seed_from_u64(seed);
        let n = StandardNormal;

        let mut f = move || n.sample(&mut rng) as f32 * std_dev;

        V3([f(), f(), f()])
    }
}

impl Shader for Rough {
    fn render(&self, s: &Scene, i: &Incidence) -> Option<Color> {
        let seed = Self::hash_hit(i.hit);
        let dnorm = Self::pseudo_rand_v3(seed, self.roughness);
        let hit = &Hit {
            norm: (i.hit.norm + dnorm).norm(),
            ..*i.hit
        };
        let i = Incidence { hit, ..*i };
        self.shader.get(s, &i)
    }
}
