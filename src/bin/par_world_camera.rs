use std::io::Write;
use std::time::Instant;
use std::{f64::consts::PI, fs::File};

use raytracerchallenge::material::Shininess;
use raytracerchallenge::{
    camera::Camera,
    color::{Color, WHITE},
    light::PointLight,
    material::Material,
    matrix::Mat4,
    ppm::write_to_ppm,
    shapes::sphere::Sphere,
    tuple::{Point, Vector},
    world::World,
};
fn main() {
    let mut floor = Sphere::default();
    floor.set_transformation_matrix(Mat4::new_scaling(10.0, 0.01, 10.0));

    floor.set_material(Material::default());
    floor.material.color = Color::new(1.0, 0.9, 0.9);
    floor.material.specular = 0.0;

    let mut left_wall = Sphere::default();
    left_wall.set_transformation_matrix(
        Mat4::new_translation(0, 0, 5)
            * Mat4::new_rotation_y(-PI / 4.0)
            * Mat4::new_rotation_x(PI / 2.0)
            * Mat4::new_scaling(10.0, 0.01, 10.0),
    );
    left_wall.set_material(floor.material());

    let mut right_wall = Sphere::default();
    right_wall.set_transformation_matrix(
        Mat4::new_translation(0, 0, 5)
            * Mat4::new_rotation_y(PI / 4.0)
            * Mat4::new_rotation_x(PI / 2.0)
            * Mat4::new_scaling(10.0, 0.01, 10.0),
    );
    right_wall.set_material(floor.material());

    let mut middle = Sphere::default();
    middle.set_transformation_matrix(Mat4::new_translation(-0.5, 1.0, 0.5));
    middle.set_material(Material::default());
    middle.material.color = Color::new(0.1, 1.0, 0.5);
    middle.material.diffuse = 0.7;
    middle.material.specular = 0.3;

    let mut right = Sphere::default();
    right.set_transformation_matrix(
        Mat4::new_translation(1.5, 0.5, -0.5) * Mat4::new_scaling(0.5, 0.5, 0.5),
    );
    right.set_material(Material::default());
    right.material.color = Color::new(0.1, 1.0, 0.5);
    right.material.diffuse = 0.7;
    right.material.specular = 0.3;

    let mut left = Sphere::default();
    left.set_transformation_matrix(
        Mat4::new_translation(-1.5, 0.33, -0.75) * Mat4::new_scaling(0.33, 0.33, 0.33),
    );
    left.set_material(Material::default());
    left.material.color = Color::new(1.0, 0.8, 0.1);
    left.material.diffuse = 0.7;
    left.material.specular = 0.3;
    left.material.shininess = 200 as Shininess;

    let mut world = World::default();

    world.add_objects(&mut vec![
        Box::new(floor),
        Box::new(left_wall),
        Box::new(right_wall),
        Box::new(middle),
        Box::new(right),
        Box::new(left),
    ]);

    let light = PointLight::new(Point::new(-10, 10, -10), WHITE);
    let light2 = PointLight::new(Point::new(10, 5, -10), Color::new(0.2, 0.2, 0.2));

    world.add_light(light);
    world.add_light(light2);

    let mut camera = Camera::new(1920, 1080, PI / 3.0);

    camera.set_transform(Camera::view_transform(
        Point::new(0.0, 1.5, -5.0),
        Point::new(0, 1, 0),
        Vector::new(0, 1, 0),
    ));

    let start_time = Instant::now();
    let world_ref = &world;
    let canvas = camera.par_render(world_ref).unwrap();

    let end_time = start_time.elapsed().as_millis();

    println!(
        "Rendered image with {} objects at {} x {} (={}) pixels in {} milliseconds.",
        world.objects().len(),
        camera.hsize,
        camera.vsize,
        camera.hsize * camera.vsize,
        end_time
    );

    let ppm = write_to_ppm(canvas);

    let mut file = File::create("./shadows-par.ppm").unwrap();
    let _ = write!(file, "{}", ppm);
}
