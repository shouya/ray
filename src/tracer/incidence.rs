// simplest ray tracing algorithm,
// only considering incidence
use super::{dist, ImageBuffer, Rgb, RgbImage, Scene};

#[allow(dead_code)]
pub fn trace(s: Scene, w: u32, h: u32) -> RgbImage {
    let mut film = ImageBuffer::new(w, h);

    for (x, y, pixel) in film.enumerate_pixels_mut() {
        let ray = s.generate_ray(x, y, w, h);

        if let Some((_obj, hit)) = s.nearest_hit(&ray) {
            let dist = dist(hit.pos, ray.orig);
            let brit = 250 - ((dist - 4.0) * 60.0) as u8;
            *pixel = Rgb([brit, brit, brit]);
        } else {
            *pixel = Rgb([0, 0, 0]);
        }
    }
    film
}
