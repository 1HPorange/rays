extern crate num_traits;
extern crate rand;
extern crate rayon;
extern crate lodepng;

mod camera;
mod color;
mod geometry;
mod material;
mod output;
mod post_processing;
mod raytracing;
mod scene;
mod vec3;
pub mod uv_mappers;

pub mod prelude;

// Re-export stuff that get only used from time to time

pub use raytracing::{render, RenderingParameters};
pub use scene::Scene;
pub use camera::Camera;
pub use output::RenderTarget;