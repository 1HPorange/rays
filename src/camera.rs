use super::vec3::*;

pub struct Camera<T> {

    pub position: Vec3<T>,
    pub view_direction: Vec3Norm<T>,
    pub viewport: ViewPort<T>,
    pub fov_horizontal: f32 // in degrees TODO: BOUND CHECK
}

pub struct ViewPort<T> {
    pub width: T,
    pub height: T
}

impl<T> Camera<T> {

    pub fn default_with_viewport(viewport: ViewPort<T>) -> Self where T: num_traits::Zero + num_traits::One + num_traits::Float + std::fmt::Debug {

        Camera {
            position: Vec3::zero(),
            view_direction: Vec3::normalized(T::zero(), T::zero(), T::one()),
            viewport,
            fov_horizontal: 60.0
        }

    }

}