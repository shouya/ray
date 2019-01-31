use shader::{DynValue, Incidence, Shader};
use scene::Scene;

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

pub fn fresnel(ior: DynValue<f32>) -> DynValue<f32> {
  let f = |s: &Scene, i: &Incidence| {
      let ior = ior.get(s, i);
      fresnel_internal(i.ray, i.hit, ior)
  };
  DynValue::Dyn(f)
}