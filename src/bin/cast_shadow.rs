use std::f64::consts::PI;
use std::fs::File;
use std::io::Write;

use raytracerchallenge::canvas::Canvas;
use raytracerchallenge::color::Color;
use raytracerchallenge::intersection::{hit, Intersect};
use raytracerchallenge::ppm::write_to_ppm;
use raytracerchallenge::ray::Ray;
use raytracerchallenge::shapes::Sphere;
use raytracerchallenge::tuple::{Point, Vector};

fn main() {
    let canvas = cast();

    let ppm = write_to_ppm(canvas);

    let mut file = File::create("./cast_japan.ppm").unwrap();
    let _ = write!(file, "{}", ppm);
}

pub fn cast() -> Canvas {
    let mut c = Canvas::new_with_color(1000, 1000, Color::new(1.0, 1.0, 1.0));
    let start_point = Point::new(0, 0, -5);
    let sphere = Sphere::new();
    //let transform = Mat4::new_scaling(1.0, 0.5, 1.0);
    //sphere.set_transformation(transform);
    for i in 0_usize..1000_usize {
        for j in 0_usize..1000_usize {
            let direction = Vector::new(
                PI / 4. * (i as f64 / 1000.) - PI / 8.,
                PI / 4. * (j as f64 / 1000.) - PI / 8.,
                1.0,
            );
            let ray = Ray::new(start_point, direction);
            let mut intersections = Vec::new();
            sphere.intersect(&ray, &mut intersections);
            match hit(intersections) {
                Some(_intersection) => {
                    c.write_pixel(i, j, Color::new(1.0, 0.0, 0.0)).unwrap();
                    //println!("hit!")
                }
                None => (),
            };
        }
    }
    c
}
