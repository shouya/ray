use common::Color;
use object::Material;
use shader;
use shader::Shader;

pub fn blank() -> impl Shader {
  shader::Plain::new(Color::White)
}

pub fn shiny(surface_color: Color, specular_color: Color, specular_index: f32) -> impl Shader {
  let diffuse = shader::Diffusion::new(surface_color.into(), 0.0.into());
  let phong = shader::Phong::new(specular_color.into(), specular_index.into());
  shader::Sum::new(Box::new(diffuse), Box::new(phong))
}

pub fn glass(surface_color: Color) -> impl Shader {
  let metalic_color = Color([0.38; 3]);
  let glossy = shader::Glossy::new(metalic_color.into(), 0.05.into());
  let diffuse = shader::Diffusion::new(surface_color.into(), 0.0.into());
  shader::Mix::new(Box::new(glossy), Box::new(diffuse), 0.89.into())
}

pub fn from_material(m: Material) -> impl Shader {
  let a1 = m.diffusion;
  let a2 = m.specular_index;
  let a3 = m.reflexivity;
  let a4 = m.transparency;

  let q1 = 1.0 / (a2 / a1 + 1.0);
  let q2 = 1.0 / (a4 / a3 + 1.0);
  let p = a1 / q1;

  let s1 = shader::Diffusion::new(m.surface_color.into(), 0.0.into());
  let s2 = shader::Phong::new(Color::White.into(), 0.0.into());
  // let s3 = shader::Transparent::new();

  s1
}
