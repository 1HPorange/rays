// TODO: Support other color formats?

#[derive(Debug, Copy, Clone)]
pub struct RGBColor {
    pub r: f32,
    pub g: f32,
    pub b: f32
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