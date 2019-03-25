use super::ray_target::*;
use super::uv_mappers::*;
use super::color::*;

pub trait SceneObject: RayTarget + HasUvMapper + Send + Sync {}
impl<'a, X> SceneObject for X where X: RayTarget + HasUvMapper + Send + Sync {}

// TODO: Remove pub
pub struct Scene {
    pub objects: Vec<Box<SceneObject>>,

    /// This is the color returned when a ray doesn't hit anything
    /// If you want a more complex skybox, add it manually as an object
    pub sky_color: RGBColor
}

impl Scene {

    pub fn new(sky_color: RGBColor) -> Scene {

        Scene {
            objects: vec![],
            sky_color
        }
    }

    pub fn add<O>(&mut self, object: O) where O: 'static + SceneObject {

        self.objects.push(Box::new(object))

    }

    pub fn validate(&self) -> bool {
        
        self.sky_color.validate() && self.objects.iter()
            .all(|obj| obj.get_uv_mapper().validate())


    }
}