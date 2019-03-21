use super::raytracing::*;
use super::material::*;

pub trait SceneObject<T>: RayTarget<T> + HasUvMapper<T> + Send + Sync {}
impl<'a, X,T> SceneObject<T> for X where X: RayTarget<T> + HasUvMapper<T> + Send + Sync {}

pub struct Scene<T> {
    pub objects: Vec<Box<SceneObject<T>>>,
    // sky
}