use super::color::*;

// TODO: Supply useful default values for all these things

#[derive(Debug, Copy, Clone)]
pub struct Material<T> {
    pub color: RGBColor, // alpha determines how many rays pass through the material and are potentially refracted
    pub opacity: OpacityParams<T>,
    pub reflection: ReflectionParams<T>,
    pub refraction: RefractionParams<T>,
}

#[derive(Debug, Copy, Clone)]
pub struct OpacityParams<T> {
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

    pub fn new(color: RGBColor, opacity: OpacityParams<T>, reflection: ReflectionParams<T>, refraction: RefractionParams<T>) -> Material<T> {
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
            opacity: OpacityParams { opacity_center: T::one(), opacity_edges: T::one(), edge_effect_power: T::one() },
            reflection,
            refraction: RefractionParams { index_of_refraction: T::one(), max_angle: T::zero() }
        }
    }

    pub fn pure(color: RGBColor) -> Material<T> where T: num_traits::Float {
        Material {
            color,
            opacity: OpacityParams { opacity_center: T::one(), opacity_edges: T::one(), edge_effect_power: T::one() },
            reflection: ReflectionParams::new(T::zero(), T::zero(), T::one(), T::zero()),
            refraction: RefractionParams { index_of_refraction: T::one(), max_angle: T::zero() }
        }
    }
}

impl<T> OpacityParams<T> {

    pub fn new(opacity_center: T, opacity_edges: T, edge_effect_power: T) -> OpacityParams<T> {
        OpacityParams { opacity_center, opacity_edges, edge_effect_power }
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