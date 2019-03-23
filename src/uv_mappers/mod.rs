use crate::material::*;
use crate::color::*;
use crate::raytracing::*;

mod texture_uv_mapper;

pub use texture_uv_mapper::{TextureUvMapper, SamplingMethod};

pub trait UvMapper<T>: Send + Sync {
    
    fn get_material_at(&self, rch: &GeometryHitInfo<T>) -> Material<T>;

}

pub trait HasUvMapper<T> {

    fn get_uv_mapper(&self) -> &Box<UvMapper<T>>;

}

// Simple UV mapper implementations

pub struct StaticUvMapper<T>(pub Material<T>);

impl<T> UvMapper<T> for StaticUvMapper<T> where Self: Send + Sync, T: num_traits::Float {

    fn get_material_at(&self, _rch: &GeometryHitInfo<T>) -> Material<T> {
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

pub struct DebugUvMapper;

impl<T> UvMapper<T> for DebugUvMapper where Self: Send + Sync, T: num_traits::Float {

    fn get_material_at(&self, rch: &GeometryHitInfo<T>) -> Material<T> {
        
        let r: f32 = num_traits::NumCast::from(rch.uv.0).unwrap();
        let g: f32 = num_traits::NumCast::from(rch.uv.1).unwrap();

        Material::pure(RGBColor::new(r, g, 0.0))
    }

}