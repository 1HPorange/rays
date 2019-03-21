extern crate lodepng;

use std::io;
use std::path::Path;

use lodepng::*;

use super::material::*;
use super::color::*;
use super::raytracing::*;

pub enum SamplingMethod {
    POINT,
    BILINEAR // TODO: Implement
}

pub struct TextureUvMapper<T> {
    base_mat: Material<T>,
    pixels: Vec<RGBColor>,
    tex_width: usize,
    tex_height: usize
}

impl <T> TextureUvMapper<T> {

    pub fn from_png_24<P: AsRef<Path>>(filepath: P, base_mat: Material<T>) -> Result<TextureUvMapper<T>, io::Error> {

        let decoded = decode24_file(filepath).map_err(|e| io::Error::new(io::ErrorKind::Other, e.as_str()))?;

        let pixels = decoded.buffer.iter()
            .map(|pix| RGBColor::new(pix.r as f32 / 255.0, pix.g as f32 / 255.0, pix.b as f32 / 255.0))
            .collect::<Vec<_>>();

        Ok(TextureUvMapper {
            base_mat,
            pixels,
            tex_width: decoded.width,
            tex_height: decoded.height
        })
    }
}

impl<T> UvMapper<T> for TextureUvMapper<T> where T: num_traits::Float + Send + Sync {

    fn get_material_at(&self, rch: &GeometryHitInfo<T>) -> Material<T> {

        let w = T::from(self.tex_width - 1).unwrap();
        let h = T::from(self.tex_height - 1).unwrap();

        let x: usize = num_traits::NumCast::from(rch.uv.0 * w).unwrap();
        let y: usize = num_traits::NumCast::from(rch.uv.1 * h).unwrap();

        Material {
            color: self.pixels[x + self.tex_width * y],
            ..self.base_mat
        }
    }

}