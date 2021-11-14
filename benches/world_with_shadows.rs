use criterion::{black_box, criterion_group, criterion_main, Criterion};

use std::{f64::consts::PI, time::Duration};

use raytracerchallenge::{
    camera::Camera,
    color::{Color, WHITE},
    light::PointLight,
    material::Material,
    matrix::Mat4,
    shapes::{shape::Shape, sphere::Sphere},
    tuple::{Point, Vector},
    world::World,
};

fn setup_world() -> World {
    let mut floor = Sphere::default();
    floor.set_transformation_matrix(Mat4::new_scaling(10.0, 0.01, 10.0));

    floor.set_material(Material::default());
    floor.material_mut().color = Color::new(1.0, 0.9, 0.9);
    floor.material_mut().specular = 0.0;

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
    middle.material_mut().color = Color::new(0.1, 1.0, 0.5);
    middle.material_mut().diffuse = 0.7;
    middle.material_mut().specular = 0.3;

    let mut right = Sphere::default();
    right.set_transformation_matrix(
        Mat4::new_translation(1.5, 0.5, -0.5) * Mat4::new_scaling(0.5, 0.5, 0.5),
    );
    right.set_material(Material::default());
    right.material_mut().color = Color::new(0.1, 1.0, 0.5);
    right.material_mut().diffuse = 0.7;
    right.material_mut().specular = 0.3;

    let mut left = Sphere::default();
    left.set_transformation_matrix(
        Mat4::new_translation(-1.5, 0.33, -0.75) * Mat4::new_scaling(0.33, 0.33, 0.33),
    );
    left.set_material(Material::default());
    left.material_mut().color = Color::new(1.0, 0.8, 0.1);
    left.material_mut().diffuse = 0.7;
    left.material_mut().specular = 0.3;

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
    let camera = setup_camera();

    let mut group = c.benchmark_group("render_complete");
    //group.sample_size(20);
    group.warm_up_time(Duration::from_secs(5));
    //group.measurement_time(Duration::from_secs(22));

    group.bench_function("world", |b| {
        b.iter(|| world_bench(black_box(setup_world()), black_box(camera.clone())))
    });

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
