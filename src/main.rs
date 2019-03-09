mod vec3;

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

    set_floor();

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
    let v2 = Vec3::normalized(1.0, 0.0, 0.000000000001);

    dbg!(v1 * v1);
    dbg!(v1 * v2);
    dbg!(v2 * v1);
    dbg!(v2 * v2);

    std::process::exit(0)
}

fn set_floor() {

}

fn set_sky() {

}

fn set_camera_parameters() {

}

fn set_camera_effects() {

}