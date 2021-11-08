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
    /// The object's normal at a given point.
    fn normal_at(&self, p: Point) -> Vector;
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
    use crate::{matrix::{IDENTITY_MATRIX_4, Mat4}, ray::Ray, tuple::{Point, Vector}};

    use super::Shape;

    struct TestShape {
        transformation_matrix: Mat4,
    }

    impl TestShape {
        pub fn complex_matrix() -> Self {
            Self {
                transformation_matrix: Mat4::new([
                    [2., 1., 4., 5.],
                    [2.1, 4., 3.5, 6.7],
                    [2.3, 5.6, 8.7, 9.7],
                    [5.6, 9.8, 4.3, 9.7],
                ]),
            }
        }
    }

    impl Default for TestShape {
        fn default() -> Self {
            Self {
                transformation_matrix: IDENTITY_MATRIX_4
            }
        }
    }

    impl<'a> Shape<'a> for TestShape {
        fn local_intersect(
            &'a self,
            ray: crate::ray::Ray,
            intersections: &mut Vec<crate::intersection::Intersection<'a>>,
        ) {
            
        }

        fn material(&self) -> crate::material::Material {
            todo!()
        }

        fn transformation_matrix(&self) -> crate::matrix::Mat4 {
            self.transformation_matrix
        }

        fn normal_at(&self, p: crate::tuple::Point) -> crate::tuple::Vector {
            todo!()
        }
    }

#[test]
fn intersect() {
    let t = TestShape::default();
    let mut ins = Vec::new();
    let ray = Ray::new(Point::new(0, 0, 2), Vector::new(0, 0, -1));
    t.intersect(&ray,&mut ins);
    assert_eq!(ins.len(), 2);
}

    #[test]
    fn inverse_transformation_matrix() {
        let t = TestShape::complex_matrix();
        assert_eq!(t.inverse_transformation_matrix(), t.transformation_matrix.inverse());
    }


}
