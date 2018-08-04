use common::{Color, Hit, Ray};
use object::Object;
use scene::Scene;

mod preset;

mod diffuse;
mod glossy;
mod mix;

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
  fn shade(&self, s: &Scene, _i: &Incidence) -> Color {
    s.ambient
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
