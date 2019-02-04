use super::{DynValue, Incidence, Shader, ShaderType};
use common::Color;
use scene::Scene;

pub struct Mix {
    frac: DynValue<f32>,
    a: ShaderType,
    b: ShaderType,
}

impl Mix {
    pub fn new(a: ShaderType, b: ShaderType, frac: DynValue<f32>) -> Self {
        Self { a, b, frac }
    }
}

impl Shader for Mix {
    fn render(&self, s: &Scene, i: &Incidence) -> Option<Color> {
        let f = self.frac.get(s, i);
        if f <= 0.0 {
            self.b.get(s, i)
        } else if f >= 1.0 {
            self.a.get(s, i)
        } else {
            let left = self.a.get(s, i)?;
            let right = self.b.get(s, i)?;
            Some(left.blend(right, f))
        }
    }
}

pub struct ChannelMix {
    frac: DynValue<Color>,
    a: ShaderType,
    b: ShaderType,
}

impl ChannelMix {
    pub fn new(a: ShaderType, b: ShaderType, frac: DynValue<Color>) -> Self {
        Self { a, b, frac }
    }
}

impl Shader for ChannelMix {
    fn render(&self, s: &Scene, i: &Incidence) -> Option<Color> {
        let frac = self.frac.get(s, i).regularize();
        if frac == Color::Zero {
            self.b.get(s, i)
        } else if frac == Color::One {
            self.a.get(s, i)
        } else {
            let left = self.a.get(s, i)?;
            let right = self.b.get(s, i)?;
            Some(left.channel_blend(right, frac))
        }
    }
}

#[allow(unused)]
pub struct Sum {
    a: ShaderType,
    b: ShaderType,
}

impl Sum {
    #[allow(unused)]
    pub fn new(a: ShaderType, b: ShaderType) -> Sum {
        Sum { a, b }
    }
}

impl Shader for Sum {
    fn render(&self, s: &Scene, i: &Incidence) -> Option<Color> {
        let a = self.a.get(s, i)?;
        let b = self.b.get(s, i)?;
        Some(a + b)
    }
}
