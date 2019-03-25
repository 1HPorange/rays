use crate::material::*;
use crate::color::*;
use crate::ray_target::*;

mod texture_uv_mapper;

pub use texture_uv_mapper::{TextureUvMapper, SamplingMethod};

pub trait UvMapper: Send + Sync {
    
    fn get_material_at(&self, rch: &GeometryHitInfo) -> Material;

    /// Should return true of the UvMapper contains (and can produce) only legal materials
    fn validate(&self) -> bool;
}

pub trait HasUvMapper {

    fn get_uv_mapper(&self) -> &Box<UvMapper>;

}

// Simple UV mapper implementations

pub struct StaticUvMapper(pub Material);

impl UvMapper for StaticUvMapper /*where Self: Send + Sync*/ {

    fn get_material_at(&self, _rch: &GeometryHitInfo) -> Material {
        self.0
    }

    fn validate(&self) -> bool {
        self.0.validate()
    }
}

pub struct CheckerboardUvMapper(pub Material, pub Material);

impl UvMapper for CheckerboardUvMapper /*where Self: Send + Sync*/ {

    fn get_material_at(&self, rch: &GeometryHitInfo) -> Material {
        
        let x = rch.uv.u > 0.5;
        let y = rch.uv.v > 0.5;

        if x != y {
            self.0
        } else {
            self.1
        }
    }

    fn validate(&self) -> bool {
        self.0.validate() && self.1.validate()
    }
}

pub struct DebugUvMapper;

impl UvMapper for DebugUvMapper /* where Self: Send + Sync*/ {

    fn get_material_at(&self, rch: &GeometryHitInfo) -> Material {
        Material::pure(RGBColor::new(rch.uv.u, rch.uv.v, 0.0))
    }

    fn validate(&self) -> bool {
        true
    }
}