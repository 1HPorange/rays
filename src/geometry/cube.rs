use crate::parser::{const_f64_one, const_true};
use crate::prelude::*;
use serde::Deserialize;
use crate::ray_target::*;
use crate::raytracing::*;
use std::sync::Arc;

pub struct Cube {
    planes: [Plane; 6],
    visible_to_camera: bool
}

#[derive(Default, Deserialize)]
#[serde(default)]
#[serde(deny_unknown_fields)]
pub struct CubeInit {
    origin: Vec3,
    rotation: Vec3,

    #[serde(default = "const_f64_one")]
    width: f64,

    #[serde(default = "const_f64_one")]
    height: f64,

    #[serde(default = "const_f64_one")]
    depth: f64,

    #[serde(default = "const_true")]
    #[serde(rename = "visible-to-camera")]
    visible_to_camera: bool
}

impl Cube {
    pub fn new(init: &CubeInit, uv_mapper: Arc<dyn UvMapper>) -> Cube {



        // Calculate the object space positions of the planes
        // respecting the global orientation
        let x_min_offset = (Vec3Norm::LEFT * init.width)
            .rotate(init.rotation);

        let x_max_offset = (Vec3Norm::RIGHT * init.width)
            .rotate(init.rotation);

        let y_min_offset = (Vec3Norm::DOWN * init.height)
            .rotate(init.rotation);

        let y_max_offset = (Vec3Norm::UP * init.height)
            .rotate(init.rotation);

        let z_min_offset = (Vec3Norm::BACK * init.depth)
            .rotate(init.rotation);

        let z_max_offset = (Vec3Norm::FORWARD * init.depth)
            .rotate(init.rotation);
        
        // Create the six planes that make up a cube

        let x_min = PlaneInit {
            origin: init.origin + x_min_offset,
            rotation: init.rotation + Vec3::new(-90.0, 90.0, 0.0),
            width: init.depth,
            height: init.height,
            visible_to_camera: init.visible_to_camera
        };

        let x_max = PlaneInit {
            origin: init.origin + x_max_offset,
            rotation: init.rotation + Vec3::new(-90.0, -90.0, 0.0),
            width: init.depth,
            height: init.height,
            visible_to_camera: init.visible_to_camera
        };

        let y_min = PlaneInit {
            origin: init.origin + y_min_offset,
            rotation: init.rotation + Vec3::new(180.0, 0.0, 0.0),
            width: init.width,
            height: init.depth,
            visible_to_camera: init.visible_to_camera
        };

        let y_max = PlaneInit {
            origin: init.origin + y_max_offset,
            rotation: init.rotation,
            width: init.width,
            height: init.depth,
            visible_to_camera: init.visible_to_camera
        };

        let z_min = PlaneInit {
            origin: init.origin + z_min_offset,
            rotation: init.rotation + Vec3::new(-90.0, 0.0, 0.0),
            width: init.width,
            height: init.height,
            visible_to_camera: init.visible_to_camera
        };

        let z_max = PlaneInit {
            origin: init.origin + z_max_offset,
            rotation: init.rotation + Vec3::new(-90.0, 180.0, 0.0),
            width: init.width,
            height: init.height,
            visible_to_camera: init.visible_to_camera
        };

        Cube {
            planes: [
                Plane::new(&x_min, Arc::clone(&uv_mapper)),
                Plane::new(&x_max, Arc::clone(&uv_mapper)),

                Plane::new(&y_min, Arc::clone(&uv_mapper)),
                Plane::new(&y_max, Arc::clone(&uv_mapper)),

                Plane::new(&z_min, Arc::clone(&uv_mapper)),
                Plane::new(&z_max, uv_mapper),
            ],
            visible_to_camera: init.visible_to_camera
        }
    }
}

impl RayTarget for Cube {

    fn is_visible_to_camera(&self) -> bool {
        self.visible_to_camera
    }

    fn test_intersection(&self, ray: &Ray) -> Option<GeometryHitInfo> {

        self.planes.iter()
            .map(|p| p.test_intersection(ray))
            .filter(|hit| hit.is_some())
            .map(|hit| hit.unwrap())
            .min_by(|a, b| hit_dist_comp(ray.origin, a, b))
    }
}

impl HasUvMapper for Cube {
    fn get_uv_mapper(&self) -> &Arc<dyn UvMapper> {
        &self.planes[0].uv_mapper
    }
}