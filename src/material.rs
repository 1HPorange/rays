use super::color::*;
use super::raytracing::*;

#[derive(Debug, Copy, Clone)]
pub struct Material<T> {
    pub color: RGBColor, // alpha determines how many rays pass through the material and are potentially refracted
    pub transparency: Transparency,
    pub reflection: ReflectionParams<T>,
    pub refraction: RefractionParams<T>,
}

#[derive(Debug, Copy, Clone)]
pub struct Transparency {
    pub opacity_center: f32,
    pub opacity_edges: f32,
    pub edge_effect_power: f32
}

#[derive(Debug, Copy, Clone)]
pub struct ReflectionParams<T> {
    pub intensity_center: f32,
    pub intensity_edges: f32,
    pub edge_effect_power: f32,
    pub max_angle: T
}

#[derive(Debug, Copy, Clone)]
pub struct RefractionParams<T> {
    pub index_of_refraction: T,
    pub max_angle: T,
}

impl<T> Material<T> {

    pub fn new(color: RGBColor, transparency: Transparency, reflection: ReflectionParams<T>, refraction: RefractionParams<T>) -> Material<T> {
        Material {
            color,
            transparency,
            reflection,
            refraction
        }
    }

    pub fn opaque_reflective(color: RGBColor, reflection: ReflectionParams<T>) -> Material<T> where T: num_traits::Float {
        Material {
            color,
            transparency: Transparency { opacity_center: 1.0, opacity_edges: 1.0, edge_effect_power: 1.0 },
            reflection,
            refraction: RefractionParams { index_of_refraction: T::zero(), max_angle: T::zero() }
        }
    }
}

impl Transparency {

    pub fn new(opacity_center: f32, opacity_edges: f32, edge_effect_power: f32) -> Transparency {
        Transparency { opacity_center, opacity_edges, edge_effect_power }
    }

}

impl<T> ReflectionParams<T> {

    pub fn new(intensity_center: f32, intensity_edges: f32, edge_effect_power: f32, max_angle: T) -> ReflectionParams<T> {
        ReflectionParams { intensity_center, intensity_edges, edge_effect_power, max_angle }
    }

}

impl<T> RefractionParams<T> {

    pub fn new(index_of_refraction: T, max_angle: T) -> RefractionParams<T> {
        RefractionParams { index_of_refraction, max_angle }
    }

}

// UV (to material) mappers

pub trait UvMapper<T>: Send + Sync {
    
    fn get_material_at(&self, rch: &GeometryHitInfo<T>) -> &Material<T>;

}

pub trait HasUvMapper<T> {

    fn get_uv_mapper(&self) -> &Box<UvMapper<T>>;

}

pub struct StaticUvMapper<T>(pub Material<T>);

impl<T> UvMapper<T> for StaticUvMapper<T> where Self: Send + Sync {

    fn get_material_at(&self, rch: &GeometryHitInfo<T>) -> &Material<T> {
        &self.0
    }

}