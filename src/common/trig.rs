use super::*;

#[derive(Debug, Clone, Copy)]
pub struct Trig(pub V3, pub V3, pub V3);

// trig with vertex normals
#[derive(Debug, Clone, Copy)]
pub struct TrigN {
  pub v: Trig,
  pub n: Trig,
}

// general trig
#[derive(Debug, Clone)]
pub enum TrigGen {
  TrigN(TrigN),
  Trig(Trig),
}

impl TrigN {
  pub fn norm_at(&self, p: V3) -> V3 {
    let uv = self.v.to_uv(p);
    self.n.at_uv(uv)
  }
}

impl TrigGen {
  pub fn new(v: Trig, n: Option<Trig>) -> TrigGen {
    if let Some(n) = n {
      TrigGen::TrigN(TrigN { v, n })
    } else {
      TrigGen::Trig(v)
    }
  }

  pub fn trig(&self) -> &Trig {
    match self {
      TrigGen::Trig(t) => t,
      TrigGen::TrigN(TrigN { v, n: _ }) => v,
    }
  }

  pub fn trig_n(&self) -> Option<&TrigN> {
    match self {
      TrigGen::Trig(_) => None,
      TrigGen::TrigN(x) => Some(x),
    }
  }
}
