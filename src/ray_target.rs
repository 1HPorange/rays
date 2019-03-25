use crate::raytracing::*;
use crate::vec::*;

pub trait RayTarget {

    fn test_intersection(&self, ray: &Ray) -> Option<GeometryHitInfo>;

}

pub struct GeometryHitInfo {

    pub position: Vec3,
    pub normal: Vec3Norm,
    pub uv: Vec2

}