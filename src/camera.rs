use crate::{
    canvas::{Canvas, CanvasError},
    matrix::{Mat4, IDENTITY_MATRIX_4},
    ray::Ray,
    tuple::{Point, Vector},
    world::World,
};

#[cfg(feature = "rayon")]
use rayon::prelude::*;

#[derive(Copy, Clone, Debug)]
pub struct Camera {
    pub hsize: usize,
    pub vsize: usize,
    pub field_of_view: f64,
    transform: Mat4,
    inverted_transform: Mat4,
    pub pixel_size: f64,
    half_width: f64,
    half_height: f64,
}

impl<'shape: 'intersection, 'intersection> Camera {
    pub fn new(hsize: usize, vsize: usize, field_of_view: f64) -> Self {
        let half_view = (field_of_view / 2.0).tan();
        let aspect = hsize as f64 / vsize as f64;

        let (half_width, half_height) = if aspect >= 1.0 {
            (half_view, half_view / aspect)
        } else {
            (half_view * aspect, half_view)
        };

        let pixel_size = (half_width * 2.0) / hsize as f64;

        Self {
            hsize,
            vsize,
            field_of_view,
            transform: IDENTITY_MATRIX_4,
            inverted_transform: IDENTITY_MATRIX_4,
            pixel_size,
            half_width,
            half_height,
        }
    }

    pub fn transform(&self) -> Mat4 {
        self.transform
    }

    pub fn set_transform(&mut self, transform: Mat4) {
        self.transform = transform;
        self.inverted_transform = transform.inverse();
    }

    fn ray_for_pixel(&self, px: usize, py: usize) -> Ray {
        let x_offset = (px as f64 + 0.5) * self.pixel_size;
        let y_offset = (py as f64 + 0.5) * self.pixel_size;

        let world_x = self.half_width - x_offset;
        let world_y = self.half_height - y_offset;

        let pixel = self.inverted_transform * Point::new(world_x, world_y, -1.);
        let origin = self.inverted_transform * Point::new(0, 0, 0);
        let direction = (pixel - origin).normalized();

        Ray::new(origin, direction)
    }

    pub fn view_transform(from: Point, to: Point, mut up: Vector) -> Mat4 {
        let forward = (to - from).normalized();
        up.normalize();
        let left = forward.cross(up);

        let true_up = left.cross(forward);
        let orientation = Mat4::new([
            [left.x, left.y, left.z, 0.0],
            [true_up.x, true_up.y, true_up.z, 0.0],
            [-forward.x, -forward.y, -forward.z, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        let translation = Mat4::new_translation(-from.x, -from.y, -from.z);
        orientation * translation
    }

    /// renders the given world using this camera.
    pub fn render(&self, world: &World) -> Result<Canvas, CanvasError> {
        let mut image = Canvas::new(self.hsize, self.vsize);

        let mut intersections = Vec::new();

        for y in 0..self.vsize {
            for x in 0..self.hsize {
                let ray = self.ray_for_pixel(x, y);
                let color = world.color_at(&ray, &mut intersections);
                image.write_pixel(x, y, color)?;
            }
        }

        Ok(image)
    }

    /// Same as ```render()```, but uses all available system threads to parallelize.
    #[cfg(feature = "rayon")]
    pub fn par_render(&self, world: &World) -> Result<Canvas, CanvasError> {
        let mut rows = Vec::with_capacity(self.vsize);
        (0..(self.vsize))
            .into_par_iter()
            .map(|y| self.render_row(world, y))
            .collect_into_vec(&mut rows);
        let mut canvas = Canvas::new(self.hsize, self.vsize);
        for (row, rowv) in rows.iter().enumerate() {
            for (col, color) in rowv.iter().enumerate() {
                canvas.write_pixel(col, row, *color)?;
            }
        }
        Ok(canvas)
    }

    #[cfg(feature = "rayon")]
    fn render_row(&self, world: &World, y: usize) -> Vec<crate::color::Color> {
        let mut vec = Vec::with_capacity(self.hsize);
        for x in 0..self.hsize {
            let ray = self.ray_for_pixel(x, y);
            let color = world.color_at(&ray);
            vec.push(color);
        }
        vec
    }
}

#[cfg(test)]
mod view_transformation_tests {
    use crate::{
        camera::Camera,
        matrix::{Mat4, IDENTITY_MATRIX_4},
        tuple::{Point, Vector},
    };

    #[test]
    fn default_matrix() {
        let from = Point::new(0, 0, 0);
        let to = Point::new(0, 0, -1);
        let up = Vector::new(0, 1, 0);
        let t = Camera::view_transform(from, to, up);
        assert_eq!(t, IDENTITY_MATRIX_4);
    }

    #[test]
    fn positive_z() {
        let from = Point::new(0, 0, 0);
        let to = Point::new(0, 0, 1);
        let up = Vector::new(0, 1, 0);
        let t = Camera::view_transform(from, to, up);
        assert_eq!(t, Mat4::new_scaling(-1, 1, -1));
    }

    #[test]
    fn moves_world() {
        let from = Point::new(0, 0, 8);
        let to = Point::new(0, 0, 0);
        let up = Vector::new(0, 1, 0);
        let t = Camera::view_transform(from, to, up);
        assert_eq!(t, Mat4::new_translation(0, 0, -8));
    }

    #[test]
    fn arbitrary_transformation() {
        let from = Point::new(1, 3, 2);
        let to = Point::new(4, -2, 8);
        let up = Vector::new(1, 1, 0);
        let t = Camera::view_transform(from, to, up);
        assert_eq!(
            t,
            Mat4::new([
                [-0.50709, 0.50709, 0.67612, -2.36643],
                [0.76772, 0.60609, 0.12122, -2.82843],
                [-0.35857, 0.59761, -0.71714, 0.00000],
                [0.00000, 0.00000, 0.00000, 1.00000]
            ])
        );
    }
}

#[cfg(test)]
mod camera_tests {
    use std::f64::consts::PI;

    use crate::{
        camera::Camera,
        color::Color,
        epsilon::epsilon_equal,
        matrix::{Mat4, IDENTITY_MATRIX_4},
        tuple::{Point, Vector},
        world::World,
    };

    #[test]
    fn new() {
        let c = Camera::new(160, 120, PI / 2.);
        assert_eq!(c.hsize, 160);
        assert_eq!(c.vsize, 120);
        assert_eq!(c.field_of_view, PI / 2.);
        assert_eq!(c.transform, IDENTITY_MATRIX_4);
        assert_eq!(c.inverted_transform, IDENTITY_MATRIX_4);
    }

    #[test]
    fn pixel_size_horizontal() {
        let c = Camera::new(200, 125, PI / 2.);
        assert!(epsilon_equal(c.pixel_size, 0.01));
    }

    #[test]
    fn pixel_size_vertical() {
        let c = Camera::new(125, 200, PI / 2.);
        assert!(epsilon_equal(c.pixel_size, 0.01));
    }

    #[test]
    fn ray_through_center() {
        let c = Camera::new(201, 101, PI / 2.);
        let r = c.ray_for_pixel(100, 50);
        assert_eq!(r.origin, Point::new(0, 0, 0));
        assert_eq!(r.direction, Vector::new(0, 0, -1));
    }

    #[test]
    fn ray_through_corner() {
        let c = Camera::new(201, 101, PI / 2.);
        let r = c.ray_for_pixel(0, 0);
        assert_eq!(r.origin, Point::new(0, 0, 0));
        assert_eq!(r.direction, Vector::new(0.66519, 0.33259, -0.66851));
    }
    #[test]
    fn ray_camera_transformed() {
        let mut c = Camera::new(201, 101, PI / 2.);
        c.set_transform(Mat4::new_rotation_y(PI / 4.) * Mat4::new_translation(0, -2, 5));
        let r = c.ray_for_pixel(100, 50);
        assert_eq!(r.origin, Point::new(0, 2, -5));
        assert_eq!(
            r.direction,
            Vector::new(2.0_f64.sqrt() / 2.0, 0.0, -(2.0_f64.sqrt() / 2.0))
        )
    }

    #[test]
    fn render() {
        let w = World::test_world();
        let mut c = Camera::new(11, 11, PI / 2.);
        let from = Point::new(0, 0, -5);
        let to = Point::new(0, 0, 0);
        let up = Vector::new(0, 1, 0);
        c.set_transform(Camera::view_transform(from, to, up));
        let image = c.render(&w).unwrap();
        assert_eq!(
            image.pixel_at(5, 5).unwrap(),
            Color::new(0.38066, 0.47583, 0.2855)
        );
    }
}

#[cfg(test)]
#[cfg(feature = "rayon")]
mod par_tests {
    use std::f64::consts::PI;

    use crate::{
        camera::Camera,
        color::Color,
        tuple::{Point, Vector},
        world::World,
    };

    #[test]
    fn render_par() {
        let w = World::test_world();
        let mut c = Camera::new(11, 11, PI / 2.);
        let from = Point::new(0, 0, -5);
        let to = Point::new(0, 0, 0);
        let up = Vector::new(0, 1, 0);
        c.set_transform(Camera::view_transform(from, to, up));
        let image = c.par_render(&w).unwrap();
        assert_eq!(
            image.pixel_at(5, 5).unwrap(),
            Color::new(0.38066, 0.47583, 0.2855)
        );
    }
}
