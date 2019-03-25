// Things that you need so often that you don't want to write rays:: before it all the time

pub use crate::material::{Material, Reflection, Refraction, Opacity};
pub use crate::color::RGBColor;
pub use crate::uv_mappers::*;
pub use crate::vec::{Vec3, Vec3Norm};
pub use crate::geometry::{sphere::*, ininite_plane::*};
pub use crate::scene::Scene;
pub use crate::camera::{Camera, ViewPort};
pub use crate::output::RenderTarget;
pub use crate::render_params::*;