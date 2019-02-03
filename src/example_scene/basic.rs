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

  scene.add_white_light(V3([-5.0, 10.0, 0.0]), 0.4);
  scene.add_white_light(V3([2.0, 10.0, -10.0]), 0.4);

  let rect = Rectangle::new(
    V3([1.0, -1.0, 0.0]),
    V3([1.0, 1.0, 0.0]),
    V3([-1.0, 1.0, 0.0]),
  )
  .double_sided(true)
  .shaded(shader::simple_mirror(Color::Red))
  .transformed()
  // .rotated(V3([3.14 * 0.0, 0.0, 3.14 * 0.25]))
  .scaled(V3([2.0, 2.0, 2.0]))
  .translated(V3([0.0, -1.0, -4.3]));

  scene.add_object(rect);

  scene.add_object(ChessBoard {
    plane: Plane::new(V3([0.0, -1.6, 0.0]), V3([0.0, 1.0, 0.0])),
    ..ChessBoard::default()
  });

  scene
}
