mod vec3;
mod raytracing;
mod geometry;
mod color;
mod output;
mod camera;
mod scene;

use vec3::*;

fn main() {

    create_scene_desc();

    create_camera_setup();

    // render();

    save_to_file();
}

// Abstract program flow. Usually you won't have to touch this

fn create_scene_desc() {

    add_geometry();

    set_sky();;
}

fn create_camera_setup() {

    set_camera_parameters();

    set_camera_effects();
}

fn save_to_file() {
    unimplemented!()
}

// Customize scene and camera setup inside the functions below
//////////////////////////////////////////////////////////////

fn add_geometry() {

    let sphere = geometry::Sphere { 
        center: Vec3(0.0,0.0,5.0),
        radius: 3.0
    };

    let scene = scene::Scene {
        geometry: vec![Box::new(sphere)]
    };

    let camera: camera::Camera<f64> = camera::Camera::default();

    let mut render_target = output::RenderTarget::new(60 * 16, 60 * 9);

    raytracing::render(&scene, &camera, &mut render_target);

    render_target.save_as_ppm("D:/Downloads/weirdo.ppm")
    .expect("Could not write to output file");

    std::process::exit(0)
}

fn set_sky() {

}

fn set_camera_parameters() {

}

fn set_camera_effects() {

}