use common::{Color, Hit, Ray};
use object::Object;
use scene::Scene;

pub mod preset;

pub mod diffusion;
pub mod reflection;
pub mod refraction;
pub mod phong;
pub mod glossy;
pub mod mix;
pub mod sum;
pub mod plain;

pub use self::diffusion::Diffusion;
pub use self::reflection::Reflection;
pub use self::refraction::Refraction;
pub use self::phong::Phong;
pub use self::glossy::Glossy;
pub use self::mix::Mix;
pub use self::sum::Sum;
pub use self::plain::Plain;

pub struct Incidence<'r, 'h, 'o> {
  pub ray: &'r Ray,
  pub hit: &'h Hit,
  pub obj: &'o dyn Object,
}

pub enum DynValue<T> {
  Const(T),
  Dyn(fn(s: &Scene, i: &Incidence) -> T),
}

pub trait Shader {
  fn render(&self, s: &Scene, i: &Incidence) -> Color {
    self.render_depth(s, i, 0).unwrap_or(s.ambient)
  }

  fn render_depth(&self, s: &Scene, i: &Incidence, d: usize) -> Option<Color> {
    Some(self.render(s, i))
  }

  fn is_transparent(&self) -> bool {
    false
  }
}

impl<T> DynValue<T>
where
  T: Clone,
{
  pub fn get(&self, s: &Scene, i: &Incidence) -> T {
    match self {
      DynValue::Const(value) => value.clone(),
      DynValue::Dyn(f) => f(s, i),
    }
  }
}

impl<T> From<T> for DynValue<T> {
  fn from(v: T) -> DynValue<T> {
    DynValue::Const(v)
  }
}
