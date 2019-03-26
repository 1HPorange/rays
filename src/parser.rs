use serde::Deserialize;
use crate::prelude::*;
use std::sync::Arc;
use std::collections::HashMap;
use std::io;

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
    base_mat: String,
    path: String,

    #[serde(default)]
    sampling: SamplingMethod
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
struct RawConfig {

    #[serde(rename = "material")]
    materials: Vec<NamedMaterial>,

    #[serde(rename = "uvm-checkerboard")]
    uvm_checkerboards: Vec<UvmCheckerboardInit>,

    #[serde(rename = "uvm-texture")]
    uvm_textures: Vec<UvmTextureInit>,

    #[serde(rename = "obj-sphere")]
    spheres: Vec<SphereInit>,

    #[serde(rename = "obj-infinite-plane")]
    infinite_planes: Vec<InfinitePlaneInit>,

    #[serde(rename = "camera")]
    cameras: Vec<Camera>,

    #[serde(rename = "render-params")]
    render_params: Vec<RenderParams>
}

pub struct Config {
    scene: Scene,
    cameras: HashMap<String, Camera>,
    render_params: HashMap<String, RenderParams>
}

// Useful defaults

fn f64_one() -> f64 {
    1.0
}

pub fn parse<P: AsRef<std::path::Path>>(path: P) -> Result<(), Box<std::error::Error>> {

    let content = std::fs::read_to_string(path)?;

    let config: RawConfig = toml::from_str(&content)?;

    let mut into_uvm_map: HashMap<&str, &dyn IntoUvMapper> = HashMap::new();

    // Check if all materials and uv mapeprs combined have unique keys

    // TODO: Get rid of code duplication
    let all_unique = config.materials.iter()
        .all(|x| into_uvm_map.insert(x.name(), x).is_none()) &&
        config.uvm_checkerboards.iter()
        .all(|x| into_uvm_map.insert(x.name(), x).is_none()) &&
        config.uvm_textures.iter()
        .all(|x| into_uvm_map.insert(x.name(), x).is_none());

    if !all_unique {
        return Err("The set of names of all materials and uv mappers 
            combined must not contain duplicates".into());
    }

    // put all materials (but not uv mappers!) into a map
    let mut mat_map = HashMap::new();
    for mat in &config.materials {
        mat_map.insert(mat.name(), mat.material);
    }

    // generate all uv mappers out material and uv mapper descriptions
    // and put them into a map
    let mut uv_mapper_map = HashMap::new();

    for (key, into_uvm) in into_uvm_map {

        let uvm = into_uvm.into_uv_mapper(&mat_map)
            .map_err(|err| format!("{}: {}", key, err))?;

        uv_mapper_map.insert(key, uvm);
    }



    std::process::exit(0);

    Ok(())
}

trait IntoUvMapper {
    fn name(&self) -> &str;
    fn into_uv_mapper(&self, mat_map: &HashMap<&str, Material>) 
        -> Result<Arc<dyn UvMapper>, Box<std::error::Error>>;
}

impl IntoUvMapper for NamedMaterial {

    fn name(&self) -> &str { &self.name }

    fn into_uv_mapper(&self, mat_map: &HashMap<&str, Material>) 
        -> Result<Arc<dyn UvMapper>, Box<std::error::Error>> {
        Ok(Arc::new(StaticUvMapper(self.material)))
    }
}

impl IntoUvMapper for UvmCheckerboardInit {

    fn name(&self) -> &str { &self.name }

    fn into_uv_mapper(&self, mat_map: &HashMap<&str, Material>) 
        -> Result<Arc<dyn UvMapper>, Box<std::error::Error>> {

        let even_mat = mat_map.get(&self.even[..])
            .ok_or("Material for key \"even\" not found")?
            .clone();

        let odd_mat = mat_map.get(&self.odd[..])
            .ok_or("Material for key \"odd\" not found")?
            .clone();
        
        Ok(Arc::new(CheckerboardUvMapper(even_mat, odd_mat)))
    }
}

impl IntoUvMapper for UvmTextureInit {

    fn name(&self) -> &str { &self.name }

   fn into_uv_mapper(&self, mat_map: &HashMap<&str, Material>) 
        -> Result<Arc<dyn UvMapper>, Box<std::error::Error>> {
        
        let base_mat = mat_map.get(&self.base_mat[..])
            .ok_or("Material for key \"base_mat\" not found")?
            .clone();

        let uvm = TextureUvMapper::from_png_24(&self.path, base_mat, self.sampling)?;

        Ok(Arc::new(uvm))
    }
}