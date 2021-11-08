use crate::{
    color::Color,
    intersection::{Intersection, PreparedComputations},
    light::PointLight,
    material::Material,
    matrix::Mat4,
    ray::Ray,
    tuple::{Point, Vector},
};

pub trait Shape<'a> {
    fn intersect(&'a self, ray: &'a Ray, intersections: &mut Vec<Intersection<'a>>) {
        let ray = ray.transformed(self.inverse_transformation_matrix());
        self.local_intersect(ray, intersections);
    }
    fn transform_ray_to_object_space(&self, ray: &Ray) -> Ray {
        ray.transformed(self.inverse_transformation_matrix())
    }
    fn local_intersect(&'a self, ray: Ray, intersections: &mut Vec<Intersection<'a>>);
    fn material(&self) -> Material;

    fn transformation_matrix(&self) -> Mat4;
    /// The inverted transformation matrix. Exists and can be overridden to cache the inverted matrix
    fn inverse_transformation_matrix(&self) -> Mat4 {
        self.transformation_matrix().inverse()
    }
    /// The transposed inverted transformation matrix. Can be overridden to cache the matrix
    fn inverse_of_transpose_of_transformation_matrix(&self) -> Mat4 {
        self.inverse_transformation_matrix().transpose()
    }
    /// The object's normal at a given point (world space).
    fn normal_at(&self, p: Point) -> Vector {
        let local_point = self.inverse_transformation_matrix() * p;
        let local_normal = self.local_normal_at(local_point);
        let world_normal = self.inverse_of_transpose_of_transformation_matrix() * local_normal;
        world_normal.normalized()
    }
    fn local_normal_at(&self, p: Point) -> Vector;
    /// converts a point to object space
    fn to_object_space(&self, p: Point) -> Point {
        self.inverse_transformation_matrix() * p
    }
    /// converts a point back to world space
    fn to_world_space(&self, p: Point) -> Point {
        self.inverse_of_transpose_of_transformation_matrix() * p
    }
    /// renders the color a ray sees at a given position
    fn render_at(
        &self,
        comps: &PreparedComputations,
        light: &PointLight,
        in_shadow: bool,
    ) -> Color {
        self.material().lighting(
            light,
            comps.over_point,
            comps.eyev,
            comps.normalv,
            in_shadow,
        )
    }
}

#[cfg(test)]
mod shape_tests {
    use std::f64::consts::PI;

    use crate::{
        matrix::{Mat4, IDENTITY_MATRIX_4},
        ray::Ray,
        tuple::{Point, Vector},
    };

    use super::Shape;

    static mut SAVED_RAY: Option<Ray> = None;

    struct TestShape {
        transformation_matrix: Mat4,
    }

    impl TestShape {
        fn complex_matrix() -> Self {
            Self {
                transformation_matrix: Mat4::new([
                    [2., 1., 4., 5.],
                    [2.1, 4., 3.5, 6.7],
                    [2.3, 5.6, 8.7, 9.7],
                    [5.6, 9.8, 4.3, 9.7],
                ]),
            }
        }

        fn set_transform(&mut self, transform: Mat4) {
            self.transformation_matrix = transform;
        }
    }

    impl Default for TestShape {
        fn default() -> Self {
            Self {
                transformation_matrix: IDENTITY_MATRIX_4,
            }
        }
    }

    impl<'a> Shape<'a> for TestShape {
        fn local_intersect(
            &'a self,
            ray: crate::ray::Ray,
            _intersections: &mut Vec<crate::intersection::Intersection<'a>>,
        ) {
            unsafe {
                SAVED_RAY = Some(ray);
            }
        }

        fn material(&self) -> crate::material::Material {
            unimplemented!()
        }

        fn transformation_matrix(&self) -> crate::matrix::Mat4 {
            self.transformation_matrix
        }

        fn local_normal_at(&self, p: Point) -> Vector {
            Vector::new(p.x, p.y, p.z)
        }
    }

    #[test]
    fn inverse_transformation_matrix() {
        let t = TestShape::complex_matrix();
        assert_eq!(
            t.inverse_transformation_matrix(),
            t.transformation_matrix.inverse()
        );
    }

    #[test]
    fn intersect_scaled_shape_with_ray() {
        let r = Ray::new(Point::new(0, 0, -5), Vector::new(0, 0, 1));
        let mut s = TestShape::default();
        s.set_transform(Mat4::new_scaling(2, 2, 2));
        let mut intersections = Vec::new();
        let _xs = s.intersect(&r, &mut intersections);
        unsafe {
            assert_eq!(SAVED_RAY.unwrap().origin, Point::new(0.0, 0.0, -2.5));
            assert_eq!(SAVED_RAY.unwrap().direction, Vector::new(0., 0., 0.5));
        }
    }
    #[test]
    fn intersect_translated_shape_with_ray() {
        let r = Ray::new(Point::new(0, 0, -5), Vector::new(0, 0, 1));
        let mut s = TestShape::default();
        s.set_transform(Mat4::new_translation(5, 0, 0));
        let mut intersections = Vec::new();
        let _xs = s.intersect(&r, &mut intersections);
        unsafe {
            assert_eq!(SAVED_RAY.unwrap().origin, Point::new(-5, 0, -5));
            assert_eq!(SAVED_RAY.unwrap().direction, Vector::new(0, 0, 1));
        }
    }

    #[test]
    fn test_normal_translated() {
        let mut s = TestShape::default();
        s.set_transform(Mat4::new_translation(0, 1, 0));
        let n = s.normal_at(Point::new(0.0, 1.70711, -0.70711));
        assert_eq!(n, Vector::new(0.0, 0.70711, -0.70711));
    }
    #[test]
    fn test_normal_transformed() {
        let mut s = TestShape::default();
        let m = Mat4::new_scaling(1.0, 0.5, 1.0) * Mat4::new_rotation_z(PI / 5.);
        s.set_transform(m);
        let n = s.normal_at(Point::new(
            0.0,
            2.0_f64.sqrt() / 2.0,
            -(2.0_f64.sqrt() / 2.0),
        ));
        assert_eq!(n, Vector::new(0.0, 0.97014, -0.24254));
    }
}
