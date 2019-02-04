use super::{AAPattern, RenderConfig};
use super::{Color, ImageBuffer, Rgb, RgbImage, Scene};
extern crate pbr;

#[allow(dead_code)]
pub fn trace(s: Scene, conf: RenderConfig) -> RgbImage {
    let (w, h) = (conf.w, conf.h);
    let mut pb = pbr::ProgressBar::new((w * h) as u64);
    pb.set_max_refresh_rate(Some(std::time::Duration::from_millis(100)));

    let mut film = ImageBuffer::new(w, h);
    for (x, y, pixel) in film.enumerate_pixels_mut() {
        let colors = AAPattern::pixel_offsets(&conf.aa, x, y)
            .into_iter()
            .map(|[x, y]| {
                let ray = s.generate_ray(x, y, w as f32, h as f32);
                s.trace_ray(&ray, 0).unwrap_or(Color::Green).regularize()
            })
            .collect::<Vec<_>>();

        *pixel = Rgb(Color::average(&colors).into());

        pb.inc();
    }

    pb.finish();

    film
}
