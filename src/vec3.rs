extern crate num_traits;

use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;

#[derive(Debug, Copy, Clone)]
pub struct Vec3<T>(pub T, pub T, pub T);

#[derive(Debug, Copy, Clone)]
pub struct Vec3Norm<T>(Vec3<T>);

impl<T> Vec3<T> {

    pub fn zero() -> Vec3<T> where T: num_traits::Zero {
        Vec3(T::zero(), T::zero(), T::zero())
    } 

    pub fn is_zero(&self) -> bool where T: num_traits::Zero + PartialEq {
        self.0 == T::zero() &&
        self.1 == T::zero() &&
        self.2 == T::zero()
    }

    pub fn is_unit_length(&self) -> bool where T: num_traits::One + num_traits::Float {
        self.length() == T::one()
    }

    pub fn normalized(x: T, y: T, z: T) -> Vec3Norm<T> where T: PartialEq + num_traits::Float + num_traits::One + std::fmt::Debug {

        let v = Vec3(x, y, z);

        assert!(v.is_unit_length(), "Cannot construct normalized vector that is not unit length");

        Vec3Norm(v)
    }

    pub fn normalize(self) -> Vec3Norm<T> where Self: Vec3View<T>, T: num_traits::Zero + PartialEq + num_traits::Float {

        assert!(!self.is_zero(), "Cannot normalize zero length vector");

        Vec3Norm(self / self.length())
    }

    pub fn into_normalized(self) -> Vec3Norm<T> where T: num_traits::Float {

        assert!(self.is_unit_length(), "Cannot construct normalized vector that is not unit length");

        Vec3Norm(self)
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

macro_rules! operators_impl {
    ($($t:ty)*) => ($(
        impl<T,S> Mul<S> for $t where Self: Vec3View<T>, S: Copy, T: Mul<S, Output=T> {

            type Output = Vec3<T>;

            fn mul(self, rhs: S) -> Self::Output {
                self.scale(rhs)
            }

        }

       impl<T,S> Div<S> for $t where Self: Vec3View<T>, S: Copy, T: Div<S, Output=T> + num_traits::One + Copy {

            type Output = Vec3<T>;

            fn div(self, rhs: S) -> Self::Output {
                self.scale(T::one() / rhs)
            }

        }

        impl<T,R> Add<R> for $t where Self: Vec3View<T>, R: Vec3View<T>, T: Add<Output=T> {

            type Output = Vec3<T>;

            fn add(self, rhs: R) -> Self::Output {
                Vec3(
                    self.x() + rhs.x(),
                    self.y() + rhs.y(),
                    self.z() + rhs.z())
            }
        }

        impl<T,R> Sub<R> for $t where Self: Vec3View<T>, R: Vec3View<T>, T: Sub<Output=T> {

            type Output = Vec3<T>;

            fn sub(self, rhs: R) -> Self::Output {
                Vec3(
                    self.x() - rhs.x(),
                    self.y() - rhs.y(),
                    self.z() - rhs.z())
            }
        }
    )*)
}

operators_impl! { Vec3<T> Vec3Norm<T> }