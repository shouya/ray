use common::*;
use scene::Scene;
use shader::Incidence;
use std::borrow::Cow;

#[derive(Debug, Clone, Copy)]
pub struct Material {
    pub surface_color: Color,
    pub emission_color: Color,
    pub ior: f32,            // 1: air, 1.3: water, 1.5: glass
    pub diffusion: f32,      // 0: no surface color, 1: solid object
    pub specular_index: f32, // specular highlight, 0: turned off
    pub transparency: f32,   // 0: opaque, 1: transparent
    pub reflexivity: f32,    // 0: black body, 1: perfect mirror
    pub roughness: f32,      // std dev of reflection rays, 0: perfect smooth
}

pub trait Object {
    // returns hit and norm
    fn intersect(&self, ray: &Ray) -> Option<Hit>;
    // returns material at the point
    fn material(&self, pos: V3) -> Cow<Material>;

    // implement this method to allow back-face bulling
    fn const_normal(&self) -> Option<V3> {
        None
    }

    fn render_depth(&self, s: &Scene, i: &Incidence, d: usize) -> Option<Color> {
        Some(Color::White)
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

#[allow(non_upper_case_globals)]
impl Material {
    pub const Mirror: Material = Material {
        surface_color: Color([0.0, 0.0, 0.0]),
        emission_color: Color([0.0, 0.0, 0.0]),
        reflexivity: 0.5,
        specular_index: 0.3,
        ior: 1.0,
        transparency: 0.0,
        roughness: 0.0,
        diffusion: 0.1,
    };
    pub const Glass: Material = Material {
        transparency: 0.95,
        ior: 1.62,
        ..Material::Mirror
    };
    pub const PlaneGlass: Material = Material {
        ior: 1.0,
        ..Material::Glass
    };
    pub const Solid: Material = Material {
        surface_color: Color([1.0, 1.0, 1.0]),
        emission_color: Color([0.0, 0.0, 0.0]),
        reflexivity: 0.0,
        ior: 1.0,
        specular_index: 0.1,
        transparency: 0.0,
        roughness: 0.0,
        diffusion: 0.1,
    };
    pub const FrostedGlass: Material = Material {
        roughness: 0.05,
        ..Material::Glass
    };
    pub const FrostedMirror: Material = Material {
        roughness: 0.05,
        specular_index: 0.1,
        ..Material::Mirror
    };

    pub fn mixer(&self) -> ColorMixer<'_> {
        ColorMixer {
            m: self,
            diffusion_color: None,
            specular_color: None,
            reflection_color: None,
            refraction_color: None,
        }
    }
}

pub struct ColorMixer<'a> {
    m: &'a Material,
    diffusion_color: Option<Color>,
    specular_color: Option<Color>,
    reflection_color: Option<Color>,
    refraction_color: Option<Color>,
}

impl ColorMixer<'_> {
    fn diffusion(&mut self, c: Color) {
        self.diffusion_color = Some(c);
    }

    fn specular(&mut self, c: Color) {
        self.specular_color = Some(c);
    }

    fn reflection(&mut self, c: Color) {
        self.reflection_color = Some(c);
    }

    fn refraction(&mut self, c: Color) {
        self.refraction_color = Some(c);
    }

    fn mix(&self) -> Color {
        use std::ops::Add;
        let colors: Vec<Color> = [
            self.diffusion_color,
            self.specular_color,
            self.reflection_color,
            self.refraction_color,
        ]
        .into_iter()
        .map(|x| x.unwrap_or(Color::from_intensity(0.0)).regularize())
        .collect();

        let albedo: [f32; 4] = [
            self.m.diffusion,
            self.m.specular_index,
            self.m.reflexivity,
            self.m.transparency,
        ];
        let total_albedo = albedo.iter().fold(0.0, Add::add) / 4.0 + 0.0001;

        let color = colors
            .into_iter()
            .zip(albedo.into_iter())
            .map(|(c, a)| c * (a / total_albedo))
            .fold(Color::Black, Add::add)
            .regularize();

        color
    }
}
