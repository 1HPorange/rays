use rays::prelude::*;
use std::sync::Arc;

pub fn create_scene() -> Scene  {

    // Materials

    let mat_white_diffuse = Material::opaque_reflective(
        RGBColor::WHITE, 
        Reflection::new(1.0, 1.0, 1.0, 90.0));

    let mat_gray = Material::pure(
        RGBColor::new(0.5, 0.5, 0.5));

    let mat_orange = Material::pure(
        RGBColor::new(1.0, 0.6, 0.1));

    let mat_blue = Material::pure(
        RGBColor::new(0.25, 0.75, 1.0));

    // Objects

    let sphere = Sphere::new(
        Vec3::new(0.0, 7.5, 11.0),
        4.5,
        Arc::new(StaticUvMapper(mat_white_diffuse)));

    // Scenery (walls)

    let floor = InifinitePlane::new(
        Vec3::new(0.0, 0.0, 0.0), 
        Arc::new(StaticUvMapper(mat_white_diffuse)), 
        1.0);

    let ceiling = InifinitePlane::with_rotation(
        Vec3::new(0.0, 15.0, 0.0), 
        Vec3::new(180.0, 0.0, 0.0), 
        Arc::new(StaticUvMapper(mat_white_diffuse)), 
        1.0);

    let back_wall = InifinitePlane::with_rotation(
        Vec3::new(0.0, 0.0, 20.0), 
        Vec3::new(-90.0, 0.0, 0.0), 
        Arc::new(StaticUvMapper(mat_gray)), 
        1.0);

    let left_wall = InifinitePlane::with_rotation(
        Vec3::new(-15.0, 0.0, 0.0), 
        Vec3::new(90.0, 90.0, 0.0), 
        Arc::new(StaticUvMapper(mat_orange)), 
        1.0);

    let right_wall = InifinitePlane::with_rotation(
        Vec3::new(15.0, 0.0, 0.0), 
        Vec3::new(90.0, -90.0, 0.0), 
        Arc::new(StaticUvMapper(mat_blue)), 
        1.0);

    // Scene

    let mut scene = Scene::new(RGBColor::WHITE);

    scene.add(floor);
    scene.add(ceiling);
    scene.add(back_wall);
    scene.add(left_wall);
    scene.add(right_wall);

    scene.add(sphere);

    scene
}

pub fn create_camera() -> Camera {

    let mut cam = Camera::default();

    cam.position = Vec3::new(0.0, 7.5, -10.0);
    cam.fov_h = 40.0;

    cam
}

pub fn create_render_parameters() -> RenderParams {

    let mut rp = RenderParams::default();

    rp.ao.strength = 0.4;
    rp.ao.distance = 5.0;

    rp
}