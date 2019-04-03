use crate::vec::*;
use crate::uv_mappers::*;
use crate::ray_target::*;
use super::InifinitePlane;
use crate::raytracing::*;
use crate::parser::{const_f64_one, const_true};
use serde::Deserialize;
use std::sync::Arc;

pub struct Plane {

    pub origin: Vec3,

    // HALF of the plane extents
    pub width: f64,
    pub height: f64,

    pub uv_mapper: Arc<UvMapper>,
    pub visible_to_camera: bool,

    // Vectors calculated at time of construction
    normal: Vec3Norm,
    right: Vec3Norm,
    forwards: Vec3Norm,
}

#[derive(Default, Deserialize)]
#[serde(default)]
#[serde(deny_unknown_fields)]
pub struct PlaneInit {
    origin: Vec3,
    rotation: Vec3,

    #[serde(default = "const_f64_one")]
    width: f64,

    #[serde(default = "const_f64_one")]
    height: f64,

    #[serde(default = "const_true")]
    #[serde(rename = "visible-to-camera")]
    visible_to_camera: bool
}

impl Plane {

    pub fn new(init: &PlaneInit, uv_mapper: Arc<dyn UvMapper>) -> Plane {

        let normal = Vec3Norm::UP.rotate(init.rotation);
        let right = Vec3Norm::RIGHT.rotate(init.rotation);
        let forwards = right.cross(normal).normalized();

        Plane {
            origin: init.origin,
            width: init.width,
            height: init.height,
            uv_mapper,
            visible_to_camera: init.visible_to_camera,
            normal,
            right,
            forwards
        }
    }

}

impl RayTarget for Plane {

    fn test_intersection(&self, ray: &Ray) -> Option<GeometryHitInfo> {

        let hitpoint = InifinitePlane::get_ray_intersection(self.origin, self.normal, ray)?;

        let origin_to_hitpoint = hitpoint - self.origin;

        let w_proj = origin_to_hitpoint.dot(self.right) / self.width;

        if w_proj.abs() <= 1.0 {
            // We are inside the width bound

            let h_proj = origin_to_hitpoint.dot(self.forwards) / self.height;

            if h_proj.abs() <= 1.0 {
                // We are inside all bounds

                let u = (w_proj / 2.0) + 0.5;
                let v = (h_proj / 2.0) + 0.5;

                Some(GeometryHitInfo {
                    position: hitpoint,
                    normal: self.normal,
                    uv: Vec2::new(u, v)
                })
            } else {
                None
            }
        } else {
            None
        }
    }

    fn is_visible_to_camera(&self) -> bool {
        self.visible_to_camera
    }
}

impl HasUvMapper for Plane {

    fn get_uv_mapper(&self) -> &Arc<dyn UvMapper> {
        &self.uv_mapper
    }
}