use super::{dist, Color, ImageBuffer, Ray, Rgb, RgbImage, Scene};

pub fn trace_ray(s: &Scene, ray: Ray, ambient: Color) -> Option<Color> {
    let hit = s.nearest_hit(&ray);
    if hit.is_none() {
        return None;
    }

    let (obj, hit) = hit.unwrap();
    let dist = dist(hit.pos, ray.orig);

    let material = obj.material(hit.pos);
    let suf_color = material.surface_color;
    let refl_color = trace_ray(s, ray.reflect(&hit), ambient).unwrap_or(ambient);

    let color = suf_color.blend(refl_color, material.reflexivity);
    // fog
    let color = color.blend(ambient, 0.01 * dist);

    Some(color)
}

#[allow(dead_code)]
pub fn trace(s: Scene, w: u32, h: u32) -> RgbImage {
    let mut film = ImageBuffer::new(w, h);
    let ambient_color = Color([0.2, 0.2, 0.2]);

    for (x, y, pixel) in film.enumerate_pixels_mut() {
        let ray = s.generate_ray(x, y, w, h);
        match trace_ray(&s, ray, ambient_color) {
            Some(color) => {
                *pixel = Rgb(color.into());
            }
            None => {
                *pixel = Rgb(ambient_color.into());
            }
        }
    }

    film
}
