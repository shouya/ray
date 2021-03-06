use crate::common::{Color, Hit, Ray, TransMat};
use crate::object::Object;
use crate::scene::Scene;

use std::rc::Rc;

pub mod diffuse;
pub mod phong;
pub mod plain;
pub mod reflection;
pub mod refraction;

pub use self::diffuse::Diffuse;
pub use self::phong::Phong;
pub use self::plain::Plain;
pub use self::reflection::Reflection;
pub use self::refraction::Refraction;

pub mod normal;
pub use self::normal::Normal;

pub mod color_noise;
pub mod rough;
pub use self::color_noise::ColorNoise;
pub use self::rough::Rough;

pub mod transparent;
pub use self::transparent::{transparent, Transparency};

pub mod preset;
pub mod simple;
pub use self::preset::{blank, glass, mirror, rough_solid, solid};
pub use self::simple::{
    simple_glass, simple_mirror, simple_rough_solid, simple_solid,
};

pub mod mix;
pub use self::mix::{ChannelMix, Mix, Sum};

pub struct Incidence<'r, 'h, 'o> {
    pub ray: &'r Ray,
    pub hit: &'h Hit,
    pub obj: &'o dyn Object,
    pub trans: Option<TransMat>,
    pub depth: usize,
}

pub trait Shader {
    fn render(&self, s: &Scene, i: &Incidence<'_, '_, '_>) -> Option<Color>;
}

pub type ShaderType = DynValue<Option<Color>>;

#[derive(Clone)]
pub enum DynValue<T> {
    Const(T),
    Dyn(Rc<dyn Fn(&Scene, &Incidence<'_, '_, '_>) -> T>),
}

impl<T> DynValue<T>
where
    T: Clone,
{
    pub fn get(&self, s: &Scene, i: &Incidence<'_, '_, '_>) -> T {
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
        DynValue::from_fn(move |s: &Scene, i: &Incidence<'_, '_, '_>| f(self.get(s, i)))
    }
}

impl<T> DynValue<T> {
    pub fn from_fn<F>(f: F) -> Self
    where
        F: Fn(&Scene, &Incidence<'_, '_, '_>) -> T,
        F: 'static,
    {
        DynValue::Dyn(Rc::new(f))
    }
}

impl<T: 'static> DynValue<Option<T>>
where
    T: Clone,
{
    #[allow(unused)]
    pub fn unwrap(self) -> DynValue<T> {
        self.map(|x| x.unwrap())
    }
}

impl<T> From<T> for ShaderType
where
    T: Shader + 'static,
{
    fn from(v: T) -> ShaderType {
        DynValue::from_fn(move |s: &Scene, i: &Incidence<'_, '_, '_>| v.render(s, i))
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
    fn render(&self, s: &Scene, i: &Incidence<'_, '_, '_>) -> Option<Color> {
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
