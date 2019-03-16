use super::color::*;
use super::raytracing::*;

pub struct Material<T> {
    pub color: RGBAColor, // alpha determines how many rays pass through the material and are potentially refracted

    pub reflection: ReflectionParams<T>,
    pub refraction: RefractionParams<T>,
}

pub struct ReflectionParams<T> {
    pub intensity_center: f32,
    pub intensity_edges: f32,
    pub edge_effect_power: f32,
    pub max_angle: T,
}

pub struct RefractionParams<T> {
    pub index_of_refraction: T,
    pub max_angle: T
}

impl<T> Material<T> {

    pub fn opaque_reflective(col: RGBColor, reflection: ReflectionParams<T>) -> Material<T> where T: num_traits::Float {
        Material {
            color: col.into(),
            reflection,
            refraction: RefractionParams { index_of_refraction: T::zero(), max_angle: T::zero() }
        }
    }
}

pub trait HasMaterial<T>: Send + Sync {
    
    fn get_material_at(&self, rch: &GeometryHitInfo<T>) -> &Material<T>;

}

// Material providers

pub trait HasMaterialProvider<T> {

    fn get_material_provider(&self) -> &Box<HasMaterial<T>>;

}

pub struct StaticMaterialProvider<T>(pub Material<T>);

impl<T> HasMaterial<T> for StaticMaterialProvider<T> where Self: Send + Sync {

    fn get_material_at(&self, rch: &GeometryHitInfo<T>) -> &Material<T> {
        &self.0
    }

}