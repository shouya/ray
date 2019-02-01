use common::*;
use scene::Scene;
use shader::Incidence;
use std::borrow::Cow;

pub trait Object {
    // returns hit and norm
    fn intersect(&self, ray: &Ray) -> Option<Hit>;

    // implement this method to allow back-face bulling
    fn const_normal(&self) -> Option<V3> {
        None
    }

    fn render(&self, s: &Scene, i: &Incidence) -> Option<Color> {
        Some(Color::Blue)
    }
}

pub trait Transform {
    fn translate(self, d: V3) -> Self;
}

pub mod bezier_surface;
pub mod chessboard;
pub mod mesh;
pub mod sphere;
pub mod triangle;

pub use self::bezier_surface::BezierSurface;
pub use self::chessboard::ChessBoard;
pub use self::mesh::TrigMesh;
pub use self::sphere::Sphere;
pub use self::triangle::{Rectangle, Triangle};

pub mod shaded;
pub use self::shaded::Shaded;
