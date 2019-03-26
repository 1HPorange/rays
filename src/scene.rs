use super::ray_target::*;
use super::uv_mappers::*;
use super::color::*;

pub trait SceneObject: RayTarget + HasUvMapper + Send + Sync {}
impl<'a, X> SceneObject for X where X: RayTarget + HasUvMapper + Send + Sync {}

// TODO: Remove pub
pub struct Scene {
    pub objects: Vec<Box<SceneObject>>
}

impl Scene {

    pub fn new() -> Scene {
        Scene {
            objects: vec![]
        }
    }

    pub fn add<O>(&mut self, object: O) where O: 'static + SceneObject {

        self.objects.push(Box::new(object))

    }

    pub fn validate(&self) -> bool {
        
        self.objects.iter()
            .all(|obj| obj.get_uv_mapper().validate())


    }
}