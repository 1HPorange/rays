// TODO: Support other formats and alpha

use super::color::*;
use std::path::{Path};

use std::fs;
use std::io;

pub struct RenderTarget {
    pub width: usize,
    pub height: usize,
    data: Vec<RGBColor>
}

impl RenderTarget {

    // Creates a new rendertarget that is cleared to black
    pub fn new(width: usize, height: usize) -> RenderTarget {

        assert!(width > 0 && height > 0);

        Self::with_clear_color(width, height, &RGBColor { r: 0.0, g: 0.0, b: 0.0 })

    }

    pub fn with_clear_color(width: usize, height: usize, clear_color: &RGBColor) -> RenderTarget {
        
        assert!(width > 0 && height > 0);

        RenderTarget {
            width, height,
            data: vec!(*clear_color; width * height)
        }

    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: RGBColor) {
        self.data[x + y * self.width] = color;
    }

    pub fn save_as_ppm<P: AsRef<Path>>(&self, path: P) -> std::io::Result<()> {

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

    pub fn save_as_png<P: AsRef<Path>>(&self, path: P) -> std::io::Result<()> {

        // Convert our float colour to a byte color format
        fn c(x: f64) -> u8 {
            (x * 255.0) as u8
        }

        let data = self.data.iter()
            .map(|pix| lodepng::RGB::new(c(pix.r), c(pix.g), c(pix.b)))
            .collect::<Vec<_>>();

        lodepng::encode24_file(path, &data, self.width, self.height)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.as_str()))?;

        Ok(())
    }
}