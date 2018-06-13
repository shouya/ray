use super::{ImageBuffer, Rgb, RgbImage, Scene, TraceMode};
use common::*;
use object::Material;

const MAX_DEPTH: u32 = 4;
const SCATTER_AMOUNT: u32 = 100;
const MAX_RETRY_COUNT: u32 = 10;
const BIAS: f32 = 1e-4;

fn trace_ray(s: &Scene, ray: Ray, depth: u32) -> Color {
    if depth >= MAX_DEPTH {
        return s.ambient;
    }

    let hit = s.nearest_hit(&ray);
    if hit.is_none() {
        return s.ambient;
    }

    let (obj, hit) = hit.unwrap();
    let dist_camera = dist(hit.pos, s.camera);
    let dist_travel = dist(hit.pos, ray.orig);

    if dist_camera > 50.0 {
        return s.ambient;
    }

    let m = obj.material(hit.pos);
    let color = match TraceMode::from_material(&m) {
        TraceMode::Diffusive => trace_ray_diffusive(&s, &ray, &hit, &m),
        TraceMode::Reflective => trace_ray_reflective(&s, &ray, &hit, &m, depth),
        TraceMode::Transparent => trace_ray_transparent(&s, &ray, &hit, &m, depth),
    };

    // fog
    let color = color.blend(s.ambient, 0.02 * dist_travel);

    return color;

    let kr = fresnel(&ray, &hit, &m);

    let suf_color = m.surface_color;
    let shadowray = ray.reflect(&hit) + hit.norm * BIAS;

    let scatter_amount = if m.specular_index == 0.0 {
        1
    } else {
        SCATTER_AMOUNT / depth
    };

    let mut refl_colors = Vec::new();
    for _ in 0..scatter_amount {
        let ray = drift_ray(&shadowray, &hit, &m);
        refl_colors.push(trace_ray(s, ray, depth + 1));
    }
    let refl_color = Color::blend_all(&refl_colors);

    let shadowray = ray.refract(&hit, m.ior);
    let refr_color = trace_ray(s, shadowray, depth + 1);

    let hitcolor = refl_color.blend(refr_color, kr);
    let color = suf_color.blend(hitcolor, m.reflexivity);

    color
}

fn trace_ray_diffusive(s: &Scene, _ray: &Ray, hit: &Hit, m: &Material) -> Color {
    let mut brightness = 0.0;

    for light in s.lights.iter() {
        let light_dir = light.pos - hit.pos;
        let shadowray = Ray::new(hit.pos, light_dir);

        if let Some((_obj, sr_hit)) = s.nearest_hit(&shadowray) {
            brightness -= light.brightness;
        } else {
            let angle = (light.pos - hit.pos).norm().dot(hit.norm);
            if angle >= 0.0 {
                brightness += angle * light.brightness;
            }
        }
    }

    let apparence_color = m.surface_color;

    if brightness >= 0.0 {
        apparence_color.blend(Color::White, brightness)
    } else {
        apparence_color.blend(Color::Black, -brightness)
    }
}

fn trace_ray_reflective(s: &Scene, ray: &Ray, hit: &Hit, m: &Material, depth: u32) -> Color {
    let scatter_amount = if m.specular_index == 0.0 {
        1
    } else {
        SCATTER_AMOUNT / depth
    };

    let shadowray = ray.reflect(&hit) + hit.norm * BIAS;
    let mut refl_colors = Vec::new();
    for _ in 0..scatter_amount {
        let ray = drift_ray(&shadowray, &hit, &m);
        refl_colors.push(trace_ray(s, ray, depth + 1));
    }
    let refl_color = Color::blend_all(&refl_colors);

    m.surface_color.blend(refl_color, m.reflexivity)
}

fn trace_ray_transparent(s: &Scene, ray: &Ray, hit: &Hit, m: &Material, depth: u32) -> Color {
    m.surface_color
}

fn drift_ray(shadowray: &Ray, hit: &Hit, material: &Material) -> Ray {
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

fn fresnel(ray: &Ray, hit: &Hit, m: &Material) -> f32 {
    use std::mem;

    let cosi = ray.dir.dot(hit.norm);
    let mut etai = 1.0;
    let mut etat = m.ior;
    if cosi > 0.0 {
        mem::swap(&mut etai, &mut etat);
    }

    let sint = etai / etat * (1.0 - cosi * cosi).max(0.0).sqrt();
    if sint >= 1.0 {
        return 1.0;
    }

    let cost = (1.0 - sint * sint).max(0.0).sqrt();
    let cosi = cosi.abs();
    let rs = (etat * cosi - etai * cost) / (etat * cosi + etai * cost);
    let rp = (etai * cosi - etat * cost) / (etai * cosi + etat * cost);
    (rs * rs + rp * rp) / 2.0
}

#[allow(dead_code)]
pub fn trace(s: Scene, w: u32, h: u32) -> RgbImage {
    let mut film = ImageBuffer::new(w, h);

    for (x, y, pixel) in film.enumerate_pixels_mut() {
        let ray = s.generate_ray(x, y, w, h);
        if x == 0 {
            print!("Process: {}/{} ({}%)\r", y, h, y * 100 / h);
        }
        let color = trace_ray(&s, ray, 1);
        *pixel = Rgb(color.into());
    }

    println!("");
    film
}
