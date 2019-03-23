extern crate rays;

#[allow(dead_code)]
mod example1;

#[allow(dead_code)]
mod example2;

use rays::prelude::*;
use std::time::Instant;

fn main() {

    let scene = example1::create_scene();

    let camera = example1::create_camera();

    let mut render_target = RenderTarget::new(1280, 720);

    let render_params = example1::create_render_parameters();

    let before = Instant::now();

    rays::render(&scene, &camera, &mut render_target, &render_params);

    let elapsed = before.elapsed();
    println!("Finished in {}.{} s", elapsed.as_secs(), elapsed.subsec_millis());

    render_target.save_as_png("D:/Downloads/weirdo.png")
        .expect("Could not write to output file");
}