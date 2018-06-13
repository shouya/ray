use super::image::{ImageBuffer, Rgb, RgbImage};
use common::*;
use scene::Scene;

// most primitive, only record encounters
pub mod incidence;

// w/ reflection
pub mod reflection;

// w/ diffusive reflection
pub mod scatter;

// w/ transparency
pub mod transparency;

enum TraceMode {
  Diffusive,
  Reflective,
  Transparent,
}

use object::Material;

impl TraceMode {
  pub fn from_material(m: &Material) -> Self {
    use self::TraceMode::*;

    if m.transparency == 0.0 {
      if m.reflexivity == 0.0 {
        Diffusive
      } else {
        Reflective
      }
    } else {
      Transparent
    }
  }
}
