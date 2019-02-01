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
  Mix::new(trans.into(), solid_.into(), transparency.into())
}

pub fn mirror(color: Color, specular_index: f32, reflectivity: f32) -> impl Shader {
  let solid_ = solid(color, specular_index);
  let refl = Reflection;
  Mix::new(refl.into(), solid_.into(), reflectivity.into())
}

pub fn simple_solid(color: Color) -> impl Shader {
  solid(color, 10.0)
}

pub fn simple_glass(color: Color, transparency: f32) -> impl Shader {
  glass(color, 125.0, transparency, 0.8, 1.5)
}

pub fn simple_mirror(color: Color) -> impl Shader {
  mirror(color, 1425.0, 0.5)
}

//pub fn diffusive(color: Color) -> impl Shader {
//}
