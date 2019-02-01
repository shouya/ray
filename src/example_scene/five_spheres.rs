use common::*;
use obj_model::ObjModel;
use object::Transform;
use object::{ChessBoard, Rectangle, Shaded, Sphere, TrigMesh};
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

  let spheres = vec![
    Sphere {
      c: V3([-2.5, 1.0, -6.0]),
      r: 1.5,
    },
    Sphere {
      c: V3([3.5, 1.5, -7.0]),
      r: 1.5,
    },
    Sphere {
      c: V3([-1.5, 3.5, -12.0]),
      r: 1.5,
    },
    Sphere {
      c: V3([3.0, 2.5, -12.0]),
      r: 1.5,
    },
  ];

  for s in spheres.into_iter() {
    let color = Color::random();
    scene.add_object(Shaded::new(s, shader::simple_solid(color)));
  }

  scene.add_object(Shaded::new(
    Sphere {
      c: V3([0.04, -0.52, -4.0]),
      r: 1.5,
    },
    shader::simple_glass(Color::Red, 0.95),
  ));

  scene.add_object(Shaded::new(
    Rectangle::new(
      V3([2.0, 1.0, -5.0]),
      V3([2.0, -1.6, -5.0]),
      V3([2.1, -1.6, -3.0]),
    )
    .double_sided(true),
    shader::simple_mirror(Color([0.2; 3]))
    // shader::preset::Transparency::new(1.0.into(), 1.2.into()),
  ));

  let model = ObjModel::from_file("models/torus.obj");
  // let torus = TrigMesh::from_model(&model.unwrap(), Material::FrostedGlass);
  let torus = TrigMesh::from_model(&model.unwrap());
  scene.add_object(torus.translate(V3([0.0, 2.0, -5.0])));

  scene.add_object(ChessBoard {
    plane: Plane::new(V3([0.0, -1.6, 0.0]), V3([0.0, 1.0, 0.0])),
    ..ChessBoard::default()
  });

  scene
}
