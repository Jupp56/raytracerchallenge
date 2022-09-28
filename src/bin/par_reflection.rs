use std::io::Write;
use std::time::Instant;
use std::{f64::consts::PI, fs::File};

use raytracerchallenge::color::BLACK;
use raytracerchallenge::material::{ColorType, Shininess};
use raytracerchallenge::pattern::Pattern;
use raytracerchallenge::shapes::plane::Plane;
use raytracerchallenge::shapes::shape::Shape;
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

#[mutants::skip]
fn main() {
    let mut floor = Plane::default();
    floor.set_transformation_matrix(Mat4::new_scaling(1, 1, 1));

    floor.set_material(Material::default());
    floor.material_mut().color = ColorType::Pattern(Pattern::checker(WHITE, BLACK));
    floor.material_mut().specular = 0.0;
    floor.material_mut().reflective = 0.2;

    let mut left_wall = Plane::default();
    left_wall.set_transformation_matrix(
        Mat4::new_translation(0, 0, 5)
            * Mat4::new_rotation_y(-PI / 4.0)
            * Mat4::new_rotation_x(PI / 2.0),
    );
    left_wall.set_material(Material::default());
    let pattern = Pattern::ring(WHITE, Color::new(0.8, 0.8, 0.0));
    //pattern.set_transformation_matrix(Mat4::new_scaling(0.1, 0.1, 0.1));
    left_wall.material_mut().color = ColorType::Pattern(pattern);

    let mut right_wall = Plane::default();
    right_wall.set_transformation_matrix(
        Mat4::new_translation(0, 0, 5)
            * Mat4::new_rotation_y(PI / 4.0)
            * Mat4::new_rotation_x(PI / 2.0),
    );
    right_wall.set_material(floor.material().clone());
    right_wall.material_mut().color = ColorType::Pattern(Pattern::stripe(
        Color::new(0.0, 0.8, 0.2),
        Color::new(0.6, 0.8, 1.0),
    ));

    let mut middle = Sphere::default();
    middle.set_transformation_matrix(Mat4::new_translation(-0.5, 1.0, 0.5));
    middle.set_material(Material::default());
    middle.material_mut().color = ColorType::Pattern(Pattern::stripe(WHITE, BLACK));
    middle.material_mut().diffuse = 0.7;
    middle.material_mut().specular = 0.3;

    let mut right = Sphere::default();
    right.set_transformation_matrix(
        Mat4::new_translation(1.5, 0.5, -0.5)
            * Mat4::new_scaling(0.5, 0.5, 0.5)
            * Mat4::new_rotation_y(PI / 4.0),
    );
    right.set_material(Material::default());
    right.material_mut().color = ColorType::Pattern(Pattern::gradient(
        Color::new(0.4, 1.0, 0.5),
        Color::new(0.6, 0.3, 0.2),
    ));
    right.material_mut().diffuse = 0.7;
    right.material_mut().specular = 0.3;

    let mut left = Sphere::default();
    left.set_transformation_matrix(
        Mat4::new_translation(-1.5, 0.33, -0.75) * Mat4::new_scaling(0.33, 0.33, 0.33),
    );
    left.set_material(Material::default());
    left.material_mut().color = ColorType::Color(Color::new(1.0, 0.8, 0.1));
    left.material_mut().diffuse = 0.7;
    left.material_mut().specular = 0.3;
    left.material_mut().shininess = 200 as Shininess;

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
    let canvas = camera.par_render(world_ref, 5).unwrap();

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

    let mut file = File::create("./reflective.ppm").unwrap();
    let _ = write!(file, "{}", ppm);
}
