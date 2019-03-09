// TODO: Support other formats and alpha

use super::color::*;

pub struct RenderTarget {
    pub width: i32,
    pub height: i32,
    data: Vec<RGBColor>
}

impl RenderTarget {

    // Creates a new rendertarget that is cleared to black
    fn new(width: i32, height: i32) -> RenderTarget {

        Self::with_clear_color(width, height, &RGBColor { r: 0.0, g: 0.0, b: 0.0 })

    }

    fn with_clear_color(width: i32, height: i32, clearColor: &RGBColor) -> RenderTarget {
        
        RenderTarget {
            width, height,
            data: vec!(*clearColor; (width * height) as usize)
        }

    }

}