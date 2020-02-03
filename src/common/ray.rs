use super::*;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
  pub orig: V3,
  // direction, must be normalized
  pub dir: V3,
}

#[derive(Debug, Clone, Copy)]
pub struct Hit {
  pub pos: V3,
  pub norm: V3,
  pub inside: bool,
}

impl Ray {
  pub fn new(orig: V3, dir: V3) -> Self {
    Self { orig,
           dir: dir.norm() }
  }

  pub fn reflect(&self, hit: &Hit) -> Ray {
    let proj_n_d = hit.norm * self.dir.dot(hit.norm);
    Self { orig: hit.pos,
           dir: self.dir - proj_n_d * 2.0 }
    // Ray::new(hit.pos, self.dir - proj_n_d * 2.0)
  }

  pub fn refract(&self, hit: &Hit, ior: f32) -> Ray {
    let i = self.dir;
    let mut n = hit.norm;
    let mut cosi = n.dot(i).max(-1.0).min(1.0);
    let (mut etai, mut etat) = (1.0, ior);
    if cosi < 0.0 {
      cosi = -cosi;
    } else {
      mem::swap(&mut etai, &mut etat);
      n = -n;
    }

    let eta = etai / etat;
    let k = 1.0 - eta * eta * (1.0 - cosi * cosi);
    let dir = if k < 0.0 {
      // full internal refl
      V3([1.0, 0.0, 0.0])
    } else {
      i * eta + n * (eta * cosi - k.sqrt())
    };

    Ray::new(hit.pos, dir)
  }

  #[allow(unused)]
  pub fn drift(&self, std_dev: f32) -> Ray {
    if std_dev == 0.0 {
      return self.clone();
    }

    let dx = randn(0.0, std_dev);
    let dy = randn(0.0, std_dev);
    let dz = randn(0.0, std_dev);
    Ray::new(self.orig, self.dir + V3([dx, dy, dz]))
  }

  // amount = 0: no drift
  // amount = 1: scattered everywhere
  #[allow(unused)]
  pub fn drift_array(&self, amount: f32, count: usize, bias: f32) -> Vec<Ray> {
    let mut v = Vec::new();
    if amount == 0.0 {
      return vec![self.biased(bias)];
    }

    for _ in 1..count {
      v.push(self.drift(amount * f32::consts::PI).biased(bias))
    }

    v
  }

  pub fn biased(self, amount: f32) -> Ray {
    self + self.dir * amount
  }
}

impl Add<V3> for Ray {
  type Output = Ray;
  fn add(self, rhs: V3) -> Ray {
    Ray { orig: self.orig + rhs,
          dir: self.dir }
  }
}

impl Neg for Ray {
  type Output = Ray;
  fn neg(self) -> Ray {
    Ray { orig: self.orig,
          dir: -self.dir }
  }
}

impl Hit {
  pub fn biased(self, amount: f32) -> Hit {
    self + self.norm * amount
  }
}

impl Add<V3> for Hit {
  type Output = Hit;

  fn add(self, rhs: V3) -> Hit {
    Hit { pos: self.pos + rhs,
          ..self }
  }
}
