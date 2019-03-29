use serde::Deserialize;
use crate::prelude::*;
use std::sync::Arc;
use std::collections::HashMap;

// TODO: This whole module is pretty awful, but I'm not sure what to do about it

#[derive(Deserialize)]
#[serde(deny_unknown_fields)] 
struct NamedMaterial {
    name: String,

    #[serde(flatten)]
    material: Material
}

#[derive(Default, Deserialize)]
#[serde(default)]
#[serde(deny_unknown_fields)] 
struct NamedCamera {

    name: String,
    
    #[serde(flatten)]
    camera: Camera
}

#[derive(Default, Deserialize)]
#[serde(default)]
#[serde(deny_unknown_fields)] 
struct NamedRenderParams {

    name: String,

    #[serde(flatten)]
    render_params: RenderParams
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)] 
struct UvmCheckerboardInit {
    name: String,
    even: String,
    odd: String
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)] 
struct UvmTextureInit {
    name: String,

    #[serde(default)]
    base: String,

    path: String,

    #[serde(default)]
    sampling: SamplingMethod
}

#[derive(Default, Deserialize)]
#[serde(default)]
#[serde(deny_unknown_fields)] 
struct SphereInit {

    #[serde(rename = "uv-mapper")]
    uv_mapper: String,

    origin: Vec3,

    #[serde(default = "const_f64_one")]
    radius: f64,

    rotation: Vec3,

    #[serde(rename = "visible-to-camera")]
    #[serde(default = "const_true")]
    visible_to_camera: bool
}

#[derive(Default, Deserialize)]
#[serde(default)]
#[serde(deny_unknown_fields)] 
struct InfinitePlaneInit {

    #[serde(rename = "uv-mapper")]
    uv_mapper: String,

    origin: Vec3,

    rotation: Vec3,

    #[serde(default = "const_f64_one")]
    #[serde(rename = "uv-scale")]
    uv_scale: f64,

    #[serde(rename = "visible-to-camera")]
    #[serde(default = "const_true")]
    visible_to_camera: bool
}

#[derive(Default, Deserialize)]
#[serde(default)]
#[serde(deny_unknown_fields)] 
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
    cameras: Vec<NamedCamera>,

    #[serde(rename = "render-params")]
    render_params: Vec<NamedRenderParams>
}

// Useful defaults

fn const_f64_one() -> f64 {
    1.0
}

fn const_true() -> bool {
    true
}

// Public stuff

pub struct Config {
    pub scene: Scene,
    pub camera_config: CameraConfig,
    pub render_params_config: RenderParamsConfig
}

pub enum CameraConfig {
    Single(Camera),
    Multiple(HashMap<String, Camera>)
}

pub enum RenderParamsConfig {
    Single(RenderParams),
    Multiple(HashMap<String, RenderParams>)
}

// TODO: Get rid of code duplication all over this module
pub fn parse<P: AsRef<std::path::Path>>(path: P) -> Result<Config, Box<std::error::Error>> {

    let content = std::fs::read_to_string(path)?;

    let config: RawConfig = toml::from_str(&content)?;

    let mut into_uvm_map: HashMap<&str, &dyn IntoUvMapper> = HashMap::new();

    // Check if all materials and uv mapeprs combined have unique keys

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

    // generate all uv mappers out of material and uv mapper descriptions
    // and put them into a map
    let mut uv_mapper_map = HashMap::new();

    for (key, into_uvm) in into_uvm_map {

        let uvm = into_uvm.into_uv_mapper(&mat_map)
            .map_err(|err| format!("{}: {}", key, err))?;

        uv_mapper_map.insert(key, uvm);
    }

    // Construct all geometry and associate it with uv mappers
    let mut scene = Scene::new();

    // Let's start with all the spheres
    for init in config.spheres {

        if init.uv_mapper.is_empty() {
            let uvm = Arc::new(StaticUvMapper(Material::default()));

            add_sphere_to_scene(&mut scene, uvm, &init);
        } else {
            let uvm = uv_mapper_map.get(&init.uv_mapper[..])
                .ok_or(format!("UV mapper or material \"{}\" not found", init.uv_mapper))?;

            add_sphere_to_scene(&mut scene, Arc::clone(uvm), &init);
        };
    }

    // And now let's do the infinite planes
    for init in config.infinite_planes {

        if init.uv_mapper.is_empty() {
            let uvm = Arc::new(StaticUvMapper(Material::default()));

            add_infinite_plane_to_scene(&mut scene, uvm, &init);
        } else {
            let uvm = uv_mapper_map.get(&init.uv_mapper[..])
                .ok_or(format!("UV mapper or material \"{}\" not found", init.uv_mapper))?;

            add_infinite_plane_to_scene(&mut scene, Arc::clone(uvm), &init);
        };
    }

    // Now we handle the cameras

    let camera_config = if config.cameras.is_empty() {
        CameraConfig::Single(Camera::default())
    } else if config.cameras.len() == 1 {
        CameraConfig::Single(config.cameras[0].camera)
    } else {
        // If we have multiple cameras, we need to make sure that
        // their keys are unique
        let mut cam_map = HashMap::new();

        for named_cam in config.cameras {
            if  !named_cam.name.is_empty() &&
                cam_map.insert(named_cam.name, named_cam.camera).is_some() {
                return Err("Multiple cameras must have unique, non-empty name keys".into());
            }
        }

        CameraConfig::Multiple(cam_map)
    };

    // And finally the render parameters

    let render_params_config = if config.render_params.is_empty() {
        RenderParamsConfig::Single(RenderParams::default())
    } else if config.render_params.len() == 1 {
        RenderParamsConfig::Single(config.render_params[0].render_params)
    } else {
        // Again, we need to make sure that all RenderParams
        // have unique keys
        let mut rp_map = HashMap::new();

        for named_rp in config.render_params {
            if  !named_rp.name.is_empty() &&
                rp_map.insert(named_rp.name, named_rp.render_params).is_some() {
                return Err("Multiple render-params structs must have unique, non-empty name keys".into());
            }
        }

        RenderParamsConfig::Multiple(rp_map)
    };

    Ok(Config {
        scene,
        camera_config,
        render_params_config
    })
}

fn add_sphere_to_scene(scene: &mut Scene, uv_mapper: Arc<dyn UvMapper>, init: &SphereInit) {

    let sphere = Sphere::with_rotation(
        init.origin, 
        init.radius, 
        init.rotation, 
        uv_mapper,
        init.visible_to_camera);

    scene.add(sphere);
}

fn add_infinite_plane_to_scene(scene: &mut Scene, uv_mapper: Arc<dyn UvMapper>, init: &InfinitePlaneInit) {

    let infinite_plane = InifinitePlane::with_rotation(
        init.origin, 
        init.rotation, 
        uv_mapper, 
        init.uv_scale,
        init.visible_to_camera);

    scene.add(infinite_plane);
}

trait IntoUvMapper {
    fn name(&self) -> &str;
    fn into_uv_mapper(&self, mat_map: &HashMap<&str, Material>) 
        -> Result<Arc<dyn UvMapper>, Box<std::error::Error>>;
}

impl IntoUvMapper for NamedMaterial {

    fn name(&self) -> &str { &self.name }

    fn into_uv_mapper(&self, _mat_map: &HashMap<&str, Material>) 
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
        
        let base_mat = if self.base.is_empty() {
            Material::default()
        } else {
            mat_map.get(&self.base[..])
                .ok_or("Material for key \"base_mat\" not found")?
                .clone()
        };

        let uvm = TextureUvMapper::from_png_24(&self.path, base_mat, self.sampling)?;

        Ok(Arc::new(uvm))
    }
}
