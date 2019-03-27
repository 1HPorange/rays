use std::io;
use std::path::Path;
use serde::Deserialize;
use lodepng::*;
use crate::uv_mappers::*;

#[derive(Copy, Clone, Deserialize)]
#[serde(deny_unknown_fields)] 
pub enum SamplingMethod {
    POINT,
    BILINEAR
}

impl Default for SamplingMethod {
    fn default() -> Self {
        SamplingMethod::BILINEAR
    }
}

#[derive(Clone)]
pub struct TextureUvMapper {
    base_mat: Material,
    pixels: Vec<RGBColor>,
    tex_width: usize,
    tex_height: usize,
    sampling_method: SamplingMethod
}

impl TextureUvMapper {

    pub fn from_png_24<P: AsRef<Path>>(filepath: P, base_mat: Material, sampling_method: SamplingMethod) -> Result<TextureUvMapper, io::Error> {

        let decoded = decode24_file(filepath).map_err(|e| io::Error::new(io::ErrorKind::Other, e.as_str()))?;

        let pixels = decoded.buffer.iter()
            .map(|pix| RGBColor::new(pix.r as f64 / 255.0, pix.g as f64 / 255.0, pix.b as f64 / 255.0))
            .collect::<Vec<_>>();

        assert!(decoded.width > 0 && decoded.height > 0);

        Ok(TextureUvMapper {
            base_mat,
            pixels,
            tex_width: decoded.width,
            tex_height: decoded.height,
            sampling_method
        })
    }
}

impl UvMapper for TextureUvMapper {

    fn get_material_at(&self, rch: &GeometryHitInfo) -> Material {

        let w = rch.uv.u * (self.tex_width - 1) as f64;
        let h = (1.0 - rch.uv.v) * (self.tex_height - 1) as f64;

        let color = match self.sampling_method {

            SamplingMethod::POINT => {
                let x = w.round() as usize;
                let y = h.round() as usize;

                self.pixels[x + self.tex_width * y]
            },

            SamplingMethod::BILINEAR => {

                // Get the four pixel coordinates needed for bilinear sampling
                let x_left = w.floor() as usize;
                let x_right = x_left + 1;

                let y_top = h.floor() as usize;
                let y_bottom = y_top + 1;

                // The four colors we need to interpolate
                let tl = self.pixels[x_left + y_top * self.tex_width];
                let tr = self.pixels[x_right + y_top * self.tex_width];
                let bl = self.pixels[x_left + y_bottom * self.tex_width];
                let br = self.pixels[x_right + y_bottom * self.tex_width];

                // Horizontal and Vertical Interpolation variables
                let th = w.fract();
                let tv = h.fract();

                // Interpolate horizontally
                let ct = tl * (1.0 - th) + tr * th;
                let cb = bl * (1.0 - th) + br * th;

                // Interpolate vertically
                ct * (1.0 - tv) + cb * tv
            }

        };

        Material {
            color,
            ..self.base_mat
        }
    }

    fn validate(&self) -> bool {
        self.base_mat.validate()
    }
}