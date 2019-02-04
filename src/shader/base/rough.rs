use common::{randn, Color, Hit, Ray, V3};
use scene::Scene;
use shader::{DynValue, Incidence, Shader, ShaderType};

// varies the hit normal
pub struct Rough {
    shader: ShaderType,
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
        let f2u = |f| unsafe { std::mem::transmute::<f32, u32>(f) };
        let pos = h.pos.0;

        hasher.write_u32(f2u(pos[0]));
        hasher.write_u32(f2u(pos[1]));
        hasher.write_u32(f2u(pos[2]));

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
            norm: i.hit.norm + dnorm,
            ..*i.hit
        };
        let i = Incidence { hit, ..*i };
        self.shader.get(s, &i)
    }
}
