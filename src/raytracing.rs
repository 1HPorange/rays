extern crate rand;
use rand::prelude::*;
use rand::FromEntropy;

use super::vec3::*;
use super::camera::*;
use super::output::*;
use super::scene::*;
use super::color::*;
use super::material::*;

use std::cmp;

pub struct Ray<T> {
    pub origin: Vec3<T>,
    pub direction: Vec3Norm<T>
}

pub trait RayTarget<T> {

    fn test_intersection(&self, ray: &Ray<T>) -> Option<GeometryHitInfo<T>>;

}

pub struct GeometryHitInfo<T> {

    pub position: Vec3<T>,
    pub normal: Vec3Norm<T> // TODO: Make the calculation of this optional or think of something else. Sometimes we just need a hit test

}

pub struct RenderingParameters {
    pub min_intensity: f32,
    pub max_bounces: i32,
    pub max_reflect_rays: i32,
    pub max_refract_rays: i32,
    pub max_dof_rays: i32
}

// Convenience structs so we don't need to pass around so much stuff
struct RaytraceParameters<'a, T> {
    scene: &'a Scene<T>,
    render_params: &'a RenderingParameters,
}

struct HitInfo<'a, T> {
    mat: &'a Material<T>,
    hit: &'a GeometryHitInfo<T>, 
    ray: &'a Ray<T>, 
    bounces: i32, 
    intensity: f32
}

pub fn render<T>(scene: &Scene<T>, camera: &Camera<T>, render_target: &mut RenderTarget, render_params: &RenderingParameters) where 
    T: num_traits::Float {

    let raytrace_params = RaytraceParameters {
        scene,
        render_params
    };

    let mut rng = SmallRng::from_entropy();

    // Some reusable stuff
    let w = T::from(render_target.width).unwrap();
    let h = T::from(render_target.height).unwrap();
    let const2 = T::from(2.0).unwrap();

    // Distances between 2 pixels
    let x_step = camera.viewport.width / w;
    let x_start = (x_step - camera.viewport.width) / const2;

    let y_step = -camera.viewport.height / h;
    let y_start = (camera.viewport.height - y_step) / const2;

    // Angle distances between two pixels
    let fov_vertical = camera.fov_horizontal / camera.viewport.aspect();

    let x_angle_step = camera.fov_horizontal / w;
    let x_angle_start = (x_angle_step - camera.fov_horizontal) / const2;

    let y_angle_step = fov_vertical / h;
    let y_angle_start = (y_angle_step - fov_vertical) / const2;

    for y_ind in 0..render_target.height {

        let y_t = T::from(y_ind).unwrap();
        let vp_y = y_start + y_t * y_step;
        let angle_y = y_angle_start + y_t * y_angle_step;

        for x_ind in 0..render_target.width {

            let x_t = T::from(x_ind).unwrap();
            let vp_x = x_start + x_t * x_step;
            let angle_x = x_angle_start + x_t * x_angle_step;
            
            let origin = get_initial_ray_origin(camera, vp_x, vp_y);

            // We render just a single ray if DoF is disabled
            let color = if camera.dof_angle.is_zero() {

                let direction = get_initial_randomized_ray_direction(camera, &mut rng, angle_x, angle_y);

                raytrace_recursive(
                    &raytrace_params,
                    &mut rng,
                    Ray { origin, direction }, 
                    0, 1.0)

            } else {

                let mut color = RGBColor::BLACK;

                let ray_influence = 1.0 / (render_params.max_dof_rays as f32);

                for _ in 0..render_params.max_dof_rays {

                    let direction = get_initial_randomized_ray_direction(camera, &mut rng, angle_x, angle_y);

                    color += raytrace_recursive(
                        &raytrace_params,
                        &mut rng,
                        Ray { origin, direction }, 
                        0, 1.0)
                    * ray_influence;

                }

                color
            };
            
            render_target.set_pixel(x_ind, y_ind, color);
        }
    }
}

fn get_initial_ray_origin<T>(camera: &Camera<T>, viewport_x: T, viewport_y: T) -> Vec3<T> where T: num_traits::Float {

    let mut origin = Vec3(viewport_x, viewport_y, T::zero());
    origin.rotate_x(camera.orientation.x);
    origin.rotate_y(camera.orientation.y);
    origin.rotate_z(camera.orientation.z);

    origin += camera.position;

    origin
}

fn get_initial_randomized_ray_direction<T, R>(camera: &Camera<T>, rng: &mut R, fov_angle_x: T, fov_angle_y: T) -> Vec3Norm<T> where 
    T: num_traits::Float,
    R: Rng + ?Sized {

    let mut direction = Vec3(T::zero(), T::zero(), T::one());

    // Randomization for DoF
    if !camera.dof_angle.is_zero() {

        let dof_rx: T = T::from(rng.gen::<f64>()).unwrap() * camera.dof_angle;
        let dof_rz: T = T::from(rng.gen::<f64>() * 360.0).unwrap();
        direction.rotate_x(dof_rx);
        direction.rotate_z(dof_rz);

    }

    // Fov Influence
    direction.rotate_y(fov_angle_x);
    direction.rotate_x(fov_angle_y);

    // Camera orientation influence
    direction.rotate_x(camera.orientation.x);
    direction.rotate_y(camera.orientation.y);
    direction.rotate_z(camera.orientation.z);

    direction.into_normalized()
}

fn raytrace_recursive<T,R>(params: &RaytraceParameters<T>, rng: &mut R, ray: Ray<T>, bounces: i32, intensity: f32) -> RGBColor where 
    Vec3<T>: Vec3View<T> + std::ops::Sub<Output=Vec3<T>>,
    T: num_traits::Float,
    R: Rng + ?Sized {

    let potential_hits = params.scene.objects.iter()
        .map(|obj| (obj, obj.test_intersection(&ray)))
        .filter(|(_, rch)| rch.is_some())
        .map(|(obj, rch)| (obj, rch.unwrap()));

    let closest_hit = potential_hits
        .min_by(|a,b| hit_dist_comp(&ray, &a.1, &b.1));

    if let Some((obj, hit)) = closest_hit {

        let mat_provider = obj.get_material_provider();

        let mat = mat_provider.get_material_at(&hit);

        let hit_info = HitInfo {
            mat,
            hit: &hit,
            ray: &ray,
            bounces,
            intensity
        };

        hit_object(params, rng, &hit_info)
    } else {
        hit_skybox(&ray)
    }
}

/////////////// Raytracing Intensity Calculation ///////////////
/// 
///                         intensity
///                       /           \
///                 alpha              1-alpha
///               /       \           /       \
///             refl.   1-refl.     1-refr.    refr.
///             int.      int.      int.       int.
///            /           \        /            \
///    total refl. int.   mat color int.    total refr. int.
/// 
////////////////////////////////////////////////////////////////

fn hit_object<T,R>(params: &RaytraceParameters<T>, rng: &mut R, hit_info: &HitInfo<T>) -> RGBColor where 
    T: num_traits::Float, 
    R: Rng + ?Sized {
    
    // Calculate intensities of color, reflection and refraction
    let mat_color_intensity = hit_info.intensity * (
        (hit_info.mat.color.a * (1.0 - hit_info.mat.reflection.intensity)) + 
        ((1.0 - hit_info.mat.color.a) * (1.0 - hit_info.mat.refraction.intensity))
    );
    let total_reflection_intensity = hit_info.intensity * hit_info.mat.color.a * hit_info.mat.reflection.intensity;
    let total_refraction_intensity = hit_info.intensity * (1.0 - hit_info.mat.color.a) * hit_info.mat.refraction.intensity;

    // Influence of material color (all rays that are neither reflected nor refracted)
    let mut output = RGBColor::from(hit_info.mat.color) * mat_color_intensity;

    // Abort if we hit the bounce limit
    if hit_info.bounces == params.render_params.max_bounces {
        return output
    }

    // Add reflective influence to output if the influence threshold is met
    if total_reflection_intensity > params.render_params.min_intensity {     
        reflect(params, rng, hit_info, total_reflection_intensity, &mut output)
    }

    // Add refractive influence to output if the influence threshold is met
    if total_refraction_intensity > params.render_params.min_intensity {
        unimplemented!()
    }

    output
}

fn reflect<T,R>(params: &RaytraceParameters<T>, rng: &mut R, hit_info: &HitInfo<T>, total_intensity: f32, output: &mut RGBColor) where 
T: num_traits::Float,
R: Rng + ?Sized {

    // Special case for perfect reflection; We only need to send out a single ray
        if hit_info.mat.reflection.max_angle.is_zero() {

            let ray = Ray {
                origin: hit_info.hit.position,
                direction: hit_info.ray.direction.reflect(hit_info.hit.normal).into_normalized()
            };

            *output += raytrace_recursive(params, rng, ray, hit_info.bounces + 1, total_intensity) * total_intensity;

        } else {
            
            let ray_directions = gen_sample_ray_cone(hit_info, rng, hit_info.mat.reflection.max_angle, params.render_params.max_reflect_rays);

            let ray_intensity = total_intensity / (ray_directions.len() as f32);

            for dir in ray_directions {

                let ray = Ray {
                    origin: hit_info.hit.position,
                    direction: dir
                };

                *output += raytrace_recursive(params, rng, ray, hit_info.bounces + 1, total_intensity) * ray_intensity;
            }

        }
}

fn gen_sample_ray_cone<T,R>(hit_info: &HitInfo<T>, rng: &mut R, max_angle: T, max_rays: i32) -> Vec<Vec3Norm<T>> where 
    T: num_traits::Float,
    R: Rng + ?Sized {

    // TODO: Think about precision issues here
    // TODO: Eliminate f32 from everything that touches a ray!

    // Reflect the incoming ray direction
    let normal = hit_info.hit.normal;
    let reflected = hit_info.ray.direction.reflect(normal).into_normalized();

    assert!(max_rays > 1); // Avoid div by zero

    (0..max_rays)
        .map(|_| {

            let mut v = Vec3(T::zero(), T::one(), T::zero());

            let rx: T = T::from(rng.gen::<f64>()).unwrap() * max_angle;
            let ry: T = T::from(rng.gen::<f64>() * 360.0).unwrap();
            v.rotate_x(rx);
            v.rotate_y(ry);

            v.reorient_y_axis(reflected)
        })
        .filter(move |v| v.dot(normal) > T::zero()) // Filter out the ones that penetrate the geometry
        .map(|v| v.into_normalized())
        .collect::<Vec<_>>()
}

fn hit_skybox<T>(ray: &Ray<T>) -> RGBColor where T: num_traits::Float {
    
    // Dumbass implementation that makes the floor a checkerboard and the sky a gradient

    if ray.direction.y() >= T::zero() {

        let mut t: f32 = num_traits::NumCast::from(ray.direction.y()).unwrap();
        t *= 2.0;
        t = t.min(1.0);

        return RGBColor::EVENING_BLUE * (1.0 - t) + RGBColor::BLACK * t
    }

    let t = -ray.origin.y() / ray.direction.y();

    let dumb = (ray.origin + ray.direction * t) * T::from(0.2).unwrap() + Vec3::one() * T::from(1000.0).unwrap();

    let x: i32 = num_traits::NumCast::from(dumb.x()).unwrap();
    let z: i32 = num_traits::NumCast::from(dumb.z()).unwrap();

    let a = x & 1 == 0;
    let b = z & 1 == 0;

    if a != b {
        RGBColor::WHITE
    } else {
        RGBColor::BLACK
    }
}

// Comparison function that determines which raycast hit is closer to the ray origin
fn hit_dist_comp<T>(ray: &Ray<T>, a: &GeometryHitInfo<T>, b: &GeometryHitInfo<T>) -> cmp::Ordering where
    Vec3<T>: Vec3View<T> + std::ops::Sub<Output=Vec3<T>>,
    T: num_traits::Float {

    let dist = |rch: &GeometryHitInfo<T>| {
        (rch.position - ray.origin).sqr_length()
    };

    dist(a).partial_cmp(&dist(b)).unwrap_or(cmp::Ordering::Equal)
}