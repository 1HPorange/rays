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

    let scene = create_scene();

    let camera = create_camera();

    let mut render_target = create_render_target();

    let render_params = create_render_parameters();

    render(&scene, &camera, &mut render_target, &render_params);

    save_to_file(&render_target);
}

// Customize scene and camera setup inside the functions below
//////////////////////////////////////////////////////////////

fn create_scene() -> Scene<f64> { // Change into f32 if you want to use single precision

    // Materials

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

    let mat_black = Material::pure(RGBColor::BLACK);

    let mat_very_reflective = Material::opaque_reflective(
        RGBColor::WHITE,
        ReflectionParams::new(0.75, 1.0, 4.0, 0.0));

    let mat_glass = Material::new(
        RGBColor::new(1.0, 1.0, 1.0),
        Opacity::new(0.1, 1.0, 2.0),
        ReflectionParams::new(1.0, 1.0, 1.0, 0.0),
        RefractionParams::new(1.33, 0.0));

    // UV Mappers

    let alan_mapper = TextureUvMapper::from_png_24("D:/Downloads/alan.png", mat_black, SamplingMethod::BILINEAR)
        .expect("Could not load Alan. Won't work without him!");

    // Objects

    let back_right = Sphere::new(
        Vec3(4.0, 7.0, 14.0), 7.0, 
        Box::new(StaticUvMapper(mat_very_reflective)), 
        Vec3Norm::up(), 
        Vec3Norm::right());

    let floor = InifinitePlane::new(
        Vec3(0.0,0.0,0.0), 
        Vec3Norm::up(), 
        Vec3Norm::right(),
        Box::new(CheckerboardUvMapper(mat_black, mat_white_reflect)), 
        0.1);

    // Sky Sphere

    let sky_sphere = Sphere::new(
        Vec3::zero(), 
        1000.0, 
        Box::new(alan_mapper), 
        Vec3Norm::up(), 
        Vec3Norm::right());

    // Scene

    let mut scene = Scene::new(RGBColor::WHITE);

    //scene.add_object(sky_sphere);

    scene.add_object(back_right);
    scene.add_object(floor);

    scene
}

fn create_camera<T>() -> Camera<T> where T: num_traits::Float {

    let camera = Camera::default();

    camera
}

fn create_render_target() -> RenderTarget {
    RenderTarget::new(1280/2, 720/2)
}

fn create_render_parameters<T>() -> RenderingParameters<T> where T: num_traits::Float {

    // TODO: Move into new function
    RenderingParameters { 
        min_intensity: T::zero(), 
        max_bounces: 2, 
        max_reflect_rays: 2,
        max_refract_rays: 2,
        max_dof_rays: 20,
        ao_strength: T::from(0.8).unwrap(),
        ao_distance: T::from(2.0).unwrap(),
        ao_rays: 3,
        float_correction_bias: T::from(0.001).unwrap()
    }
}

fn save_to_file(render_target: &RenderTarget) {

    render_target.save_as_ppm("D:/Downloads/weirdo.ppm")
        .expect("Could not write to output file");

}