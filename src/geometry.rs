use super::vec3::*;
use super::raytracing::*;
use super::material::*;

// Sphere

pub struct Sphere<T> {

    pub center: Vec3<T>,
    pub radius: T,
    pub material_provider: Box<HasMaterial<T>> // TODO: Rename this atrocious trait

}

impl<'a, T> Sphere<T> {

    pub fn new(center: Vec3<T>, radius: T, material_provider: Box<HasMaterial<T>>) -> Sphere<T> {
        Sphere {
            center,
            radius,
            material_provider
        }
    }

}

impl<T> RayTarget<T> for Sphere<T> where 
    T: num_traits::Float, Vec3<T>: Vec3View<T> {

    fn test_intersection(&self, ray: &Ray<T>) -> Option<GeometryHitInfo<T>> {
        
        // Squared radius
        let rad_sqr = self.radius * self.radius;

        // Ray origin to sphere center
        let orig_to_center = self.center - ray.origin;

        // If the ray starts inside, we can skip some checks, since it is a guaranteed hit
        let ray_starts_inside = orig_to_center.sqr_length() < rad_sqr;

        // Distance from ray origin to sphere center projected onto ray direction
        let orig_to_midpoint_len = orig_to_center.dot(ray.direction);
        
        let ray_starts_behind_center = orig_to_midpoint_len < T::zero();

        if !ray_starts_inside && ray_starts_behind_center {

            // We are completely behind the sphere, so we abort
            return Option::None
        }
            
        let orig_to_midpoint = ray.direction * orig_to_midpoint_len;
        let midpoint_to_center_sqr = (ray.origin + orig_to_midpoint - self.center).sqr_length();

        if !ray_starts_inside && midpoint_to_center_sqr > rad_sqr {

            // Abort when our ray misses the sphere completely
            return Option::None
        }

        let midpoint_to_surface = (rad_sqr - midpoint_to_center_sqr).sqrt();

        // If we start inside the sphere, we always hit the "back wall"
        let hitpoint = if ray_starts_inside {
            ray.origin + ray.direction * (orig_to_midpoint_len + midpoint_to_surface)
        } else {
            ray.origin + ray.direction * (orig_to_midpoint_len - midpoint_to_surface)
        };
        let normal = ((hitpoint - self.center) / self.radius).into_normalized();

        Option::Some(GeometryHitInfo {
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