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

    let mat_white_reflect = Material::opaque_reflective(
        RGBColor::WHITE, 
        ReflectionParams::new(0.75, 0.75, 1.0, 10.0));

    let mat_black = Material::opaque_reflective(
        RGBColor::BLACK, 
        ReflectionParams::new(0.7, 0.7, 1.0, 90.0));

    let mat_very_reflective = Material::opaque_reflective(
        RGBColor::WHITE,
        ReflectionParams::new(0.75, 1.0, 4.0, 0.0));

    let mat_glass = Material::new(
        RGBColor::WHITE, 
        Opacity::new(0.05, 1.0, 2.0), 
        ReflectionParams::new(1.0, 1.0, 1.0, 0.0), 
        RefractionParams::new(1.33, 0.0));

    let mat_refract_blurry = Material::new(
        RGBColor::WHITE, 
        Opacity::new(0.1, 0.75, 3.0), 
        ReflectionParams::new(1.0, 1.0, 1.0, 0.0), 
        RefractionParams::new(1.0, 10.0));

    let mat_coloured_diffuse = Material::opaque_reflective(
        RGBColor::new(0.0, 0.0, 0.3), 
        ReflectionParams::new(0.6, 1.0, 2.0, 90.0));

    let mat_marble = Material::opaque_reflective(
        RGBColor::PINK, // will be overwritten by uv mapper
        ReflectionParams::new(0.15, 1.0, 2.0, 0.0));

    // UV Mappers

    let marble_mapper = TextureUvMapper::from_png_24(
        "D:/Downloads/skysphere3.png", 
        mat_marble,
        SamplingMethod::BILINEAR)
        .expect("Could not load marble texture!");

    // Objects

    let back_left = Sphere::new(
        Vec3(-8.0, 7.0, 20.0), 7.0, 
        Box::new(StaticUvMapper(mat_glass)), 
        Vec3Norm::up(), 
        Vec3Norm::right());

    let back_right = Sphere::new(
        Vec3(8.0, 7.0, 20.0), 7.0, 
        Box::new(StaticUvMapper(mat_very_reflective)), 
        Vec3Norm::up(), 
        Vec3Norm::right());

    let front_left = Sphere::new(
        Vec3(-12.0, 4.0, 7.5), 
        4.0, 
        Box::new(marble_mapper), 
        Vec3Norm::up(), 
        Vec3Norm::right());

    let front_center = Sphere::new(
        Vec3(0.0, 4.5, 5.0), 
        4.5, 
        Box::new(StaticUvMapper(mat_coloured_diffuse)), 
        Vec3Norm::up(), 
        Vec3Norm::right());

    let front_right = Sphere::new(
        Vec3(12.0, 4.0, 7.5), 
        4.0, 
        Box::new(StaticUvMapper(mat_refract_blurry)), 
        Vec3Norm::up(), 
        Vec3Norm::right());

    // Scenery

    let skysphere_mapper = TextureUvMapper::from_png_24(
        "D:/Downloads/skysphere4.png", 
        Material::pure(RGBColor::BLACK), 
        SamplingMethod::BILINEAR)
        .expect("Could not load sky texture!");

    let sky_sphere = Sphere::new(
        Vec3::zero(), 
        1000.0, 
        Box::new(skysphere_mapper), 
        Vec3Norm::up(), 
        Vec3Norm::right());

    let floor = InifinitePlane::new(
        Vec3(0.0,0.0,0.0), 
        Vec3Norm::up(), 
        Vec3Norm::right(),
        Box::new(CheckerboardUvMapper(mat_black, mat_white_reflect)), 
        0.1);

    // Scene

    let mut scene = Scene::new(RGBColor::WHITE);

    scene.add_object(floor);
    scene.add_object(sky_sphere);

    scene.add_object(back_left);
    scene.add_object(back_right);
    scene.add_object(front_left);
    scene.add_object(front_center);
    scene.add_object(front_right);

    scene
}

fn create_camera<T>() -> Camera<T> where T: num_traits::Float {

    let camera = Camera::default();

    camera
}

fn create_render_target() -> RenderTarget {
    RenderTarget::new(1920, 1080)
}

fn create_render_parameters<T>() -> RenderingParameters<T> where T: num_traits::Float {

    // TODO: Move into new function
    RenderingParameters { 
        min_intensity: T::from(0.01).unwrap(), 
        max_bounces: 3, 
        max_reflect_rays: 5,
        max_refract_rays: 1,
        max_dof_rays: 30,
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