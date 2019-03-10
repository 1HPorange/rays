use super::vec3::*;
use super::raytracing::*;
use super::material::*;
use super::color::*;

pub struct Sphere<T> {

    pub center: Vec3<T>,
    pub radius: T,
    pub material_provider: Box<HasMaterial<T>>

}

impl<T> RayTarget<T> for Sphere<T> where 
    T: num_traits::Float, Vec3<T>: Vec3View<T> {

    fn test_intersection(&self, ray: &Ray<T>) -> Option<RayHitInfo<T>> {
        
        // Squared radius
        let rad_sqr = self.radius * self.radius;

        // Distance from ray origin to sphere center
        let dist = self.center - ray.origin;

        if dist.sqr_length() < rad_sqr {

            // Abort when ray starts inside the sphere
            return Option::None
        }

        // Distance from ray origin to sphere center projected onto ray direction
        let dist_proj_len = dist.dot(ray.direction);
        
        if dist_proj_len < T::zero() {

            // Abort when we are behind sphere
            return Option::None
        }
            
        let orig_to_midpoint = ray.direction * dist_proj_len;
        let midpoint_to_center_sqr = (ray.origin + orig_to_midpoint - self.center).sqr_length();

        if midpoint_to_center_sqr > rad_sqr {

            // Abort when we miss the sphere
            return Option::None
        }

        let midpoint_to_surface = (rad_sqr - midpoint_to_center_sqr).sqrt();

        let hitpoint = ray.origin + ray.direction * (dist_proj_len - midpoint_to_surface);
        let normal = ((hitpoint - self.center) / self.radius).into_normalized();

        Option::Some(RayHitInfo {
            position: hitpoint,
            normal
        })

    }

}

impl<T> HasMaterialProvider<T> for Sphere<T> {

    fn get_material_provider(&self) -> &Box<HasMaterial<T>> {
        &self.material_provider
    }

}