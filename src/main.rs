// #![feature(exact_chunks)]
// #![feature(try_from)]
// #![feature(plugin, custom_attribute)]
// #![plugin(flamer)]

extern crate image;
extern crate rand;

// extern crate flame;

#[macro_use]
extern crate derive_builder;

mod common;
mod obj_model;
mod object;
mod scene;
mod shader;
mod tracer;
mod example_scene;

fn main() {
  let scene = example_scene::transformed::scene();
  let img = tracer::modular::trace(scene, 800, 800);
  img.save("./trace.png").ok();
}
