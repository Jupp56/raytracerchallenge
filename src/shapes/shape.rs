use crate::{
    color::Color,
    intersection::{Intersection, PreparedComputations},
    light::PointLight,
    material::Material,
    matrix::Mat4,
    ray::Ray,
    tuple::{Point, Vector},
};

use std::{any::Any, fmt::Debug};

#[cfg(feature = "rayon")]
/// Trait dependencies for Shape - differ depending on rayon being active
pub trait ShapeBound: Any + Debug + Send + Sync {}

#[cfg(not(feature = "rayon"))]
/// Trait dependencies for Shape - differ depending on rayon being active
pub trait ShapeBound: Any + Debug {}

/// This trait encapsulates the shared behaviour of all objects in the world (not lights, though!).
///
/// If you want to add your own shape, implement this trait for it.
/// Most of the default methods take work from you (i.e. converting coordinates to object space).
/// It is heavily recommended though to override [`Self::inverse_transformation_matrix`] to cache the matrix somehow (maybe when setting the original matrix), as this hugely increases performance.
pub trait Shape: ShapeBound {
    /// The intersection of a ray with this shape.
    /// This method converts the coordinates of the ray to object space and then calls local_intersect for the concrete impelementation.
    /// You probably don't need to overwrite this.
    fn intersect<'a>(&'a self, ray: &Ray, intersections: &mut Vec<Intersection<'a>>) {
        let ray = ray.transformed(self.inverse_transformation_matrix());
        self.local_intersect(&ray, intersections);
    }
    /// This method transforms a ray to object space.
    /// You probably don't need to overwrite this.
    fn transform_ray_to_object_space(&self, ray: &Ray) -> Ray {
        ray.transformed(self.inverse_transformation_matrix())
    }
    /// Implement your intersection logic here!
    fn local_intersect<'a>(&'a self, ray: &Ray, intersections: &mut Vec<Intersection<'a>>);
    /// Returns the material of this shape.
    fn material(&self) -> &Material;
    /// Returns a mutable handle to the material of this shape.
    fn material_mut(&mut self) -> &mut Material;
    /// Replaces this shape's material with the provided one.
    fn set_material(&mut self, m: Material);

    /// Returns the transformation matrix of the shape.
    fn transformation_matrix(&self) -> Mat4;
    /// The inverted transformation matrix. Exists and can be overridden to cache the inverted matrix
    fn inverse_transformation_matrix(&self) -> Mat4 {
        self.transformation_matrix().inverse()
    }
    /// The transposed inverted transformation matrix. Can be overridden to cache the matrix
    fn inverse_of_transpose_of_transformation_matrix(&self) -> Mat4 {
        self.inverse_transformation_matrix().transpose()
    }
    /// Sets a new transformation matrix for this shape.
    fn set_transformation_matrix(&mut self, matrix: Mat4);
    /// The object's normal at a given point (world space).
    fn normal_at(&self, p: Point) -> Vector {
        let local_point = self.inverse_transformation_matrix() * p;
        let local_normal = self.local_normal_at(local_point);
        let world_normal = self.inverse_of_transpose_of_transformation_matrix() * local_normal;
        world_normal.normalized()
    }
    /// Returns the normal at a given point (in object space)
    fn local_normal_at(&self, p: Point) -> Vector;
    /// Converts a point to object space.
    fn to_object_space(&self, p: Point) -> Point {
        self.inverse_transformation_matrix() * p
    }
    /// Converts a point back to world space.
    fn to_world_space(&self, p: Point) -> Point {
        self.inverse_of_transpose_of_transformation_matrix() * p
    }
    /// Renders the color a ray sees at a given position.
    /// Ambient determines whether to include ambient lighting (not included for every light source)
    fn render_at(
        &self,
        comps: &PreparedComputations,
        light: &PointLight,
        in_shadow: bool,
        ambient: bool,
    ) -> Color {
        let shape: &dyn Shape = self.as_shape();
        self.material().lighting(
            light,
            shape,
            comps.over_point,
            comps.eyev,
            comps.normalv,
            in_shadow,
            ambient,
        )
    }
    /// Compares this shape to any other one.
    ///
    /// Needed to implement PartialEq for all shapes.
    fn eq(&self, other: &dyn Any) -> bool;
    /// Converts this to any, used to implement PartialEq.
    ///
    /// Solves a similar problem to the [`Shape::as_shape()`] method.
    fn as_any(&self) -> &dyn Any;
    /// Creates a [`Self`] out of a trait implementor.
    ///
    /// Every concrete type needs to implement this,
    /// because otherwise a trait method cannot access the object as &dyn Trait.
    /// That is because the &self parameter of a trait function isn't [`Sized`]},
    /// so you cannot cast it to &dyn Trait.
    fn as_shape(&self) -> &dyn Shape;
}

impl PartialEq for dyn Shape {
    fn eq(&self, other: &dyn Shape) -> bool {
        self.eq(other.as_any())
    }
}

#[cfg(test)]
mod shape_tests {
    use std::f64::consts::PI;

    use crate::{
        material::Material,
        matrix::{Mat4, IDENTITY_MATRIX_4},
        ray::Ray,
        tuple::{Point, Vector},
    };

    use super::{Shape, ShapeBound};

    static mut SAVED_RAY: Option<Ray> = None;

    #[derive(Copy, Clone, Debug)]
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

    impl ShapeBound for TestShape {}

    impl Shape for TestShape {
        fn local_intersect<'a>(
            &'a self,
            ray: &crate::ray::Ray,
            _intersections: &mut Vec<crate::intersection::Intersection<'a>>,
        ) {
            unsafe {
                SAVED_RAY = Some(*ray);
            }
        }

        fn material(&self) -> &crate::material::Material {
            unimplemented!()
        }

        fn transformation_matrix(&self) -> crate::matrix::Mat4 {
            self.transformation_matrix
        }

        fn local_normal_at(&self, p: Point) -> Vector {
            Vector::new(p.x, p.y, p.z)
        }

        fn eq(&self, _other: &dyn std::any::Any) -> bool {
            unimplemented!()
        }

        fn as_any(&self) -> &dyn std::any::Any {
            unimplemented!()
        }

        fn material_mut(&mut self) -> &mut crate::material::Material {
            unimplemented!()
        }

        fn set_material(&mut self, _m: Material) {
            unimplemented!()
        }

        fn set_transformation_matrix(&mut self, _matrix: Mat4) {
            unimplemented!()
        }

        fn as_shape(&self) -> &dyn Shape {
            todo!()
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
