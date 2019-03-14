extern crate num_traits;

use std::ops::*;

#[derive(Debug, Copy, Clone)]
pub struct Vec3<T>(pub T, pub T, pub T);

#[derive(Debug, Copy, Clone)]
pub struct Vec3Norm<T>(Vec3<T>);

const EPSILON: f32 = 0.00001;

// TODO: Move these constants somewhere smarter
pub const DEG_2_RAD: f64 = 0.0174532925199433;
//pub const RAD_2_DEG: f64 = 57.2957795130823;

impl<T> Vec3<T> where T: num_traits::Float {

    pub fn zero() -> Vec3<T> {
        Vec3(T::zero(), T::zero(), T::zero())
    } 

    pub fn one() -> Vec3<T> {
        Vec3(T::one(), T::one(), T::one())
    } 

    pub fn is_unit_length(&self) -> bool {
        (self.sqr_length() - T::one()).abs() < T::from(EPSILON).unwrap()
    }

    pub fn normalized(x: T, y: T, z: T) -> Vec3Norm<T> {

        let v = Vec3(x, y, z);

        assert!(v.is_unit_length(), "Cannot construct normalized vector that is not unit length");

        Vec3Norm(v)
    }

    pub fn normalize(self) -> Vec3Norm<T> {

        assert!(!self.is_zero(), "Cannot normalize zero length vector");

        Vec3Norm(self / self.length())
    }

    pub fn into_normalized(self) -> Vec3Norm<T> {

        assert!(self.is_unit_length(), "Cannot construct normalized vector that is not unit length");

        Vec3Norm(self)
    }

    // TODO: Make these function not mutate self. Then they can move into vecview

    pub fn rotate_x(&mut self, mut deg: T) {

        deg = deg * T::from(DEG_2_RAD).unwrap();

        let old = *self;

        self.1 = old.1 * deg.cos() - old.2 * deg.sin();
        self.2 = old.1 * deg.sin() + old.2 * deg.cos();
    }

    pub fn rotate_y(&mut self, mut deg: T) {

        deg = deg * T::from(DEG_2_RAD).unwrap();

        let old = *self;

        self.0 = old.0 * deg.cos() + old.2 * deg.sin();
        self.2 = -old.0 * deg.sin() + old.2 * deg.cos();
    }

    pub fn rotate_z(&mut self, mut deg: T) {

        deg = deg * T::from(DEG_2_RAD).unwrap();

        let old = *self;

        self.0 = old.0 * deg.cos() - old.1 * deg.sin();
        self.1 = old.0 * deg.sin() + old.1 * deg.cos();
    }
}

impl<T,R> AddAssign<R> for Vec3<T> where R: Vec3View<T>, T: num_traits::Float {

    fn add_assign(&mut self, other: R) {
        self.0 = self.0 + other.x();
        self.1 = self.1 + other.y();
        self.2 = self.2 + other.z();
    }

}

// This trait is there to grant/force read-only access to the fields of a vector
pub trait Vec3View<T>: Sized + Copy where T: num_traits::Float {
    fn x(&self) -> T;
    fn y(&self) -> T;
    fn z(&self) -> T;

    fn scale<S>(self, scalar: S) -> Vec3<T> where T: Mul<S, Output=T>, S: Copy {
        Vec3(self.x() * scalar, self.y() * scalar, self.z() * scalar)
    }

    fn dot<R>(self, rhs: R) -> T where R: Vec3View<T> {
        self.x() * rhs.x() + self.y() * rhs.y() + self.z() * rhs.z()
    }

    fn cross<R>(self, rhs: R) -> Vec3<T> where R: Vec3View<T> {
        Vec3(
            self.y() * rhs.z() - self.z() * rhs.y(),
            self.z() * rhs.x() - self.x() * rhs.z(),
            self.x() * rhs.y() - self.y() * rhs.x()
        )
    }

    fn length(&self) -> T {
        self.sqr_length().sqrt()
    }

    fn sqr_length(&self) -> T {
        self.x() * self.x() + self.y() * self.y() + self.z() * self.z()
    }

    fn is_zero(&self) -> bool {
        self.x().is_zero() &&
        self.y().is_zero() &&
        self.z().is_zero()
    }

    fn reflect(&self, normal: Vec3Norm<T>) -> Vec3<T> where Self: Sub<Vec3<T>, Output=Vec3<T>> {
        *self - normal * (self.dot(normal)) * T::from(2.0).unwrap()
    }

    // Rotates the vector so that the old y axis is rotated into ny
    // the x and z axis are chosen arbitrarily, but deterministically
    fn reorient_y_axis<V>(self, ny: V) -> Vec3<T> where V: Vec3View<T> {

        assert!(!ny.is_zero());

        // Calculate other base vectors

        let nx = if !ny.z().is_zero() {
            Vec3(T::one(), T::one(), -(ny.x() + ny.y())/ny.z() ).normalize()
        } else if !ny.y().is_zero() {
            Vec3(T::one(), T::one(), -(ny.x() + ny.z())/ny.y() ).normalize()
        } else {
            Vec3(T::one(), T::one(), -(ny.y() + ny.z())/ny.x() ).normalize()
        };

        let nz = nx.cross(ny);

        Vec3(
            self.x() * nx.x() + self.y() * ny.x() + self.z() * nz.x(),
            self.x() * nx.y() + self.y() * ny.y() + self.z() * nz.y(),
            self.x() * nx.z() + self.y() * ny.z() + self.z() * nz.z()
        )
    }
}

impl<T> Vec3View<T> for Vec3<T> where T: num_traits::Float {

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

impl<T> Vec3View<T> for Vec3Norm<T> where T: num_traits::Float {

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

impl<T> From<Vec3Norm<T>> for Vec3<T> where Vec3Norm<T>: Vec3View<T>, T: num_traits::Float {

    fn from(v: Vec3Norm<T>) -> Vec3<T> {
        Vec3(
            v.0.x(),
            v.0.y(),
            v.0.z()
        )
    }

}

macro_rules! operators_impl {
    ($($t:ty)*) => ($(

        impl<T> Neg for $t where Self: Vec3View<T>, T: num_traits::Float {

            type Output = Vec3<T>;

            fn neg(self) -> Self::Output {
                Vec3(
                    -self.x(),
                    -self.y(),
                    -self.z()
                )
            }

        }

        impl<T,S> Mul<S> for $t where Self: Vec3View<T>, S: Copy, T: num_traits::Float + Mul<S, Output=T> {

            type Output = Vec3<T>;

            fn mul(self, rhs: S) -> Self::Output {
                self.scale(rhs)
            }

        }

       impl<T,S> Div<S> for $t where Self: Vec3View<T>, S: Copy, T: Div<S, Output=T> + num_traits::Float {

            type Output = Vec3<T>;

            fn div(self, rhs: S) -> Self::Output {
                self.scale(T::one() / rhs)
            }

        }

        impl<T,R> Add<R> for $t where Self: Vec3View<T>, R: Vec3View<T>, T: num_traits::Float {

            type Output = Vec3<T>;

            fn add(self, rhs: R) -> Self::Output {
                Vec3(
                    self.x() + rhs.x(),
                    self.y() + rhs.y(),
                    self.z() + rhs.z())
            }
        }

        impl<T,R> Sub<R> for $t where Self: Vec3View<T>, R: Vec3View<T>, T: num_traits::Float {

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