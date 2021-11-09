use std::f64::consts::PI;
use std::fs::File;
use std::io::Write;

use raytracerchallenge::canvas::Canvas;
use raytracerchallenge::color::Color;
use raytracerchallenge::intersection::hit;
use raytracerchallenge::light::PointLight;
use raytracerchallenge::material::Material;
use raytracerchallenge::matrix::Mat4;
use raytracerchallenge::ppm::write_to_ppm;
use raytracerchallenge::ray::Ray;
use raytracerchallenge::shapes::shape::Shape;
use raytracerchallenge::shapes::sphere::Sphere;
use raytracerchallenge::tuple::{Point, Vector};

fn main() {
    let canvas = cast();

    let ppm = write_to_ppm(canvas);

    let mut file = File::create("./shaded-2.ppm").unwrap();
    let _ = write!(file, "{}", ppm);
}

pub fn cast() -> Canvas {
    let resolution: (usize, usize) = (1000, 1000);

    let mut c = Canvas::new_with_color(resolution.0, resolution.1, Color::new(0.0, 0.0, 0.0));
    let start_point = Point::new(0, 0, -5);
    let mut sphere = Sphere::default();
    sphere.material = Material::default();
    sphere.material.color = Color::new(0.2, 0.6, 0.2);
    sphere.material.shininess = 70;

    let light_position = Point::new(-10, 10, -10);
    let light_color = Color::new(1, 1, 1);
    let light = PointLight::new(light_position, light_color);

    let transform = Mat4::new_scaling(1.0, 0.2, 1.0);
    sphere.set_transformation(transform);
    for i in 0_usize..resolution.0 {
        for j in 0_usize..resolution.1 {
            let mut direction = Vector::new(
                PI / 4. * (i as f64 / resolution.0 as f64) - PI / 8.,
                PI / 4. * (j as f64 / resolution.1 as f64) - PI / 8.,
                1.0,
            );
            direction.normalize();
            let ray = Ray::new(start_point, direction);
            let mut intersections = Vec::new();
            sphere.intersect(&ray, &mut intersections);

            if let Some(intersection) = hit(intersections) {
                let object  = intersection.object.as_any().downcast_ref::<Sphere>().unwrap();
                let point = ray.position(intersection.t);
                let normal = object.normal_at(point);

                let eye = -ray.direction;

                let color = object.material.lighting(&light, point, eye, normal, false);

                c.write_pixel(i, resolution.1 - j, color).unwrap();
            }
        }
    }
    c
}
