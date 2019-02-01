mod preset;
mod simple;
mod transparent;

pub use self::transparent::{transparent, Transparency};

pub use self::preset::{blank, glass, mirror, solid};

pub use self::simple::{simple_glass, simple_mirror, simple_solid};
