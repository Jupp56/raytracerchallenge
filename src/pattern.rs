//! Patterns on objects
use core::fmt::Debug;

#[cfg(not(feature = "rayon"))]
use std::rc::Rc;

#[cfg(feature = "rayon")]
use std::sync::Arc;

use crate::{
    color::Color,
    epsilon::EPSILON,
    matrix::{Mat4, IDENTITY_MATRIX_4},
    shapes::shape::Shape,
    tuple::Point,
};

#[cfg(not(feature = "rayon"))]
/// A function to apply a pattern onto an object. Takes a point (in object space) and returns the color at that point.
pub type PatternFunction = Rc<dyn Fn(Point) -> Color>;

#[cfg(feature = "rayon")]
/// A function to apply a pattern onto an object. Takes a point (in object space) and returns the color at that point.
pub type PatternFunction = Arc<dyn Fn(Point) -> Color + Send + Sync>;

#[derive(Clone)]
/// A pattern to apply to an object.
pub struct Pattern {
    /// The [`PatternFunction`] that converts the point into a color
    pub pattern_fn: PatternFunction,
    transformation_matrix: Mat4,
    inverse_transformation_matrix: Mat4,
}

impl Pattern {
    /// Creates a new pattern with a user-defined pattern function.
    pub fn new(pattern_fn: PatternFunction, transformation_matrix: Mat4) -> Self {
        Self {
            pattern_fn,
            transformation_matrix,
            inverse_transformation_matrix: transformation_matrix.inverse(),
        }
    }

    /// Sets this object's transformation matrix which is used to scale, rotate,... the pattern on the object itself
    pub fn set_transformation_matrix(&mut self, matrix: Mat4) {
        self.transformation_matrix = matrix;
        self.inverse_transformation_matrix = matrix.inverse();
    }

    /// Renders pattern but using world space coordinates
    pub fn apply_pattern_world_space(&self, object: &dyn Shape, point: Point) -> Color {
        let point_object_space = object.inverse_transformation_matrix() * point;
        let point_pattern_space = self.inverse_transformation_matrix * point_object_space;
        (self.pattern_fn)(point_pattern_space)
    }
}

impl From<PatternFunction> for Pattern {
    fn from(pattern_fn: PatternFunction) -> Self {
        Self {
            pattern_fn,
            transformation_matrix: IDENTITY_MATRIX_4,
            inverse_transformation_matrix: IDENTITY_MATRIX_4,
        }
    }
}

/// Built-in patterns
impl Pattern {
    /// Creates a new stripe pattern
    pub fn stripe(color_a: Color, color_b: Color) -> Self {
        let pattern_fn = move |point| stripe_at(color_a, color_b, &point);

        #[cfg(not(feature = "rayon"))]
        let pattern_fn: PatternFunction = Rc::new(pattern_fn);
        #[cfg(feature = "rayon")]
        let pattern_fn: PatternFunction = Arc::new(pattern_fn);

        pattern_fn.into()
    }

    /// Creates a new gradient pattern
    pub fn gradient(color_a: Color, color_b: Color) -> Self {
        let pattern_fn = move |point| gradient_at(color_a, color_b, &point);

        #[cfg(not(feature = "rayon"))]
        let pattern_fn: PatternFunction = Rc::new(pattern_fn);
        #[cfg(feature = "rayon")]
        let pattern_fn: PatternFunction = Arc::new(pattern_fn);

        pattern_fn.into()
    }

    /// Creates a new ring pattern
    pub fn ring(color_a: Color, color_b: Color) -> Self {
        let pattern_fn = move |point| ring_at(color_a, color_b, &point);

        #[cfg(not(feature = "rayon"))]
        let pattern_fn: PatternFunction = Rc::new(pattern_fn);
        #[cfg(feature = "rayon")]
        let pattern_fn: PatternFunction = Arc::new(pattern_fn);

        pattern_fn.into()
    }
    /// Creates a new ring pattern
    pub fn checker(color_a: Color, color_b: Color) -> Self {
        let pattern_fn = move |point| checker_at(color_a, color_b, &point);

        #[cfg(not(feature = "rayon"))]
        let pattern_fn: PatternFunction = Rc::new(pattern_fn);
        #[cfg(feature = "rayon")]
        let pattern_fn: PatternFunction = Arc::new(pattern_fn);

        pattern_fn.into()
    }

    /// test pattern that returns the point hit as color. x -> red, y -> green, z -> blue
    pub fn test_pattern() -> Self {
        let pattern_fn = move |point| test_at(&point);

        #[cfg(not(feature = "rayon"))]
        let pattern_fn: PatternFunction = Rc::new(pattern_fn);
        #[cfg(feature = "rayon")]
        let pattern_fn: PatternFunction = Arc::new(pattern_fn);

        pattern_fn.into()
    }
}

/// Returns the result of the stripe pattern at a given coordinate in pattern space
fn stripe_at(color_a: Color, color_b: Color, point: &Point) -> Color {
    match (point.x.floor() % 2.0).abs() < EPSILON {
        true => color_a,
        false => color_b,
    }
}

/// Returns the result of the stripe pattern at a given coordinate in pattern space
fn gradient_at(color_a: Color, color_b: Color, point: &Point) -> Color {
    let distance = color_b - color_a;
    let mut fraction = point.x - point.x.floor();
    if (point.x.floor() % 2.0).abs() > EPSILON {
        fraction = 1.0 - fraction;
    }
    color_a + distance * fraction
}

fn ring_at(color_a: Color, color_b: Color, point: &Point) -> Color {
    let squared = point.x.powi(2) + point.z.powi(2);
    let unsquared = squared.sqrt();
    let floored = unsquared.floor();
    let is_mod = floored % 2.0;
    if is_mod.abs() < EPSILON {
        color_a
    } else {
        color_b
    }
}

/// Checker pattern function
fn checker_at(color_a: Color, color_b: Color, point: &Point) -> Color {
    let combined_magnitude = point.x.floor() + point.y.floor() + point.z.floor();
    if combined_magnitude.abs() % 2.0 < EPSILON {
        color_a
    } else {
        color_b
    }
}

/// Test function, converts the point into a color.
fn test_at(point: &Point) -> Color {
    Color::new(point.x, point.y, point.z)
}

impl Debug for Pattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Pattern")
            .field("transformation_matrix", &self.transformation_matrix)
            .field(
                "inverse_transformation_matrix",
                &self.inverse_transformation_matrix,
            )
            .finish()
    }
}

impl PartialEq for Pattern {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::addr_of!(self.pattern_fn) == std::ptr::addr_of!(other.pattern_fn)
            && self.transformation_matrix == other.transformation_matrix
            && self.inverse_transformation_matrix == other.inverse_transformation_matrix
    }
}

#[cfg(test)]
mod pattern_tests {
    use std::rc::Rc;

    use crate::{
        color::{Color, BLACK, WHITE},
        matrix::{Mat4, IDENTITY_MATRIX_4},
        pattern::Pattern,
        shapes::shape::Shape,
        shapes::sphere::Sphere,
        tuple::Point,
    };
    #[test]
    fn object_transformation() {
        let mut object = Sphere::default();
        object.set_transformation_matrix(Mat4::new_scaling(2, 2, 2));
        let pattern = Pattern::stripe(WHITE, BLACK);
        let c = pattern.apply_pattern_world_space(&object, Point::new(1.5, 0., 0.));
        assert_eq!(c, WHITE);
    }

    #[test]
    fn pattern_transformation() {
        let object = Sphere::default();
        let mut pattern = Pattern::stripe(WHITE, BLACK);
        pattern.set_transformation_matrix(Mat4::new_scaling(2, 2, 2));
        let c = pattern.apply_pattern_world_space(&object, Point::new(1.5, 0., 0.));
        assert_eq!(c, WHITE);
    }

    #[test]
    fn pattern_and_object_transformation() {
        let mut object = Sphere::default();
        object.set_transformation_matrix(Mat4::new_scaling(2, 2, 2));
        let mut pattern = Pattern::stripe(WHITE, BLACK);
        pattern.set_transformation_matrix(Mat4::new_translation(0.5, 0., 0.));
        let c = pattern.apply_pattern_world_space(&object, Point::new(2.5, 0., 0.));
        assert_eq!(c, WHITE);
    }

    #[test]
    fn partial_eq() {
        let p = Pattern::stripe(WHITE, BLACK);
        assert_eq!(p, p);
        let p2 = Pattern::stripe(BLACK, WHITE);
        assert_ne!(p, p2);
        let p3 = Pattern::new(Rc::new(|_p| WHITE), IDENTITY_MATRIX_4);
        assert_eq!(p3, p3);
        assert_ne!(p, p3);
    }

    fn test_xyz_pattern() -> Pattern {
        Pattern::new(Rc::new(|p| Color::new(p.x, p.y, p.z)), IDENTITY_MATRIX_4)
    }

    #[test]
    fn pattern_with_object_transform() {
        let mut shape = Sphere::default();
        shape.set_transformation_matrix(Mat4::new_scaling(2, 2, 2));
        let pattern = test_xyz_pattern();
        let c = pattern.apply_pattern_world_space(&shape, Point::new(2, 3, 4));
        assert_eq!(c, Color::new(1., 1.5, 2.));
    }

    #[test]
    fn pattern_with_pattern_transform() {
        let shape = Sphere::default();
        let mut pattern = test_xyz_pattern();
        pattern.set_transformation_matrix(Mat4::new_scaling(2, 2, 2));
        let c = pattern.apply_pattern_world_space(&shape, Point::new(2, 3, 4));
        assert_eq!(c, Color::new(1.0, 1.5, 2.0));
    }
    #[test]
    fn pattern_with_pattern_and_object_transform() {
        let mut shape = Sphere::default();
        shape.set_transformation_matrix(Mat4::new_scaling(2, 2, 2));
        let mut pattern = test_xyz_pattern();
        pattern.set_transformation_matrix(Mat4::new_translation(0.5, 1.0, 1.5));
        let c = pattern.apply_pattern_world_space(&shape, Point::new(2.5, 3.0, 3.5));
        assert_eq!(c, Color::new(0.75, 0.5, 0.25));
    }
}

#[cfg(test)]
mod stripe_tests {
    use crate::{
        color::{BLACK, WHITE},
        pattern::Pattern,
        tuple::Point,
    };

    #[test]
    fn stripe_constant_in_y() {
        let pattern = Pattern::stripe(WHITE, BLACK);
        assert_eq!((pattern.pattern_fn)(Point::new(0, 0, 0)), WHITE);
        assert_eq!((pattern.pattern_fn)(Point::new(0, 1, 0)), WHITE);
        assert_eq!((pattern.pattern_fn)(Point::new(0, 2, 0)), WHITE);
    }

    #[test]
    fn stripe_constant_in_z() {
        let pattern = Pattern::stripe(WHITE, BLACK);
        assert_eq!((pattern.pattern_fn)(Point::new(0, 0, 0)), WHITE);
        assert_eq!((pattern.pattern_fn)(Point::new(0, 0, 1)), WHITE);
        assert_eq!((pattern.pattern_fn)(Point::new(0, 0, 2)), WHITE);
    }

    #[test]
    fn stripe_alternates_in_x() {
        let pattern = Pattern::stripe(WHITE, BLACK);
        assert_eq!((pattern.pattern_fn)(Point::new(0, 0, 0)), WHITE);
        assert_eq!((pattern.pattern_fn)(Point::new(0.9, 0, 0)), WHITE);
        assert_eq!((pattern.pattern_fn)(Point::new(1, 0, 0)), BLACK);
        assert_eq!((pattern.pattern_fn)(Point::new(-0.1, 0, 0)), BLACK);
        assert_eq!((pattern.pattern_fn)(Point::new(-1, 0, 0)), BLACK);
        assert_eq!((pattern.pattern_fn)(Point::new(-1.1, 0, 2)), WHITE);
    }
}

#[cfg(test)]
mod gradient_tests {
    use crate::{
        color::{Color, BLACK, WHITE},
        pattern::gradient_at,
        tuple::Point,
    };

    use super::Pattern;

    #[test]
    fn gradient_function_linear_interpolation() {
        let color = gradient_at(WHITE, BLACK, &Point::new(0, 0, 0));
        assert_eq!(color, WHITE);
        let color = gradient_at(WHITE, BLACK, &Point::new(0.25, 0, 0));
        assert_eq!(color, Color::new(0.75, 0.75, 0.75));
        let color = gradient_at(WHITE, BLACK, &Point::new(0.5, 0, 0));
        assert_eq!(color, Color::new(0.5, 0.5, 0.5));
        let color = gradient_at(WHITE, BLACK, &Point::new(0.75, 0, 0));
        assert_eq!(color, Color::new(0.25, 0.25, 0.25));
        let color = gradient_at(WHITE, BLACK, &Point::new(1, 0, 0));
        assert_eq!(color, BLACK);
        let color = gradient_at(WHITE, BLACK, &Point::new(1.25, 0, 0));
        assert_eq!(color, Color::new(0.25, 0.25, 0.25));

        let color = gradient_at(WHITE, BLACK, &Point::new(1.5, 0, 0));
        assert_eq!(color, Color::new(0.5, 0.5, 0.5));
        let color = gradient_at(WHITE, BLACK, &Point::new(1.75, 0, 0));
        assert_eq!(color, Color::new(0.75, 0.75, 0.75));
    }

    #[test]
    fn gradient_linear_interpolation() {
        let pattern = Pattern::gradient(WHITE, BLACK);
        assert_eq!((pattern.pattern_fn)(Point::new(0, 0, 0)), WHITE);
    }
}

#[cfg(test)]
mod ring_tests {
    use crate::{
        color::{BLACK, WHITE},
        pattern::{ring_at, Pattern},
        tuple::Point,
    };

    #[test]
    fn ring_function() {
        let color = ring_at(WHITE, BLACK, &Point::new(0, 0, 0));
        assert_eq!(color, WHITE);
        let color = ring_at(WHITE, BLACK, &Point::new(0.2, 0, 0.2));
        assert_eq!(color, WHITE);
        let color = ring_at(WHITE, BLACK, &Point::new(1, 0, 0));
        assert_eq!(color, BLACK);
        let color = ring_at(WHITE, BLACK, &Point::new(0, 0, 1));
        assert_eq!(color, BLACK);
        let color = ring_at(WHITE, BLACK, &Point::new(0.708, 0., 0.708));
        assert_eq!(color, BLACK);
    }
    #[test]
    fn ring() {
        let pattern = Pattern::ring(WHITE, BLACK);
        let color = (pattern.pattern_fn)(Point::new(0, 0, 0));
        assert_eq!(color, WHITE);
        let color = (pattern.pattern_fn)(Point::new(1, 0, 0));
        assert_eq!(color, BLACK);
    }
}

#[cfg(test)]
mod checkers_tests {
    use crate::{
        color::{BLACK, WHITE},
        pattern::checker_at,
        tuple::Point,
    };

    #[test]
    fn checker_function_repeats_in_x() {
        assert_eq!(checker_at(WHITE, BLACK, &Point::new(0, 0, 0)), WHITE);
        assert_eq!(checker_at(WHITE, BLACK, &Point::new(0.99, 0.0, 0.0)), WHITE);
        assert_eq!(checker_at(WHITE, BLACK, &Point::new(1.01, 0, 0)), BLACK);
        assert_eq!(
            checker_at(WHITE, BLACK, &Point::new(-0.01, 0.0, 0.0)),
            BLACK
        );
        assert_eq!(
            checker_at(WHITE, BLACK, &Point::new(-0.99, 0.0, 0.0)),
            BLACK
        );
        assert_eq!(checker_at(WHITE, BLACK, &Point::new(-1.01, 0, 0)), WHITE);
    }

    #[test]
    fn checker_function_repeats_in_y() {
        assert_eq!(checker_at(WHITE, BLACK, &Point::new(0, 0, 0)), WHITE);
        assert_eq!(checker_at(WHITE, BLACK, &Point::new(0, 0.99, 0)), WHITE);
        assert_eq!(checker_at(WHITE, BLACK, &Point::new(0, 1.01, 0)), BLACK);
    }

    #[test]
    fn checker_function_repeats_in_z() {
        assert_eq!(checker_at(WHITE, BLACK, &Point::new(0, 0, 0)), WHITE);
        assert_eq!(checker_at(WHITE, BLACK, &Point::new(0, 0, 0.99)), WHITE);
        assert_eq!(checker_at(WHITE, BLACK, &Point::new(0, 0, 1.01)), BLACK);
    }
}
