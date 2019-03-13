use super::color::*;
use super::raytracing::*;

pub struct Material<T> {
    pub color: RGBAColor, // alpha determines how many rays pass through the material and are potentially refracted

    pub reflection: RaySpreadInfo<T>,
    pub refraction: RaySpreadInfo<T>,

    pub last_bounce_color: RGBColor
}

pub struct RaySpreadInfo<T> {
    pub intensity: f32,
    pub max_angle: T,
    pub edge_multiplier: f32 // TODO: Hespec this value in the calcs
}

impl<T> Material<T> {

    pub fn opaque_reflective(col: RGBColor, reflectiveness: f32, edge_reflection_multiplier: f32) -> Material<T> where T: num_traits::Float {
        Material {
            color: col.into(),

            reflection: RaySpreadInfo { intensity: reflectiveness, max_angle: T::from(5.0).unwrap(), edge_multiplier: edge_reflection_multiplier },
            refraction: RaySpreadInfo { intensity: 0.0, max_angle: T::zero(), edge_multiplier: 1.0 },

            last_bounce_color: col * (1.0 - reflectiveness)
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