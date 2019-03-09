use super::vec3::*;

pub struct Ray<T> {
    pub origin: Vec3<T>,
    pub direction: Vec3Norm<T>
}

pub struct RayHitInfo {


}

pub trait RayTarget<T> {

    fn test_intersection(&self, ray: &Ray<T>) -> Option<RayHitInfo>;

}