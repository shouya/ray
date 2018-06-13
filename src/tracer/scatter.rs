use super::{dist, Color, ImageBuffer, Ray, Rgb, RgbImage, Scene};

const MAX_DEPTH: u32 = 7;
const SCATTER_AMOUNT: u32 = 50;

pub fn trace_ray(s: &Scene, ray: Ray, ambient: Color, depth: u32) -> Option<Color> {
    if depth >= MAX_DEPTH {
        return None;
    }

    let hit = s.nearest_hit(&ray);
    if hit.is_none() {
        return None;
    }

    let (obj, hit) = hit.unwrap();
    let dist = dist(hit.pos, ray.orig);

    let material = obj.material(hit.pos);
    let suf_color = material.surface_color;
    let shadowray = ray.reflect(&hit);
    let mut refl_colors = Vec::new();

    for _ in 0..(SCATTER_AMOUNT / depth) {
        let ray = shadowray.drift(material.specular_index);
        if ray.dir.dot(hit.norm) <= 0.0 {
            continue;
        }
        if let Some(color) = trace_ray(s, ray, ambient, depth + 1) {
            refl_colors.push(color);
        } else {
            refl_colors.push(ambient);
        }
    }
    let refl_color = Color::blend_all(&refl_colors);

    let color = suf_color.blend(refl_color, material.reflexivity);
    // fog
    let color = color.blend(ambient, 0.01 * dist);

    Some(color)
}

pub fn trace(s: Scene, w: u32, h: u32) -> RgbImage {
    let mut film = ImageBuffer::new(w, h);
    let ambient_color = Color([0.2, 0.2, 0.2]);

    for (x, y, pixel) in film.enumerate_pixels_mut() {
        let ray = s.generate_ray(x, y, w, h);
        print!("Process: {}/{} ({}%)\r", y, h, y * 100 / h);
        match trace_ray(&s, ray, ambient_color, 1) {
            Some(color) => {
                *pixel = Rgb(color.into());
            }
            None => {
                *pixel = Rgb(ambient_color.into());
            }
        }
    }

    println!("");
    film
}
