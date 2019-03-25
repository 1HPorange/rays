use serde::Deserialize;
use crate::material::*;
use crate::vec::*;

// TODO: This whole module is pretty awful, but I'm not sure what to do about it

#[derive(Deserialize)]
struct NamedMaterial {
    name: String,

    #[serde(flatten)]
    material: Material
}

#[derive(Deserialize)]
struct UvmCheckerboardInit {
    name: String,
    even: String,
    odd: String
}

#[derive(Deserialize)]
struct UvmTextureInit {
    name: String,
    path: String
}

#[derive(Deserialize)]
struct SphereInit {

    #[serde(rename = "uv-mapper")]
    uv_mapper: String,

    #[serde(default)]
    origin: Vec3,

    #[serde(default = "f64_one")]
    radius: f64,

    #[serde(default)]
    rotation: Vec3
}

#[derive(Deserialize)]
struct InfinitePlaneInit {

    #[serde(rename = "uv-mapper")]
    uv_mapper: String,

    #[serde(default)]
    origin: Vec3,

    #[serde(default)]
    rotation: Vec3,

    #[serde(default = "f64_one")]
    uv_scale: f64
}

#[derive(Deserialize)]
struct Config {

    materials: Vec<NamedMaterial>,

    #[serde(rename = "uvm-checkerboard")]
    uvm_checkerboards: Vec<UvmCheckerboardInit>,

    #[serde(rename = "uvm-texture")]
    uvm_textures: Vec<UvmTextureInit>,

    #[serde(rename = "obj-sphere")]
    spheres: Vec<SphereInit>,

    #[serde(rename = "obj-infinite-plane")]
    infinite_planes: Vec<InfinitePlaneInit>
}

// Useful defaults

fn f64_one() -> f64 {
    1.0
}

pub fn parse<P: AsRef<std::path::Path>>(path: P) -> Result<(), Box<std::error::Error>> {

    let content = std::fs::read_to_string(path)?;

    let config: Config = toml::from_str(&content)?;

    for m in config.spheres {
        dbg!(m.uv_mapper);
        //dbg!(mat.material.reflection.center);
    }

    std::process::exit(0);

    Ok(())
}