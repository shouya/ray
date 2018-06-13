use super::{ImageBuffer, Rgb, RgbImage, Scene};
use common::*;
use object::Material;

const MAX_DEPTH: u32 = 4;
const SCATTER_AMOUNT: u32 = 100;
const MAX_RETRY_COUNT: u32 = 10;

pub fn trace_ray(s: &Scene, ray: Ray, ambient: Color, depth: u32) -> Option<Color> {
    if depth >= MAX_DEPTH {
        return None;
    }

    let hit = s.nearest_hit(&ray);
    if hit.is_none() {
        return None;
    }

    let (obj, hit) = hit.unwrap();
    let dist_camera = dist(hit.pos, s.camera);
    let dist_travel = dist(hit.pos, ray.orig);

    if dist_camera > 50.0 {
        return Some(ambient);
    }

    let material = obj.material(hit.pos);
    let suf_color = material.surface_color;
    let shadowray = ray.reflect(&hit);
    let mut refl_colors = Vec::new();

    let scatter_amount = if material.specular_index == 0.0 {
        1
    } else {
        SCATTER_AMOUNT / depth
    };

    for _ in 0..scatter_amount {
        let ray = drift_ray(&shadowray, &hit, &material);
        if let Some(color) = trace_ray(s, ray, ambient, depth + 1) {
            refl_colors.push(color);
        } else {
            refl_colors.push(ambient);
        }
    }
    let refl_color = Color::blend_all(&refl_colors);

    let color = suf_color.blend(refl_color, material.reflexivity);
    // fog
    let color = color.blend(ambient, 0.02 * dist_travel);

    Some(color)
}

pub fn drift_ray(shadowray: &Ray, hit: &Hit, material: &Material) -> Ray {
    if shadowray.dir.dot(hit.norm) <= 0.0 {
        return *shadowray;
    }

    let mut ray = shadowray.drift(material.specular_index);
    let mut count = 0;
    loop {
        if ray.dir.dot(hit.norm) >= 0.0 {
            break;
        }
        if count >= MAX_RETRY_COUNT {
            return *shadowray;
        }
        ray = shadowray.drift(material.specular_index);
        count += 1;
    }
    ray
}

pub fn trace(s: Scene, w: u32, h: u32) -> RgbImage {
    let mut film = ImageBuffer::new(w, h);
    let ambient_color = Color([0.8, 0.8, 0.8]);

    for (x, y, pixel) in film.enumerate_pixels_mut() {
        let ray = s.generate_ray(x, y, w, h);
        if x == 0 {
            print!("Process: {}/{} ({}%)\r", y, h, y * 100 / h);
        }
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
