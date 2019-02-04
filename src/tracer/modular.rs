use super::{Color, ImageBuffer, Rgb, RgbImage, Scene};
extern crate pbr;

#[allow(dead_code)]
pub fn trace(s: Scene, w: u32, h: u32) -> RgbImage {
    let mut pb = pbr::ProgressBar::new((w * h).into());
    pb.set_max_refresh_rate(Some(std::time::Duration::from_millis(100)));

    let mut film = ImageBuffer::new(w, h);
    for (x, y, pixel) in film.enumerate_pixels_mut() {
        let ray = s.generate_ray(x, y, w, h);
        let color = s.trace_ray(&ray, 0).unwrap_or(Color::Green).regularize();
        *pixel = Rgb(color.into());

        pb.inc();
    }

    pb.finish();

    film
}
