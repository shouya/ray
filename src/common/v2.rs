// Represents barycentric coordinates or planar coordinates
#[derive(Debug, Clone, Copy)]
pub struct V2(pub [f32; 2]);

impl V2 {
  pub fn u(&self) -> f32 {
    self.0[0]
  }
  pub fn v(&self) -> f32 {
    self.0[1]
  }
  pub fn w(&self) -> f32 {
    1.0 - self.u() - self.v()
  }
}
