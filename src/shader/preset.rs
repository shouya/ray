use shader;
use shader::Shader;
use common::Color;

pub fn blank() -> impl Shader {
  shader::Plain::new(Color::White)
}
