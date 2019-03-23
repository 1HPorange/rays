use rays::prelude::*;

pub fn create_scene() -> Scene<f64>  {

    // Materials

    let mat_white_diffuse = Material::opaque_reflective(
        RGBColor::WHITE, 
        ReflectionParams::new(0.3, 0.3, 1.0, 90.0));

    let mat_green_diffuse = Material::opaque_reflective(
        RGBColor::new(0.0, 1.0, 0.0), 
        ReflectionParams::new(0.3, 0.3, 1.0, 90.0));

    // Objects



    // Scenery (walls)

    let floor = InifinitePlane::new(
        Vec3(0.0, 0.0, 0.0), 
        Vec3Norm::up(), 
        Vec3Norm::right(),
        StaticUvMapper(mat_white_diffuse), 
        1.0);

    let back_wall = InifinitePlane::new(
        Vec3(0.0, 0.0, 20.0), 
        Vec3Norm::back(), 
        Vec3Norm::right(),
        StaticUvMapper(mat_green_diffuse), 
        1.0);

    // Scene

    let mut scene = Scene::new(RGBColor::WHITE);

    scene.add_object(floor);
    scene.add_object(back_wall);

    scene
}

pub fn create_camera() -> Camera<f64> {

    Camera::new(
        Vec3(0.0, 15.0, -10.0),
        Vec3(25.0, 0.0, 0.0),
        ViewPort { width: 16.0, height: 9.0 },
        60.0)
}

pub fn create_render_parameters() -> RenderParameters<f64> {

    RenderParameters::default()

}