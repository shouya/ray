extern crate image;
extern crate rand;

#[macro_use]
extern crate derive_builder;

mod common;
mod object;
mod scene;
mod tracer;

mod example_scene {
    use common::*;
    use object::{ChessBoard, Material, Sphere};
    use scene::{Scene, SceneBuilder};

    pub fn five_spheres() -> Scene {
        let mut scene = SceneBuilder::default()
            .vp_plane(Plane::new(
                V3([2.0, 0.0, 0.0]), // r0
                V3([1.0, 0.0, 0.0]), // n
            ))
            .vp_width(2.0)
            .vp_height(2.0)
            .camera(V3::zero())
            .projection(Projection::Perspective)
            .ambient(Color::White * 0.8)
            .build()
            .unwrap();

        scene.add_light(V3([1.0, -5.0, 10.0]), 0.7);

        let colors = [
            Color::Red,
            Color::Green,
            Color::Blue,
            Color::White,
            Color::Black,
        ];

        for i in 0..5 {
            let x = 7.0 + i as f32 * 3.2;
            let y = -5.0 + i as f32 * 0.3 + (i * i) as f32 * 1.0;

            scene.add_object(Sphere {
                c: V3([x, y, -1.0 + 0.5 * i as f32]),
                r: 1.5,
                material: Material {
                    surface_color: colors[i],
                    emission_color: Color([0.1, 0.1, 0.1]),
                    reflexivity: 0.0,
                    ior: 1.1,
                    specular_index: 0.01,
                    transparency: 0.9,
                },
            });
        }

        scene.add_object(ChessBoard {
            plane: Plane::new(V3([0.0, 0.0, -1.6]), V3([0.0, 0.0, -1.0])),
            ..ChessBoard::default()
        });

        scene
    }
}

fn main() {
    let scene = example_scene::five_spheres();
    let img = tracer::transparency::trace(scene, 1200, 1200);
    img.save("./trace.png").ok();
}
