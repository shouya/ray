use common::*;
use scene::Scene;
use shader::{Incidence, ShaderType};

pub trait Object {
    // returns hit and norm
    fn intersect(&self, ray: &Ray) -> Option<Hit>;

    // implement this method to allow back-face bulling
    fn const_normal(&self) -> Option<V3> {
        None
    }

    fn render(&self, _s: &Scene, _i: &Incidence) -> Option<Color> {
        Some(Color::Blue)
    }

    fn shaded(self, shader: ShaderType) -> Shaded
    where
        Self: Sized + 'static,
    {
        Shaded::new(self, shader)
    }

    // accelerate computation when intersection is slow to compute
    fn bound(&self) -> Option<Bound> {
        None
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
pub use self::shaded::Shaded;
