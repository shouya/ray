use common::Color;
use shader::{
  ChannelMix, Diffusion, DynValue, Incidence, Mix, Phong, Plain, Reflection, Refraction, Shader,
};

pub fn blank() -> impl Shader {
  Plain::new(Color::White)
}

pub fn solid(color: Color, specular_index: f32) -> impl Shader {
  let diffusion: DynValue<Option<Color>> = Diffusion::new(color.into()).into();
  let phong: DynValue<Option<Color>> = Phong::new(specular_index.into()).into();

  ChannelMix::new(
    Some(Color::White).into(),
    diffusion,
    phong.map(|x| x.unwrap()),
  )
}

use shader::preset::transparent;

pub fn glass(
  color: Color,
  specular_index: f32,
  transparency: f32,
  reflectivity: f32,
  ior: f32,
) -> impl Shader {
  let trans = transparent(reflectivity, ior);
  let solid_ = solid(color, specular_index);
  Mix::new(solid_.into(), trans.into(), transparency.into())
}

pub fn mirror(color: Color, specular_index: f32, reflectivity: f32) -> impl Shader {
  let solid_ = solid(color, specular_index);
  let refl = Reflection;
  Mix::new(solid_.into(), refl.into(), reflectivity.into())
}

pub fn simple_solid(color: Color) -> impl Shader {
  solid(color, 1.5)
}

pub fn simple_glass(color: Color, transparency: f32) -> impl Shader {
  glass(color, 3.0, transparency, 1.0, 1.3)
}

pub fn simple_mirror(color: Color) -> impl Shader {
  mirror(color, 3.0, 0.9)
}

//pub fn diffusive(color: Color) -> impl Shader {
//}
