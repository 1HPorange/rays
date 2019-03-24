use std::io;
use std::path::Path;

use lodepng::*;

use crate::uv_mappers::*;

#[derive(Copy, Clone)]
pub enum SamplingMethod {
    POINT,
    BILINEAR
}

#[derive(Clone)]
pub struct TextureUvMapper<T> {
    base_mat: Material<T>,
    pixels: Vec<RGBColor>,
    tex_width: usize,
    tex_height: usize,
    sampling_method: SamplingMethod
}

impl <T> TextureUvMapper<T> {

    pub fn from_png_24<P: AsRef<Path>>(filepath: P, base_mat: Material<T>, sampling_method: SamplingMethod) -> Result<TextureUvMapper<T>, io::Error> {

        let decoded = decode24_file(filepath).map_err(|e| io::Error::new(io::ErrorKind::Other, e.as_str()))?;

        let pixels = decoded.buffer.iter()
            .map(|pix| RGBColor::new(pix.r as f32 / 255.0, pix.g as f32 / 255.0, pix.b as f32 / 255.0))
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

impl<T> UvMapper<T> for TextureUvMapper<T> where T: num_traits::Float + Send + Sync {

    fn get_material_at(&self, rch: &GeometryHitInfo<T>) -> Material<T> {

        let w = rch.uv.0 * T::from(self.tex_width - 1).unwrap();
        let h = (T::one() - rch.uv.1) * T::from(self.tex_height - 1).unwrap();

        let color = match self.sampling_method {

            SamplingMethod::POINT => {
                let x: usize = num_traits::NumCast::from(w.round()).unwrap();
                let y: usize = num_traits::NumCast::from(h.round()).unwrap();

                self.pixels[x + self.tex_width * y]
            },

            SamplingMethod::BILINEAR => {

                // Get the four pixel coordinates needed for bilinear sampling
                let x_left: usize = num_traits::NumCast::from(w.floor()).unwrap();
                let x_right = x_left + 1;

                let y_top: usize = num_traits::NumCast::from(h.floor()).unwrap();
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
                let ct = tl * (T::one() - th) + tr * th;
                let cb = bl * (T::one() - th) + br * th;

                // Interpolate vertically
                ct * (T::one() - tv) + cb * tv
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