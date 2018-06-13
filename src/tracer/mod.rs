use super::image::{GenericImage, GrayImage, ImageBuffer, Luma, Pixel, Rgb, RgbImage};
use common::*;
use scene::Scene;

// most primitive, only record encounters
pub mod incidence;

// w/ reflection
pub mod reflection;

// w/ diffusive reflection
pub mod scatter;

// w/ light source
// pub mod light;
