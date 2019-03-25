use super::vec::*;
use super::util;

#[derive(Debug, Copy, Clone)]
pub struct Camera {

    pub position: Vec3,
    pub orientation: Vec3,
    pub viewport: ViewPort,
    pub fov_horizontal: f64
}

#[derive(Debug, Copy, Clone)]
pub struct ViewPort {
    pub width: f64,
    pub height: f64
}

impl ViewPort {

    pub fn aspect(&self) -> f64 {
        self.width / self.height
    }

}

impl Camera {

    // Orientation in degrees around the x,y, and z axis
    pub fn new(position: Vec3, orientation: Vec3, viewport: ViewPort, fov_horizontal: f64) -> Camera {

        Camera {
            position,
            orientation,
            viewport,
            fov_horizontal
        }

    }

    pub fn validate(&self) -> bool {

        let mut success = true;

        if !util::is_in_range_exclusive(self.viewport.width, 0.0, std::f64::INFINITY) {
            println!("Error: Vieport width must be positive and finite");
            success = false;
        }

        if !util::is_in_range_exclusive(self.viewport.height, 0.0, std::f64::INFINITY) {
            println!("Error: Vieport height must be positive and finite");
            success = false;
        }

        if !util::is_in_range(self.fov_horizontal, 0.0, 180.0) {
            println!("Warning: Viewport outside of usual range. This can be intential, but will look pretty weird.");
        }

        success
    }
}