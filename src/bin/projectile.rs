use std::fs::File;
use std::io::Write;

use raytracerchallenge::canvas::Canvas;
use raytracerchallenge::color::Color;
use raytracerchallenge::ppm::write_to_ppm;
use raytracerchallenge::tuple::{Point, Vector};

#[derive(Debug)]
struct Projectile {
    position: Point,
    velocity: Vector,
}

#[derive(Debug)]
struct Environment {
    gravity: Vector,
    wind: Vector,
}

#[allow(dead_code)]
#[mutants::skip]
fn tick(environment: &Environment, projectile: &mut Projectile) {
    projectile.position = projectile.position + projectile.velocity;
    projectile.velocity = projectile.velocity + environment.gravity + environment.wind;
}

#[mutants::skip]
pub fn main() {
    let mut p = Projectile {
        position: Point::new(0., 1., 0.),
        velocity: Vector::new(1., 1.8, 0.).normalized() * 11.25,
    };
    let env = Environment {
        gravity: Vector::new(0., -0.1, 0.),
        wind: Vector::new(-0.01, 0., 0.),
    };

    println!("creating canvas");

    let mut canvas = Canvas::new(900, 550);

    println!("starting sim");

    while p.position.y > 0. {
        //println!("{:?}", p);
        tick(&env, &mut p);
        println!("x: {}, y: {}", p.position.x, p.position.y);
        let ypos = 550_usize.checked_sub(p.position.y as usize);
        if let Some(pos) = ypos {
            canvas
                .write_pixel(p.position.x as usize, pos, Color::new(1., 0., 0.))
                .unwrap();
        }
    }

    let ppm = write_to_ppm(canvas);

    let mut file = File::create("./test.ppm").unwrap();
    let _ = write!(file, "{}", ppm);
}
