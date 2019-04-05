use rand::prelude::*;
use rand::FromEntropy;
use rayon::prelude::*;

use super::vec::*;
use super::camera::*;
use super::output::*;
use super::scene::*;
use super::color::*;
use super::material::*;
use super::ray_target::*;
use super::render_params::*;

use std::cmp;

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3Norm
}

// Convenience structs so we don't need to pass around so much stuff
struct RaytraceParameters<'a> {
    scene: &'a Scene,
    render_params: &'a RenderParams,
}

struct HitInfo<'a> {
    mat: &'a Material,
    hit: &'a GeometryHitInfo, 
    ray: &'a Ray, 
    bounces: u32, 
    intensity: f64
}

pub fn render(scene: &Scene, camera: &Camera, render_target: &mut RenderTarget, render_params: &RenderParams) {

    if !render_params.validate() {
        panic!("Invalid RenderParameters. Aborting.")
    }

    if !camera.validate() {
        panic!("Invalid Camera settings. Aborting.");
    }

    if !scene.validate() {
        panic!("Scene contains illegal materials or colors. Aborting.")
    }

    let raytrace_params = RaytraceParameters {
        scene,
        render_params
    };

    // Reusable stuff to avoid casting so much
    let w = render_target.width as f64;
    let h = render_target.height as f64;

    // Distances between 2 pixels
    let x_step = camera.viewport.width / w;
    let x_start = (x_step - camera.viewport.width) / 2.0;

    let y_step = -camera.viewport.height / h;
    let y_start = (camera.viewport.height - y_step) / 2.0;

    // Angle distances between two pixels
    let fov_v = camera.fov_h / camera.viewport.aspect();

    let x_angle_step = camera.fov_h / w;
    let x_angle_start = (x_angle_step - camera.fov_h) / 2.0;

    let y_angle_step = fov_v / h;
    let y_angle_start = (y_angle_step - fov_v) / 2.0;

    let rt_width = render_target.width;
    let rt_height = render_target.height;

    let render_target = std::sync::Mutex::new(render_target);

    (0..rt_height).into_par_iter().for_each(|y_ind| {

        let y_ind_f = y_ind as f64;
        let vp_y = y_start + y_ind_f * y_step;
        let angle_y = y_angle_start + y_ind_f * y_angle_step;

        (0..rt_width).into_par_iter().for_each(|x_ind| {

            let x_ind_f = x_ind as f64;
            let vp_x = x_start + x_ind_f * x_step;
            let angle_x = x_angle_start + x_ind_f * x_angle_step;
            
            let mut rng = SmallRng::from_entropy();

            let origin = get_initial_ray_origin(camera, vp_x, vp_y);

            // We render just a single ray if DoF is disabled
            let color = if render_params.dof.max_angle == 0.0 {

                let direction = get_initial_ray_direction(camera, &mut rng, render_params.dof.max_angle, angle_x, angle_y);

                raytrace_recursive(
                    &raytrace_params,
                    &mut rng,
                    Ray { origin, direction }, 
                    0, 1.0)

            } else {

                let mut color = RGBColor::BLACK;

                let ray_influence = 1.0 / render_params.dof.samples as f64;

                for _ in 0..render_params.dof.samples {

                    let direction = get_initial_ray_direction(camera, &mut rng, render_params.dof.max_angle, angle_x, angle_y);

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
                // TODO: Wrap this in unsafe code. We will never write to the same pixel twice anyway
                let mut lock = render_target.lock().unwrap();

                lock.set_pixel(x_ind, y_ind, color);
            }
        });
    });
}

fn get_initial_ray_origin(camera: &Camera, viewport_x: f64, viewport_y: f64) -> Vec3 {

    let mut origin = Vec3::new(viewport_x, viewport_y, 0.0)
        .rotate(camera.rotation);

    origin += camera.position;

    origin
}

fn get_initial_ray_direction<R: Rng + ?Sized>(camera: &Camera, rng: &mut R, dof_angle: f64, fov_angle_x: f64, fov_angle_y: f64) -> Vec3Norm {

    let mut direction = Vec3Norm::FORWARD;

    // Randomization for DoF
    if dof_angle != 0.0 {

        let dof_rx = rng.gen::<f64>() * dof_angle;
        let dof_rz = rng.gen::<f64>() * 360.0;

        direction = direction
            .rotate_x(dof_rx)
            .rotate_z(dof_rz);

    }

    direction = direction
    // Fov Influence
        .rotate_y(fov_angle_x)
        .rotate_x(fov_angle_y)
    // Camera orientation influence
        .rotate(camera.rotation);

    direction
}

fn raytrace_recursive<R: Rng + ?Sized>(params: &RaytraceParameters, rng: &mut R, ray: Ray, bounces: u32, intensity: f64) -> RGBColor {

    let closest_hit = get_closest_hit(params, &ray, bounces);

    if let Some((obj, hit)) = closest_hit {

        let uv_mapper = obj.get_uv_mapper();

        let mat = uv_mapper.get_material_at(&hit);

        // Intensity scale factor based on lighting effects
        let mut intensity_scale = 1.0;

        // Ambient Occlusion
        if bounces < params.render_params.quality.max_bounces && params.render_params.ao.strength != 0.0 {
            apply_ao(&mut intensity_scale, rng, params, &hit);
        }

        let hit_info = HitInfo {
            mat: &mat,
            hit: &hit,
            ray: &ray,
            bounces,
            intensity: intensity * intensity_scale
        };

        hit_object(params, rng, &hit_info)
    } else {
        
        // Ray didn't hit anything
        params.render_params.sky_color * intensity

    }
}

fn apply_ao<R: Rng + ?Sized>(intensity: &mut f64, rng: &mut R, params: &RaytraceParameters, hit: &GeometryHitInfo) {

    // Generate ray cone with full spread
    let origin = hit.position + hit.normal * params.render_params.quality.bias;
    let directions = gen_sample_ray_cone(rng, 90.0, params.render_params.ao.samples, hit.normal, hit.normal);

    let closest = directions.into_iter()
        .flat_map(|direction| get_closest_hit(params, &Ray { origin, direction }, 1))
        // TODO: Investigate this:
        // .filter(|(_, other_hit)| hit.normal.dot(other_hit.normal) > T::zero()) // Only do AO on EXTERNAL reflections
        .min_by(|a,b| hit_dist_comp(origin, &a.1, &b.1));

    if let Some((_, hit)) = closest {
        
        let distance_normalized = ((hit.position - origin).length() / params.render_params.ao.distance).min(1.0);

        let ao_strength = (1.0 - distance_normalized).powi(2) * params.render_params.ao.strength;

        *intensity = *intensity * (1.0 - ao_strength);
    }
}

fn get_closest_hit<'a>(params: &'a RaytraceParameters, ray: &Ray, bounces: u32) -> Option<(&'a Box<SceneObject>, GeometryHitInfo)> {

    let potential_hits = params.scene.objects.iter()
        .map(|obj| (obj, obj.test_intersection(&ray)))
        // Filter out the objects that are actually hit AND visible to the camera
        .filter(|(obj, rch)| rch.is_some() && (bounces > 0 || obj.is_visible_to_camera()) )
        .map(|(obj, rch)| (obj, rch.unwrap()));

    potential_hits
        .min_by(|a,b| hit_dist_comp(ray.origin, &a.1, &b.1))

}

fn hit_object<R: Rng + ?Sized>(params: &RaytraceParameters, rng: &mut R, hit_info: &HitInfo) -> RGBColor {
    
    // Calculate the angle btween our incoming ray and surface normal
    let incidence_angle_steepness = calc_steepness(hit_info.ray.direction, hit_info.hit.normal);

    // Calculate the effect of the angle of incidence on reflectivity
    let incidence_reflection_influence = incidence_angle_steepness.powf(hit_info.mat.reflection.power);
    let scaled_reflection_intensity = 
        (1.0 - incidence_reflection_influence)  * hit_info.mat.reflection.center + 
        incidence_reflection_influence          * hit_info.mat.reflection.edges;

    // Calculate the effect of the angle of incidence on refraction (object alpha)
    let incidence_alpha_influence = incidence_angle_steepness.powf(hit_info.mat.opacity.power);
    let scaled_alpha = 
        (1.0 - incidence_alpha_influence)   * hit_info.mat.opacity.center +
        incidence_alpha_influence           * hit_info.mat.opacity.edges;

    // Useful for debugging: Return some interesting value as a color
    //return RGBColor::PINK * scaled_reflection_intensity;

    // Calculate intensities of color, reflection and refraction and multiply to total ray intensity
    let mat_color_intensity =           scaled_alpha * (1.0 - scaled_reflection_intensity)  * hit_info.intensity;
    let total_reflection_intensity =    scaled_alpha * scaled_reflection_intensity          * hit_info.intensity;
    let total_refraction_intensity =    (1.0 - scaled_alpha)                                * hit_info.intensity;

    // Influence of material color (all rays that are neither reflected nor refracted)
    let mut output = RGBColor::from(hit_info.mat.color) * mat_color_intensity;

    // Abort recursion if we hit the bounce limit
    if hit_info.bounces == params.render_params.quality.max_bounces {
        return output
    }

    // Add reflective influence to output if the influence threshold is met
    if total_reflection_intensity > params.render_params.quality.min_intensity {

        let tint = hit_info.mat.reflection.color.unwrap_or(hit_info.mat.color);

        output += tint * reflect(params, rng, hit_info, total_reflection_intensity);
    }

    // Add refractive influence to output if the influence threshold is met
    if total_refraction_intensity > params.render_params.quality.min_intensity {

        let tint = hit_info.mat.refraction.color.unwrap_or(hit_info.mat.color);

        output += tint * refract(params, rng, hit_info, total_refraction_intensity)
    }

    output
}

fn reflect<R: Rng + ?Sized>(params: &RaytraceParameters, rng: &mut R, hit_info: &HitInfo, total_intensity: f64) -> RGBColor {

    // Origin of all reflected rays including bias
    let origin = hit_info.hit.position + hit_info.hit.normal * params.render_params.quality.bias;
    let direction = hit_info.ray.direction.reflect(hit_info.hit.normal)
        .interpolate_towards(hit_info.hit.normal, hit_info.mat.reflection.max_angle / 90.0)
        .normalized();

    // Special case for perfect reflection; We only need to send out a single ray
    if hit_info.mat.reflection.max_angle == 0.0 {

        let ray = Ray { origin, direction };

        raytrace_recursive(params, rng, ray, hit_info.bounces + 1, total_intensity)
    } else {

        let ray_count = get_ray_count_for_intensity(total_intensity, params.render_params.max_samples.reflection);

        let ray_directions = gen_sample_ray_cone(rng, hit_info.mat.reflection.max_angle, ray_count, hit_info.hit.normal, direction);

        let ray_intensity = total_intensity / ray_directions.len() as f64;

        let mut output = RGBColor::BLACK;

        for dir in ray_directions {

            let ray = Ray {
                origin,
                direction: dir
            };

            output += raytrace_recursive(params, rng, ray, hit_info.bounces + 1, ray_intensity);
        }

        output
    }
}

fn refract<R: Rng + ?Sized>(params: &RaytraceParameters, rng: &mut R, hit_info: &HitInfo, total_intensity: f64) -> RGBColor {
    
    // This closure is magic and was stolen from:
    // https://www.scratchapixel.com/lessons/3d-basic-rendering/introduction-to-shading/reflection-refraction-fresnel
    let get_refr_ray = |ior_from: f64, ior_into: f64, n: Vec3Norm, hit_cos: f64| {

        let refr_ratio = ior_from / ior_into;

        let k = 1.0 - refr_ratio*refr_ratio * (1.0 - hit_cos*hit_cos);

        if k < 0.0 {
            let origin = hit_info.hit.position - hit_info.hit.normal * params.render_params.quality.bias;
            let direction = hit_info.ray.direction.reflect(-hit_info.hit.normal);
            Ray { origin, direction }
        } else {
            // Be careful here: When we leave the medium, we need the bias to take us outside of the object!
            let origin = hit_info.hit.position - n * params.render_params.quality.bias;
            let direction = (hit_info.ray.direction * refr_ratio + hit_info.hit.normal * (refr_ratio * hit_cos - k.sqrt())).normalized();
            Ray { origin, direction }
        }
    };

    let hit_cos = hit_info.ray.direction.dot(hit_info.hit.normal);

    let going_inside_object = hit_cos <= 0.0;

    let refr_ray = if going_inside_object {
        // Air into sth else
        get_refr_ray(1.0, hit_info.mat.refraction.ior, hit_info.hit.normal, -hit_cos)
    } else {
        // Sth else into air
        get_refr_ray(hit_info.mat.refraction.ior, 1.0, -hit_info.hit.normal, hit_cos)
    };

   
    if hit_info.mat.refraction.max_angle == 0.0 {

         // Special case for perfect refraction: We only need to send out a single ray
        raytrace_recursive(params, rng, refr_ray, hit_info.bounces + 1, total_intensity)

    } else {

        // Otherwise, we send many rays

        let origin = refr_ray.origin;

        let cutoff_normal = if going_inside_object { -hit_info.hit.normal } else { hit_info.hit.normal };

        let ray_count = get_ray_count_for_intensity(total_intensity, params.render_params.max_samples.refraction);

        let directions = gen_sample_ray_cone(rng, hit_info.mat.refraction.max_angle, ray_count, cutoff_normal, refr_ray.direction);

        let ray_intensity = total_intensity / directions.len() as f64;

        let mut output = RGBColor::BLACK;

        for dir in directions {

            let ray = Ray {
                origin,
                direction: dir
            };

            output += raytrace_recursive(params, rng, ray, hit_info.bounces + 1, ray_intensity);
        }

        output
    }
}

fn calc_steepness(incoming: Vec3Norm, normal: Vec3Norm) -> f64 {

    let i_dot_n = incoming.dot(normal);

    let angle_rad = if i_dot_n <= 0.0 {
        // Normal reflection
        std::f64::consts::PI - i_dot_n.acos()
    } else {
        // Interior reflection (we are a refracted ray inside of an object)
        i_dot_n.acos()
    };

    angle_rad / std::f64::consts::FRAC_PI_2
}

fn gen_sample_ray_cone<R: Rng + ?Sized>(rng: &mut R, max_angle: f64, max_rays: u32, cutoff_normal: Vec3Norm, cone_direction: Vec3Norm) -> Vec<Vec3Norm> {

    let cone_dir_right_angle = cone_direction.get_random_90_deg_vector().normalized();

    (0..max_rays)
        .map(|_| {

            let deviation = rng.gen::<f64>() * max_angle;
            let spin = rng.gen::<f64>() * 360.0;

            // Copy initial direction
            cone_direction
            // Deviate from initial direction by up to max_angle
                .rotate_around_axis(cone_dir_right_angle, deviation)
            // Randomize the direction
                .rotate_around_axis(cone_direction, spin)
        })
        .filter(move |v| v.dot(cutoff_normal) > 0.0) // Filter out the ones that penetrate the geometry
        .collect::<Vec<_>>()
}

fn get_ray_count_for_intensity(intensity: f64, max_rays: u32) -> u32 {

    (1.0 + intensity * (max_rays - 1) as f64).round() as u32
}

// TODO: This should probably be somewhere else
// Comparison function that determines which raycast hit is closer to the supplied point
pub fn hit_dist_comp(point: Vec3, a: &GeometryHitInfo, b: &GeometryHitInfo) -> cmp::Ordering {

    let dist_a = (a.position - point).sqr_length();
    let dist_b = (b.position - point).sqr_length();

    dist_a.partial_cmp(&dist_b).unwrap_or(cmp::Ordering::Equal)
}