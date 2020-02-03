use crate::common::*;
use crate::obj_model::ObjModel;
use crate::object::{ChessBoard, Rectangle, TrigMesh};
use crate::object::{Object};
use crate::scene::{Scene, SceneBuilder};
use crate::shader;

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

    scene.add_white_light(V3([-5.0, 10.0, 0.0]), 0.4);
    scene.add_white_light(V3([10.0, 10.0, -10.0]), 0.4);

    scene.add_object(
        Rectangle::new(
            V3([2.0, 1.0, -5.0]),
            V3([2.0, -1.6, -5.0]),
            V3([2.1, -1.6, -3.0]),
        )
        .double_sided(true)
        .shaded(shader::simple_mirror(Color([0.2; 3]))),
    );

    let model = ObjModel::from_file("models/torus.obj");
    let torus = TrigMesh::from_model(&model.unwrap());
    scene.add_object(
        torus
            .shaded(shader::simple_mirror(Color::Blue))
            .transformed()
            .rotated(V3([3.14 * 0.6, 3.14 * 0.1, 3.14 * 0.5]))
            .translated(V3([0.04, -0.52, -4.0])),
    );

    scene.add_object(ChessBoard {
        plane: Plane::new(V3([0.0, -1.6, 0.0]), V3([0.0, 1.0, 0.0])),
        ..ChessBoard::default()
    });

    scene
}
