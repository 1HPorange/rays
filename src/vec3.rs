extern crate num_traits;

use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;

#[derive(Debug, Copy, Clone)]
pub struct Vec3<T>(pub T, pub T, pub T);

#[derive(Debug, Copy, Clone)]
pub struct Vec3Norm<T>(Vec3<T>);

impl<T> Vec3<T> {
    pub fn normalized(x: T, y: T, z: T) -> Vec3Norm<T> where T: PartialEq + num_traits::Float + num_traits::One + std::fmt::Debug {

        let v = Vec3(x, y, z);

        assert_eq!(v.length(), T::one(), "Cannot construct normalized vector out of input data!");

        Vec3Norm(v)
    }
}

// This trait is there to grant/force read-only access to the fields of a vector
pub trait Vec3View<T>: Copy {
    fn x(&self) -> T;
    fn y(&self) -> T;
    fn z(&self) -> T;

    fn scale<S>(self, scalar: S) -> Vec3<T> where T: Mul<S, Output=T>, S: Copy, Self: Sized {
        Vec3(self.x() * scalar, self.y() * scalar, self.z() * scalar)
    }

    fn dot<R>(self, rhs: R) -> T where R: Vec3View<T>, T: Mul<Output=T> + Add<Output=T>, Self: Sized {
        self.x() * rhs.x() + self.y() * rhs.y() + self.z() * rhs.z()
    }

    fn cross<R>(self, rhs: R) -> Vec3<T> where R: Vec3View<T>, T: Mul<Output=T> + Sub<Output=T> + Copy, Self: Sized {
        Vec3(
            self.y() * rhs.z() - self.z() * rhs.y(),
            self.z() * rhs.x() - self.x() * rhs.z(),
            self.x() * rhs.y() - self.y() * rhs.x()
        )
    }

    fn length(&self) -> T where T: Mul<Output=T> + Add<Output=T> + num_traits::Float {
        (self.x() * self.x() + self.y() * self.y() + self.z() * self.z()).sqrt()
    }
}

impl<T> Vec3View<T> for Vec3<T> where T: Copy {

    fn x(&self) -> T {
        self.0
    }

    fn y(&self) -> T {
        self.1
    }

    fn z(&self) -> T {
        self.2
    }
}

impl<T> Vec3View<T> for Vec3Norm<T> where T: Copy {

    fn x(&self) -> T {
        self.0.x()
    }

    fn y(&self) -> T {
        self.0.y()
    }

    fn z(&self) -> T {
        self.0.z()
    }
}

// TODO: Write macro for the stuff below

impl<T,R> Mul<R> for Vec3<T> where Self: Vec3View<T>, R: Vec3View<T>, T: Mul<Output=T> + Add<Output=T> {

    type Output = T;

    fn mul(self, rhs: R) -> Self::Output {
        self.dot(rhs)
    }

}

impl<T,R> Mul<R> for Vec3Norm<T> where Self: Vec3View<T>, R: Vec3View<T>, T: Mul<Output=T> + Add<Output=T> {

    type Output = T;

    fn mul(self, rhs: R) -> Self::Output {
        self.dot(rhs)
    }

}