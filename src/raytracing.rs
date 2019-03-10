use super::vec3::*;
use super::camera::*;
use super::output::*;
use super::scene::*;
use super::color::*;

use std::convert::From;

pub struct Ray<T> {
    pub origin: Vec3<T>,
    pub direction: Vec3Norm<T>
}

pub struct RayHitInfo {


}

pub trait RayTarget<T> {

    fn test_intersection(&self, ray: &Ray<T>) -> Option<RayHitInfo>;

}

pub fn render<T>(scene: &Scene<T>, camera: &Camera<T>, render_target: &mut RenderTarget) where 
    T: num_traits::Float + From<i32> + From<f32> + Copy {

    // TODO: Replace this with something properly respecting camera parameters

    // Distances between 2 pixels
    let x_step = camera.viewport.width / (From::from(render_target.width));
    let x_start = (x_step - camera.viewport.width) / From::from(2.0);

    let y_step = camera.viewport.height / (From::from(render_target.height));
    let y_start = (y_step - camera.viewport.height) / From::from(2.0);

    for y_ind in 0..render_target.height {

        let wtf_rust_y: T = From::from(y_ind);
        let vp_y = y_start + wtf_rust_y * y_step;

        for x_ind in 0..render_target.width {

            let wtf_rust_x: T = From::from(x_ind);
            let vp_x = x_start + wtf_rust_x * x_step;
            
            let color = raytrace_recursive(scene, Ray{
                origin: Vec3(vp_x, vp_y, T::zero()), // TODO: Respect camera position and orientation
                direction: Vec3::normalized(T::zero(), T::zero(), T::one())
            });

            render_target.set_pixel(x_ind, y_ind, color);
        }
    }
}

fn raytrace_recursive<T>(scene: &Scene<T>, ray: Ray<T>) -> RGBColor {
    RGBColor {
        r: 1.0,
        g: 0.0,
        b: 0.0
    }
}