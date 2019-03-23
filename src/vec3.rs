use std::ops::*;

// TODO: Rename this module to "vec" because this thing is in it
#[derive(Debug, Copy, Clone)]
pub struct Vec2<T>(pub T, pub T);

#[derive(Debug, Copy, Clone)]
pub struct Vec3<T>(pub T, pub T, pub T);

#[derive(Debug, Copy, Clone)]
pub struct Vec3Norm<T>(pub T, pub T, pub T);

/// Use this if the normal T::epsilon() is too small
const BIG_EPSILON: f64 = 0.00001;

// TODO: Move these constants somewhere smarter
//pub const DEG_2_RAD: f64 = 0.0174532925199433;
//pub const RAD_2_DEG: f64 = 57.2957795130823;

impl<T> Vec3<T> where T: num_traits::Float {

    pub fn zero() -> Vec3<T> {
        Vec3(T::zero(), T::zero(), T::zero())
    } 

    pub fn one() -> Vec3<T> {
        Vec3(T::one(), T::one(), T::one())
    } 

    pub fn is_unit_length(&self) -> bool {
        (self.sqr_length() - T::one()).abs() < T::from(BIG_EPSILON).unwrap()
    }

    pub fn normalized(x: T, y: T, z: T) -> Vec3Norm<T> {

        let v = Vec3(x, y, z);

        assert!(v.is_unit_length(), "Cannot construct normalized vector that is not unit length");

        Vec3Norm(x, y, z)
    }

    pub fn normalize(self) -> Vec3Norm<T> {

        assert!(!self.is_zero(), "Cannot normalize zero length vector");

        let norm = self / self.length();

        Vec3Norm(norm.0, norm.1, norm.2)
    }

    pub fn into_normalized(self) -> Vec3Norm<T> {

        // let discrepancy: f32 = num_traits::NumCast::from(self.length() - T::one()).unwrap();
        assert!(self.is_unit_length(), "Cannot construct normalized vector that is not unit length");// (Discrepancy: {})", discrepancy);

        Vec3Norm(self.0, self.1, self.2)
    }

    // TODO: Make these function not mutate self. Then they can move into vecview

    pub fn rotate_x(&mut self, mut deg: T) {

        let rad = deg.to_radians();

        let old = *self;

        self.1 = old.1 * rad.cos() - old.2 * rad.sin();
        self.2 = old.1 * rad.sin() + old.2 * rad.cos();
    }

    pub fn rotate_y(&mut self, mut deg: T) {

        let rad = deg.to_radians();

        let old = *self;

        self.0 = old.0 * rad.cos() + old.2 * rad.sin();
        self.2 = -old.0 * rad.sin() + old.2 * rad.cos();
    }

    pub fn rotate_z(&mut self, mut deg: T) {

        let rad = deg.to_radians();

        let old = *self;

        self.0 = old.0 * rad.cos() - old.1 * rad.sin();
        self.1 = old.0 * rad.sin() + old.1 * rad.cos();
    }

    pub fn interpolate_into<V: Vec3View<T>>(self, target: V, t: T) -> Vec3<T> where V: Mul<T, Output=Vec3<T>> {
        self * (T::one() - t) + target * t
    }
}

impl<T> Vec3Norm<T> where T: num_traits::Float {

    pub fn up() -> Vec3Norm<T> { Vec3Norm(T::zero(), T::one(), T::zero()) }
    pub fn down() -> Vec3Norm<T> { Vec3Norm(T::zero(), -T::one(), T::zero()) }

    pub fn right() -> Vec3Norm<T> { Vec3Norm(T::one(), T::zero(), T::zero()) }
    pub fn left() -> Vec3Norm<T> { Vec3Norm(-T::one(), T::zero(), T::zero()) }

    pub fn forward() -> Vec3Norm<T> { Vec3Norm(T::zero(), T::zero(), T::one()) }
    pub fn back() -> Vec3Norm<T> { Vec3Norm(T::zero(), T::zero(), -T::one()) }

    // https://en.wikipedia.org/wiki/Rotation_matrix#Rotation_matrix_from_axis_and_angle
    pub fn rotate_around_axis(&mut self, u: Vec3Norm<T>, deg: T) {
        
        let rad = deg.to_radians();

        let old = *self;

        let cos = rad.cos();
        let sin = rad.sin();

        self.0 =    old.0 * (cos + u.0 * u.0 * (T::one() - cos)) + 
                    old.1 * (u.0 * u.1 * (T::one() - cos) - u.2 * sin) +
                    old.2 * (u.0 * u.2 * (T::one() - cos) + u.1 * sin);
        
        self.1 =    old.0 * (u.1 * u.0 * (T::one() - cos) + u.2 * sin) +
                    old.1 * (cos + u.1 * u.1 * (T::one() - cos)) +
                    old.2 * (u.1 * u.2 * (T::one() - cos) - u.0 * sin);

        self.2 =    old.0 * (u.2 * u.0 * (T::one() - cos) - u.1 * sin) +
                    old.1 * (u.2 * u.1 * (T::one() - cos) + u.0 * sin) +
                    old.2 * (cos + u.2 * u.2 * (T::one() - cos));
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

    // TODO: Think about when and why to take moved self here

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

    fn get_random_90_deg_vector(self) -> Vec3<T> {

        assert!(!self.is_zero());

        if !self.x().is_zero() {
            Vec3(- (self.y() + self.z()) / self.x(), T::one(), T::one())
        } else if !self.y().is_zero() {
            Vec3(T::one(), - (self.x() + self.z()) / self.y(), T::one())
        } else {
            Vec3(T::one(), T::one(), - (self.x() + self.y()) / self.z())
        }
    }

    // https://math.stackexchange.com/questions/878785/how-to-find-an-angle-in-range0-360-between-2-vectors
    // The normal parameter is the normal of the plane that self and v lie on. This allows us to define 360 deg rotation sensibly
    fn angle_to_planar<V: Vec3View<T>>(self, v: V, n: Vec3Norm<T>, allow_negative_angles: bool) -> T where T: num_traits::FloatConst {
        
        let dot = self.dot(v);
        let det = n.dot(self.cross(v));

        let angle = det.atan2(dot).to_degrees();

        if allow_negative_angles {
            angle
        } else {
            if angle >= T::zero() {
                angle
            } else {
                angle + T::from(360.0).unwrap()
            }
        }
        
    }

    fn project_onto_plane_from_same_origin(self, plane_normal: Vec3Norm<T>) -> Vec3<T> where Self: Sub<Vec3<T>, Output=Vec3<T>> {

        let non_projected_part = self.dot(plane_normal);

        self - plane_normal * non_projected_part
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

    // TODO: Remove this vector nesting so we can use macros to switch out Vec3 and Vec3Norm constructors for specialized return values

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

impl<T> Vec3View<T> for Vec2<T> where T: num_traits::Float {

    fn x(&self) -> T {
        self.0
    }

    fn y(&self) -> T {
        self.1
    }

    fn z(&self) -> T {
        T::zero()
    }
}

impl<T> From<Vec3Norm<T>> for Vec3<T> where Vec3Norm<T>: Vec3View<T>, T: num_traits::Float {

    fn from(v: Vec3Norm<T>) -> Vec3<T> {
        Vec3(
            v.0,
            v.1,
            v.2
        )
    }

}

// It would be nice to implement this for a V: Vec3View<T>, but: https://github.com/rust-lang/rust/issues/50238
impl<T> From<Vec3<T>> for Vec2<T> where {

    fn from(v3: Vec3<T>) -> Vec2<T> {
        Vec2(v3.0, v3.1)
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