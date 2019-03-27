use std::ops::*;

// TODO: Rename this module to "vec" because this thing is in it
#[derive(Debug, Copy, Clone)]
pub struct Vec2 {
    pub u: f64,
    pub v: f64
}

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

#[derive(Debug, Copy, Clone)]
pub struct Vec3Norm {
    x: f64,
    y: f64,
    z: f64
}

impl Vec2 {

    pub fn new(u: f64, v: f64) -> Vec2 {
        Vec2 { u, v }
    }
}

impl Vec3 {

    pub const ZERO: Vec3 = Vec3 { x: 0.0, y: 0.0, z: 0.0 };

    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn normalized(self) -> Vec3Norm {

        assert!(!self.is_zero(), "Cannot normalize zero length vector");

        let norm = self / self.length();

        Vec3Norm { x: norm.x, y: norm.y, z: norm.z }
    }

    pub fn sqr_length(self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(self) -> f64 {
        self.sqr_length().sqrt()
    }

    fn is_unit_length(&self) -> bool {

        const BIG_EPSILON: f64 = 0.00001;

        (self.sqr_length() - 1.0).abs() < BIG_EPSILON
    }

    pub fn into_normalized_unsafe(self) -> Vec3Norm {

        debug_assert!(self.is_unit_length());

        Vec3Norm {
            x: self.x,
            y: self.y,
            z: self.z
        }
    }
}

impl Vec3Norm {

    pub const FORWARD: Vec3Norm = Vec3Norm { x: 0.0, y: 0.0, z: 1.0 };
    pub const BACK: Vec3Norm = Vec3Norm { x: 0.0, y: 0.0, z: -1.0 };

    pub const UP: Vec3Norm = Vec3Norm { x: 0.0, y: 1.0, z: 0.0 };
    pub const DOWN: Vec3Norm = Vec3Norm { x: 0.0, y: -1.0, z: 0.0 };

    pub const RIGHT: Vec3Norm = Vec3Norm { x: 1.0, y: 0.0, z: 0.0 };
    pub const LEFT: Vec3Norm = Vec3Norm { x: -1.0, y: 0.0, z: 0.0 };

    pub fn new(x: f64, y: f64, z: f64) -> Vec3Norm {

        let v = Vec3 { x, y, z };

        assert!(v.is_unit_length(), "Cannot construct normalized vector that is not unit length");

        Vec3Norm { x, y, z }
    }
}

pub trait Vec3View: Copy {

    fn x(self) -> f64;
    fn y(self) -> f64;
    fn z(self) -> f64;

    fn is_zero(&self) -> bool;

    // Free implementations

    fn interpolate_towards<V: Vec3View>(self, target: V, t: f64) -> Vec3 where V: Mul<f64, Output=Vec3>, Self: Mul<f64, Output=Vec3> {
        self * (1.0 - t) + target * t
    }

    fn scale(self, scalar: f64) -> Vec3 {
        Vec3 { 
            x: self.x() * scalar,
            y: self.y() * scalar,
            z: self.z() * scalar
        }
    }

    fn dot<R>(self, rhs: R) -> f64 where R: Vec3View {
        self.x() * rhs.x() + self.y() * rhs.y() + self.z() * rhs.z()
    }

    fn cross<R>(self, rhs: R) -> Vec3 where R: Vec3View {
        Vec3 {
            x: self.y() * rhs.z() - self.z() * rhs.y(),
            y: self.z() * rhs.x() - self.x() * rhs.z(),
            z: self.x() * rhs.y() - self.y() * rhs.x()
        }
    }

    /// https://math.stackexchange.com/questions/878785/how-to-find-an-angle-in-range0-360-between-2-vectors
    /// The normal parameter is the normal of the plane that self and v lie on. This allows us to define 360 deg rotation sensibly
    /// Setting the bool parameter to true shifts the range from [0, 360] to [-180,180]
    fn angle_to_on_plane<V: Vec3View>(self, v: V, n: Vec3Norm, allow_negative_angles: bool) -> f64 {
        
        let dot = self.dot(v);
        let det = n.dot(self.cross(v));

        let angle = det.atan2(dot).to_degrees();

        if allow_negative_angles {
            angle
        } else {
            if angle >= 0.0 {
                angle
            } else {
                angle + 360.0
            }
        }
        
    }

    /// Projects a vector onto a plane that goes through the origin
    fn project_onto_plane_through_origin(self, plane_normal: Vec3Norm) -> Vec3 where Self: Sub<Vec3, Output=Vec3> {

        let non_projected_part = self.dot(plane_normal);

        self - plane_normal * non_projected_part
    }

    fn get_random_90_deg_vector(self) -> Vec3 {

        assert!(!self.is_zero());

        if self.x() != 0.0 {
            Vec3::new(- (self.y() + self.z()) / self.x(), 1.0, 1.0)
        } else if self.y() != 0.0 {
            Vec3::new(1.0, - (self.x() + self.z()) / self.y(), 1.0)
        } else {
            Vec3::new(1.0, 1.0, - (self.x() + self.y()) / self.z())
        }
    }
}

impl Vec3View for Vec3 {

    fn x(self) -> f64 { self.x }
    fn y(self) -> f64 { self.y }
    fn z(self) -> f64 { self.z }

    fn is_zero(&self) -> bool {
        self.x() == 0.0 &&
        self.y() == 0.0 &&
        self.z() == 0.0
    }
}

impl Vec3View for Vec3Norm {

    fn x(self) -> f64 { self.x }
    fn y(self) -> f64 { self.y }
    fn z(self) -> f64 { self.z }

    fn is_zero(&self) -> bool {
        false
    }
}

impl From<Vec3Norm> for Vec3 {

    fn from(v: Vec3Norm) -> Vec3 {
        Vec3{
            x: v.x(),
            y: v.y(),
            z: v.z()
        }
    }
}

impl Default for Vec3 {

    fn default() -> Vec3 {
        Vec3::ZERO
    }
}

macro_rules! overlapping_impl {
    ($($t:ty)*) => ($(

        impl $t {
            pub fn rotate_x(self, deg: f64) -> Self {

                let rad = deg.to_radians();

                let x = self.x;
                let y = self.y * rad.cos() - self.z * rad.sin();
                let z = self.y * rad.sin() + self.z * rad.cos();

                Self { x, y, z }
            }

            pub fn rotate_y(self, deg: f64) -> Self {

                let rad = deg.to_radians();

                let x = self.x * rad.cos() + self.z * rad.sin();
                let y = self.y;
                let z = -self.x * rad.sin() + self.z * rad.cos();

                Self { x, y, z }
            }

            pub fn rotate_z(self, deg: f64) -> Self {

                let rad = deg.to_radians();

                let x = self.x * rad.cos() - self.y * rad.sin();
                let y = self.x * rad.sin() + self.y * rad.cos();
                let z =  self.z;

                Self { x, y, z }
            }

            pub fn rotate(self, rotation: Vec3) -> Self {

                self.rotate_x(rotation.x)
                    .rotate_y(rotation.y)
                    .rotate_z(rotation.z)
            }

            // https://en.wikipedia.org/wiki/Rotation_matrix#Rotation_matrix_from_axis_and_angle
            pub fn rotate_around_axis(self, u: Vec3Norm, deg: f64) -> Self {
                
                let rad = deg.to_radians();

                let cos = rad.cos();
                let sin = rad.sin();

                let x =    self.x * (cos + u.x * u.x * (1.0 - cos)) + 
                            self.y * (u.x * u.y * (1.0 - cos) - u.z * sin) +
                            self.z * (u.x * u.z * (1.0 - cos) + u.y * sin);
                
                let y =    self.x * (u.y * u.x * (1.0 - cos) + u.z * sin) +
                            self.y * (cos + u.y * u.y * (1.0 - cos)) +
                            self.z * (u.y * u.z * (1.0 - cos) - u.x * sin);

                let z =    self.x * (u.z * u.x * (1.0 - cos) - u.y * sin) +
                            self.y * (u.z * u.y * (1.0 - cos) + u.x * sin) +
                            self.z * (cos + u.z * u.z * (1.0 - cos));

                Self { x, y, z }
            }

            pub fn reflect(self, normal: Vec3Norm) -> Self {
                let reflected = self - normal * self.dot(normal) * 2.0;

                Self {
                    x: reflected.x,
                    y: reflected.y,
                    z: reflected.z
                }
            }
        }

        impl Mul<f64> for $t {

            type Output = Vec3;

            fn mul(self, rhs: f64) -> Self::Output {
                Vec3 {
                    x: self.x() * rhs,
                    y: self.y() * rhs,
                    z: self.z() * rhs
                }
            }
        }

        impl Div<f64> for $t {

            type Output = Vec3;

            fn div(self, rhs: f64) -> Self::Output {
                Vec3 {
                    x: self.x() / rhs,
                    y: self.y() / rhs,
                    z: self.z() / rhs
                }
            }
        }

        impl<V: Vec3View> Add<V> for $t {

            type Output = Vec3;

            fn add(self, rhs: V) -> Self::Output {
                Vec3 {
                    x: self.x() + rhs.x(),
                    y: self.y() + rhs.y(),
                    z: self.z() + rhs.z()
                }
            }
        }

        impl<V: Vec3View> Sub<V> for $t {

            type Output = Vec3;

            fn sub(self, rhs: V) -> Self::Output {
                Vec3 {
                    x: self.x() - rhs.x(),
                    y: self.y() - rhs.y(),
                    z: self.z() - rhs.z()
                }
            }
        }

        impl Neg for $t {

            type Output = $t;

            fn neg(self) -> Self::Output {
                Self {
                    x: -self.x,
                    y: -self.y,
                    z: -self.z
                }
            }
        }

        impl AddAssign for $t {

            fn add_assign(&mut self, other: Self) {
                self.x += other.x;
                self.y += other.y;
                self.z += other.z;
            }

        }
    )*)
}

overlapping_impl! { Vec3 Vec3Norm }

// Deserialization
use serde::{Deserialize, Deserializer};

impl<'de> Deserialize<'de> for Vec3 {
    fn deserialize<D>(deserializer: D) -> Result<Vec3, D::Error>
    where
        D: Deserializer<'de>,
    {
        let v = <[f64; 3]>::deserialize(deserializer)?;

        Ok(Vec3::new(v[0], v[1], v[2]))
    }
}