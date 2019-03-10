use super::vec3::*;

use std::convert::From;

pub struct Camera<T> {

    pub position: Vec3<T>,
    pub view_direction: Vec3Norm<T>,
    pub viewport: ViewPort<T>,
    pub fov_horizontal: T // in degrees TODO: BOUND CHECK TODO: Respect
    // TODO: DoF
}

pub struct ViewPort<T> {
    pub width: T,
    pub height: T
}

impl<T> Camera<T> where T: num_traits::Float{

    pub fn default() -> Self where T: std::fmt::Debug + From<f32> {

        Camera {
            position: Vec3::zero(),
            view_direction: Vec3::normalized(T::zero(), T::zero(), T::one()),
            viewport: ViewPort {
                width: From::from(16.0),
                height: From::from(9.0)
            },
            fov_horizontal: From::from(60.0)
        }

    }

}