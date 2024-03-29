use std::f64::consts::PI;

use raytracerchallenge::material::{ColorType, Shininess};
use raytracerchallenge::shapes::plane::Plane;
use raytracerchallenge::shapes::shape::Shape;
use raytracerchallenge::{
    camera::Camera,
    color::{Color, WHITE},
    light::PointLight,
    material::Material,
    matrix::Mat4,
    shapes::sphere::Sphere,
    tuple::{Point, Vector},
    world::World,
};

#[mutants::skip]
fn main() {
    let mut floor = Plane::default();
    //floor.set_transformation_matrix(Mat4::new_scaling(10.0, 0.01, 10.0));

    floor.set_material(Material::default());
    floor.material_mut().color = ColorType::Color(Color::new(1.0, 0.9, 0.9));
    floor.material_mut().specular = 0.0;

    let mut left_wall = Sphere::default();
    left_wall.set_transformation_matrix(
        Mat4::new_translation(0, 0, 5)
            * Mat4::new_rotation_y(-PI / 4.0)
            * Mat4::new_rotation_x(PI / 2.0)
            * Mat4::new_scaling(10.0, 0.01, 10.0),
    );
    left_wall.set_material(floor.material().to_owned());

    let mut right_wall = Sphere::default();
    right_wall.set_transformation_matrix(
        Mat4::new_translation(0, 0, 5)
            * Mat4::new_rotation_y(PI / 4.0)
            * Mat4::new_rotation_x(PI / 2.0)
            * Mat4::new_scaling(10.0, 0.01, 10.0),
    );
    right_wall.set_material(floor.material().to_owned());

    let mut middle = Sphere::default();
    middle.set_transformation_matrix(Mat4::new_translation(-0.5, 1.0, 0.5));
    middle.set_material(Material::default());
    middle.material_mut().color = ColorType::Color(Color::new(0.1, 1.0, 0.5));
    middle.material_mut().diffuse = 0.7;
    middle.material_mut().specular = 0.3;

    let mut right = Sphere::default();
    right.set_transformation_matrix(
        Mat4::new_translation(1.5, 0.5, -0.5) * Mat4::new_scaling(0.5, 0.5, 0.5),
    );
    right.set_material(Material::default());
    right.material_mut().color = ColorType::Color(Color::new(0.1, 1.0, 0.5));
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

    let world_ref = &world;
    let _canvas = camera.render(world_ref, 0).unwrap();
}
