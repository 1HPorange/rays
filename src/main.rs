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
        center: Vec3(-1.0, 3.0, 5.0),
        radius: 3.0,
        material_provider: Box::new(StaticMaterialProvider(Material::opaque_reflective(RGBColor {
            r: 1.0,
            g: 0.5,
            b: 0.0
        }, 0.25, 1.0)))
    };

    let chrome_big = geometry::Sphere { 
        center: Vec3(4.0, 7.0, 14.0),
        radius: 7.0,
        material_provider: Box::new(StaticMaterialProvider(Material::opaque_reflective(RGBColor {
            r: 0.0,
            g: 0.5,
            b: 1.0
        }, 0.8, 1.0)))
    };

    let purple = geometry::Sphere { 
        center: Vec3(-6.0, 4.0, 10.0),
        radius: 4.0,
        material_provider: Box::new(StaticMaterialProvider(Material::opaque_reflective(RGBColor {
            r: 0.5,
            g: 0.0,
            b: 1.0
        }, 0.5, 2.0)))
    };

    let scene = scene::Scene {
        objects: vec![Box::new(orange), Box::new(chrome_big), Box::new(purple)]
    };

    let camera: camera::Camera<f64> = camera::Camera::default();

    let mut render_target = output::RenderTarget::new(1280, 720);

    let render_params = raytracing::RenderingParameters { 
        min_intensity: 0.05, 
        max_bounces: 2, 
        max_reflect_rays: 20,
        max_refract_rays: 2,
        max_dof_rays: 20
    };

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