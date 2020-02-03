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
mod example_scene;
mod obj_model;
mod object;
mod scene;
mod shader;
mod tracer;

fn main() {
    let scene = example_scene::transformed::scene();
    let conf = tracer::RenderConfigBuilder::default()
        .aa(Some(tracer::AAPattern::ssaa4x()))
        // .w(1200)
        // .h(1200)
        .w(1024)
        .h(1024)
        .build()
        .unwrap();
    let img = tracer::modular::trace(scene, conf);
    img.save("./trace.png").unwrap();
}
