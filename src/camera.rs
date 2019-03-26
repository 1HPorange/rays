use super::vec::*;
use super::util;
use serde::Deserialize;

#[derive(Debug, Copy, Clone, Deserialize)]
#[serde(default)]
pub struct Camera {

    #[serde(default = "default_position")]
    pub position: Vec3,

    pub rotation: Vec3,
    pub viewport: ViewPort,
    
    #[serde(default = "default_fov")]
    pub fov_h: f64
}

fn default_position() -> Vec3 {
    Vec3::new(0.0, 0.0, -10.0)
}

fn default_fov() -> f64 {
    60.0
}

#[derive(Debug, Copy, Clone, Deserialize)]
pub struct ViewPort {
    pub width: f64,
    pub height: f64
}

impl ViewPort {

    pub fn aspect(&self) -> f64 {
        self.width / self.height
    }

}

impl Default for ViewPort {
    fn default() -> ViewPort {
        ViewPort {
            width: 16.0,
            height: 9.0
        }
    }
}

impl Camera {

    /// Rotation in degrees around the x,y, and z axis
    pub fn new(position: Vec3, rotation: Vec3, viewport: ViewPort, fov_h: f64) -> Camera {

        Camera {
            position,
            rotation,
            viewport,
            fov_h
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

        if !util::is_in_range(self.fov_h, 0.0, 180.0) {
            println!("Warning: FoV outside of usual range. This can be intentional, but will look pretty weird.");
        }

        success
    }
}

impl Default for Camera {
    fn default() -> Self {
        Camera {
            position: Vec3::new(0.0, 0.0, -10.0),
            rotation: Vec3::ZERO,
            viewport: ViewPort::default(),
            fov_h: 60.0
        }
    }
}