extern crate rand;
use rand::prelude::*;
use rand::FromEntropy;

extern crate rayon;
use rayon::prelude::*;

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
    pub normal: Vec3Norm<T>,
    pub uv: Vec2<T>

}

pub struct RenderingParameters<T> {
    pub min_intensity: f32,
    pub max_bounces: i32,
    pub max_reflect_rays: i32,
    pub max_refract_rays: i32,
    pub max_dof_rays: i32,

    // TODO: MOve to AO struct
    pub ao_strength: f32,
    pub ao_distance: T,
    pub ao_rays: i32,

    /// Floating point errors can cause visual artifacts in reflections and refraction.
    /// This bias introduces slight inaccuracies with these phenomena, but removes the
    /// artifacts. Basically: Keep lowering this until you see artifacts
    pub float_correction_bias: T
}

// Convenience structs so we don't need to pass around so much stuff
struct RaytraceParameters<'a, T> {
    scene: &'a Scene<T>,
    render_params: &'a RenderingParameters<T>,
}

struct HitInfo<'a, T> {
    mat: &'a Material<T>,
    hit: &'a GeometryHitInfo<T>, 
    ray: &'a Ray<T>, 
    bounces: i32, 
    intensity: f32
}

pub fn render<T>(scene: &Scene<T>, camera: &Camera<T>, render_target: &mut RenderTarget, render_params: &RenderingParameters<T>) where 
    T: num_traits::Float + Send + Sync {

    let raytrace_params = RaytraceParameters {
        scene,
        render_params
    };

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

    let rt_width = render_target.width;
    let rt_height = render_target.height;

    let render_target = std::sync::Mutex::new(render_target);

    (0..rt_height).into_par_iter().for_each(|y_ind| {

        let y_t = T::from(y_ind).unwrap();
        let vp_y = y_start + y_t * y_step;
        let angle_y = y_angle_start + y_t * y_angle_step;

        (0..rt_width).into_par_iter().for_each(|x_ind| {

            let mut rng = SmallRng::from_entropy();

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
            
            {
                let mut lock = render_target.lock().unwrap();

                lock.set_pixel(x_ind, y_ind, color);
            }
        });
    });
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

    let closest_hit = get_closest_hit(params, &ray);

    if let Some((obj, hit)) = closest_hit {

        let uv_mapper = obj.get_uv_mapper();

        let mat = uv_mapper.get_material_at(&hit);

        // Intensity scale factor based on lighting effects
        let mut intensity_scale: f32 = 1.0;

        // Ambient Occlusion
        if params.render_params.ao_strength > 0.0 {

            apply_ao(&mut intensity_scale, rng, params, &hit);
        }

        let hit_info = HitInfo {
            mat,
            hit: &hit,
            ray: &ray,
            bounces,
            intensity: intensity * intensity_scale
        };

        hit_object(params, rng, &hit_info) * hit_info.intensity
    } else {
        hit_skybox(&ray)
    }
}

fn apply_ao<T,R>(intensity: &mut f32, rng: &mut R, params: &RaytraceParameters<T>, hit: &GeometryHitInfo<T>) where 
    T: num_traits::Float,
    R: Rng + ?Sized {

    // Generate ray cone with full spread
    let origin = hit.position + hit.normal * params.render_params.float_correction_bias;
    let directions = gen_sample_ray_cone(rng, T::from(90.0).unwrap(), params.render_params.ao_rays, hit.normal, hit.normal);

    let closest = directions.into_iter()
        .flat_map(|direction| get_closest_hit(params, &Ray { origin, direction }))
        // TODO: Investigate this:
        // .filter(|(_, other_hit)| hit.normal.dot(other_hit.normal) > T::zero()) // Only do AO on EXTERNAL reflections
        .min_by(|a,b| hit_dist_comp(origin, &a.1, &b.1));

    if let Some((_, hit)) = closest {
        
        let distance_normalized = ((hit.position - origin).length() / params.render_params.ao_distance).min(T::one());
        let distance_normalized: f32 = num_traits::NumCast::from(distance_normalized).unwrap();

        let ao_strength = (1.0 - distance_normalized).powf(2.0) * params.render_params.ao_strength;

        *intensity *= 1.0 - ao_strength;
    }
}

fn get_closest_hit<'a, T>(params: &'a RaytraceParameters<T>, ray: &Ray<T>) -> Option<(&'a Box<SceneObject<T>>, GeometryHitInfo<T>)> where T: num_traits::Float {

    let potential_hits = params.scene.objects.iter()
        .map(|obj| (obj, obj.test_intersection(&ray)))
        .filter(|(_, rch)| rch.is_some())
        .map(|(obj, rch)| (obj, rch.unwrap()));

    potential_hits
        .min_by(|a,b| hit_dist_comp(ray.origin, &a.1, &b.1))

}

fn hit_object<T,R>(params: &RaytraceParameters<T>, rng: &mut R, hit_info: &HitInfo<T>) -> RGBColor where 
    T: num_traits::Float, 
    R: Rng + ?Sized {
    
    // Calculate the angle of incidence and it's steepness
    let rev_incoming_dot_normal: f32 = num_traits::NumCast::from((-hit_info.ray.direction).dot(hit_info.hit.normal)).unwrap();
    let incidence_angle_steepness = 1.0 - (rev_incoming_dot_normal.acos() / std::f32::consts::FRAC_PI_2 - 1.0).abs();
    
    // Calculate the effect of the angle of incidence on reflectivity
    let incidence_reflection_influence = incidence_angle_steepness.powf(hit_info.mat.reflection.edge_effect_power);
    let scaled_reflection_intensity = 
        (1.0 - incidence_reflection_influence)  * hit_info.mat.reflection.intensity_center + 
        incidence_reflection_influence          * hit_info.mat.reflection.intensity_edges;

    // Calculate the effect of the angle of incidence on refraction (object alpha)
    let incidence_alpha_influence = incidence_angle_steepness.powf(hit_info.mat.transparency.edge_effect_power);
    let scaled_alpha = 
        (1.0 - incidence_alpha_influence)       * hit_info.mat.transparency.opacity_center +
        incidence_alpha_influence               * hit_info.mat.transparency.opacity_edges;

    // Useful for debugging: Return some interesting value as a color
    //return RGBColor::PINK * scaled_reflection_intensity;

    // Calculate intensities of color, reflection and refraction
    let mat_color_intensity = scaled_alpha * (1.0 - scaled_reflection_intensity);
    let total_reflection_intensity = scaled_alpha * scaled_reflection_intensity;
    let total_refraction_intensity = 1.0 - scaled_alpha;

    // Influence of material color (all rays that are neither reflected nor refracted)
    let mut output = RGBColor::from(hit_info.mat.color) * mat_color_intensity;

    // Abort recursion if we hit the bounce or intensity limit
    if hit_info.bounces == params.render_params.max_bounces || hit_info.intensity < params.render_params.min_intensity {
        return output
    }

    // Add reflective influence to output if the influence threshold is met
    if total_reflection_intensity > params.render_params.min_intensity {     
        reflect(params, rng, hit_info, total_reflection_intensity, &mut output)
    }

    // Add refractive influence to output if the influence threshold is met
    if total_refraction_intensity > params.render_params.min_intensity {
        refract(params, rng, hit_info, total_refraction_intensity, &mut output)
    }

    output
}

fn reflect<T,R>(params: &RaytraceParameters<T>, rng: &mut R, hit_info: &HitInfo<T>, total_intensity: f32, output: &mut RGBColor) where 
    T: num_traits::Float,
    R: Rng + ?Sized {

    // Origin of all reflected rays including bias
    let origin = hit_info.hit.position + hit_info.hit.normal * params.render_params.float_correction_bias;
    let direction = hit_info.ray.direction.reflect(hit_info.hit.normal).into_normalized();

    // Special case for perfect reflection; We only need to send out a single ray
    if hit_info.mat.reflection.max_angle.is_zero() {

        let ray = Ray { origin, direction };

        *output += raytrace_recursive(params, rng, ray, hit_info.bounces + 1, total_intensity) * total_intensity;

    } else {

        let ray_directions = gen_sample_ray_cone(rng, hit_info.mat.reflection.max_angle, params.render_params.max_reflect_rays, hit_info.hit.normal, direction);

        let ray_intensity = total_intensity / (ray_directions.len() as f32);

        for dir in ray_directions {

            let ray = Ray {
                origin,
                direction: dir
            };

            *output += raytrace_recursive(params, rng, ray, hit_info.bounces + 1, total_intensity) * ray_intensity;
        }

    }
}

fn gen_sample_ray_cone<T,R>(rng: &mut R, max_angle: T, max_rays: i32, cutoff_normal: Vec3Norm<T>, reorient_axis: Vec3Norm<T>) -> Vec<Vec3Norm<T>> where 
    T: num_traits::Float,
    R: Rng + ?Sized {

    (0..max_rays)
        .map(|_| {

            let mut v = Vec3(T::zero(), T::one(), T::zero());

            let rx: T = T::from(rng.gen::<f64>()).unwrap() * max_angle;
            let ry: T = T::from(rng.gen::<f64>() * 360.0).unwrap();
            v.rotate_x(rx);
            v.rotate_y(ry);

            v.reorient_y_axis(reorient_axis)
        })
        .filter(move |v| v.dot(cutoff_normal) > T::zero()) // Filter out the ones that penetrate the geometry
        //.map(|v| v.into_normalized()) // TODO: Look why this sometimes fails, or use this:
        .map(|v| v.normalize())
        .collect::<Vec<_>>()
}

fn refract<T,R>(params: &RaytraceParameters<T>, rng: &mut R, hit_info: &HitInfo<T>, total_intensity: f32, output: &mut RGBColor) where 
    T: num_traits::Float,
    R: Rng + ?Sized {
    
    // This closure is magic and was stolen from:
    // https://www.scratchapixel.com/lessons/3d-basic-rendering/introduction-to-shading/reflection-refraction-fresnel
    let get_refr_ray = |ior_from: T, ior_into: T, n: Vec3Norm<T>, hit_cos: T| {

        let refr_ratio = ior_from / ior_into;

        let k = T::one() - refr_ratio*refr_ratio * (T::one() - hit_cos*hit_cos);

        if k < T::zero() {
            let origin = hit_info.hit.position - hit_info.hit.normal * params.render_params.float_correction_bias;
            let direction = hit_info.ray.direction.reflect((-hit_info.hit.normal).into_normalized()).into_normalized();
            Ray { origin, direction }
        } else {
            // Be careful here: When we leave the medium, we need the bias to take us outside of the object!
            let origin = hit_info.hit.position - n * params.render_params.float_correction_bias;
            let direction = (hit_info.ray.direction * refr_ratio + hit_info.hit.normal * (refr_ratio * hit_cos - k.sqrt())).normalize();
            Ray { origin, direction }
        }
    };

    let hit_cos = hit_info.ray.direction.dot(hit_info.hit.normal);

    let going_inside_object = hit_cos <= T::zero();

    let refr_ray = if going_inside_object {
        // Air into sth else
        get_refr_ray(T::one(), hit_info.mat.refraction.index_of_refraction, hit_info.hit.normal, -hit_cos)
    } else {
        // Sth else into air
        get_refr_ray(hit_info.mat.refraction.index_of_refraction, T::one(), (-hit_info.hit.normal).into_normalized(), hit_cos)
    };

   
    if hit_info.mat.refraction.max_angle.is_zero() {

         // Special case for perfect refraction: We only need to send out a single ray

        *output += raytrace_recursive(params, rng, refr_ray, hit_info.bounces + 1, total_intensity) * total_intensity;

    } else {

        // Otherwise, we send many rays

        let origin = refr_ray.origin;

        let cutoff_normal = if going_inside_object { (-hit_info.hit.normal).into_normalized() } else { hit_info.hit.normal };

        let directions = gen_sample_ray_cone(rng, hit_info.mat.refraction.max_angle, params.render_params.max_refract_rays, cutoff_normal, refr_ray.direction);

        let ray_intensity = total_intensity / (directions.len() as f32);

        for dir in directions {

            let ray = Ray {
                origin,
                direction: dir
            };

            *output += raytrace_recursive(params, rng, ray, hit_info.bounces + 1, total_intensity) * ray_intensity;
        }

    }
}

fn hit_skybox<T>(ray: &Ray<T>) -> RGBColor where T: num_traits::Float {
    
    // Dumbass implementation that makes the floor a checkerboard and the sky a gradient

    if ray.direction.y() >= T::zero() {

        let mut t: f32 = num_traits::NumCast::from(ray.direction.y()).unwrap();
        t *= 2.0;
        t = t.min(1.0);

        RGBColor::EVENING_BLUE * (1.0 - t) + RGBColor::WHITE * t
    } else {
        RGBColor::PINK
    }
}

// Comparison function that determines which raycast hit is closer to the supplied point
fn hit_dist_comp<T>(point: Vec3<T>, a: &GeometryHitInfo<T>, b: &GeometryHitInfo<T>) -> cmp::Ordering where
    Vec3<T>: Vec3View<T> + std::ops::Sub<Output=Vec3<T>>,
    T: num_traits::Float {

    let dist = |rch: &GeometryHitInfo<T>| {
        (rch.position - point).sqr_length()
    };

    dist(a).partial_cmp(&dist(b)).unwrap_or(cmp::Ordering::Equal)
}