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
    pub center: T,
    pub edges: T,
    pub power: T
}

#[derive(Debug, Copy, Clone)]
pub struct ReflectionParams<T> {
    pub center: T,
    pub edges: T,
    pub power: T,
    pub max_angle: T
}

#[derive(Debug, Copy, Clone)]
pub struct RefractionParams<T> {
    pub ior: T,
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
            opacity: OpacityParams { center: T::one(), edges: T::one(), power: T::one() },
            reflection,
            refraction: RefractionParams { ior: T::one(), max_angle: T::zero() }
        }
    }

    pub fn pure(color: RGBColor) -> Material<T> {
        Material {
            color,
            opacity: OpacityParams { center: T::one(), edges: T::one(), power: T::one() },
            reflection: ReflectionParams::new(T::zero(), T::zero(), T::one(), T::zero()),
            refraction: RefractionParams { ior: T::one(), max_angle: T::zero() }
        }
    }

    pub fn validate(&self) -> bool {
        
        let mut success = true;
        
        success = success && self.color.validate();

        if  !util::is_in_range(self.opacity.center, T::zero(), T::one()) ||
            !util::is_in_range(self.opacity.edges, T::zero(), T::one()) {
            println!("Warning: Opacity out of usual range 0-1. This can be desired, but might look really weird.");
        }

        if !util::is_in_range(self.opacity.power, T::zero(), T::infinity()) {
            println!("Error: Opacity edge effect power must be 0 or positive");
            success = false;
        }

        if !util::is_in_range(self.reflection.power, T::zero(), T::infinity()) {
            println!("Error: Reflectivity edge effect power must be 0 or positive");
            success = false;
        }

        success
    }
}

impl<T> OpacityParams<T> {

    pub fn new(center: T, edges: T, power: T) -> OpacityParams<T> {
        OpacityParams { center, edges, power }
    }

}

impl<T> ReflectionParams<T> {

    pub fn new(center: T, edges: T, power: T, max_angle: T) -> ReflectionParams<T> {
        ReflectionParams { center, edges, power, max_angle }
    }

}

impl<T> RefractionParams<T> {

    pub fn new(ior: T, max_angle: T) -> RefractionParams<T> {
        RefractionParams { ior, max_angle }
    }

}