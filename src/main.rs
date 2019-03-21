mod vec3;
mod raytracing;
mod geometry;
mod color;
mod output;
mod camera;
mod scene;
mod material;
mod texture_uv_mapper;

use scene::*;
use camera::*;
use vec3::*;
use material::*;
use color::*;
use raytracing::*;
use geometry::*;
use output::*;
use texture_uv_mapper::*;

fn main() {

    let scene = Scene {
        objects: create_geometry(),
        // Sky
    };

    let camera = create_camera();

    let mut render_target = create_render_target();

    let render_params = create_render_parameters();

    render(&scene, &camera, &mut render_target, &render_params);

    save_to_file(&render_target);
}

// Customize scene and camera setup inside the functions below
//////////////////////////////////////////////////////////////

fn create_geometry() -> Vec<Box<SceneObject<f64>>> {

    let mat_refract = Material::new(
        RGBColor::new(1.0, 0.9, 1.0),
        Opacity::new(0.05, 0.6, 2.0),
        ReflectionParams::new(0.0, 1.0, 5.0, 5.0),
        RefractionParams::new(1.33, 2.5));

    let mat_blue_reflect = Material::opaque_reflective(
        RGBColor::new(0.0, 0.1, 0.2), 
        ReflectionParams::new(0.025, 1.0, 4.0, 0.0));

    let mat_red_diffuse = Material::opaque_reflective(
        RGBColor::new(1.0, 0.0, 0.0), 
        ReflectionParams::new(0.35, 0.6, 2.0, 90.0));

    let mat_white_reflect = Material::opaque_reflective(
        RGBColor::WHITE, 
        ReflectionParams::new(0.75, 0.75, 1.0, 10.0));

    let mat_black = Material::opaque_reflective(
        RGBColor::BLACK, 
        ReflectionParams::new(0.25, 0.25, 1.0, 10.0));

    let mat_very_reflective = Material::opaque_reflective(
        RGBColor::WHITE,
        ReflectionParams::new(0.75, 1.0, 4.0, 0.0));

    let mat_glass = Material::new(
        RGBColor::new(1.0, 1.0, 1.0),
        Opacity::new(0.1, 1.0, 2.0),
        ReflectionParams::new(1.0, 1.0, 1.0, 0.0),
        RefractionParams::new(1.33, 0.0));

    let alan_mapper = TextureUvMapper::from_png_24("D:/Downloads/alan.png", mat_black)
        .expect("Could not load Alan. Won't work without him!");

    let front = Sphere::new(Vec3(-0.5, 3.0, 5.0), 3.0, Box::new(StaticUvMapper(mat_refract)));

    let back_right = Sphere::new(Vec3(4.0, 7.0, 14.0), 7.0, Box::new(StaticUvMapper(mat_very_reflective)));

    let back_left_lower = Sphere::new(Vec3(-6.0, 4.0, 10.0), 4.0, Box::new(StaticUvMapper(mat_red_diffuse)));

    let back_left_upper = Sphere::new(Vec3(-9.5, 8.0, 10.0), 6.0, Box::new(StaticUvMapper(mat_glass)));

    let floor = InifinitePlane::with_random_right(
        Vec3(0.0,0.0,0.0), 
        Vec3::normalized(0.0, 1.0, 0.0), 
        Box::new(alan_mapper), 
        0.1);

    vec![Box::new(front), Box::new(back_right), Box::new(back_left_lower), Box::new(back_left_upper), Box::new(floor)]
}

fn create_camera<T>() -> Camera<T> where T: num_traits::Float {

    let camera = Camera::default();

    camera
}

fn create_render_target() -> RenderTarget {
    RenderTarget::new(1280, 720)
}

fn create_render_parameters<T>() -> RenderingParameters<T> where T: num_traits::Float {

    RenderingParameters { 
        min_intensity: 0.0, 
        max_bounces: 2, 
        max_reflect_rays: 2,
        max_refract_rays: 2,
        max_dof_rays: 20,
        ao_strength: 0.8,
        ao_distance: T::from(2.0).unwrap(),
        ao_rays: 3,
        float_correction_bias: T::from(0.001).unwrap()
    }
}

fn save_to_file(render_target: &RenderTarget) {

    render_target.save_as_ppm("D:/Downloads/weirdo.ppm")
        .expect("Could not write to output file");

}