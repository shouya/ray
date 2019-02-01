use super::{dist, Color, ImageBuffer, Ray, Rgb, RgbImage, Scene};
use shader::Incidence;
extern crate pbr;

fn trace_ray(s: &Scene, ray: Ray) -> Color {
  let hit = s.nearest_hit(&ray);
  if let None = hit {
    return s.ambient;
  }

  let (obj, hit) = hit.unwrap();
  let inci = Incidence {
    obj: obj.as_ref(),
    hit: &hit,
    ray: &ray,
    depth: 0,
  };
  let color = obj.render(s, &inci);

  color.unwrap_or(Color::Green)
}

#[allow(dead_code)]
pub fn trace(s: Scene, w: u32, h: u32) -> RgbImage {
  let mut film = ImageBuffer::new(w, h);
  let mut pb = pbr::ProgressBar::new((w * h).into());
  pb.set_max_refresh_rate(Some(std::time::Duration::from_millis(100)));

  for (x, y, pixel) in film.enumerate_pixels_mut() {
    let ray = s.generate_ray(x, y, w, h);
    let color = s.trace_ray(&ray, 0).unwrap_or(Color::Green).regularize();
    *pixel = Rgb(color.into());
    pb.inc();
  }
  pb.finish();

  film
}
