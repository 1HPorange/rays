use super::color::*;
use super::raytracing::*;

pub struct Material<T> {
    pub color: RGBAColor, // alpha determines how many rays pass through the material and are potentially refracted

    pub reflection: RaySpreadInfo<T>,
    pub refraction: RaySpreadInfo<T>,

    pub edge_effect_power: f32
}

pub struct RaySpreadInfo<T> {
    pub intensity: f32,
    pub max_angle: T,

    /// Reflection: Interpolates intensity towards this value depending on the angle of incidence
    /// Refraction: Maximum angle of refraction
    pub edge_effect: f32
}

impl<T> Material<T> {

    pub fn opaque_reflective(col: RGBColor, reflectivity: f32, edge_reflectivity_multiplier: f32, edge_effect_power: f32) -> Material<T> where T: num_traits::Float {
        Material {
            color: col.into(),

            reflection: RaySpreadInfo { intensity: reflectivity, max_angle: T::zero(), edge_effect: edge_reflectivity_multiplier },
            refraction: RaySpreadInfo { intensity: 0.0, max_angle: T::zero(), edge_effect: 1.0 },

            edge_effect_power
        }
    }

    pub fn opaque_diffuse(col: RGBColor, gi_influence: f32, edge_reflectivity_multiplier: f32, edge_effect_power: f32) -> Material<T> where T: num_traits::Float {
        Material {
            color: col.into(),

            reflection: RaySpreadInfo { intensity: gi_influence, max_angle: T::from(90.0).unwrap(), edge_effect: edge_reflectivity_multiplier },
            refraction: RaySpreadInfo { intensity: 0.0, max_angle: T::zero(), edge_effect: 1.0 },

            edge_effect_power
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