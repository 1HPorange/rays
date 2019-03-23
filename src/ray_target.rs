use crate::raytracing::*;
use crate::vec3::*;

pub trait RayTarget<T> {

    fn test_intersection(&self, ray: &Ray<T>) -> Option<GeometryHitInfo<T>>;

}

pub struct GeometryHitInfo<T> {

    pub position: Vec3<T>,
    pub normal: Vec3Norm<T>,
    pub uv: Vec2<T>

}