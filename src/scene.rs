use super::raytracing::*;
use super::uv_mappers::*;
use super::color::*;

pub trait SceneObject<T>: RayTarget<T> + HasUvMapper<T> + Send + Sync {}
impl<'a, X,T> SceneObject<T> for X where X: RayTarget<T> + HasUvMapper<T> + Send + Sync {}

// TODO: Remove pub
pub struct Scene<T> {
    pub objects: Vec<Box<SceneObject<T>>>,

    /// This is the color returned when a ray doesn't hit anything
    /// If you want a more complex skybox, add it manually as an object
    pub sky_color: RGBColor
}

impl<T> Scene<T> where T: num_traits::Float + num_traits::FloatConst + Send + Sync {

    // TODO: Do same thing for geometry (= do the boxing inside, not outside the call)
    pub fn new(sky_color: RGBColor) -> Scene<T> {

        Scene {
            objects: vec![],
            sky_color
        }
    }

    pub fn add_object<O>(&mut self, object: O) where O: 'static + SceneObject<T> {

        self.objects.push(Box::new(object))

    }
}