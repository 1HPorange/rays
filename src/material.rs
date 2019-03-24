use crate::color::*;
use crate::util;

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

impl<T> Material<T> where T: num_traits::Float {

    pub fn new(color: RGBColor, opacity: OpacityParams<T>, reflection: ReflectionParams<T>, refraction: RefractionParams<T>) -> Material<T> {
        Material {
            color,
            opacity,
            reflection,
            refraction
        }
    }

    pub fn opaque_reflective(color: RGBColor, reflection: ReflectionParams<T>) -> Material<T> {
        Material {
            color,
            opacity: OpacityParams { opacity_center: T::one(), opacity_edges: T::one(), edge_effect_power: T::one() },
            reflection,
            refraction: RefractionParams { index_of_refraction: T::one(), max_angle: T::zero() }
        }
    }

    pub fn pure(color: RGBColor) -> Material<T> {
        Material {
            color,
            opacity: OpacityParams { opacity_center: T::one(), opacity_edges: T::one(), edge_effect_power: T::one() },
            reflection: ReflectionParams::new(T::zero(), T::zero(), T::one(), T::zero()),
            refraction: RefractionParams { index_of_refraction: T::one(), max_angle: T::zero() }
        }
    }

    pub fn validate(&self) -> bool {
        
        let mut success = true;
        
        success = success && self.color.validate();

        if  !util::is_in_range(self.opacity.opacity_center, T::zero(), T::one()) ||
            !util::is_in_range(self.opacity.opacity_edges, T::zero(), T::one()) {
            println!("Warning: Opacity out of usual range 0-1. This can be desired, but might look really weird.");
        }

        if !util::is_in_range(self.opacity.edge_effect_power, T::zero(), T::infinity()) {
            println!("Error: Opacity edge effect power must be 0 or positive");
            success = false;
        }

        if !util::is_in_range(self.reflection.edge_effect_power, T::zero(), T::infinity()) {
            println!("Error: Reflectivity edge effect power must be 0 or positive");
            success = false;
        }

        success
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