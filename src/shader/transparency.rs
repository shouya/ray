mod fresnel;

use shader::{Reflection, Refraction, Mix};
use fresnel::fresnel;


pub struct Transparency {
  refl: Reflection,
  refr: Refraction
}

impl Shader for Transparency {
  fn render_depth(&self, s: &Scene, i: &Incidence, d: usize) -> Option<Color> {
    let frac = fresnel(self.refr.ior);
    let mix = Mix::new(
      Box::new(self.refl),
      Box::new(self.refr),
      frac
    );
    mix.render_depth(s, i, d)
  }
}
