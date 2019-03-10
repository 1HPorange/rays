use super::color::*;
use super::raytracing::*;

pub struct Material {
    pub color: RGBAColor, // alpha determines how many rays pass through the material
    pub reflectivity: f32, // 0: many rays will bounce off in every direction, 1: single ray does a perfect reflective bounce
    pub intensity: f32, // 0-1: ratio of how much the color of the material influences output vs. the color of the reflected rays
    pub refraction_index: f32 // only has an effect when alpha < 1
}

impl Material {

    pub fn perfect_diffuse(col: RGBColor) -> Material {
        Material {
            color: col.into(),
            reflectivity: 0.0,
            intensity: 1.0,
            refraction_index: 0.0
        }
    }

}

pub trait HasMaterial<T> {
    
    fn get_material_at(&self, rch: &RayHitInfo<T>) -> &Material;

}

// Material providers

pub trait HasMaterialProvider<T> {

    fn get_material_provider(&self) -> &Box<HasMaterial<T>>;

}

pub struct StaticMaterialProvider(pub Material);

impl<T> HasMaterial<T> for StaticMaterialProvider {

    fn get_material_at(&self, rch: &RayHitInfo<T>) -> &Material {
        &self.0
    }

}