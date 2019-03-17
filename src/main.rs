mod vec3;
mod raytracing;
mod geometry;
mod color;
mod output;
mod camera;
mod scene;
mod material;

use scene::*;
use camera::*;
use vec3::*;
use material::*;
use color::*;
use raytracing::*;
use geometry::*;
use output::*;

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
        Transparency::new(0.05, 1.0, 2.0),
        ReflectionParams::new(0.0, 1.0, 5.0, 5.0),
        RefractionParams::new(1.33, 2.5));

    let mat_blue_reflect = Material::opaque_reflective(
        RGBColor::new(0.0, 0.1, 0.2), 
        ReflectionParams::new(0.025, 1.0, 4.0, 0.0));

    let mat_red_diffuse = Material::opaque_reflective(
        RGBColor::new(1.0, 0.0, 0.0), 
        ReflectionParams::new(0.65, 0.9, 2.0, 90.0));

    let front = Sphere::new(Vec3(-0.5, 3.0, 5.0), 3.0, Box::new(StaticMaterialProvider(mat_refract)));

    let back_right = Sphere::new(Vec3(4.0, 7.0, 14.0), 7.0, Box::new(StaticMaterialProvider(mat_blue_reflect)));

    let back_left_lower = Sphere::new(Vec3(-6.0, 4.0, 10.0), 4.0, Box::new(StaticMaterialProvider(mat_red_diffuse)));

    let back_left_upper = Sphere::new(Vec3(-9.5, 8.0, 10.0), 4.0, Box::new(StaticMaterialProvider(mat_red_diffuse)));

    vec![Box::new(front), Box::new(back_right), Box::new(back_left_lower), Box::new(back_left_upper)]
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
        max_bounces: 3, 
        max_reflect_rays: 4,
        max_refract_rays: 4,
        max_dof_rays: 30,
        ao_strength: 0.8,
        ao_distance: T::from(3.0).unwrap(),
        ao_rays: 4,
        float_correction_bias: T::from(0.001).unwrap()
    }
}

fn save_to_file(render_target: &RenderTarget) {

    render_target.save_as_ppm("D:/Downloads/weirdo.ppm")
        .expect("Could not write to output file");

}