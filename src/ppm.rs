use crate::canvas::Canvas;

/// Creates a PPM file format string from the canvas that can then be written to a file.
pub fn write_to_ppm(canvas: Canvas) -> String {
    let mut header = format!("P3\n{} {}\n255", canvas.width(), canvas.height());
    let mut body = "\n".to_string();

    for y in 0..canvas.height() {
        let mut row = String::new();
        let mut len = 0;
        for x in 0..canvas.width() {
            let color = canvas
                .pixel_at(x, y)
                .expect("Canvas WIDTH and HEIGHT volation.");

            let red = format!("{} ", convert_color(color.red));
            let green = format!("{} ", convert_color(color.green));
            let blue = format!("{} ", convert_color(color.blue));

            len += red.chars().count();
            if len > 70 {
                row.push('\n');
                len = red.chars().count();
            }
            row.push_str(&red);
            len += green.chars().count();
            if len > 70 {
                row.push('\n');
                len = green.chars().count();
            }
            row.push_str(&green);
            len += blue.chars().count();
            if len > 70 {
                row.push('\n');
                len = blue.chars().count();
            }
            row.push_str(&blue);
        }
        row.push('\n');
        body.push_str(&row);
    }

    header.push_str(&body);

    header
}

fn convert_color(color: f64) -> usize {
    if color > 1. {
        255
    } else if color <= 0. {
        0
    } else {
        (color * 255.).round() as usize
    }
}

#[cfg(test)]
mod ppm_tests {
    use crate::{
        canvas::Canvas,
        color::Color,
        ppm::{convert_color, write_to_ppm},
    };

    #[test]
    fn header() {
        let c = Canvas::new(5, 3);
        let ppm: String = write_to_ppm(c);
        let reference: String = "P3\n5 3\n255".to_string();
        assert!(ppm.contains(&reference));
    }

    #[test]
    fn simple_values() {
        let mut c = Canvas::new(5, 3);
        let c1 = Color::new(1.5, 0., 0.);
        let c2 = Color::new(0., 0.5, 0.);
        let c3 = Color::new(-0.5, 0., 1.);
        c.write_pixel(0, 0, c1).unwrap();
        c.write_pixel(2, 1, c2).unwrap();
        c.write_pixel(4, 2, c3).unwrap();
        let ppm: String = write_to_ppm(c);

        let reference = "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0 \n0 0 0 0 0 0 0 128 0 0 0 0 0 0 0 \n0 0 0 0 0 0 0 0 0 0 0 0 0 0 255 ";
        assert!(ppm.contains(reference));
    }

    #[test]
    fn newline_70_chars() {
        let color: Color = Color::new(1., 0.8, 0.6);
        let c = Canvas::new_with_color(10, 2, color);
        let ppm: String = write_to_ppm(c);
        let reference = "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204 \n153 255 204 153 255 204 153 255 204 153 255 204 153 \n255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204 \n153 255 204 153 255 204 153 255 204 153 255 204 153";
        assert!(ppm.contains(reference));
    }

    #[test]
    fn end_is_newline() {
        let color: Color = Color::new(1., 0.8, 0.6);
        let c = Canvas::new_with_color(10, 2, color);
        let ppm: String = write_to_ppm(c);
        assert!(ppm.ends_with('\n'));
    }

    #[test]
    fn convert_color_to_255() {
        assert_eq!(convert_color(1.), 255);
        assert_eq!(convert_color(2.4), 255);
        assert_eq!(convert_color(0.5), 128);
        assert_eq!(convert_color(-1.), 0);
        assert_eq!(convert_color(-0.5), 0);
        assert_eq!(convert_color(0.), 0);
    }
}
