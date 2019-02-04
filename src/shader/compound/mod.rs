mod preset;
mod simple;
mod transparent;

pub use self::transparent::{transparent, Transparency};

pub use self::preset::{blank, glass, mirror, rough_solid, solid};

pub use self::simple::{
    simple_glass, simple_mirror, simple_rough_solid, simple_solid,
};
