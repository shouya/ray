use common::*;
use std::borrow::Cow;

#[derive(Debug, Clone, Copy)]
pub struct Material {
    pub surface_color: Color,
    pub emission_color: Color,
    pub ior: f32,            // 1: air, 1.3: water, 1.5: glass
    pub transparency: f32,   // 0: opaque, 1: transparent
    pub reflexivity: f32,    // 0: black body, 1: perfect mirror
    pub specular_index: f32, // std dev of reflected shadow rays, 0: perfect smooth
}

pub trait Object {
    // returns hit and norm
    fn intersect(&self, ray: &Ray) -> Option<Hit>;
    fn material(&self, pos: V3) -> Cow<Material>;
}

pub mod chessboard;
pub mod sphere;

pub use self::chessboard::ChessBoard;
pub use self::sphere::Sphere;

#[allow(non_upper_case_globals)]
impl Material {
    pub const Mirror: Material = Material {
        surface_color: Color([0.0, 0.0, 0.0]),
        emission_color: Color([0.0, 0.0, 0.0]),
        reflexivity: 0.95,
        ior: 1.62,
        specular_index: 0.3,
        transparency: 0.0,
    };
    pub const Glass: Material = Material {
        transparency: 0.95,
        ..Material::Mirror
    };
    pub const Solid: Material = Material {
        surface_color: Color([1.0, 1.0, 1.0]),
        emission_color: Color([0.0, 0.0, 0.0]),
        reflexivity: 0.0,
        ior: 1.0,
        specular_index: 0.5,
        transparency: 0.0,
    };
    pub const FrostedGlass: Material = Material {
        specular_index: 0.3,
        ..Material::Glass
    };
    pub const FrostedMirror: Material = Material {
        specular_index: 0.3,
        ..Material::Mirror
    };
}
