use super::color::*;
use super::raytracing::*;

pub struct Material {
    pub color: RGBAColor, // alpha determines how many rays pass through the material and are potentially refracted

    pub reflection: RaySpreadInfo,
    pub refraction: RaySpreadInfo,

    pub last_bounce_color: RGBColor
}

pub struct RaySpreadInfo {
    pub intensity: f32,
    pub max_angle: f32,
    pub edge_multiplier: f32
}

impl Material {

    pub fn opaque_reflective(col: RGBColor, reflectiveness: f32, edge_reflection_multiplier: f32) -> Material {
        Material {
            color: col.into(),

            reflection: RaySpreadInfo { intensity: reflectiveness, max_angle: 0.0, edge_multiplier: edge_reflection_multiplier },
            refraction: RaySpreadInfo { intensity: 0.0, max_angle: 90.0, edge_multiplier: 1.0 },

            last_bounce_color: col * (1.0 - reflectiveness)
        }
    }

}

pub trait HasMaterial<T> {
    
    fn get_material_at(&self, rch: &GeometryHitInfo<T>) -> &Material;

}

// Material providers

pub trait HasMaterialProvider<T> {

    fn get_material_provider(&self) -> &Box<HasMaterial<T>>;

}

pub struct StaticMaterialProvider(pub Material);

impl<T> HasMaterial<T> for StaticMaterialProvider {

    fn get_material_at(&self, rch: &GeometryHitInfo<T>) -> &Material {
        &self.0
    }

}