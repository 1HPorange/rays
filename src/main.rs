extern crate rays;

use rays::prelude::*;
use std::time::Instant;
use clap::{App, Arg};

fn main() {

    const ARG_CAMERA: &str = "camera";
    const ARG_RENDERPARAMS: &str = "render-params";
    const ARG_QUALITY_HINT: &str = "quality-hint";
    const ARG_QUALITY_OVERRIDE: &str = "quality-override";
    const ARG_WIDTH: &str = "width";
    const ARG_HEIGHT: &str = "height";
    const ARG_SCENE: &str = "SCENE";
    const ARG_OUTPUT: &str = "OUTPUT";
    
    const QUALITY_LEVELS: &[&str] = &["sketch", "low", "medium", "high", "ultra"];

    let cla = App::new("rays")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Markus W. <markuswebel@gmail.com>")
        .about("Renders a scene configuration file into a PNG file")
        .arg(Arg::with_name(ARG_CAMERA)
            .short("c")
            .long(ARG_CAMERA)
            .takes_value(true)
            .help("The name of the camera that you want to render the scene with. \
            This argument is not required if there are less than 2 cameras in the scene config."))
        .arg(Arg::with_name(ARG_RENDERPARAMS)
            .short("p")
            .long(ARG_RENDERPARAMS)
            .takes_value(true)
            .help("The name of the renderparameters struct in the config that you want to use. \
            can be omitted if there are less the 2 such structs in the scene config."))
        .arg(Arg::with_name(ARG_QUALITY_HINT)
            .short("q")
            .long(ARG_QUALITY_HINT)
            .takes_value(true)
            .possible_values(&QUALITY_LEVELS)
            .help("Activates a quality preset that will overwrite any render-params that are not explicitly set in the scene config"))
        .arg(Arg::with_name(ARG_QUALITY_OVERRIDE)
            .short("Q")
            .long(ARG_QUALITY_OVERRIDE)
            .takes_value(true)
            .possible_values(&QUALITY_LEVELS)
            .conflicts_with_all(&[ARG_QUALITY_HINT, ARG_RENDERPARAMS])
            .help("Activates a quality preset that will overwrite any render-params"))
        .arg(Arg::with_name(ARG_WIDTH)
            .short("w")
            .long(ARG_WIDTH)
            .takes_value(true)
            .help("Width of the output picture. If only height is supplied, this value is calculated from the camera aspect ratio"))
        .arg(Arg::with_name(ARG_HEIGHT)
            .short("h")
            .long(ARG_HEIGHT)
            .takes_value(true)
            .help("Height of the output picture. If only width is supplied, this value is calculated from the camera aspect ratio"))
        .arg(Arg::with_name(ARG_SCENE)
            .required(true)
            .help("A scene configuration file in the TOML format"))
        .arg(Arg::with_name(ARG_OUTPUT)
            .help("Path the the PNG output file"))
        .get_matches();

    let config = rays::parse(cla.value_of(ARG_SCENE).unwrap())
        .expect("Error parsing config. Please fix and try again.");

    let camera = extract_camera(cla.value_of(ARG_CAMERA), config.camera_config);

    let render_params = extract_render_params(
        cla.value_of(ARG_RENDERPARAMS), 
        cla.value_of(ARG_QUALITY_HINT),
        cla.value_of(ARG_QUALITY_OVERRIDE),
        config.render_params_config);

    let (width, height) = extract_rt_dimensions(cla.value_of(ARG_WIDTH), cla.value_of(ARG_HEIGHT), camera.viewport.aspect());

    let mut render_target = RenderTarget::new(width, height); 

    let before = Instant::now();

    rays::render(&config.scene, &camera, &mut render_target, &render_params);

    let elapsed = before.elapsed();
    println!("Finished in {}.{} s", elapsed.as_secs(), elapsed.subsec_millis());

    let output_path = if let Some(path) = cla.value_of(ARG_OUTPUT) {
        path.to_owned()
    } else {
        std::path::Path::new(cla.value_of(ARG_SCENE).unwrap()).file_stem().unwrap().to_str().unwrap().to_owned() + ".png"
    };

    render_target.save_as_png(&output_path).expect(&format!("Could not write to output file ({})", &output_path));
}

fn extract_rt_dimensions(w_input: Option<&str>, h_input: Option<&str>, aspect: f64) -> (usize, usize) {

    let w = w_input.map(|w| {
        let w = w.parse::<usize>().expect("Could not parse width as positive integer");

        if w == 0 {
            panic!("Width must be larger than 0")
        }

        w
    });

    let h = h_input.map(|h| {
        let h = h.parse::<usize>().expect("Could not parse height as positive integer");

        if h == 0 {
            panic!("Height must be larger than 0")
        }

        h
    });

    if let Some(w) = w {
        if let Some(h) = h {
            (w, h)
        } else {
            (w, (w as f64 / aspect).round().max(1.0) as usize)
        }
    } else {
        if let Some(h) = h {
            ((h as f64 * aspect).round().max(1.0) as usize, h)
        } else {
            (1280, 720)
        }
    }
}

fn extract_camera(cla_cam: Option<&str>, cam_cfg: CameraConfig) -> Camera {

    if let Some(cam_name) = cla_cam {

        if let CameraConfig::Multiple(cam_map) = cam_cfg {
            cam_map.get(cam_name)
                .expect("Provided camera name did not correspond to any camera name in scene config")
                .clone()
        } else {
            panic!("Not allowed to provide camera name when there are less than 2 cameras in the scene config")
        }

    } else {

        if let CameraConfig::Single(camera) = cam_cfg {
            camera
        } else {
            panic!("You must provide a camera name argument when there are multiple cameras in the scene config")
        }
    }
}

fn extract_render_params(
    cla_rp: Option<&str>, 
    cla_quality_hint: Option<&str>,
    cla_quality_override: Option<&str>,
    rp_cfg: RenderParamsConfig) -> RenderParams {

    // If we have a quality override, we can immediately return
    if let Some(quality_override) = cla_quality_override {
        return get_quality_from_cla(quality_override)
    }

    // Otherwise, we set a baseline quality by evaluating the quality hint
    let baseline_quality = if let Some(quality_hint) = cla_quality_hint {
        get_quality_from_cla(quality_hint)
    } else {
        RenderParams::preset_medium()
    };

    if let Some(rp_name) = cla_rp {
        
        if let RenderParamsConfig::Multiple(rp_map) = rp_cfg {
            let rp = rp_map.get(rp_name)
                .expect("Provided render-params name did not correspond to any such struct in the scene config");

            baseline_quality.override_with(rp)
        } else {
            panic!("Not allowed to provide a render-params struct name argument when there are less than 2 such structs in the scene config")
        }
    } else {
        if let RenderParamsConfig::Single(rp) = rp_cfg {
            baseline_quality.override_with(&rp)
        } else {
            panic!("You must provide a render-parameter struct name argument when there are multiple such structs in the scene config")
        }
    }
}

fn get_quality_from_cla(quality_name: &str) -> RenderParams {
    match quality_name {
        "sketch" => RenderParams::preset_sketch(),
        "low" => RenderParams::preset_low(),
        "medium" => RenderParams::preset_medium(),
        "high" => RenderParams::preset_high(),
        "ultra" => RenderParams::preset_ultra(),
        _ => unreachable!() // Unreachable because clap should catch illegal variants early
    }
}