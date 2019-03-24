use rand::prelude::*;
use rand::FromEntropy;
use rayon::prelude::*;

use super::vec3::*;
use super::camera::*;
use super::output::*;
use super::scene::*;
use super::color::*;
use super::material::*;
use super::ray_target::*;
use super::render_parameters::*;

use std::cmp;

pub struct Ray<T> {
    pub origin: Vec3<T>,
    pub direction: Vec3Norm<T>
}

// Convenience structs so we don't need to pass around so much stuff
struct RaytraceParameters<'a, T> {
    scene: &'a Scene<T>,
    render_params: &'a RenderParameters<T>,
}

struct HitInfo<'a, T> {
    mat: &'a Material<T>,
    hit: &'a GeometryHitInfo<T>, 
    ray: &'a Ray<T>, 
    bounces: u32, 
    intensity: T
}

pub fn render<T>(scene: &Scene<T>, camera: &Camera<T>, render_target: &mut RenderTarget, render_params: &RenderParameters<T>) where 
    T: num_traits::Float + num_traits::FloatConst + Send + Sync {

    if !render_params.validate() {
        panic!("Invalid RenderParameters")
    }

    if !camera.validate() {
        panic!("Invalid Camera settings");
    }

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
            let color = if render_params.dof.max_angle.is_zero() {

                let direction = get_initial_randomized_ray_direction(camera, &mut rng, render_params.dof.max_angle, angle_x, angle_y);

                raytrace_recursive(
                    &raytrace_params,
                    &mut rng,
                    Ray { origin, direction }, 
                    0, T::one())

            } else {

                let mut color = RGBColor::BLACK;

                let ray_influence = T::one() / T::from(render_params.dof.samples).unwrap();

                for _ in 0..render_params.dof.samples {

                    let direction = get_initial_randomized_ray_direction(camera, &mut rng, render_params.dof.max_angle, angle_x, angle_y);

                    color += raytrace_recursive(
                        &raytrace_params,
                        &mut rng,
                        Ray { origin, direction }, 
                        0, T::one())
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

fn get_initial_ray_origin<T>(camera: &Camera<T>, viewport_x: T, viewport_y: T) -> Vec3<T> where T: num_traits::Float {

    let mut origin = Vec3(viewport_x, viewport_y, T::zero());
    origin.rotate_x(camera.orientation.0);
    origin.rotate_y(camera.orientation.1);
    origin.rotate_z(camera.orientation.2);

    origin += camera.position;

    origin
}

fn get_initial_randomized_ray_direction<T, R>(camera: &Camera<T>, rng: &mut R, dof_angle: T, fov_angle_x: T, fov_angle_y: T) -> Vec3Norm<T> where 
    T: num_traits::Float,
    R: Rng + ?Sized {

    let mut direction = Vec3(T::zero(), T::zero(), T::one());

    // Randomization for DoF
    if !dof_angle.is_zero() {

        let dof_rx: T = T::from(rng.gen::<f64>()).unwrap() * dof_angle;
        let dof_rz: T = T::from(rng.gen::<f64>() * 360.0).unwrap();
        direction.rotate_x(dof_rx);
        direction.rotate_z(dof_rz);

    }

    // Fov Influence
    direction.rotate_y(fov_angle_x);
    direction.rotate_x(fov_angle_y);

    // Camera orientation influence
    direction.rotate_x(camera.orientation.0);
    direction.rotate_y(camera.orientation.1);
    direction.rotate_z(camera.orientation.2);

    direction.into_normalized()
}

fn raytrace_recursive<T,R>(params: &RaytraceParameters<T>, rng: &mut R, ray: Ray<T>, bounces: u32, intensity: T) -> RGBColor where 
    Vec3<T>: Vec3View<T> + std::ops::Sub<Output=Vec3<T>>,
    T: num_traits::Float + num_traits::FloatConst,
    R: Rng + ?Sized {

    let closest_hit = get_closest_hit(params, &ray);

    if let Some((obj, hit)) = closest_hit {

        let uv_mapper = obj.get_uv_mapper();

        let mat = uv_mapper.get_material_at(&hit);

        // Intensity scale factor based on lighting effects
        let mut intensity_scale = T::one();

        // Ambient Occlusion
        if !params.render_params.ao.strength.is_zero() {

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
        params.scene.sky_color * intensity

    }
}

fn apply_ao<T,R>(intensity: &mut T, rng: &mut R, params: &RaytraceParameters<T>, hit: &GeometryHitInfo<T>) where 
    T: num_traits::Float,
    R: Rng + ?Sized {

    // Generate ray cone with full spread
    let origin = hit.position + hit.normal * params.render_params.quality.float_correction_bias;
    let directions = gen_sample_ray_cone(rng, T::from(90.0).unwrap(), params.render_params.ao.samples, hit.normal, hit.normal);

    let closest = directions.into_iter()
        .flat_map(|direction| get_closest_hit(params, &Ray { origin, direction }))
        // TODO: Investigate this:
        // .filter(|(_, other_hit)| hit.normal.dot(other_hit.normal) > T::zero()) // Only do AO on EXTERNAL reflections
        .min_by(|a,b| hit_dist_comp(origin, &a.1, &b.1));

    if let Some((_, hit)) = closest {
        
        let distance_normalized = ((hit.position - origin).length() / params.render_params.ao.distance).min(T::one());

        let ao_strength = (T::one() - distance_normalized).powf(T::from(2.0).unwrap()) * params.render_params.ao.strength;

        *intensity = *intensity * (T::one() - ao_strength);
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
    T: num_traits::Float + num_traits::FloatConst, 
    R: Rng + ?Sized {
    
    // Calculate the angle btween our incoming ray and surface normal
    let incidence_angle_steepness = calc_steepness(hit_info.ray.direction, hit_info.hit.normal);

    // Calculate the effect of the angle of incidence on reflectivity
    let incidence_reflection_influence = incidence_angle_steepness.powf(hit_info.mat.reflection.edge_effect_power);
    let scaled_reflection_intensity = 
        (T::one() - incidence_reflection_influence)     * hit_info.mat.reflection.intensity_center + 
        incidence_reflection_influence                  * hit_info.mat.reflection.intensity_edges;

    // Calculate the effect of the angle of incidence on refraction (object alpha)
    let incidence_alpha_influence = incidence_angle_steepness.powf(hit_info.mat.opacity.edge_effect_power);
    let scaled_alpha = 
        (T::one() - incidence_alpha_influence)          * hit_info.mat.opacity.opacity_center +
        incidence_alpha_influence                       * hit_info.mat.opacity.opacity_edges;

    // Useful for debugging: Return some interesting value as a color
    //return RGBColor::PINK * scaled_reflection_intensity;

    // Calculate intensities of color, reflection and refraction and multiply to total ray intensity
    let mat_color_intensity = scaled_alpha * (T::one() - scaled_reflection_intensity)   * hit_info.intensity;
    let total_reflection_intensity = scaled_alpha * scaled_reflection_intensity         * hit_info.intensity;
    let total_refraction_intensity = (T::one() - scaled_alpha)                          * hit_info.intensity;

    // Influence of material color (all rays that are neither reflected nor refracted)
    let mut output = RGBColor::from(hit_info.mat.color) * mat_color_intensity;

    // Abort recursion if we hit the bounce limit
    if hit_info.bounces == params.render_params.quality.max_bounces {
        return output
    }

    // Add reflective influence to output if the influence threshold is met
    if total_reflection_intensity > params.render_params.quality.min_intensity {     
        reflect(params, rng, hit_info, total_reflection_intensity, &mut output)
    }

    // Add refractive influence to output if the influence threshold is met
    if total_refraction_intensity > params.render_params.quality.min_intensity {
        refract(params, rng, hit_info, total_refraction_intensity, &mut output)
    }

    output
}

fn reflect<T,R>(params: &RaytraceParameters<T>, rng: &mut R, hit_info: &HitInfo<T>, total_intensity: T, output: &mut RGBColor) where 
    T: num_traits::Float + num_traits::FloatConst,
    R: Rng + ?Sized {

    // Origin of all reflected rays including bias
    let origin = hit_info.hit.position + hit_info.hit.normal * params.render_params.quality.float_correction_bias;
    let direction = hit_info.ray.direction.reflect(hit_info.hit.normal)
        .interpolate_into(hit_info.hit.normal, hit_info.mat.reflection.max_angle / T::from(90.0).unwrap())
        .normalize();

    // Special case for perfect reflection; We only need to send out a single ray
    if hit_info.mat.reflection.max_angle.is_zero() {

        let ray = Ray { origin, direction };

        *output += raytrace_recursive(params, rng, ray, hit_info.bounces + 1, total_intensity);

    } else {

        let ray_count = get_ray_count_for_intensity(total_intensity, params.render_params.sample_limits.max_reflection_samples);

        let ray_directions = gen_sample_ray_cone(rng, hit_info.mat.reflection.max_angle, ray_count, hit_info.hit.normal, direction);

        let ray_intensity = total_intensity / T::from(ray_directions.len()).unwrap();

        for dir in ray_directions {

            let ray = Ray {
                origin,
                direction: dir
            };

            *output += raytrace_recursive(params, rng, ray, hit_info.bounces + 1, ray_intensity);
        }

    }
}

fn refract<T,R>(params: &RaytraceParameters<T>, rng: &mut R, hit_info: &HitInfo<T>, total_intensity: T, output: &mut RGBColor) where 
    T: num_traits::Float + num_traits::FloatConst,
    R: Rng + ?Sized {
    
    // This closure is magic and was stolen from:
    // https://www.scratchapixel.com/lessons/3d-basic-rendering/introduction-to-shading/reflection-refraction-fresnel
    let get_refr_ray = |ior_from: T, ior_into: T, n: Vec3Norm<T>, hit_cos: T| {

        let refr_ratio = ior_from / ior_into;

        let k = T::one() - refr_ratio*refr_ratio * (T::one() - hit_cos*hit_cos);

        if k < T::zero() {
            let origin = hit_info.hit.position - hit_info.hit.normal * params.render_params.quality.float_correction_bias;
            let direction = hit_info.ray.direction.reflect((-hit_info.hit.normal).into_normalized()).into_normalized();
            Ray { origin, direction }
        } else {
            // Be careful here: When we leave the medium, we need the bias to take us outside of the object!
            let origin = hit_info.hit.position - n * params.render_params.quality.float_correction_bias;
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

        *output += raytrace_recursive(params, rng, refr_ray, hit_info.bounces + 1, total_intensity);

    } else {

        // Otherwise, we send many rays

        let origin = refr_ray.origin;

        let cutoff_normal = if going_inside_object { (-hit_info.hit.normal).into_normalized() } else { hit_info.hit.normal };

        let ray_count = get_ray_count_for_intensity(total_intensity, params.render_params.sample_limits.max_refraction_samples);

        let directions = gen_sample_ray_cone(rng, hit_info.mat.refraction.max_angle, ray_count, cutoff_normal, refr_ray.direction);

        let ray_intensity = total_intensity / T::from(directions.len()).unwrap();

        for dir in directions {

            let ray = Ray {
                origin,
                direction: dir
            };

            *output += raytrace_recursive(params, rng, ray, hit_info.bounces + 1, ray_intensity);
        }

    }
}

fn calc_steepness<T>(incoming: Vec3Norm<T>, normal: Vec3Norm<T>) -> T where T: num_traits::Float + num_traits::FloatConst {

    let i_dot_n = incoming.dot(normal);

    let angle_rad = if i_dot_n <= T::zero() {
        // Normal reflection
        T::PI() - i_dot_n.acos()
    } else {
        // Interior reflection (we are a refracted ray inside of an object)
        i_dot_n.acos()
    };

    angle_rad / T::FRAC_PI_2()
}

fn gen_sample_ray_cone<T,R>(rng: &mut R, max_angle: T, max_rays: u32, cutoff_normal: Vec3Norm<T>, cone_direction: Vec3Norm<T>) -> Vec<Vec3Norm<T>> where 
    T: num_traits::Float,
    R: Rng + ?Sized {

    let cone_dir_right_angle = cone_direction.get_random_90_deg_vector().normalize();

    (0..max_rays)
        .map(|_| {

            let deviation = T::from(rng.gen::<f64>()).unwrap() * max_angle;
            let spin = T::from(rng.gen::<f64>() * 360.0).unwrap();

            // Copy initial direction
            let mut dir = cone_direction;

            // Deviate from initial direction by up to max_angle
            dir.rotate_around_axis(cone_dir_right_angle, deviation);

            // Randomize the direction
            dir.rotate_around_axis(cone_direction, spin);

            dir
        })
        .filter(move |v| v.dot(cutoff_normal) > T::zero()) // Filter out the ones that penetrate the geometry
        .collect::<Vec<_>>()
}

fn get_ray_count_for_intensity<T>(intensity: T, max_rays: u32) -> u32 where T: num_traits::Float {

    let max_rays = T::from(max_rays).unwrap();
    let ray_count = (T::one() + intensity * (max_rays - T::one())).round();

    num_traits::NumCast::from(ray_count).unwrap()
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