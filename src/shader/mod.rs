use common::{Color, Hit, Ray};
use object::Object;
use scene::Scene;

use std::rc::Rc;

pub mod preset;

pub mod diffusion;
pub mod glossy;
pub mod mix;
pub mod phong;
pub mod plain;
pub mod reflection;
pub mod refraction;
pub mod sum;
pub mod transparency;

pub use self::diffusion::Diffusion;
pub use self::glossy::Glossy;
pub use self::mix::Mix;
pub use self::phong::Phong;
pub use self::plain::Plain;
pub use self::reflection::Reflection;
pub use self::refraction::Refraction;
pub use self::sum::Sum;
pub use self::transparency::{fresnel, Transparency};

pub struct Incidence<'r, 'h, 'o> {
  pub ray: &'r Ray,
  pub hit: &'h Hit,
  pub obj: &'o dyn Object,
}

#[derive(Clone)]
pub enum DynValue<T> {
  Const(T),
  Dyn(Rc<Fn(&Scene, &Incidence) -> T>),
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

impl<T> DynValue<T> {
  pub fn from_fn<F>(f: F) -> Self
  where
    F: Fn(&Scene, &Incidence) -> T,
    F: 'static,
  {
    DynValue::Dyn(Rc::new(f))
  }
}

impl<T> From<T> for DynValue<T> {
  fn from(v: T) -> DynValue<T> {
    DynValue::Const(v)
  }
}
