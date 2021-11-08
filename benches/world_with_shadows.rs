use criterion::{black_box, criterion_group, criterion_main, Criterion};

use std::f64::consts::PI;

use raytracerchallenge::{
    camera::Camera,
    color::{Color, WHITE},
    light::PointLight,
    material::Material,
    matrix::Mat4,
    object::Object,
    shapes::sphere::Sphere,
    tuple::{Point, Vector},
    world::World,
};

fn setup_world() -> World {
    let mut floor = Sphere::default();
    floor.set_transformation(Mat4::new_scaling(10.0, 0.01, 10.0));

    floor.material = Material::default();
    floor.material.color = Color::new(1.0, 0.9, 0.9);
    floor.material.specular = 0.0;

    let mut left_wall = Sphere::default();
    left_wall.set_transformation(
        Mat4::new_translation(0, 0, 5)
            * Mat4::new_rotation_y(-PI / 4.0)
            * Mat4::new_rotation_x(PI / 2.0)
            * Mat4::new_scaling(10.0, 0.01, 10.0),
    );
    left_wall.material = floor.material;

    let mut right_wall = Sphere::default();
    right_wall.set_transformation(
        Mat4::new_translation(0, 0, 5)
            * Mat4::new_rotation_y(PI / 4.0)
            * Mat4::new_rotation_x(PI / 2.0)
            * Mat4::new_scaling(10.0, 0.01, 10.0),
    );
    right_wall.material = floor.material;

    let mut middle = Sphere::default();
    middle.set_transformation(Mat4::new_translation(-0.5, 1.0, 0.5));
    middle.material = Material::default();
    middle.material.color = Color::new(0.1, 1.0, 0.5);
    middle.material.diffuse = 0.7;
    middle.material.specular = 0.3;

    let mut right = Sphere::default();
    right.set_transformation(
        Mat4::new_translation(1.5, 0.5, -0.5) * Mat4::new_scaling(0.5, 0.5, 0.5),
    );
    right.material = Material::default();
    right.material.color = Color::new(0.1, 1.0, 0.5);
    right.material.diffuse = 0.7;
    right.material.specular = 0.3;

    let mut left = Sphere::default();
    left.set_transformation(
        Mat4::new_translation(-1.5, 0.33, -0.75) * Mat4::new_scaling(0.33, 0.33, 0.33),
    );
    left.material = Material::default();
    left.material.color = Color::new(1.0, 0.8, 0.1);
    left.material.diffuse = 0.7;
    left.material.specular = 0.3;

    let mut world = World::default();

    world.add_objects(&mut vec![
        Object::Sphere(floor),
        Object::Sphere(left_wall),
        Object::Sphere(right_wall),
        Object::Sphere(middle),
        Object::Sphere(right),
        Object::Sphere(left),
    ]);

    let light = PointLight::new(Point::new(-10, 10, -10), WHITE);
    let light2 = PointLight::new(Point::new(10, 5, -10), Color::new(0.2, 0.2, 0.2));

    world.add_light(light);
    world.add_light(light2);

    world
}

fn setup_camera() -> Camera {
    let mut camera = Camera::new(1000, 500, PI / 3.0);

    camera.set_transform(Camera::view_transform(
        Point::new(0.0, 1.5, -5.0),
        Point::new(0, 1, 0),
        Vector::new(0, 1, 0),
    ));

    camera
}

fn world_bench(world: World, camera: Camera) {
    let _canvas = camera.render(&world).unwrap();
}

fn criterion_benchmark(c: &mut Criterion) {
    let world = setup_world();
    let camera = setup_camera();

    c.bench_function("world", |b| {
        b.iter(|| world_bench(black_box(world.clone()), black_box(camera.clone())))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
