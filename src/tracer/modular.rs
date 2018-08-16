use super::{dist, Color, ImageBuffer, Ray, Rgb, RgbImage, Scene};
use shader::Incidence;



fn trace_ray(s: &Scene, ray: Ray) -> Color {
  let hit = s.nearest_hit(&ray);
  if let None = hit {
    return s.ambient;
  }

  let (obj, hit) = hit.unwrap();
  let shader = obj.shader(hit.pos);
  let inci = Incidence {
    obj: obj.as_ref(),
    hit: &hit,
    ray: &ray
  };

  shader.render(s, &inci)
}


#[allow(dead_code)]
pub fn trace(s: Scene, w: u32, h: u32) -> RgbImage {
    let mut film = ImageBuffer::new(w, h);

    for (x, y, pixel) in film.enumerate_pixels_mut() {
        let ray = s.generate_ray(x, y, w, h);
        if x == 0 {
            print!("Process: {}/{} ({}%)\r", y, h, y * 100 / h);
        }
        let color = trace_ray(&s, ray);
        *pixel = Rgb(color.into());
    }

    println!("");
    film
}
