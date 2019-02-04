use common::*;
use obj_model::ObjModel;
use object::{ChessBoard, Rectangle, Shaded, Sphere, TrigMesh};
use object::{Object, Transform};
use scene::{Scene, SceneBuilder};
use shader;

pub fn scene() -> Scene {
    let mut scene = SceneBuilder::default()
        .vp_plane(Plane::new(
            V3([0.0, 0.0, -2.0]), // r0
            V3([0.0, 0.0, -1.0]), // n
        ))
        .vp_width(2.0)
        .vp_height(2.0)
        .camera(V3::zero())
        .projection(Projection::Perspective)
        .ambient(Color::White * 0.8)
        .max_depth(15)
        .build()
        .unwrap();

    // scene.add_white_light(V3([-5.0, 10.0, 0.0]), 0.4);
    scene.add_white_light(V3([2.0, 10.0, -10.0]), 0.4);
    // scene.add_object(
    //   Sphere {
    //     c: V3([2.0, 11.0, -11.0]),
    //     r: 0.5,
    //   }
    // );

    scene.add_object(
        Sphere {
            c: V3([0.04 - 3.5, -0.32, -6.0]),
            r: 1.5,
        }
        .shaded(shader::Diffuse::new(Color::Red.into()).into())
        .shaded(shader::simple_mirror(Color::Blue))
        .shaded(shader::Normal.into())
    );

    // scene.add_object(
    //   Sphere {
    //     c: V3([0.04, -0.32, -6.0]),
    //     r: 1.5,
    //   }
    //   .shaded(shader::simple_solid(Color::Blue)),
    // );

    scene.add_object(
        Sphere {
            c: V3([0.0, 0.0, 0.0]),
            r: 0.5,
        }
        .shaded(shader::simple_mirror(Color::Red))
        .shaded(shader::Normal.into())
        //.shaded(shader::Diffuse::new(Color::Red.into()).into())
        .transformed()
        .rotated(V3([1.5, 1.2, 1.0]))
        // .rotated(V3([0.0, 0.0, 3.14*0.5]))
        .scaled(V3([2.5, 3.5, 2.5]))
        .translated(V3([0.04, -0.32, -6.0])),
    );

    scene.add_object(
        Rectangle::new(
            V3([6.0, -3.0, -8.0]),
            V3([6.0, 5.0, -8.0]),
            V3([-0.0, 5.0, -12.0]),
        )
        .double_sided(true)
        .shaded(shader::simple_mirror(Color([0.1, 0.8, 0.3]))),
    );

    scene.add_object(ChessBoard {
        plane: Plane::new(V3([0.0, -1.6, 0.0]), V3([0.0, 1.0, 0.0])),
        ..ChessBoard::default()
    });

    scene
}
