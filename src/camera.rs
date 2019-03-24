use super::vec3::*;
use super::util;

#[derive(Debug, Copy, Clone)]
pub struct Camera<T> {

    pub position: Vec3<T>,
    pub orientation: Vec3<T>,
    pub viewport: ViewPort<T>,
    pub fov_horizontal: T
}

#[derive(Debug, Copy, Clone)]
pub struct ViewPort<T> {
    pub width: T,
    pub height: T
}

impl<T> ViewPort<T> where T: num_traits::Float {

    pub fn aspect(&self) -> T {
        self.width / self.height
    }

}

impl<T> Camera<T> where T: num_traits::Float {

    // Orientation in degrees around the x,y, and z axis
    pub fn new(position: Vec3<T>, orientation: Vec3<T>, viewport: ViewPort<T>, fov_horizontal: T) -> Camera<T> {

        Camera {
            position,
            orientation,
            viewport,
            fov_horizontal
        }

    }

    pub fn validate(&self) -> bool {

        let mut success = true;

        if !util::is_in_range_exclusive(self.viewport.width, T::zero(), T::infinity()) {
            println!("Error: Vieport width must be positive and finite");
            success = false;
        }

        if !util::is_in_range_exclusive(self.viewport.height, T::zero(), T::infinity()) {
            println!("Error: Vieport height must be positive and finite");
            success = false;
        }

        if !util::is_in_range(self.fov_horizontal, T::zero(), T::from(180.0).unwrap()) {
            println!("Warning: Viewport outside of usual range. This can be intential, but will look pretty weird.");
        }

        success
    }
}