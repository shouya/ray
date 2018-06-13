extern crate image;
extern crate rand;

mod common;
mod object;
mod scene;
mod tracer;

use common::*;

fn main() {
    use common::*;
    use object::{Material, Sphere};
    use scene::Scene;

    let mut scene1 = Scene::new(
        V3::zero(),
        Plane::new(
            V3([2.0, 0.0, 0.0]), // r0
            V3([1.0, 0.0, 0.0]), // n
        ),
        2.0,
        2.0,
    );

    let colors = [
        color::Red,
        color::Green,
        color::Blue,
        color::White,
        color::Red,
    ];

    for i in 0..5 {
        scene1.add_object(Sphere {
            c: V3([7.0 + i as f32 * 2.0, i as f32 * 2.0, 0.0]),
            r: 1.5,
            material: Material {
                surface_color: colors[i],
                emission_color: Color([0.1, 0.1, 0.1]),
                reflexivity: 0.5,
                refractive_index: 0.9,
                specular_index: 0.00,
                transparency: 0.2,
            },
        });
    }
    // scene1.add_object(Sphere {
    //     c: V3([4.0, -2.0, 0.0]),
    //     r: 1.5,
    // });

    let img = tracer::scatter::trace(scene1, 400, 400);
    img.save("./trace.png").ok();
}
