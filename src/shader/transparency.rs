use common::{Color, Hit, Ray};
use scene::Scene;
use shader::{DynValue, Incidence, Mix, Reflection, Refraction, Shader};

// return reflection ratio
fn fresnel_internal(ray: &Ray, hit: &Hit, ior: f32) -> f32 {
  use std::mem;

  let cosi = ray.dir.dot(hit.norm);
  let mut etai = ior;
  let mut etat = 1.0;
  if cosi < 0.0 {
    mem::swap(&mut etai, &mut etat);
  }
  let eta = etai / etat;

  let sint2 = eta * eta * (1.0 - cosi * cosi).max(0.0);
  if sint2 >= 1.0 {
    return 1.0;
  }

  let cost = (1.0 - sint2).max(0.0).sqrt();
  let cosi = cosi.abs();
  let rs = (etat * cosi - etai * cost) / (etat * cosi + etai * cost);
  let rp = (etai * cosi - etat * cost) / (etai * cosi + etat * cost);
  (rs * rs + rp * rp) / 2.0
}

pub fn fresnel(ior: &DynValue<f32>) -> DynValue<f32> {
  let ior = ior.clone();
  let f = move |s: &Scene, i: &Incidence| {
    let ior = ior.get(s, i);
    fresnel_internal(i.ray, i.hit, ior)
  };
  DynValue::from_fn(f)
}

pub struct Transparency {
  mix: Mix,
  // refl: Reflection,
  // refr: Refraction,
}

impl Shader for Transparency {
  fn render_depth(&self, s: &Scene, i: &Incidence, d: usize) -> Option<Color> {
    self.mix.render_depth(s, i, d)
  }
}

impl Transparency {
  fn new(reflectivity: DynValue<f32>, ior: DynValue<f32>) -> Self {
    let refl = Reflection { reflectivity };
    let refr = {
      let ior2 = ior.clone();
      Refraction { ior: ior2 }
    };
    let frac = fresnel(&ior);
    let mix = Mix::new(Box::new(refl), Box::new(refr), frac);
    Self { mix }
  }
}
