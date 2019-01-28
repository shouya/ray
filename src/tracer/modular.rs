use super::{dist, Color, ImageBuffer, Ray, Rgb, RgbImage, Scene};
use shader::Incidence;
extern crate indicatif;

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
  };
  let color = obj.render_depth(s, &inci, 0);

  color.unwrap_or(s.ambient)
}

#[allow(dead_code)]
pub fn trace(s: Scene, w: u32, h: u32) -> RgbImage {
  let mut film = ImageBuffer::new(w, h);
  let pb = indicatif::ProgressBar::new((w * h).into());
  pb.set_draw_delta(w.into());
  pb.set_style(
    indicatif::ProgressStyle::default_bar()
      .template("[{elapsed_precise}] {wide_bar:.cyan} {percent}%")
      .progress_chars("=>-"),
  );

  for (x, y, pixel) in film.enumerate_pixels_mut() {
    let ray = s.generate_ray(x, y, w, h);
    pb.tick();
    let color = trace_ray(&s, ray).regularize();
    *pixel = Rgb(color.into());
  }
  pb.finish_with_message("done");

  film
}
