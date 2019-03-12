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
    pub max_rays: i32
}

// Convenience struct so we don't need to pass around so much stuff
struct RaytraceParameters<'a, T> {
    scene: &'a Scene<T>,
    render_params: &'a RenderingParameters,
}

pub fn render<T>(scene: &Scene<T>, camera: &Camera<T>, render_target: &mut RenderTarget, render_params: &RenderingParameters) where 
    T: num_traits::Float {

    // TODO: Replace this with something properly respecting camera parameters

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

    for y_ind in 0..render_target.height {

        let y_t = T::from(y_ind).unwrap();
        let vp_y = y_start + y_t * y_step;
        let angle_y = y_angle_start + y_t * y_angle_step;

        for x_ind in 0..render_target.width {

            let x_t = T::from(x_ind).unwrap();
            let vp_x = x_start + x_t * x_step;
            let angle_x = x_angle_start + x_t * x_angle_step;
            
            let mut origin = Vec3(vp_x, vp_y, T::zero());
            origin.rotate_x(camera.orientation.x);
            origin.rotate_y(camera.orientation.y);
            origin.rotate_z(camera.orientation.z);

            origin += camera.position;

            let mut direction = Vec3(T::zero(), T::zero(), T::one());
            direction.rotate_y(angle_x);
            direction.rotate_x(angle_y);

            direction.rotate_x(camera.orientation.x);
            direction.rotate_y(camera.orientation.y);
            direction.rotate_z(camera.orientation.z);

            let color = raytrace_recursive(
                &raytrace_params,
                Ray { origin, direction: direction.into_normalized() }, 
                0, 1.0);

            render_target.set_pixel(x_ind, y_ind, color);
        }
    }
}

fn raytrace_recursive<T>(params: &RaytraceParameters<T>, ray: Ray<T>, bounces: i32, intensity: f32) -> RGBColor where 
    Vec3<T>: Vec3View<T> + std::ops::Sub<Output=Vec3<T>>,
    T: num_traits::Float {

    let potential_hits = params.scene.objects.iter()
        .map(|obj| (obj, obj.test_intersection(&ray)))
        .filter(|(_, rch)| rch.is_some())
        .map(|(obj, rch)| (obj, rch.unwrap()));

    let closest_hit = potential_hits
        .min_by(|a,b| hit_dist_comp(&ray, &a.1, &b.1));

    if let Some((obj, hit)) = closest_hit {

        let mat_provider = obj.get_material_provider();

        let mat = mat_provider.get_material_at(&hit);

        hit_object(params, mat, &hit, &ray, bounces, intensity)
    } else {
        hit_skybox(&ray)
    }
}

fn hit_object<T>(params: &RaytraceParameters<T>, mat: &Material, hit: &GeometryHitInfo<T>, ray: &Ray<T>, bounces: i32, intensity: f32) -> RGBColor
    where T: num_traits::Float {
    
    let mut output = RGBColor::BLACK;

    // Influence of material color (all rays that are neither reflected nor refracted)
    output += RGBColor::from(mat.color)
    * intensity
    * ((mat.color.a * (1.0 - mat.reflection.intensity)) + ((1.0 - mat.color.a) * (1.0 - mat.refraction.intensity)));

    // Abort if we hit the bounce limit
    if bounces == params.render_params.max_bounces {
        return output
    }

    // Intensity "budgets" for reflection and refraction
    let reflection_intensity = intensity * mat.color.a * mat.reflection.intensity;
    let refraction_intensity = intensity * (1.0 - mat.color.a) * mat.refraction.intensity;

    // Ray budget for reflection
    let reflection_rays = ((mat.reflection.intensity / (mat.reflection.intensity + mat.refraction.intensity)) * (params.render_params.max_rays as f32)) as i32;

    // Calculate reflective influence on color if the influence threshold is met
    if reflection_intensity > params.render_params.min_intensity {
        
        // Special case for perfect reflection; We only need to send out a single ray
        if 0.0 == mat.reflection.max_angle {

            output += raytrace_recursive(params, 
                Ray {
                    origin: hit.position,
                    direction: ray.direction.reflect(hit.normal).into_normalized()
                }, bounces + 1, reflection_intensity) * reflection_intensity;

        } else {
            unimplemented!()
        }

    }

    // Ray budget for refraction
    let refraction_rays = params.render_params.max_rays - reflection_rays;

    // Calculate refractive influence on color if the influence threshold is met
    if refraction_intensity > params.render_params.min_intensity {
        unimplemented!()
    }

    output
}

fn hit_skybox<T>(ray: &Ray<T>) -> RGBColor where T: num_traits::Float {
    
    // Dumbass implementation that makes the sky a checkerboard

    if ray.direction.y() >= T::zero() {
        return RGBColor::BLACK
    }

    let t = -(ray.origin.y() + T::from(10.0).unwrap()) / ray.direction.y();

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

fn hit_dist_comp<T>(ray: &Ray<T>, a: &GeometryHitInfo<T>, b: &GeometryHitInfo<T>) -> cmp::Ordering where
    Vec3<T>: Vec3View<T> + std::ops::Sub<Output=Vec3<T>>,
    T: num_traits::Float {

    let dist = |rch: &GeometryHitInfo<T>| {
        (rch.position - ray.origin).sqr_length()
    };

    dist(a).partial_cmp(&dist(b)).unwrap_or(cmp::Ordering::Equal)
}