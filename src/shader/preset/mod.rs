use common::Color;
use shader;
use shader::Shader;

mod simple;
mod transparent;

pub use self::simple::{blank, glass, mirror, solid};
pub use self::simple::{simple_solid, simple_glass, simple_mirror};
pub use self::transparent::{transparent, Transparency};
