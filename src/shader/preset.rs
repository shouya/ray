use crate::common::Color;
use crate::shader::{ChannelMix, Diffuse, Mix, Phong, Reflection, Rough, ShaderType};

#[allow(unused)]
pub fn blank() -> ShaderType {
    Some(Color::White).into()
}

pub fn solid(color: Color, specular_index: f32) -> ShaderType {
    let diffusion: ShaderType = Diffuse::new(color.into()).into();
    let phong: ShaderType = Phong::new(specular_index.into()).into();

    ChannelMix::new(
        Some(Color::White).into(),
        diffusion,
        phong.map(|x| x.unwrap()),
    )
    .into()
}

#[allow(unused)]
pub fn rough_solid(
    color: Color,
    roughness: f32,
    specular_index: f32,
) -> ShaderType {
    let diffusion: ShaderType = Diffuse::new(color.into()).into();
    let phong: ShaderType = Phong::new(specular_index.into()).into();

    let shader = ChannelMix::new(
        Some(Color::White).into(),
        diffusion,
        phong.map(|x| x.unwrap()),
    )
    .into();
    Rough::new(shader, roughness).into()
}

use super::transparent;

pub fn glass(
    color: Color,
    specular_index: f32,
    transparency: f32,
    reflectivity: f32,
    ior: f32,
) -> ShaderType {
    let trans = transparent(reflectivity, ior);
    let solid_ = solid(color, specular_index);
    Mix::new(trans.into(), solid_.into(), transparency.into()).into()
}

pub fn mirror(
    color: Color,
    specular_index: f32,
    reflectivity: f32,
) -> ShaderType {
    let solid_ = solid(color, specular_index);
    let refl = Reflection;
    Mix::new(refl.into(), solid_.into(), reflectivity.into()).into()
}
