use crate::vec::*;
use crate::uv_mappers::*;
use crate::raytracing::*;
use crate::ray_target::*;

pub struct InifinitePlane {
    origin: Vec3,
    normal: Vec3Norm,

    /// Used for mapping the uv x direction
    right: Vec3Norm,
    uv_mapper: Box<UvMapper>,

    /// A scale of one goes through one UV cycle per world unit.
    /// Higher scales squeeze the uv mapping, while lower scales
    /// stretch it.
    uv_scale: f64,

    /// Calculated from normal and right. Helps with uv calculation
    up: Vec3Norm
}

impl InifinitePlane {

    pub fn new<U: 'static + UvMapper>(origin: Vec3, normal: Vec3Norm, right: Vec3Norm, uv_mapper: U, uv_scale: f64) -> InifinitePlane {

        // Normal and right Vector have to be at right angles
        assert!(normal.dot(right) < std::f64::EPSILON);

        let up = (-normal.cross(right)).normalized();

        InifinitePlane {
            origin,
            normal,
            right,
            uv_mapper: Box::new(uv_mapper),
            uv_scale,
            up
        }
    }

    pub fn with_random_right<U: 'static + UvMapper>(origin: Vec3, normal: Vec3Norm, uv_mapper: U, uv_scale: f64) -> InifinitePlane {

        let right = normal.get_random_90_deg_vector().normalized();
        let up = (-normal.cross(right)).normalized();

        InifinitePlane {
            origin,
            normal,
            right,
            uv_mapper: Box::new(uv_mapper),
            uv_scale,
            up
        }
    }
}

impl RayTarget for InifinitePlane {

    fn  test_intersection(&self, ray: &Ray) -> Option<GeometryHitInfo> {

        let cos_ray_to_plane = self.normal.dot(ray.direction);

        if cos_ray_to_plane < -std::f64::EPSILON {

            // angle larger 90 deg, so they have to meet at some point

            let plane_origin_to_ray_origin = ray.origin - self.origin;
            let plane_origin_distance_to_ray_origin = plane_origin_to_ray_origin.dot(self.normal);

            if plane_origin_distance_to_ray_origin > 0.0 {

                let ray_origin_to_plane_intersection_distance = plane_origin_distance_to_ray_origin / -cos_ray_to_plane;

                // Ray origin is not behind plane
                let hitpoint = ray.origin + ray.direction * ray_origin_to_plane_intersection_distance;

                // uv calculation
                let orig_to_hitpoint = hitpoint - self.origin;
                let mut uv_x = (orig_to_hitpoint.dot(self.right) * self.uv_scale).fract();
                let mut uv_y = (orig_to_hitpoint.dot(self.up) * self.uv_scale).fract();

                if uv_x < 0.0 {
                    uv_x = 1.0 + uv_x;
                }

                if uv_y < 0.0 {
                    uv_y = 1.0 + uv_y;
                }

                return Option::Some(GeometryHitInfo {
                    position: hitpoint,
                    normal: self.normal,
                    uv: Vec2::new(uv_x, uv_y)
                })
            }
        }
            
        Option::None
    }
}

impl HasUvMapper for InifinitePlane {

    fn get_uv_mapper(&self) -> &Box<UvMapper> {
        &self.uv_mapper
    }

}