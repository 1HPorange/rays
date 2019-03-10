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

pub struct RayHitInfo<T> {

    pub position: Vec3<T>,
    pub normal: Vec3Norm<T> // TODO: Make the calculation of this optional or think of something else. Sometimes we just need a hit test

}

pub struct RenderingParameters {
    pub max_bounces: i32,
    pub max_rays: i32
}

pub trait RayTarget<T> {

    fn test_intersection(&self, ray: &Ray<T>) -> Option<RayHitInfo<T>>;

}

pub fn render<T>(scene: &Scene<T>, camera: &Camera<T>, render_target: &mut RenderTarget, rendering_parameters: &RenderingParameters) where 
    T: num_traits::Float + Copy {

    // TODO: Replace this with something properly respecting camera parameters

    // Distances between 2 pixels
    let x_step = camera.viewport.width / (T::from(render_target.width).unwrap());
    let x_start = (x_step - camera.viewport.width) / T::from(2.0).unwrap();

    let y_step = camera.viewport.height / (T::from(render_target.height).unwrap());
    let y_start = (y_step - camera.viewport.height) / T::from(2.0).unwrap();

    for y_ind in 0..render_target.height {

        let wtf_rust_y: T = T::from(y_ind).unwrap();
        let vp_y = y_start + wtf_rust_y * y_step;

        for x_ind in 0..render_target.width {

            let wtf_rust_x: T = T::from(x_ind).unwrap();
            let vp_x = x_start + wtf_rust_x * x_step;
            
            let color = raytrace_recursive(scene, Ray{
                origin: Vec3(vp_x, vp_y, T::zero()), // TODO: Respect camera position and orientation
                direction: Vec3::normalized(T::zero(), T::zero(), T::one())
            }, 0);

            render_target.set_pixel(x_ind, y_ind, color);
        }
    }
}

fn raytrace_recursive<T>(scene: &Scene<T>, ray: Ray<T>, bounces: i32) -> RGBColor where 
    Vec3<T>: Vec3View<T> + std::ops::Sub<Output=Vec3<T>>,
    T: num_traits::Float {

    let potential_hits = scene.objects.iter()
        .map(|obj| (obj, obj.test_intersection(&ray)))
        .filter(|(_, rch)| rch.is_some())
        .map(|(obj, rch)| (obj, rch.unwrap()));

    let closest_hit = potential_hits
        .min_by(|a,b| hit_dist_comp(&ray, &a.1, &b.1));

    if let Some((obj, hit)) = closest_hit {

        // TODO: Proper recursion and evaluation
        let mat_provider = obj.get_material_provider();

        let mat = mat_provider.get_material_at(&hit);

        mat.color.into()

    } else {
        RGBColor::BLACK // TODO: Evaluate skybox
    }
}

fn hit_dist_comp<T>(ray: &Ray<T>, a: &RayHitInfo<T>, b: &RayHitInfo<T>) -> cmp::Ordering where
    Vec3<T>: Vec3View<T> + std::ops::Sub<Output=Vec3<T>>,
    T: num_traits::Float {

    let dist = |rch: &RayHitInfo<T>| {
        (rch.position - ray.origin).sqr_length()
    };

    dist(a).partial_cmp(&dist(b)).unwrap_or(cmp::Ordering::Equal)
}