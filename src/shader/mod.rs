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

pub use self::diffusion::Diffusion;
pub use self::glossy::Glossy;
pub use self::mix::Mix;
pub use self::phong::Phong;
pub use self::plain::Plain;
pub use self::reflection::Reflection;
pub use self::refraction::Refraction;
pub use self::sum::Sum;

pub struct Incidence<'r, 'h, 'o> {
  pub ray: &'r Ray,
  pub hit: &'h Hit,
  pub obj: &'o dyn Object,
  pub depth: usize,
}

#[derive(Clone)]
pub enum DynValue<T> {
  Const(T),
  Dyn(Rc<Fn(&Scene, &Incidence) -> T>),
}

pub trait Shader {
  fn render(&self, s: &Scene, i: &Incidence) -> Option<Color>;
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

  pub fn map<F, U>(self, f: F) -> DynValue<U>
  where
    F: Fn(T) -> U,
    F: 'static,
    T: 'static,
  {
    DynValue::from_fn(move |s: &Scene, i: &Incidence| f(self.get(s, i)))
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

impl<T: 'static> From<T> for DynValue<Option<Color>>
where
  T: Shader,
{
  fn from(v: T) -> DynValue<Option<Color>> {
    DynValue::from_fn(move |s: &Scene, i: &Incidence| v.render(s, i))
  }
}
