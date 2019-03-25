use rays::prelude::*;

pub fn create_scene() -> Scene {

    // Materials

    let mat_white = Material::opaque_reflective(
        RGBColor::WHITE, 
        Reflection::new(-0.1, 0.8, 1.0, 20.0));

    let mat_black = Material::opaque_reflective(
        RGBColor::BLACK, 
        Reflection::new(0.35, 0.9, 1.0, 20.0));

    let mat_very_reflective = Material::opaque_reflective(
        RGBColor::WHITE,
        Reflection::new(0.75, 1.0, 4.0, 0.0));

    let mat_glass = Material::new(
        RGBColor::WHITE, 
        Opacity::new(0.05, 1.0, 2.0), 
        Reflection::new(1.0, 1.0, 1.0, 0.0), 
        Refraction::new(1.33, 0.0));

    let mat_refract_blurry = Material::new(
        RGBColor::WHITE, 
        Opacity::new(0.1, 0.75, 3.0), 
        Reflection::new(0.5, 0.5, 1.0, 0.0), 
        Refraction::new(1.0, 6.0));

    let mat_coloured_diffuse = Material::opaque_reflective(
        RGBColor::new(0.45, 0.3, 0.45), 
        Reflection::new(0.15, 0.6, 3.0, 8.0));

    let mat_marble = Material::opaque_reflective(
        RGBColor::PINK, // will be overwritten by uv mapper
        Reflection::new(0.05, 0.8, 3.0, 0.0));

    // Textured UV Mappers

    let skysphere_mapper = TextureUvMapper::from_png_24(
        "D:/Downloads/skysphere4.png", 
        Material::pure(RGBColor::BLACK), 
        SamplingMethod::BILINEAR)
        .expect("Could not load sky texture!");

    let marble_mapper = TextureUvMapper::from_png_24(
        "D:/Downloads/skysphere3.png", 
        mat_marble,
        SamplingMethod::BILINEAR)
        .expect("Could not load marble texture!");

    // Objects

    let back_left = Sphere::new(
        Vec3::new(-8.0, 7.0, 20.0), 7.0, 
        StaticUvMapper(mat_glass), 
        Vec3Norm::UP, 
        Vec3Norm::RIGHT);

    let back_right = Sphere::new(
        Vec3::new(8.0, 7.0, 20.0), 7.0, 
        StaticUvMapper(mat_very_reflective), 
        Vec3Norm::UP, 
        Vec3Norm::RIGHT);

    let front_left = Sphere::new(
        Vec3::new(-12.0, 4.0, 7.5), 
        4.0, 
        marble_mapper, 
        Vec3Norm::UP, 
        Vec3Norm::RIGHT);

    let front_center = Sphere::new(
        Vec3::new(0.0, 4.5, 5.0), 
        4.5, 
        StaticUvMapper(mat_coloured_diffuse), 
        Vec3Norm::UP, 
        Vec3Norm::RIGHT);

    let front_right = Sphere::new(
        Vec3::new(12.0, 4.0, 7.5), 
        4.0, 
        StaticUvMapper(mat_refract_blurry), 
        Vec3Norm::UP, 
        Vec3Norm::RIGHT);

    // Scenery

    let sky_sphere = Sphere::new(
        Vec3::ZERO, 
        1000.0, 
        skysphere_mapper, 
        Vec3Norm::UP, 
        Vec3Norm::RIGHT);

    let floor = InifinitePlane::new(
        Vec3::new(0.0, 0.0, 0.0), 
        Vec3Norm::UP, 
        Vec3Norm::RIGHT,
        CheckerboardUvMapper(mat_black, mat_white), 
        0.1);

    // Scene

    let mut scene = Scene::new(RGBColor::WHITE);

    scene.add(floor);
    scene.add(sky_sphere);

    scene.add(back_left);
    scene.add(back_right);
    scene.add(front_left);
    scene.add(front_center);
    scene.add(front_right);

    scene
}

pub fn create_camera() -> Camera {

    let mut cam = Camera::default();

    cam.position.y = 15.0;
    cam.orientation = Vec3::new(25.0, 0.0, 0.0);

    cam
}

pub fn create_render_parameters() -> RenderParams {

    RenderParams::default()
}