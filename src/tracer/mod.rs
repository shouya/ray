use super::image::{ImageBuffer, Rgb, RgbImage};
use common::*;
use scene::Scene;

pub mod modular;

#[derive(Debug, Clone, Builder)]
pub struct RenderConfig {
    // antialiasing
    pub aa: Option<AAPattern>,
    #[builder(default = "1200")]
    pub w: u32,
    #[builder(default = "1200")]
    pub h: u32,
}

#[derive(Debug, Clone)]
pub struct AAPattern(Vec<[f32; 2]>);

impl AAPattern {
    // See: https://en.wikipedia.org/wiki/Supersampling
    pub fn ssaa4x() -> Self {
        AAPattern(
            [[0.25, 0.25], [0.25, 0.75], [0.75, 0.25], [0.75, 0.75]].to_vec(),
        )
    }
    pub fn hraa() -> Self {
        AAPattern(
            [[0.0, 0.0], [0.0, 1.0], [1.0, 0.0], [1.0, 1.0], [0.5, 0.5]]
                .to_vec(),
        )
    }

    pub fn pixel_offsets(aa: &Option<Self>, x: u32, y: u32) -> Vec<[f32; 2]> {
        match aa {
            None => vec![[x as f32, y as f32]],
            Some(aa) => {
                aa.0.iter()
                    .map(|[dx, dy]| [x as f32 + dx, y as f32 + dy])
                    .collect()
            }
        }
    }
}
