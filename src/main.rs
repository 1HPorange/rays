extern crate rays;

#[allow(dead_code)]
mod example1;

#[allow(dead_code)]
mod example2;

use rays::prelude::*;
use std::time::Instant;

fn main() {

    let config = rays::parse("example1.toml")
        .expect("Could not read config"); // TODO: Detailed error

    let scene = example2::create_scene();

    let camera = example2::create_camera();

    let mut render_target = RenderTarget::new(1920/2, 1080/2);

    let render_params = example2::create_render_parameters();

    let before = Instant::now();

    rays::render(&scene, &camera, &mut render_target, &render_params);

    let elapsed = before.elapsed();
    println!("Finished in {}.{} s", elapsed.as_secs(), elapsed.subsec_millis());

    render_target.save_as_png("D:/Downloads/weirdo.png")
        .expect("Could not write to output file");
}