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