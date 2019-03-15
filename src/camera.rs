use super::vec3::*;

use std::convert::From;
use num_traits::NumCast;

pub struct Camera<T> {

    pub position: Vec3<T>,
    pub orientation: Orientation<T>,
    pub viewport: ViewPort<T>,
    pub fov_horizontal: T,
    pub dof_angle: T
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
                T::from(15.0).unwrap(),
                T::from(-10.0).unwrap()
            ),
            orientation: Orientation { 
                x: T::from(25.0).unwrap(),
                y: T::zero(),
                z: T::zero()
            },
            viewport: ViewPort {
                width: T::from(16.0).unwrap(),
                height: T::from(9.0).unwrap()
            },
            fov_horizontal: NumCast::from(60.0).unwrap(),
            dof_angle: T::from(0.1).unwrap()
        }

    }

}