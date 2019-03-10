use super::raytracing::*;

pub struct Scene<T> {
    pub geometry: Vec<Box<RayTarget<T>>>,
    // sky
}