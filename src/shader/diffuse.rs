use crate::common::{dist2, Color, Ray};
use crate::scene::Scene;
use crate::shader::{DynValue, Incidence, Shader};

pub struct Diffuse {
    pub color: DynValue<Color>,
}

impl Diffuse {
    pub fn new(color: DynValue<Color>) -> Self {
        Self { color }
    }
}

const BIAS: f32 = 1e-4;

impl Shader for Diffuse {
    fn render(&self, s: &Scene, i: &Incidence<'_, '_, '_>) -> Option<Color> {
        let color = self.color.get(s, i);
        let mut intensity = s.background_light;
        let Incidence { hit, .. } = i;

        for light in s.lights.iter() {
            let light_pos = light.pos;
            let shadowray_dir = light_pos - hit.pos;
            let shadowray = Ray::new(hit.pos, shadowray_dir).biased(BIAS);

            if !s.is_blocked(&shadowray, dist2(light_pos, hit.pos)) {
                let angle = shadowray_dir.norm().dot(hit.norm).max(0.0);
                intensity = intensity + light.color * light.brightness * angle;
            }
        }

        Some(color * intensity)
    }
}
