use crate::vec3::*;
use crate::uv_mappers::*;
use crate::raytracing::*;

pub struct Sphere<T> {

    center: Vec3<T>,
    radius: T,
    uv_mapper: Box<UvMapper<T>>,

    up: Vec3Norm<T>,
    right: Vec3Norm<T>
}

impl<T> Sphere<T> where T: num_traits::Float {

    pub fn new<U: 'static + UvMapper<T>>(center: Vec3<T>, radius: T, uv_mapper: U, up: Vec3Norm<T>, right: Vec3Norm<T>) -> Sphere<T> {

        assert!(up.dot(right) < T::epsilon());

        Sphere {
            center,
            radius,
            uv_mapper: Box::new(uv_mapper),
            up,
            right
        }
    }

    pub fn with_random_right<U: 'static + UvMapper<T>>(center: Vec3<T>, radius: T, uv_mapper: U, up: Vec3Norm<T>) -> Sphere<T> {

        let right = up.get_random_90_deg_vector().normalize();

        Sphere {
            center,
            radius,
            uv_mapper: Box::new(uv_mapper),
            up,
            right
        }
    }
}

impl<T> RayTarget<T> for Sphere<T> where 
    T: num_traits::Float + num_traits::FloatConst, Vec3<T>: Vec3View<T> {

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

        // If we start inside the sphere, we always hit the "back wall" (else clause)
        let hitpoint = if ray_starts_inside {
            ray.origin + ray.direction * (orig_to_midpoint_len + midpoint_to_surface)
        } else {
            ray.origin + ray.direction * (orig_to_midpoint_len - midpoint_to_surface)
        };

        let normal = ((hitpoint - self.center) / self.radius).into_normalized();

        let uv_x = normal
            .project_onto_plane_from_same_origin(self.up)
            .angle_to(self.right, self.up, false)
            / T::from(360.0).unwrap();

        let uv_y = T::one() - normal
            .project_onto_plane_from_same_origin(self.right)
            .angle_to(self.up, self.right, true)
            .abs()
            / T::from(180.0).unwrap();

        Option::Some(GeometryHitInfo {
            position: hitpoint,
            normal,
            uv: Vec2(uv_x, uv_y) // TODO: Implement. Also add an upAxis field to the sphere so the uvs can rotate
        })
    }
}

impl<T> HasUvMapper<T> for Sphere<T> {

    fn get_uv_mapper(&self) -> &Box<UvMapper<T>> {
        &self.uv_mapper
    }

}