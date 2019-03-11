// TODO: Support other color formats?

#[derive(Debug, Copy, Clone)]
pub struct RGBColor {
    pub r: f32,
    pub g: f32,
    pub b: f32
}

#[derive(Debug, Copy, Clone)]
pub struct RGBAColor {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32
}

impl RGBColor {
    pub const BLACK: RGBColor = RGBColor { r: 0.0, g: 0.0, b: 0.0 };
    pub const PINK: RGBColor = RGBColor { r: 1.0, g: 0.0, b: 1.0 };
    pub const WHITE: RGBColor = RGBColor { r: 1.0, g: 1.0, b: 1.0 };
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

impl<S> std::ops::Mul<S> for RGBColor where f32: std::ops::Mul<S, Output=f32>, S: Copy {

    type Output = RGBColor;

    fn mul(self, rhs: S) -> Self::Output {
        RGBColor {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs
        }
    }
}

impl<S> std::ops::Div<S> for RGBColor where f32: std::ops::Div<S, Output=f32>, S: Copy {

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

impl From<RGBAColor> for RGBColor {

    fn from(col: RGBAColor) -> RGBColor {
        RGBColor { r: col.r, g: col.g, b: col.b }
    }

}

impl From<RGBColor> for RGBAColor {

    fn from(col: RGBColor) -> RGBAColor {
        RGBAColor { r: col.r, g: col.g, b: col.b, a: 1.0 }
    }

}