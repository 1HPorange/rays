use super::vec3::*;
use super::raytracing::*;
use super::material::*;

// Sphere

pub struct Sphere<T> {

    pub center: Vec3<T>,
    pub radius: T,
    pub uv_mapper: Box<UvMapper<T>>

}

impl<T> Sphere<T> {

    pub fn new(center: Vec3<T>, radius: T, uv_mapper: Box<UvMapper<T>>) -> Sphere<T> {
        Sphere {
            center,
            radius,
            uv_mapper
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
            normal,
            uv: Vec2(T::zero(), T::zero()) // TODO: Implement. Also add an upAxis field to the sphere so the uvs can rotate
        })
    }
}

impl<T> HasUvMapper<T> for Sphere<T> {

    fn get_uv_mapper(&self) -> &Box<UvMapper<T>> {
        &self.uv_mapper
    }

}

// Infinite Plane

pub struct InifinitePlane<T> {
    origin: Vec3<T>,
    normal: Vec3Norm<T>,

    /// Used for mapping the uv x direction
    right: Vec3Norm<T>,
    uv_mapper: Box<UvMapper<T>>,

    /// A scale of one goes through one UV cycle per world unit.
    /// Higher scales squeeze the uv mapping, while lower scales
    /// stretch it.
    uv_scale: T,

    /// Calculated from normal and right. Helps with uv calculation
    up: Vec3Norm<T>
}

impl<T> InifinitePlane<T> where T: num_traits::Float {

    pub fn new(origin: Vec3<T>, normal: Vec3Norm<T>, right: Vec3Norm<T>, uv_mapper: Box<UvMapper<T>>, uv_scale: T) -> InifinitePlane<T> {

        // Normal and right Vector have to be at right angles
        assert!(normal.dot(right) < T::from(EPSILON).unwrap());

        let up = normal.cross(right).normalize();

        InifinitePlane {
            origin,
            normal,
            right,
            uv_mapper,
            uv_scale,
            up
        }
    }

    pub fn with_random_right(origin: Vec3<T>, normal: Vec3Norm<T>, uv_mapper: Box<UvMapper<T>>, uv_scale: T) -> InifinitePlane<T> {

        let right = normal.get_random_90_deg_vector().normalize();
        let up = normal.cross(right).normalize();

        InifinitePlane {
            origin,
            normal,
            right,
            uv_mapper,
            uv_scale,
            up
        }
    }
}

impl<T> RayTarget<T> for InifinitePlane<T> where T: num_traits::Float {

    fn  test_intersection(&self, ray: &Ray<T>) -> Option<GeometryHitInfo<T>> {

        let cos_ray_to_plane = self.normal.dot(ray.direction);

        if cos_ray_to_plane < -T::from(EPSILON).unwrap() {

            // angle larger 90 deg, so they have to meet at some point

            let plane_origin_to_ray_origin = ray.origin - self.origin;
            let plane_origin_distance_to_ray_origin = plane_origin_to_ray_origin.dot(self.normal);

            if plane_origin_distance_to_ray_origin > T::zero() {

                let ray_origin_to_plane_intersection_distance = plane_origin_distance_to_ray_origin / -cos_ray_to_plane;

                // Ray origin is not behind plane
                let hitpoint = ray.origin + ray.direction * ray_origin_to_plane_intersection_distance;

                // uv calculation
                let orig_to_hitpoint = hitpoint - self.origin;
                let mut uv_x = (orig_to_hitpoint.dot(self.right) * self.uv_scale).fract();
                let mut uv_y = (orig_to_hitpoint.dot(self.up) * self.uv_scale).fract();

                if uv_x < T::zero() {
                    uv_x = T::one() + uv_x;
                }

                if uv_y < T::zero() {
                    uv_y = T::one() + uv_y;
                }

                return Option::Some(GeometryHitInfo {
                    position: hitpoint,
                    normal: self.normal,
                    uv: Vec2(uv_x, uv_y)
                })
            }
        }
            
        Option::None
    }
}

impl<T> HasUvMapper<T> for InifinitePlane<T> {

    fn get_uv_mapper(&self) -> &Box<UvMapper<T>> {
        &self.uv_mapper
    }

}