use super::color::*;
use super::raytracing::*;

// TODO: Supply useful default values for all these things

#[derive(Debug, Copy, Clone)]
pub struct Material<T> {
    pub color: RGBColor, // alpha determines how many rays pass through the material and are potentially refracted
    pub opacity: Opacity<T>,
    pub reflection: ReflectionParams<T>,
    pub refraction: RefractionParams<T>,
}

#[derive(Debug, Copy, Clone)]
pub struct Opacity<T> {
    pub opacity_center: T,
    pub opacity_edges: T,
    pub edge_effect_power: T
}

#[derive(Debug, Copy, Clone)]
pub struct ReflectionParams<T> {
    pub intensity_center: T,
    pub intensity_edges: T,
    pub edge_effect_power: T,
    pub max_angle: T
}

#[derive(Debug, Copy, Clone)]
pub struct RefractionParams<T> {
    pub index_of_refraction: T,
    pub max_angle: T,
}

impl<T> Material<T> {

    // TODO: Use new constructors of all member in here, not struct initializers

    pub fn new(color: RGBColor, opacity: Opacity<T>, reflection: ReflectionParams<T>, refraction: RefractionParams<T>) -> Material<T> {
        Material {
            color,
            opacity,
            reflection,
            refraction
        }
    }

    pub fn opaque_reflective(color: RGBColor, reflection: ReflectionParams<T>) -> Material<T> where T: num_traits::Float {
        Material {
            color,
            opacity: Opacity { opacity_center: T::one(), opacity_edges: T::one(), edge_effect_power: T::one() },
            reflection,
            refraction: RefractionParams { index_of_refraction: T::zero(), max_angle: T::zero() }
        }
    }

    pub fn pure(color: RGBColor) -> Material<T> where T: num_traits::Float {
        Material {
            color,
            opacity: Opacity { opacity_center: T::one(), opacity_edges: T::one(), edge_effect_power: T::one() },
            reflection: ReflectionParams::new(T::zero(), T::zero(), T::one(), T::zero()),
            refraction: RefractionParams { index_of_refraction: T::zero(), max_angle: T::zero() }
        }
    }
}

impl<T> Opacity<T> {

    pub fn new(opacity_center: T, opacity_edges: T, edge_effect_power: T) -> Opacity<T> {
        Opacity { opacity_center, opacity_edges, edge_effect_power }
    }

}

impl<T> ReflectionParams<T> {

    pub fn new(intensity_center: T, intensity_edges: T, edge_effect_power: T, max_angle: T) -> ReflectionParams<T> {
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
    
    fn get_material_at(&self, rch: &GeometryHitInfo<T>) -> Material<T>;

}

pub trait HasUvMapper<T> {

    fn get_uv_mapper(&self) -> &Box<UvMapper<T>>;

}

pub struct StaticUvMapper<T>(pub Material<T>);

impl<T> UvMapper<T> for StaticUvMapper<T> where Self: Send + Sync, T: num_traits::Float {

    fn get_material_at(&self, rch: &GeometryHitInfo<T>) -> Material<T> {
        self.0
    }

}

pub struct CheckerboardUvMapper<T>(pub Material<T>, pub Material<T>);

impl<T> UvMapper<T> for CheckerboardUvMapper<T> where Self: Send + Sync, T: num_traits::Float {

    fn get_material_at(&self, rch: &GeometryHitInfo<T>) -> Material<T> {
        
        let half = T::from(0.5).unwrap();
        let x = rch.uv.0 > half;
        let y = rch.uv.1 > half;

        if x != y {
            self.0
        } else {
            self.1
        }
    }

}