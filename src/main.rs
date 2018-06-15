#![feature(exact_chunks)]
#![feature(try_from)]

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
    use object::{ChessBoard, Material, Sphere, Triangle};
    use scene::{Scene, SceneBuilder};
    use std::borrow::Cow;

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

        let spheres = vec![
            Sphere {
                c: V3([7.0, 0.0, -0.5]),
                r: 1.5,
                material: Material::Glass,
            },
            Sphere {
                c: V3([7.0, -3.5, 1.0]),
                r: 1.5,
                material: Material {
                    surface_color: Color::Black,
                    ..Material::Mirror
                },
            },
            Sphere {
                c: V3([7.0, 3.5, 1.5]),
                r: 1.5,
                material: Material::FrostedGlass,
            },
            Sphere {
                c: V3([12.0, -1.5, 3.5]),
                r: 1.5,
                material: Material::FrostedMirror,
            },
            Sphere {
                c: V3([12.0, 3.0, 2.5]),
                r: 1.5,
                material: Material {
                    surface_color: Color::Green,
                    ..Material::Solid
                },
            },
        ];

        for s in spheres.into_iter() {
            scene.add_object(s);
        }

        scene.add_object(ChessBoard {
            plane: Plane::new(V3([0.0, 0.0, -1.6]), V3([0.0, 0.0, 1.0])),
            ..ChessBoard::default()
        });

        scene.add_object(Triangle {
            trig: Trig(
                V3([3.0, 2.0, -1.6]),
                V3([5.0, 3.0, 1.0]),
                V3([5.0, 2.0, -1.6]),
            ),
            material: Cow::Owned(Material::PlaneGlass),
        });

        scene
    }
}

fn main() {
    let scene = example_scene::five_spheres();
    let img = tracer::transparency::trace(scene, 1200, 1200);
    img.save("./trace.png").ok();
}
