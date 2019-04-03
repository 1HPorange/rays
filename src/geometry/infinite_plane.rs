use crate::vec::*;
use crate::uv_mappers::*;
use crate::raytracing::*;
use crate::ray_target::*;
use crate::parser::{const_f64_one, const_true};
use serde::Deserialize;
use std::sync::Arc;

pub struct InifinitePlane {
    origin: Vec3,
    normal: Vec3Norm,

    /// Used for mapping the uv x direction
    right: Vec3Norm,
    uv_mapper: Arc<dyn UvMapper>,

    uv_scale: f64,

    visible_to_camera: bool,

    /// Calculated from normal and right. Helps with uv calculation
    forwards: Vec3Norm
}

#[derive(Default, Deserialize)]
#[serde(default)]
#[serde(deny_unknown_fields)]
pub struct InfinitePlaneInit {
    origin: Vec3,
    rotation: Vec3,

    #[serde(default = "const_f64_one")]
    #[serde(rename = "uv-scale")]
    uv_scale: f64,

    #[serde(default = "const_true")]
    #[serde(rename = "visible-to-camera")]
    visible_to_camera: bool
}

impl InifinitePlane {

    pub fn new(init: &InfinitePlaneInit, uv_mapper: Arc<dyn UvMapper>) -> InifinitePlane {

        let normal = Vec3Norm::UP.rotate(init.rotation);
        let right = Vec3Norm::RIGHT.rotate(init.rotation);
        let forwards = right.cross(normal).normalized();

        // TODO: Fix this atrocious order
        InifinitePlane {
            origin: init.origin,
            normal,
            right,
            uv_mapper,
            uv_scale: init.uv_scale,
            visible_to_camera: init.visible_to_camera,
            forwards
        }
    }

    pub fn get_ray_intersection(origin: Vec3, normal: Vec3Norm, ray: &Ray) -> Option<Vec3> {

        let cos_ray_to_plane = normal.dot(ray.direction);

        if cos_ray_to_plane < -std::f64::EPSILON {

            // angle larger 90 deg, so they have to meet at some point (but could still meet behind the ray origin)

            let plane_origin_to_ray_origin = ray.origin - origin;
            let plane_origin_distance_to_ray_origin = plane_origin_to_ray_origin.dot(normal);

            if plane_origin_distance_to_ray_origin > 0.0 {

                // Ray origin is not behind plane, so we have a guaranteed hit on an inifinite plane

                let ray_origin_to_plane_intersection_distance = plane_origin_distance_to_ray_origin / -cos_ray_to_plane;
                
                Some(ray.origin + ray.direction * ray_origin_to_plane_intersection_distance)
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl RayTarget for InifinitePlane {

    fn is_visible_to_camera(&self) -> bool {
        self.visible_to_camera
    }

    fn test_intersection(&self, ray: &Ray) -> Option<GeometryHitInfo> {

        let hitpoint = InifinitePlane::get_ray_intersection(self.origin, self.normal, ray)?;

        // uv calculation
        let orig_to_hitpoint = hitpoint - self.origin;
        let mut uv_x = (orig_to_hitpoint.dot(self.right) * self.uv_scale).fract();
        let mut uv_y = (orig_to_hitpoint.dot(self.forwards) * self.uv_scale).fract();

        if uv_x < 0.0 {
            uv_x = 1.0 + uv_x;
        }

        if uv_y < 0.0 {
            uv_y = 1.0 + uv_y;
        }

        Some(GeometryHitInfo {
            position: hitpoint,
            normal: self.normal,
            uv: Vec2::new(uv_x, uv_y)
        })
    }
}

impl HasUvMapper for InifinitePlane {

    fn get_uv_mapper(&self) -> &Arc<dyn UvMapper> {
        &self.uv_mapper
    }

}