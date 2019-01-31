use common::Color;
use shader::{Plain, Diffusion, DynValue, Incidence, Mix, Reflection, Refraction, Shader};

pub fn blank() -> impl Shader {
  Plain::new(Color::White)
}

//pub fn diffusive(color: Color) -> impl Shader {
//}

