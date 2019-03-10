use super::vec3::*;
use super::raytracing::*;

pub struct Sphere<T> {

    pub center: Vec3<T>,
    pub radius: T

}

impl<T> RayTarget<T> for Sphere<T> where 
    T: num_traits::Float, Vec3<T>: Vec3View<T> {

    fn test_intersection(&self, ray: &Ray<T>) -> Option<RayHitInfo> {
        
        // Distance from ray origin to sphere center projected onto ray direction
        let dist_proj_len = (self.center - ray.origin).dot(ray.direction);
        
        if dist_proj_len < T::zero() {

            // Abort when we are beyond the center of the sphere
            Option::None

        } else {
            
            // TODO: Replace this with a better hit test
            if (ray.origin + ray.direction * dist_proj_len - self.center).length() < self.radius {
                Option::Some(RayHitInfo {})
            } else {
                Option::None
            }
        }
    }

}