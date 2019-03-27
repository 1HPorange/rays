extern crate rays;

use rays::prelude::*;
use std::time::Instant;
use clap::{App, Arg};

fn main() {

    const ARG_CAMERA: &str = "camera";
    const ARG_RENDERPARAMS: &str = "render-params";
    const ARG_WIDTH: &str = "width";
    const ARG_HEIGHT: &str = "height";
    const ARG_SCENE: &str = "SCENE";
    const ARG_OUTPUT: &str = "OUTPUT";

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
            .default_value("output.png")
            .help("Path the the PNG output file"))
        .get_matches();

    let config = rays::parse(cla.value_of(ARG_SCENE).unwrap())
        .expect("Error parsing config. Please fix and try again.");

    let camera = extract_camera(cla.value_of(ARG_CAMERA), config.camera_config);

    let render_params = extract_render_params(cla.value_of(ARG_RENDERPARAMS), config.render_params_config);

    let (width, height) = extract_rt_dimensions(cla.value_of(ARG_WIDTH), cla.value_of(ARG_HEIGHT), camera.viewport.aspect());

    let mut render_target = RenderTarget::new(width, height); 

    let before = Instant::now();

    rays::render(&config.scene, &camera, &mut render_target, &render_params);

    let elapsed = before.elapsed();
    println!("Finished in {}.{} s", elapsed.as_secs(), elapsed.subsec_millis());

    render_target.save_as_png(cla.value_of(ARG_OUTPUT).unwrap())
        .expect("Could not write to output file");
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

fn extract_render_params(cla_rp: Option<&str>, rp_cfg: RenderParamsConfig) -> RenderParams {

    if let Some(rp_name) = cla_rp {
        
        if let RenderParamsConfig::Multiple(rp_map) = rp_cfg {
            rp_map.get(rp_name)
                .expect("Provided render-params name did not correspond to any such struct in the scene config")
                .clone()
        } else {
            panic!("Not allowed to provide a render-params struct name argument when there are less than 2 such structs in the scene config")
        }
    } else {
        if let RenderParamsConfig::Single(rp) = rp_cfg {
            rp
        } else {
            panic!("You must provide a render-parameter struct name argument when there are multiple such structs in the scene config")
        }
    }
}