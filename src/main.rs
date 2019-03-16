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

    let scene = create_scene_desc();

    let camera = create_camera();

    let mut render_target = create_render_target();

    let render_params = create_render_parameters();

    render(&scene, &camera, &mut render_target, &render_params);

    save_to_file(&render_target);
}

// Abstract program flow. Usually you won't have to touch this

fn create_scene_desc() -> Scene<f64> {

    let scene = Scene {
        objects: create_geometry()
    };

    //set_sky(&mut scene);

    scene
}

// Customize scene and camera setup inside the functions below
//////////////////////////////////////////////////////////////

fn create_geometry() -> Vec<Box<SceneObject<f64>>> {

    let orange = Sphere { 
        center: Vec3(-1.0, 4.5, 5.0),
        radius: 4.5,
        material_provider: Box::new(StaticMaterialProvider(Material {
            color: RGBColor {
                r: 1.0,
                g: 0.9,
                b: 1.0
            }, 
            transparency: Transparency {
                opacity_center: 0.05,
                opacity_edges: 1.0,
                edge_effect_power: 2.0
            },
            reflection: ReflectionParams {
                intensity_center: 0.0,
                intensity_edges: 1.0,
                edge_effect_power: 5.0,
                max_angle: 5.0
            },
            refraction: RefractionParams {
                index_of_refraction: 1.33,
                max_angle: 2.5
            }}))
    };

    let blue = Sphere { 
        center: Vec3(4.0, 7.0, 14.0),
        radius: 7.0,
        material_provider: Box::new(StaticMaterialProvider(Material::opaque_reflective(
            RGBColor {
                r: 0.0,
                g: 0.1,
                b: 0.2
            }, 
            ReflectionParams {
                intensity_center: 0.025,
                intensity_edges: 1.0,
                edge_effect_power: 4.0,
                max_angle: 0.0
            })))
    };

    let red = Sphere { 
        center: Vec3(-6.0, 4.0, 10.0),
        radius: 4.0,
        material_provider: Box::new(StaticMaterialProvider(Material::opaque_reflective(
            RGBColor {
                r: 1.0,
                g: 0.0,
                b: 0.0,
            }, 
            ReflectionParams {
                intensity_center: 0.65,
                intensity_edges: 0.9,
                edge_effect_power: 2.0,
                max_angle: 90.0
            })))
    };

    vec![Box::new(orange), Box::new(blue), Box::new(red)]
}

fn create_camera<T>() -> Camera<T> where T: num_traits::Float {

    let camera = Camera::default();

    camera
}

fn create_render_target() -> RenderTarget {
    RenderTarget::new(1280/2, 720/2)
}

fn create_render_parameters<T>() -> RenderingParameters<T> where T: num_traits::Float {

    RenderingParameters { 
        min_intensity: 0.0, 
        max_bounces: 3, 
        max_reflect_rays: 5,
        max_refract_rays: 5,
        max_dof_rays: 60,
        float_correction_bias: T::from(0.001).unwrap()
    }
}

fn save_to_file(render_target: &RenderTarget) {

    render_target.save_as_ppm("D:/Downloads/weirdo.ppm")
        .expect("Could not write to output file");

}