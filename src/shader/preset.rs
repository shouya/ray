use common::Color;
use shader;
use shader::Shader;

pub fn blank() -> impl Shader {
  shader::Plain::new(Color::White)
}

pub fn shiny(surface_color: Color, specular_color: Color, specular_index: f32) -> impl Shader {
  let diffuse = shader::Diffuse::new(surface_color.into(), 0.0.into());
  let phong = shader::Phong::new(specular_color.into(), specular_index.into());
  shader::Sum::new(Box::new(diffuse), Box::new(phong))
}

pub fn glass(surface_color: Color) -> impl Shader {
  let metalic_color = Color([0.38; 3]);
  let glossy = shader::Glossy::new(metalic_color.into(), 0.05.into());
  let diffuse = shader::Diffuse::new(surface_color.into(), 0.0.into());
  shader::Mix::new(Box::new(glossy), Box::new(diffuse), 0.89.into())
}
