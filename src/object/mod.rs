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
