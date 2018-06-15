use super::*;
use common::*;

#[derive(Debug, Clone)]
pub struct BezierSurface<'a> {
    pub p: [V3; 16],
    pub material: Cow<'a, Material>,
}

pub fn eval_bezier_curve(p: &[V3; 4], t: f32) -> V3 {
    let b0 = (1.0 - t) * (1.0 - t) * (1.0 - t);
    let b1 = 3.0 * t * (1.0 - t) * (1.0 - t);
    let b2 = 3.0 * t * t * (1.0 - t);
    let b3 = t * t * t;
    return p[0] * b0 + p[1] * b1 + p[2] * b2 + p[3] * b3;
}

impl<'a> BezierSurface<'a> {
    pub fn eval(&self, u: f32, v: f32) -> V3 {
        use std::convert::TryInto;
        let curve = self
            .p
            .exact_chunks(4)
            .map(|chunk| eval_bezier_curve(chunk.try_into().unwrap(), u))
            .collect::<Vec<_>>();
        eval_bezier_curve(curve.as_slice().try_into().unwrap(), v)
    }
}
