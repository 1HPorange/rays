use crate::vec3::*;
use crate::uv_mappers::*;
use crate::raytracing::*;

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
        assert!(normal.dot(right) < T::epsilon());

        let up = (-normal.cross(right)).normalize();

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
        let up = (-normal.cross(right)).normalize();

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

        if cos_ray_to_plane < -T::epsilon() {

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