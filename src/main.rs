mod vec3;
mod raytracing;
mod geometry;
mod color;
mod output;
mod camera;

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

    let v1 = Vec3(2.0, 3.0, 4.0);
    let v2 = Vec3::normalized(1.0, 0.0, 0.0);

    dbg!(v1.normalize() / 2.0);

    std::process::exit(0)
}

fn set_sky() {

}

fn set_camera_parameters() {

}

fn set_camera_effects() {

}