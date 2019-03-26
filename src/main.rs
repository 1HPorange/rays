extern crate rays;

#[allow(dead_code)]
mod example1;

#[allow(dead_code)]
mod example2;

use rays::prelude::*;
use std::time::Instant;

fn main() {

    let config = rays::parse("example1.toml")
        .expect("Config contains some errors. Please fix and try again.");

    // let scene = example1::create_scene();

    // let camera = example1::create_camera();

    let mut render_target = RenderTarget::new(1920/2, 1080/2);

    // let render_params = example1::create_render_parameters();

    // let before = Instant::now();

    // rays::render(&scene, &camera, &mut render_target, &render_params);

    // let elapsed = before.elapsed();
    // println!("Finished in {}.{} s", elapsed.as_secs(), elapsed.subsec_millis());

    let camera = match config.camera_config {
        CameraConfig::Single(c) => c,
        CameraConfig::Multiple(_) => unreachable!()
    };

    let render_params = match config.render_params_config {
        RenderParamsConfig::Single(rp) => rp,
        RenderParamsConfig::Multiple(_) => unreachable!()
    };

    let before = Instant::now();

    rays::render(&config.scene, &camera, &mut render_target, &render_params);

    let elapsed = before.elapsed();
    println!("Finished in {}.{} s", elapsed.as_secs(), elapsed.subsec_millis());

    render_target.save_as_png("D:/Downloads/weirdo.png")
        .expect("Could not write to output file");
}