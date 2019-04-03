use crate::vec::*;
use crate::uv_mappers::*;
use crate::raytracing::*;
use crate::ray_target::*;
use crate::parser::{const_f64_one, const_true};
use serde::Deserialize;
use std::sync::Arc;

pub struct Sphere {

    origin: Vec3,
    radius: f64,
    uv_mapper: Arc<dyn UvMapper>,

    up: Vec3Norm,
    right: Vec3Norm,

    visible_to_camera: bool
}

#[derive(Default, Deserialize)]
#[serde(default)]
#[serde(deny_unknown_fields)] 
pub struct SphereInit {

    origin: Vec3,

    #[serde(default = "const_f64_one")]
    radius: f64,

    rotation: Vec3,

    #[serde(default = "const_true")]
    #[serde(rename = "visible-to-camera")]
    visible_to_camera: bool
}

impl Sphere {

    pub fn new(init: &SphereInit, uv_mapper: Arc<dyn UvMapper>) -> Sphere {
        let up = Vec3Norm::UP.rotate(init.rotation);
        let right = Vec3Norm::RIGHT.rotate(init.rotation);

        Sphere {
            origin: init.origin,
            radius: init.radius,
            uv_mapper,
            up,
            right,
            visible_to_camera: init.visible_to_camera
        }
    }
}

impl RayTarget for Sphere where {

    fn is_visible_to_camera(&self) -> bool {
        self.visible_to_camera
    }

    fn test_intersection(&self, ray: &Ray) -> Option<GeometryHitInfo> {
        
        // Squared radius
        let rad_sqr = self.radius * self.radius;

        // Ray origin to sphere center
        let orig_to_center = self.origin - ray.origin;

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
        let midpoint_to_center_sqr = (ray.origin + orig_to_midpoint - self.origin).sqr_length();

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

        let normal = ((hitpoint - self.origin) / self.radius).into_normalized_unsafe();

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

    fn get_uv_mapper(&self) -> &Arc<dyn UvMapper> {
        &self.uv_mapper
    }

}