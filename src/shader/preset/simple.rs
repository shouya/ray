use common::Color;
use shader::{
  ChannelMix, Diffusion, DynValue, Incidence, Mix, Phong, Plain, Reflection, Refraction, Shader,
};

pub fn blank() -> impl Shader {
  Plain::new(Color::White)
}

pub fn solid(color: Color, specular_index: f32) -> impl Shader {
  let diffusion: DynValue<Option<Color>> = Diffusion::new(color.into(), 1.0.into()).into();
  let phong: DynValue<Option<Color>> =
    Phong::new(Color::White.into(), specular_index.into()).into();
  ChannelMix::new(
    diffusion,
    Some(Color::White).into(),
    phong.map(|x| x.unwrap()),
  )
}

//pub fn diffusive(color: Color) -> impl Shader {
//}
