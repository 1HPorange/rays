use std::ops::*;

use super::vec3::*;
use super::raytracing::*;

struct Sphere<T> {

    center: Vec3<T>,
    radius: T

}

impl<T> RayTarget<T> for Sphere<T> where 
T: Sub<Output=T> + Mul<Output=T> + Add<Output=T> + Copy + PartialOrd + num_traits::Zero + num_traits::Float, 
Vec3<T>: Sub<Output=Vec3<T>> + Vec3View<T> {

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