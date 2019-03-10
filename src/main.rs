mod vec3;
mod raytracing;
mod geometry;
mod color;
mod output;
mod camera;
mod scene;
mod material;

use vec3::*;
use material::*;
use color::*;

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

    let orange = geometry::Sphere { 
        center: Vec3(0.0,0.0,5.0),
        radius: 3.0,
        material_provider: Box::new(StaticMaterialProvider(Material::perfect_diffuse(RGBColor {
            r: 1.0,
            g: 0.5,
            b: 0.0
        })))
    };

    let turqoise = geometry::Sphere { 
        center: Vec3(4.0,0.0,8.0),
        radius: 2.0,
        material_provider: Box::new(StaticMaterialProvider(Material::perfect_diffuse(RGBColor {
            r: 0.0,
            g: 1.0,
            b: 0.5
        })))
    };

    let purple = geometry::Sphere { 
        center: Vec3(-3.0,0.0,7.0),
        radius: 3.0,
        material_provider: Box::new(StaticMaterialProvider(Material::perfect_diffuse(RGBColor {
            r: 1.0,
            g: 0.0,
            b: 0.5
        })))
    };

    let scene = scene::Scene {
        objects: vec![Box::new(orange), Box::new(turqoise), Box::new(purple)]
    };

    let camera: camera::Camera<f64> = camera::Camera::default();

    let mut render_target = output::RenderTarget::new(60 * 16, 60 * 9);

    let render_params = raytracing::RenderingParameters { max_bounces: 2, max_rays: 10 };

    raytracing::render::<f64>(&scene, &camera, &mut render_target, &render_params);

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