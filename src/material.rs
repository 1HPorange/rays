use crate::color::*;
use crate::util;
use serde::Deserialize;

// TODO: Supply useful default values for all these things

#[derive(Debug, Copy, Clone, Default, Deserialize)]
#[serde(default)]
#[serde(deny_unknown_fields)] 
pub struct Material {
    pub color: RGBColor,
    pub opacity: Opacity,
    pub reflection: Reflection,
    pub refraction: Refraction,
}

#[derive(Debug, Copy, Clone, Deserialize)]
#[serde(deny_unknown_fields)] 
pub struct Opacity {
    pub center: f64,
    pub edges: f64,
    pub power: f64
}

#[derive(Debug, Copy, Clone, Deserialize)]
#[serde(deny_unknown_fields)] 
pub struct Reflection {
    pub center: f64,
    pub edges: f64,
    pub power: f64,
    pub max_angle: f64
}

#[derive(Debug, Copy, Clone, Deserialize)]
#[serde(deny_unknown_fields)] 
pub struct Refraction {
    pub ior: f64,
    pub max_angle: f64,
}

impl Material {

    pub fn new(color: RGBColor, opacity: Opacity, reflection: Reflection, refraction: Refraction) -> Material {
        Material {
            color,
            opacity,
            reflection,
            refraction
        }
    }

    pub fn opaque_reflective(color: RGBColor, reflection: Reflection) -> Material {
        Material {
            color,
            opacity: Opacity::default(),
            reflection,
            refraction: Refraction::default()
        }
    }

    pub fn pure(color: RGBColor) -> Material {
        Material {
            color,
            opacity: Opacity::default(),
            reflection: Reflection::default(),
            refraction: Refraction::default()
        }
    }

    pub fn validate(&self) -> bool {
        
        let mut success = true;
        
        success = success && self.color.validate();

        if  !util::is_in_range(self.opacity.center, 0.0, 1.0) ||
            !util::is_in_range(self.opacity.edges, 0.0, 1.0) {
            println!("Warning: Opacity out of usual range 0-1. This can be desired, but might look really weird.");
        }

        if !util::is_in_range(self.opacity.power, 0.0, std::f64::INFINITY) {
            println!("Error: Opacity edge effect power must be 0 or positive");
            success = false;
        }

        if !util::is_in_range(self.reflection.power, 0.0, std::f64::INFINITY) {
            println!("Error: Reflectivity edge effect power must be 0 or positive");
            success = false;
        }

        success
    }
}

impl Opacity {

    pub fn new(center: f64, edges: f64, power: f64) -> Opacity {
        Opacity { center, edges, power }
    }
}

impl Default for Opacity {

    fn default() -> Self {
        Opacity { 
            center: 1.0,
            edges: 1.0,
            power: 1.0
         }
    }
}

impl Reflection {

    pub fn new(center: f64, edges: f64, power: f64, max_angle: f64) -> Reflection {
        Reflection { center, edges, power, max_angle }
    }
}

impl Default for Reflection {

    fn default() -> Self {
        Reflection {
            center: 0.0,
            edges: 0.0,
            power: 1.0,
            max_angle: 0.0
        }
    }
}

impl Refraction {

    pub fn new(ior: f64, max_angle: f64) -> Refraction {
        Refraction { ior, max_angle }
    }
}

impl Default for Refraction {

    fn default() -> Self {
        Refraction {
            ior: 1.33,
            max_angle: 0.0
        }
    }
}
