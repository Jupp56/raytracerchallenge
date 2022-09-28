use std::io::Write;
use std::{f64::consts::PI, fs::File};

use raytracerchallenge::{
    canvas::Canvas, color::Color, matrix::Mat4, ppm::write_to_ppm, tuple::Point,
};

#[mutants::skip]
fn main() {
    let mut c = Canvas::new(100, 100);

    let mut points = Vec::new();
    for _i in 0..12 {
        points.push(Point::new(30, 0, 0));
    }

    for (i, point) in points.iter_mut().enumerate() {
        let rotation: f64 = i as f64 * PI / 6.;
        println!("rotation: {}", rotation);
        let rot_matrix = Mat4::new_rotation_z(rotation);
        *point = rot_matrix * *point;
    }

    for p in points {
        println!("writing at x {} y {}", p.x, p.y);
        c.write_pixel(
            (p.x + 50.) as usize,
            (p.y + 50.) as usize,
            Color::new(1, 1, 1),
        )
        .unwrap();
    }

    let ppm = write_to_ppm(c);

    let mut file = File::create("./clock.ppm").unwrap();
    let _ = write!(file, "{}", ppm);
}
