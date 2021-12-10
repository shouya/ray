use super::{AAPattern, RenderConfig};
use super::{Color, ImageBuffer, Rgb, RgbImage, Scene};

use pbr;
use rayon::prelude::*;

#[allow(dead_code)]
pub fn trace(s: Scene, conf: RenderConfig) -> RgbImage {
  let (w, h) = (conf.w, conf.h);
  let mut pb = pbr::ProgressBar::new((w * h) as u64);
  pb.set_max_refresh_rate(Some(std::time::Duration::from_millis(100)));

  let mut film = ImageBuffer::new(w, h);

  let coords = film
    .enumerate_pixels()
    .map(|(x, y, _)| (x, y))
    .collect::<Vec<_>>();

  coords
    .into_par_iter()
    .map(move |(x, y)| {
      let colors = AAPattern::pixel_offsets(&conf.aa, x, y)
        .into_iter()
        .map(|[x, y]| {
          let ray = s.generate_ray(x, y, w as f32, h as f32);
          s.trace_ray(&ray, 0).unwrap_or(Color::Green).regularize()
        })
        .collect::<Vec<_>>();

      let color = Color::average(&colors);
      pb.inc();
      (x, y, color)
    })
    .for_each(|(x, y, color)| {
      let pixel = Rgb(color.into());
      film.put_pixel(x, y, pixel)
    });

  pb.finish();

  film
}
