use crate::util;

// TODO: Support other color formats?

#[derive(Debug, Copy, Clone)]
pub struct RGBColor {
    pub r: f64,
    pub g: f64,
    pub b: f64
}

// #[derive(Debug, Copy, Clone)]
// pub struct RGBAColor {
//     pub r: f64,
//     pub g: f64,
//     pub b: f64,
//     pub a: f64
// }

impl RGBColor {
    pub const BLACK: RGBColor = RGBColor { r: 0.0, g: 0.0, b: 0.0 };
    pub const PINK: RGBColor = RGBColor { r: 1.0, g: 0.0, b: 1.0 };
    pub const WHITE: RGBColor = RGBColor { r: 1.0, g: 1.0, b: 1.0 };
    pub const EVENING_BLUE: RGBColor = RGBColor { r: 0.090, g: 0.160, b: 0.368 };

    pub fn new(r: f64, g: f64, b: f64) -> RGBColor {
        RGBColor { r, g, b }
    }

    pub fn validate(&self) -> bool {

        if  !util::is_in_range(self.r, 0.0, 1.0) ||
            !util::is_in_range(self.g, 0.0, 1.0) ||
            !util::is_in_range(self.b, 0.0, 1.0) {
            println!("Warning: Color contains one or more components outside of usual range 0-1. This can be deliberate, but might look weird.");
        }
        
        true
    }
}

impl From<RGBColor> for [u8;3] {

    fn from(col: RGBColor)-> [u8;3] {
        [
            (col.r * 255.0) as u8,
            (col.g * 255.0) as u8,
            (col.b * 255.0) as u8,
        ]
    }
}

impl Default for RGBColor {
    fn default() -> Self {
        RGBColor::WHITE
    }
}

impl std::ops::Mul<f64> for RGBColor {

    type Output = RGBColor;

    fn mul(self, rhs: f64) -> Self::Output {

        RGBColor {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs
        }
    }
}

impl<S> std::ops::Div<S> for RGBColor where f64: std::ops::Div<S, Output=f64>, S: Copy {

    type Output = RGBColor;

    fn div(self, rhs: S) -> Self::Output {
        RGBColor {
            r: self.r / rhs,
            g: self.g / rhs,
            b: self.b / rhs
        }
    }
}

impl std::ops::Add for RGBColor {

    type Output = RGBColor;

    fn add(self, rhs: Self) -> Self::Output {
        RGBColor {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b
        }
    }

}

impl std::ops::AddAssign for RGBColor {

    fn add_assign(&mut self, other: RGBColor) {
        self.r += other.r;
        self.g += other.g;
        self.b += other.b;
    }

}

// impl From<RGBAColor> for RGBColor {

//     fn from(col: RGBAColor) -> RGBColor {
//         RGBColor { r: col.r, g: col.g, b: col.b }
//     }

// }

// impl From<RGBColor> for RGBAColor {

//     fn from(col: RGBColor) -> RGBAColor {
//         RGBAColor { r: col.r, g: col.g, b: col.b, a: 1.0 }
//     }

// }

// Deserialization
use serde::{Deserialize, Deserializer};

impl<'de> Deserialize<'de> for RGBColor {
    fn deserialize<D>(deserializer: D) -> Result<RGBColor, D::Error>
    where
        D: Deserializer<'de>,
    {
        let v = <[f64; 3]>::deserialize(deserializer)?;

        Ok(RGBColor::new(v[0], v[1], v[2]))
    }
}