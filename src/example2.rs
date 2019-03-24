use rays::prelude::*;

type Precision = f64;

pub fn create_scene() -> Scene<Precision>  {

    // Materials

    let mat_white_diffuse = Material::opaque_reflective(
        RGBColor::WHITE, 
        ReflectionParams::new(1.0, 1.0, 1.0, 90.0));

    let mat_gray = Material::pure(
        RGBColor::new(0.5, 0.5, 0.5));

    let mat_orange = Material::pure(
        RGBColor::new(1.0, 0.6, 0.1));

    let mat_blue = Material::pure(
        RGBColor::new(0.25, 0.75, 1.0));

    // Objects

    //let sphere = // TODO: Interpolate reflection vector towards normal with growing angle!

    let sphere = Sphere::with_random_right(
        Vec3(0.0, 7.5, 11.0),
        4.5,
        StaticUvMapper(mat_white_diffuse),
        Vec3Norm::up());

    // Scenery (walls)

    let floor = InifinitePlane::new(
        Vec3(0.0, 0.0, 0.0), 
        Vec3Norm::up(), 
        Vec3Norm::right(),
        StaticUvMapper(mat_white_diffuse), 
        1.0);

    let ceiling = InifinitePlane::new(
        Vec3(0.0, 15.0, 0.0), 
        Vec3Norm::down(), 
        Vec3Norm::right(),
        StaticUvMapper(mat_white_diffuse), 
        1.0);

    let back_wall = InifinitePlane::new(
        Vec3(0.0, 0.0, 20.0), 
        Vec3Norm::back(), 
        Vec3Norm::right(),
        StaticUvMapper(mat_gray), 
        1.0);

    let left_wall = InifinitePlane::new(
        Vec3(-15.0, 0.0, 0.0), 
        Vec3Norm::right(), 
        Vec3Norm::forward(),
        StaticUvMapper(mat_orange), 
        1.0);

    let right_wall = InifinitePlane::new(
        Vec3(15.0, 0.0, 0.0), 
        Vec3Norm::left(), 
        Vec3Norm::forward(),
        StaticUvMapper(mat_blue), 
        1.0);


    // Scene

    let mut scene = Scene::new(RGBColor::WHITE);

    scene.add_object(floor);
    scene.add_object(ceiling);
    scene.add_object(back_wall);
    scene.add_object(left_wall);
    scene.add_object(right_wall);

    scene.add_object(sphere);

    scene
}

pub fn create_camera() -> Camera<Precision> {

    Camera::new(
        Vec3(0.0, 7.5, -10.0),
        Vec3::zero(),
        ViewPort { width: 16.0, height: 9.0 },
        40.0)
}

pub fn create_render_parameters() -> RenderParameters<Precision> {

    let mut rp = RenderParameters::default();

    rp.ao.strength = 0.4;
    rp.ao.distance = 5.0;

    rp
}