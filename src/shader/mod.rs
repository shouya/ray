use common::{Color, Hit, Ray, M4};
use object::Object;
use scene::Scene;

use std::rc::Rc;

pub mod base;
pub mod compound;
pub mod mix;

pub use self::base::diffuse::Diffuse;
pub use self::base::phong::Phong;
pub use self::base::plain::Plain;
pub use self::base::reflection::Reflection;
pub use self::base::refraction::Refraction;

pub use self::base::normal::Normal;

pub use self::compound::*;

pub use self::mix::{ChannelMix, Mix, Sum};

pub struct Incidence<'r, 'h, 'o> {
  pub ray: &'r Ray,
  pub hit: &'h Hit,
  pub obj: &'o dyn Object,
  pub depth: usize,
}

pub trait Shader {
  fn render(&self, s: &Scene, i: &Incidence) -> Option<Color>;
}

pub type ShaderType = DynValue<Option<Color>>;

#[derive(Clone)]
pub enum DynValue<T> {
  Const(T),
  Dyn(Rc<Fn(&Scene, &Incidence) -> T>),
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

impl<T> From<T> for ShaderType
where
  T: Shader + 'static,
{
  fn from(v: T) -> ShaderType {
    DynValue::from_fn(move |s: &Scene, i: &Incidence| v.render(s, i))
  }
}

impl<T> From<T> for DynValue<T> {
  fn from(v: T) -> DynValue<T> {
    DynValue::Const(v)
  }
}

#[allow(unused)]
struct DynValueShader(ShaderType);

impl Shader for DynValueShader {
  fn render(&self, s: &Scene, i: &Incidence) -> Option<Color> {
    self.0.get(s, i)
  }
}

impl DynValue<Color> {
  #[allow(unused)]
  fn into_shader(self) -> DynValueShader {
    DynValueShader(self.map(|x| Some(x)))
  }
}

impl ShaderType {
  #[allow(unused)]
  fn into_shader(self) -> DynValueShader {
    DynValueShader(self)
  }
}
