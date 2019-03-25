use crate::vec::*;
use crate::uv_mappers::*;
use crate::raytracing::*;
use crate::ray_target::*;

pub struct Sphere {

    center: Vec3,
    radius: f64,
    uv_mapper: Box<UvMapper>,

    up: Vec3Norm,
    right: Vec3Norm
}

impl Sphere {

    pub fn new<U: 'static + UvMapper>(center: Vec3, radius: f64, uv_mapper: U, up: Vec3Norm, right: Vec3Norm) -> Sphere {

        assert!(up.dot(right) < std::f64::EPSILON);

        Sphere {
            center,
            radius,
            uv_mapper: Box::new(uv_mapper),
            up,
            right
        }
    }

    pub fn with_random_right<U: 'static + UvMapper>(center: Vec3, radius: f64, uv_mapper: U, up: Vec3Norm) -> Sphere {

        let right = up.get_random_90_deg_vector().normalized();

        Sphere {
            center,
            radius,
            uv_mapper: Box::new(uv_mapper),
            up,
            right
        }
    }
}

impl RayTarget for Sphere where {

    fn test_intersection(&self, ray: &Ray) -> Option<GeometryHitInfo> {
        
        // Squared radius
        let rad_sqr = self.radius * self.radius;

        // Ray origin to sphere center
        let orig_to_center = self.center - ray.origin;

        // If the ray starts inside, we can skip some checks, since it is a guaranteed hit
        let ray_starts_inside = orig_to_center.sqr_length() < rad_sqr;

        // Distance from ray origin to sphere center projected onto ray direction
        let orig_to_midpoint_len = orig_to_center.dot(ray.direction);
        
        let ray_starts_behind_center = orig_to_midpoint_len < 0.0;

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

        let normal = ((hitpoint - self.center) / self.radius).into_normalized_unsafe();

        let uv_x = normal
            .project_onto_plane_through_origin(self.up)
            .angle_to_on_plane(self.right, self.up, false)
            / 360.0;

        let uv_y = 1.0 - normal
            .project_onto_plane_through_origin(self.right)
            .angle_to_on_plane(self.up, self.right, true)
            .abs()
            / 180.0;

        Option::Some(GeometryHitInfo {
            position: hitpoint,
            normal,
            uv: Vec2::new(uv_x, uv_y)
        })
    }
}

impl HasUvMapper for Sphere {

    fn get_uv_mapper(&self) -> &Box<UvMapper> {
        &self.uv_mapper
    }

}