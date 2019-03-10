use super::raytracing::*;
use super::material::*;

pub trait SceneObject<T>: RayTarget<T> + HasMaterialProvider<T> {}
impl<X,T> SceneObject<T> for X where X: RayTarget<T> + HasMaterialProvider<T> {}

pub struct Scene<T> {
    pub objects: Vec<Box<SceneObject<T>>>,
    // sky
}