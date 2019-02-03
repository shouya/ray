use common::*;
use scene::Scene;
use shader::{Incidence, ShaderType};

pub trait Object {
    // required implementation: intersect & usually render

    // returns hit and norm
    fn intersect(&self, ray: &Ray) -> Option<Hit>;
    fn render(&self, _s: &Scene, _i: &Incidence) -> Option<Color> {
        Some(Color::Blue)
    }

    // implement these two functions for accelerating computation if necessary
    // back-face bulling
    fn const_normal(&self) -> Option<V3> {
        None
    }
    // bounding box or bouding sphere
    fn bound(&self) -> Option<Bound> {
        None
    }

    // Assign shader to object
    fn shaded(self, shader: ShaderType) -> Shaded
    where
        Self: Sized + 'static,
    {
        Shaded::new(self, shader)
    }

    // Assign transformer to object
    fn transformed(self) -> Transformed
    where
        Self: Sized + 'static,
    {
        Transformed::new(self)
    }
}

pub trait Transform {
    fn translate(self, d: V3) -> Self;
}

// pub mod bezier_surface;
pub mod chessboard;
pub mod mesh;
pub mod sphere;
pub mod triangle;

// pub use self::bezier_surface::BezierSurface;
pub use self::chessboard::ChessBoard;
pub use self::mesh::TrigMesh;
pub use self::sphere::Sphere;
pub use self::triangle::{Rectangle, Triangle};

pub mod shaded;
pub mod transformed;

pub use self::shaded::Shaded;
pub use self::transformed::Transformed;
