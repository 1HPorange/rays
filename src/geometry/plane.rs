use crate::vec::*;
use crate::uv_mappers::*;
use crate::ray_target::*;
use super::InifinitePlane;
use crate::raytracing::*;
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

impl Plane {

    pub fn new(origin: Vec3, rotation: Vec3, width: f64, height: f64, uv_mapper: Arc<dyn UvMapper>, visible_to_camera: bool) -> Plane {

        let normal = Vec3Norm::UP.rotate(rotation);
        let right = Vec3Norm::RIGHT.rotate(rotation);
        let forwards = right.cross(normal).normalized();

        Plane {
            origin,
            width,
            height,
            uv_mapper,
            visible_to_camera,
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