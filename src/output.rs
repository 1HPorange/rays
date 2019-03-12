// TODO: Support other formats and alpha

use super::color::*;

use std::fs;

pub struct RenderTarget {
    pub width: i32,
    pub height: i32,
    data: Vec<RGBColor>
}

impl RenderTarget {

    // Creates a new rendertarget that is cleared to black
    pub fn new(width: i32, height: i32) -> RenderTarget {

        Self::with_clear_color(width, height, &RGBColor { r: 0.0, g: 0.0, b: 0.0 })

    }

    pub fn with_clear_color(width: i32, height: i32, clear_color: &RGBColor) -> RenderTarget {
        
        RenderTarget {
            width, height,
            data: vec!(*clear_color; (width * height) as usize)
        }

    }

    pub fn set_pixel(&mut self, x: i32, y: i32, color: RGBColor) {
        self.data[(x + y * self.width) as usize] = color;
    }

    pub fn save_as_ppm(&self, path: &str) -> std::io::Result<()> {

        let mut bytes = vec![
            80u8, 54, // Magic number
            32, // space
        ];
        bytes.extend_from_slice(self.width.to_string().as_bytes());
        bytes.push(32); // space
        bytes.extend_from_slice(self.height.to_string().as_bytes());
        bytes.extend_from_slice(&[
            32, // space
            50, 53, 53, // Max color value. 255 (ASCII) for normal pictures
            10 // newline
        ]);
        bytes.extend(self.data.iter().flat_map(|&col| {
            let bytes: [u8;3] = col.into();
            bytes.into_iter().map(ToOwned::to_owned).collect::<Vec<_>>()
        }));

        fs::write(path, bytes)?;

        Ok(())
    }
}