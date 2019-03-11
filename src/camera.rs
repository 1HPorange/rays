use super::vec3::*;

use std::convert::From;
use num_traits::NumCast;

pub struct Camera<T> {

    pub position: Vec3<T>,
    pub orientation: Orientation<T>,
    pub viewport: ViewPort<T>,
    pub fov_horizontal: T // in degrees TODO: BOUND CHECK TODO: Respect
    // TODO: DoF
}

pub struct ViewPort<T> {
    pub width: T,
    pub height: T
}

pub struct Orientation<T> {
    pub x: T,
    pub y: T,
    pub z: T
}

impl<T> ViewPort<T> where T: num_traits::Float {

    pub fn aspect(&self) -> T {
        self.width / self.height
    }

}

impl<T> Camera<T> where T: num_traits::Float{

    pub fn default() -> Self {

        Camera {
            position: Vec3(
                T::zero(),
                T::from(5.0).unwrap(),
                T::zero()
            ),
            orientation: Orientation { 
                x: T::from(36.0).unwrap(),
                y: T::zero(),
                z: T::zero()
            },
            viewport: ViewPort {
                width: NumCast::from(16.0).unwrap(),
                height: NumCast::from(9.0).unwrap()
            },
            fov_horizontal: NumCast::from(60.0).unwrap()
        }

    }

}